use std::collections::HashSet;
use std::result;

use super::storage::ScraperGenerator;
use super::storage::StorageOptions;
use html_parser::{Dom, Element};

pub struct ScraperYAMLGenerator<'a> {
    tags: ScraperGenerator<'a>,
    index: usize,
    order: Vec<String>,
    first: bool,
    indent: usize,
}

impl<'a> ScraperYAMLGenerator<'a> {
    pub fn new(data: &'a Vec<String>, options: &'a StorageOptions) -> Self {
        Self {
            tags: ScraperGenerator::new(data, options),
            index: 0,
            order: vec![],
            first: true,
            indent: 0,
        }
    }
    fn first_gen(&mut self){
        let mut yaml_order: HashSet<String> = HashSet::new();
        if !self.tags.options.include_tag_content.unwrap_or(false){
            self.order.push("text".to_string());
            return;
        }
        if self.tags.options.include_tag_names.unwrap_or(true){
            self.order.push("tag".to_string());
        }
        if let Some(attributes) = &self.tags.options.include_attributes{
            for attribute in attributes{
                self.order.push(attribute.to_string());
            }
        }
        else {
            for tag in self.tags.data{
                let tag = Dom::parse(&tag).unwrap().children[0].element().unwrap().clone();
                if !tag.id.is_none(){
                    yaml_order.insert("id".to_string());
                }
                if tag.classes.len() > 0{
                    yaml_order.insert("class".to_string());
                }
                for (key, _) in tag.attributes.iter(){
                    yaml_order.insert(key.to_string());
                }
            }
        }
        for key in yaml_order{
            self.order.push(key);
        }
        self.order.push("text".to_string());
    }
    
    fn pretify_str(&self, val:&str)->String{
        let mut result = "\n".to_string();
        for _ in 0..self.indent{
            result.push_str("  ");
        }
        result.push_str(val);
        result
    }
    fn pretify_string(&self, val:&String)->String{
        let mut result = "\n".to_string();
        for _ in 0..self.indent{
            result.push_str("  ");
        }
        result.push_str(val);
        result
    }

    fn reset(&mut self){
        self.index = 0;
        self.order.clear();
        self.first = true;
        self.indent = 0;
    }

    fn handle_html_tagname_extract(&self, tag: &Element, yaml_row: &mut String, header: &String){
        let tagname = &tag.name;
        yaml_row.push_str(&format!("- tag: {}", tagname));
    }
    fn handle_html_classes_extract(&mut self, tag: &Element, yaml_row: &mut String, header: &String){
        if tag.classes.len() == 0{
            yaml_row.clear();
            return;
        }
        yaml_row.push_str("- class: ");
        self.indent +=1;
        let classes = tag.classes
            .iter()
            .fold("".to_string()
                ,|acc, e| {
                    let mut result = acc.clone();
                    result.push_str(
                    &self.pretify_string(&format!("- {}", e)).as_str() 
                );
                result
            }
            );
        self.indent -=1;
        yaml_row.push_str(&classes);
    }
    fn handle_html_id_extract(&self, tag: &Element, yaml_row: &mut String, header: &String){
        let Some(id) = &tag.id else{
            yaml_row.clear();
            return;
        };
        yaml_row.push_str(&format!("- id: {}", id).as_str());
    }
    fn handle_html_text_extract(&self, tag: &Element, yaml_row: &mut String, header: &String){
        let text = tag.children
            .iter()
            .map(|node| node.text().unwrap_or(""))
            .collect::<Vec<&str>>()
            .join(" ");
        if !text.chars().any(|cha| cha.is_alphanumeric()){
            yaml_row.clear();
            return;
        }
        yaml_row.push_str(
                &format!(
                    r#"- text: {}"#,
                    text
                ).as_str())
    }
    fn handle_extract_attribute(&self, tag: &Element, yaml_row: &mut String, header: &String, default: &str){
        let Some(attribute_value) = tag.attributes.get(default) else {
            yaml_row.clear();
            return;
        };

        yaml_row.push_str(
            &format!(r#"- {}: {}"#, header, attribute_value.clone().unwrap_or("null".to_string())).as_str(),
        )
    }
}

impl<'a> Iterator for ScraperYAMLGenerator<'a> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        let mut result = String::new();
        if self.first{
            self.first = false;
            self.first_gen();
        }
        if self.index < self.tags.data.len(){
            let tag = Dom::parse(&self.tags.data[self.index]).unwrap().children[0].element().unwrap().clone();
            if self.index > 0 {
                result.push_str("\n");
            }
            self.index += 1;
            result.push_str("data:");
            self.indent += 1;
            let order = self.order.clone();
            for header in order.iter(){
                let mut yaml_append = String::new();
                match header.as_str() {
                    "tag" => self.handle_html_tagname_extract(&tag, &mut yaml_append, header),
                    "class" => self.handle_html_classes_extract(&tag, &mut yaml_append, header),
                    "id" => self.handle_html_id_extract(&tag, &mut yaml_append, header),
                    "text" => self.handle_html_text_extract(&tag, &mut yaml_append, header),
                    default => self.handle_extract_attribute(&tag, &mut yaml_append, header, default),
                }
                if yaml_append.is_empty(){
                    continue;
                }
                result.push_str(&self.pretify_string(&yaml_append));
            }
            self.indent -= 1;
            if self.index == self.tags.data.len(){
                result.push_str("\n");
            }
            return Some(result);
        }

        None
    }    
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::storage::FileFormat;

    #[test]
    fn test_scraper_yaml_generator_empty() {
        let data = vec![];
        let options = StorageOptions {
            file_format: Some(FileFormat::Yaml),
            ..StorageOptions::new("test.yaml".to_string())
        };
        let mut generator = ScraperYAMLGenerator::new(&data, &options);
        assert_eq!(generator.next(), None);
    }

    #[test]
    fn test_scraper_yaml_generator_non_empty() {
        let data = vec![
            "<div class='test' id='div1' data-role='main'>hello world</div>".to_string(),
            "<span class='test' id='span1' data-role='secondary'>hello rust</span>".to_string(),
            "<div class='test' id='div2' data-role='main'>goodbye world</div>".to_string(),
        ];
        let options = StorageOptions {
            file_format: Some(FileFormat::Yaml),
            include_tag_content: Some(true),
            include_attributes: Some(vec!["class".to_string(), "id".to_string(), "data-role".to_string()]),
            include_text_content: Some(true),
            include_tag_names: Some(true),
            ..StorageOptions::new("test.yaml".to_string())
        };
        let mut generator = ScraperYAMLGenerator::new(&data, &options);
        let mut result = String::new();
        while let Some(line) = generator.next() {
            result.push_str(&line);
        }
        let expected = r#"data:
  - tag: div
  - class: 
    - test
  - id: div1
  - data-role: main
  - text: hello world
data:
  - tag: span
  - class: 
    - test
  - id: span1
  - data-role: secondary
  - text: hello rust
data:
  - tag: div
  - class: 
    - test
  - id: div2
  - data-role: main
  - text: goodbye world
"#.to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_scraper_yaml_generator_missing_attributes() {
        let data = vec![
            "<div class='test' data-role='main'>hello world</div>".to_string(),
            "<span class='test' id='span1'>hello rust</span>".to_string(),
            "<div class='test' id='div2' data-role='main'>goodbye world</div>".to_string(),
        ];
        let options = StorageOptions {
            file_format: Some(FileFormat::Yaml),
            include_tag_content: Some(true),
            include_attributes: Some(vec!["class".to_string(), "id".to_string(), "data-role".to_string()]),
            include_text_content: Some(true),
            include_tag_names: Some(true),
            ..StorageOptions::new("test.yaml".to_string())
        };
        let mut generator = ScraperYAMLGenerator::new(&data, &options);
        let mut result = String::new();
        while let Some(line) = generator.next() {
            result.push_str(&line);
        }
        let expected = r#"data:
  - tag: div
  - class: 
    - test
  - data-role: main
  - text: hello world
data:
  - tag: span
  - class: 
    - test
  - id: span1
  - text: hello rust
data:
  - tag: div
  - class: 
    - test
  - id: div2
  - data-role: main
  - text: goodbye world
"#.to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_scraper_yaml_generator_text_only() {
        let data = vec![
            "<div class='test' id='div1' data-role='main'>hello world</div>".to_string(),
            "<span class='test' id='span1' data-role='secondary'>hello rust</span>".to_string(),
            "<div class='test' id='div2' data-role='main'>goodbye world</div>".to_string(),
        ];
        let options = StorageOptions {
            file_format: Some(FileFormat::Yaml),
            include_tag_content: Some(false),
            include_attributes: None,
            include_text_content: Some(true),
            include_tag_names: Some(true),
            ..StorageOptions::new("test.yaml".to_string())
        };
        let mut generator = ScraperYAMLGenerator::new(&data, &options);
        let mut result = String::new();
        while let Some(line) = generator.next() {
            result.push_str(&line);
        }
        let expected = r#"data:
  - text: hello world
data:
  - text: hello rust
data:
  - text: goodbye world
"#.to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_scraper_yaml_generator_multiple_classes() {
        let data = vec![
            "<div class='test example' id='div1' data-role='main'>hello world</div>".to_string(),
            "<span class='test example' id='span1' data-role='secondary'>hello rust</span>".to_string(),
            "<div class='test example' id='div2' data-role='main'>goodbye world</div>".to_string(),
        ];
        let options = StorageOptions {
            file_format: Some(FileFormat::Yaml),
            include_tag_content: Some(true),
            include_attributes: Some(vec!["class".to_string(), "id".to_string(), "data-role".to_string()]),
            include_text_content: Some(true),
            include_tag_names: Some(true),
            ..StorageOptions::new("test.yaml".to_string())
        };
        let mut generator = ScraperYAMLGenerator::new(&data, &options);
        let mut result = String::new();
        while let Some(line) = generator.next() {
            result.push_str(&line);
        }
        let expected = r#"data:
  - tag: div
  - class: 
    - test
    - example
  - id: div1
  - data-role: main
  - text: hello world
data:
  - tag: span
  - class: 
    - test
    - example
  - id: span1
  - data-role: secondary
  - text: hello rust
data:
  - tag: div
  - class: 
    - test
    - example
  - id: div2
  - data-role: main
  - text: goodbye world
"#.to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_scraper_yaml_generator_no_classes() {
        let data = vec![
            "<div id='div1' data-role='main'>hello world</div>".to_string(),
            "<span id='span1' data-role='secondary'>hello rust</span>".to_string(),
            "<div id='div2' data-role='main'>goodbye world</div>".to_string(),
        ];
        let options = StorageOptions {
            file_format: Some(FileFormat::Yaml),
            include_tag_content: Some(true),
            include_attributes: Some(vec!["class".to_string(), "id".to_string(), "data-role".to_string()]),
            include_text_content: Some(true),
            include_tag_names: Some(true),
            ..StorageOptions::new("test.yaml".to_string())
        };
        let mut generator = ScraperYAMLGenerator::new(&data, &options);
        let mut result = String::new();
        while let Some(line) = generator.next() {
            result.push_str(&line);
        }
        let expected = r#"data:
  - tag: div
  - id: div1
  - data-role: main
  - text: hello world
data:
  - tag: span
  - id: span1
  - data-role: secondary
  - text: hello rust
data:
  - tag: div
  - id: div2
  - data-role: main
  - text: goodbye world
"#.to_string();
        assert_eq!(result, expected);
    }
}
