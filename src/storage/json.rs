use std::collections::HashSet;

use super::storage::ScraperGenerator;
use super::storage::StorageOptions;

use html_parser::{Dom, Element};


pub struct ScraperJSONGenerator<'a> {
    tags: ScraperGenerator<'a>,
    first: bool,
    last: bool,
    index: usize,
    identation: usize,
    order: Vec<String>,
}

impl<'a> ScraperJSONGenerator<'a> {
    pub fn new(data: &'a Vec<String>, options: &'a StorageOptions) -> Self {
        Self {
            tags: ScraperGenerator::new(data, options),
            first: true,
            last: true,
            index: 0,
            identation: 0,
            order: vec![],
        }
    }

    fn pretty_print(&self) -> bool {
        self.tags.options.pretty_print.unwrap_or(false)
    }

    fn first_gen(&mut self) -> String {
        // tag content only required if option is true
        if self.tags.options.include_tag_content.unwrap_or(false) {
            if self.tags.options.include_tag_names.unwrap_or(true) {
                self.order.push("tag".to_string());
            }
            if let Some(attributes) = &self.tags.options.include_attributes {
                for attribute in attributes {
                    self.order.push(attribute.to_string());
                }
            } else {
                let mut attr_order: HashSet<String> = HashSet::new();
                for tag in self.tags.data {
                    let tag = Dom::parse(&tag)
                        .unwrap()
                        .children[0]
                        .element()
                        .unwrap()
                        .clone();
                    if !tag.id.is_none() {
                        attr_order.insert("id".to_string());
                    }
                    if tag.classes.len() > 0 {
                        attr_order.insert("class".to_string());
                    }
                    for (key, _) in tag.attributes.iter() {
                        attr_order.insert(key.to_string());
                    }
                }
                for key in attr_order {
                    self.order.push(key);
                }
            }
        }
        self.order.push("text".to_string());
        return "[".to_string();
    }

    fn prettify(&self, line: String) -> String {
        let mut result = String::new();
        if self.pretty_print() {
            result.push_str("\n");
        }
        for _ in 0..self.identation {
            result.push_str("  ");
        }
        result.push_str(&line);
        return result;
    }
    
    fn handle_html_tagname_extract(&self, tag: &Element, json_row: &mut String, header: &String) {
        json_row.push_str(
            format!(r#""{}":"{}""#, header, tag.name)
                .as_str(),
        )
    }
    
    fn handle_html_classes_extract(&self, tag: &Element, json_row: &mut String, header: &String) {
        let separator = if self.pretty_print() {
            ", "
        } else {
            ","
        };
        json_row.push_str(
            format!(
                r#""{}":[{}]"#,
                header,
                tag.classes
                    .iter()
                    .map(|class| format!(r#""{}""#, class))
                    .collect::<Vec<String>>()
                    .join(separator)
            )
            .as_str(),
        )
    }
    
    fn handle_html_id_extract(&self, tag: &Element, json_row: &mut String, header: &String) {
        let Some(id) = tag.id.clone() else {
            json_row.clear();
            return;
        };
        json_row.push_str(
            format!(
                r#""{}":"{}""#,
                header,
                id
            )
            .as_str(),
        )
    }
    
    fn handle_html_text_extract(&self, tag: &Element, json_row: &mut String, header: &String) {
        json_row.push_str(
            format!(
                r#""{}":"{}""#,
                header,
                tag.children
                    .iter()
                    .map(|node| node.text().unwrap_or(""))
                    .collect::<Vec<&str>>()
                    .join(" ")
            )
            .as_str(),
        )
    }
    
    fn handle_extract_attribute(&self, tag: &Element, json_row: &mut String, header: &String, default: &str) {
        let Some(attribute_value) = tag.attributes.get(default) else {
            json_row.clear();
            return;
        };
        json_row.push_str(
            format!(
                r#""{}":"{}""#,
                header,
                attribute_value
                    .clone()
                    .unwrap_or("".to_string())
            )
            .as_str(),
        )
    }
}

impl<'a> Iterator for ScraperJSONGenerator<'a> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            let header = self.first_gen();
            let header = header;
            if self.pretty_print() {
                self.identation += 1;
            }
            return Some(header);
        }
        if self.index < self.tags.data.len() {
            let tag = Dom::parse(&self.tags.data[self.index])
                .unwrap()
                .children[0]
                .element()
                .unwrap()
                .clone();
            let mut json_row = String::new();
            json_row.push_str(self.prettify("{".to_string()).as_str());
            if self.pretty_print() {
                self.identation += 1;
            }
            for (i,header) in self.order.iter().enumerate() {
                let mut json_append = String::new();
                match header.as_str() {
                    "tag" => self.handle_html_tagname_extract(&tag, &mut json_append, header),
                    "class" => self.handle_html_classes_extract(&tag, &mut json_append, header),
                    "id" => self.handle_html_id_extract(&tag, &mut json_append, header),
                    "text" => self.handle_html_text_extract(&tag, &mut json_append, header),
                    default => self.handle_extract_attribute(&tag, &mut json_append, header, default),
                }
                if json_append.is_empty() {
                    continue;
                }
                json_append = self.prettify(json_append);
                if i > 0 {
                    json_row.push_str(",");
                }
                json_row.push_str(json_append.as_str());
            }
            if self.pretty_print() {
                self.identation -= 1;
            }
            let mut row_tail = "}".to_string();
            if self.index < self.tags.data.len() - 1 {
                row_tail.push_str(",");
            }
            json_row.push_str(self.prettify(row_tail).as_str());
            self.index += 1;
            return Some(json_row);
        }
        if self.last {
            self.last = false;
            if self.pretty_print() {
                self.identation -= 1;
            }
            let footer = self.prettify("]".to_string());
            return Some(footer);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::storage::FileFormat;

    #[test]
    fn test_scraper_json_generator_empty() {
        let data = vec![];
        let options = StorageOptions {
            file_format: Some(FileFormat::Json),
            ..StorageOptions::new("test.json".to_string())
        };
        let mut generator = ScraperJSONGenerator::new(&data, &options);
        assert_eq!(generator.next(), Some("[".to_string()));
        assert_eq!(generator.next(), Some("]".to_string()));
    }

    #[test]
    fn test_scraper_json_generator_non_empty() {
        let data = vec![
            "<div class='test' id='div1' data-role='main'>hello world</div>".to_string(),
            "<span class='test' id='span1' data-role='secondary'>hello rust</span>".to_string(),
            "<div class='test' id='div2' data-role='main'>goodbye world</div>".to_string(),
        ];
        let options = StorageOptions {
            file_format: Some(FileFormat::Json),
            include_tag_content: Some(true),
            include_attributes: Some(vec!["class".to_string(), "id".to_string(), "data-role".to_string()]),
            include_text_content: Some(true),
            include_tag_names: Some(true),
            pretty_print: Some(false),
            ..StorageOptions::new("test.json".to_string())
        };
        let mut generator = ScraperJSONGenerator::new(&data, &options);
        assert_eq!(generator.next(), Some("[".to_string()));
        assert_eq!(generator.next(), Some(r#"{"tag":"div","class":["test"],"id":"div1","data-role":"main","text":"hello world"},"#.to_string()));
        assert_eq!(generator.next(), Some(r#"{"tag":"span","class":["test"],"id":"span1","data-role":"secondary","text":"hello rust"},"#.to_string()));
        assert_eq!(generator.next(), Some(r#"{"tag":"div","class":["test"],"id":"div2","data-role":"main","text":"goodbye world"}"#.to_string()));
        assert_eq!(generator.next(), Some("]".to_string()));
    }

    #[test]
    fn test_scraper_json_generator_missing_attributes() {
        let data = vec![
            "<div class='test' data-role='main'>hello world</div>".to_string(),
            "<span class='test' id='span1'>hello rust</span>".to_string(),
            "<div class='test' id='div2' data-role='main'>goodbye world</div>".to_string(),
        ];
        let options = StorageOptions {
            file_format: Some(FileFormat::Json),
            include_tag_content: Some(true),
            include_attributes: Some(vec!["class".to_string(), "id".to_string(), "data-role".to_string()]),
            include_text_content: Some(true),
            include_tag_names: Some(true),
            pretty_print: Some(false),
            ..StorageOptions::new("test.json".to_string())
        };
        let mut generator = ScraperJSONGenerator::new(&data, &options);
        assert_eq!(generator.next(), Some("[".to_string()));
        assert_eq!(generator.next(), Some(r#"{"tag":"div","class":["test"],"data-role":"main","text":"hello world"},"#.to_string())); // Missing id
        assert_eq!(generator.next(), Some(r#"{"tag":"span","class":["test"],"id":"span1","text":"hello rust"},"#.to_string())); // Missing data-role
        assert_eq!(generator.next(), Some(r#"{"tag":"div","class":["test"],"id":"div2","data-role":"main","text":"goodbye world"}"#.to_string()));
        assert_eq!(generator.next(), Some("]".to_string()));
    }

    #[test]
    fn test_scraper_json_generator_text_only() {
        let data = vec![
            "<div class='test' id='div1' data-role='main'>hello world</div>".to_string(),
            "<span class='test' id='span1' data-role='secondary'>hello rust</span>".to_string(),
            "<div class='test' id='div2' data-role='main'>goodbye world</div>".to_string(),
        ];
        let options = StorageOptions {
            file_format: Some(FileFormat::Json),
            include_tag_content: Some(false),
            include_attributes: None,
            include_text_content: Some(true),
            include_tag_names: Some(true),
            pretty_print: Some(false),
            ..StorageOptions::new("test.json".to_string())
        };
        let mut generator = ScraperJSONGenerator::new(&data, &options);
        assert_eq!(generator.next(), Some("[".to_string()));
        assert_eq!(generator.next(), Some(r#"{"text":"hello world"},"#.to_string()));
        assert_eq!(generator.next(), Some(r#"{"text":"hello rust"},"#.to_string()));
        assert_eq!(generator.next(), Some(r#"{"text":"goodbye world"}"#.to_string()));
        assert_eq!(generator.next(), Some("]".to_string()));
    }

    #[test]
    fn test_scraper_json_generator_pretty_print() {
        let data = vec![
            "<div class='test' id='div1' data-role='main'>hello world</div>".to_string(),
            "<span class='test' id='span1' data-role='secondary'>hello rust</span>".to_string(),
            "<div class='test' id='div2' data-role='main'>goodbye world</div>".to_string(),
        ];
        let options = StorageOptions {
            file_format: Some(FileFormat::Json),
            include_tag_content: Some(true),
            include_attributes: Some(vec!["class".to_string(), "id".to_string(), "data-role".to_string()]),
            include_text_content: Some(true),
            include_tag_names: Some(true),
            pretty_print: Some(true),
            ..StorageOptions::new("test.json".to_string())
        };
        let mut generator = ScraperJSONGenerator::new(&data, &options);
        let mut result = String::new();
        while let Some(line) = generator.next() {
            result.push_str(&line);
        }
        let expected = r#"[
  {
    "tag":"div",
    "class":["test"],
    "id":"div1",
    "data-role":"main",
    "text":"hello world"
  },
  {
    "tag":"span",
    "class":["test"],
    "id":"span1",
    "data-role":"secondary",
    "text":"hello rust"
  },
  {
    "tag":"div",
    "class":["test"],
    "id":"div2",
    "data-role":"main",
    "text":"goodbye world"
  }
]"#.to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_scraper_json_generator_pretty_print_multiple_classes() {
        let data = vec![
            "<div class='test example' id='div1' data-role='main'>hello world</div>".to_string(),
            "<span class='test example' id='span1' data-role='secondary'>hello rust</span>".to_string(),
            "<div class='test example' id='div2' data-role='main'>goodbye world</div>".to_string(),
        ];
        let options = StorageOptions {
            file_format: Some(FileFormat::Json),
            include_tag_content: Some(true),
            include_attributes: Some(vec!["class".to_string(), "id".to_string(), "data-role".to_string()]),
            include_text_content: Some(true),
            include_tag_names: Some(true),
            pretty_print: Some(true),
            ..StorageOptions::new("test.json".to_string())
        };
        let mut generator = ScraperJSONGenerator::new(&data, &options);
        let mut result = String::new();
        while let Some(line) = generator.next() {
            result.push_str(&line);
        }
        let expected = r#"[
  {
    "tag":"div",
    "class":["test", "example"],
    "id":"div1",
    "data-role":"main",
    "text":"hello world"
  },
  {
    "tag":"span",
    "class":["test", "example"],
    "id":"span1",
    "data-role":"secondary",
    "text":"hello rust"
  },
  {
    "tag":"div",
    "class":["test", "example"],
    "id":"div2",
    "data-role":"main",
    "text":"goodbye world"
  }
]"#.to_string();
        assert_eq!(result, expected);
    }
}
