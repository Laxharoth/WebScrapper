use std::collections::HashSet;

use super::storage::ScraperGenerator;
use super::storage::StorageOptions;

use html_parser::{Dom, Element};

pub struct ScraperXMLGenerator<'a> {
    tags: ScraperGenerator<'a>,
    first: bool,
    order: Vec<String>,
    indent: usize,
    iter: usize,
}

impl<'a> ScraperXMLGenerator<'a> {
    pub fn new(data: &'a Vec<String>, options: &'a StorageOptions) -> Self {
        Self {
            tags: ScraperGenerator::new(data, options),
            first: true,
            order: vec![],
            indent: 0,
            iter: 0,
        }
    }

    fn pretty_print(&self) -> bool {
        self.tags.options.pretty_print.unwrap_or(false)
    }

    pub fn first_gen(&mut self) -> String {
        let mut result = String::new();
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
        result.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
        result
    }

    fn prettify(&self, line: String) -> String {
        if self.pretty_print() {
            let mut result = String::new();
            result.push_str("\n");
            for _ in 0..self.indent {
                result.push_str("  ");
            }
            result.push_str(line.as_str());
            return result;
        }
        line
    }

    fn handle_html_tagname_extract(&self, tag: &Element, json_row: &mut String) {
        json_row.push_str(format!(r#"<tag>{}</tag>"#, tag.name).as_str())
    }

    fn handle_html_classes_extract(& mut self, tag: &Element, json_row: &mut String) {
        json_row.push_str("<classes>");
        self.indent += 1;
        for class in &tag.classes {
            json_row.push_str(
                &self.prettify(
                    format!(r#"<class>{}</class>"#, class)).as_str()
                );
        }
        self.indent -= 1;
        json_row.push_str(&self.prettify("</classes>".to_string()).as_str());
    }

    fn handle_html_id_extract(&self, tag: &Element, json_row: &mut String) {
        let Some(id) = tag.id.clone() else {
            json_row.clear();
            return;
        };
        json_row.push_str(format!(r#"<id>{}</id>"#, id).as_str())
    }

    fn handle_html_text_extract(&self, tag: &Element, csv_row: &mut String) {
        csv_row.push_str(
            format!(
                r#"<text>{}</text>"#,
                tag.children
                    .iter()
                    .map(|node| node.text().unwrap_or(""))
                    .collect::<Vec<&str>>()
                    .join(" ")
            )
            .as_str(),
        )
    }

    fn handle_extract_attribute(&self, tag: &Element, xml_row: &mut String, attr: &str) {
        let Some(attribute_value) = tag.attributes.get(attr) else {
            xml_row.clear();
            return;
        };
        xml_row.push_str(
            format!(
                r#"<{0}>{1}</{0}>"#,
                attr,
                attribute_value.clone().unwrap_or("".to_string())
            )
            .as_str(),
        )
    }
}

impl<'a> Iterator for ScraperXMLGenerator<'a> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            let result = self.first_gen();
            return Some(result);
        }
        if self.iter < self.tags.data.len() {
            let mut result = String::new();
            let tag = Dom::parse(&self.tags.data[self.iter])
                .unwrap()
                .children[0]
                .element()
                .unwrap()
                .clone();
            result.push_str(self.prettify("<data>".to_string()).as_str());
            self.indent += 1;
            let order = self.order.clone();
            for header in order {
                let mut csv_append = String::new();
                match header.as_str() {
                    "tag" => self.handle_html_tagname_extract(&tag, &mut csv_append),
                    "class" => self.handle_html_classes_extract(&tag, &mut csv_append),
                    "id" => self.handle_html_id_extract(&tag, &mut csv_append),
                    "text" => self.handle_html_text_extract(&tag, &mut csv_append),
                    default => self.handle_extract_attribute(&tag, &mut csv_append, default),
                }
                result.push_str(&self.prettify(csv_append).as_str());
            }
            self.indent -= 1;
            result.push_str(self.prettify("</data>".to_string()).as_str());
            self.iter += 1;
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
    fn test_scraper_xml_generator_empty() {
        let data = vec![];
        let options = StorageOptions {
            file_format: Some(FileFormat::Xml),
            ..StorageOptions::new("test.xml".to_string())
        };
        let mut generator = ScraperXMLGenerator::new(&data, &options);
        assert_eq!(
            generator.next(),
            Some(r#"<?xml version="1.0" encoding="UTF-8"?>"#.to_string())
        );
        assert_eq!(generator.next(), None);
    }

    #[test]
    fn test_scraper_xml_generator_non_empty() {
        let data = vec![
            "<div class='test' id='div1' data-role='main'>hello world</div>".to_string(),
            "<span class='test' id='span1' data-role='secondary'>hello rust</span>".to_string(),
            "<div class='test' id='div2' data-role='main'>goodbye world</div>".to_string(),
        ];
        let options = StorageOptions {
            file_format: Some(FileFormat::Xml),
            include_tag_content: Some(true),
            include_attributes: Some(vec![
                "class".to_string(),
                "id".to_string(),
                "data-role".to_string(),
            ]),
            include_text_content: Some(true),
            include_tag_names: Some(true),
            pretty_print: Some(false),
            ..StorageOptions::new("test.xml".to_string())
        };
        let mut generator = ScraperXMLGenerator::new(&data, &options);
        assert_eq!(
            generator.next(),
            Some(r#"<?xml version="1.0" encoding="UTF-8"?>"#.to_string())
        );
        assert_eq!(
            generator.next(),
            Some(
                r#"<data><tag>div</tag><classes><class>test</class></classes><id>div1</id><data-role>main</data-role><text>hello world</text></data>"#
                    .to_string()
            )
        );
        assert_eq!(
            generator.next(),
            Some(
                r#"<data><tag>span</tag><classes><class>test</class></classes><id>span1</id><data-role>secondary</data-role><text>hello rust</text></data>"#
                    .to_string()
            )
        );
        assert_eq!(
            generator.next(),
            Some(
                r#"<data><tag>div</tag><classes><class>test</class></classes><id>div2</id><data-role>main</data-role><text>goodbye world</text></data>"#
                    .to_string()
            )
        );
        assert_eq!(generator.next(), None);
    }

    #[test]
    fn test_scraper_xml_generator_missing_attributes() {
        let data = vec![
            "<div class='test' data-role='main'>hello world</div>".to_string(),
            "<span class='test' id='span1'>hello rust</span>".to_string(),
            "<div class='test' id='div2' data-role='main'>goodbye world</div>".to_string(),
        ];
        let options = StorageOptions {
            file_format: Some(FileFormat::Xml),
            include_tag_content: Some(true),
            include_attributes: Some(vec![
                "class".to_string(),
                "id".to_string(),
                "data-role".to_string(),
            ]),
            include_text_content: Some(true),
            include_tag_names: Some(true),
            pretty_print: Some(false),
            ..StorageOptions::new("test.xml".to_string())
        };
        let mut generator = ScraperXMLGenerator::new(&data, &options);
        assert_eq!(
            generator.next(),
            Some(r#"<?xml version="1.0" encoding="UTF-8"?>"#.to_string())
        );
        assert_eq!(
            generator.next(),
            Some(
                r#"<data><tag>div</tag><classes><class>test</class></classes><data-role>main</data-role><text>hello world</text></data>"#
                    .to_string()
            )
        ); // Missing id
        assert_eq!(
            generator.next(),
            Some(
                r#"<data><tag>span</tag><classes><class>test</class></classes><id>span1</id><text>hello rust</text></data>"#
                    .to_string()
            )
        ); // Missing data-role
        assert_eq!(
            generator.next(),
            Some(
                r#"<data><tag>div</tag><classes><class>test</class></classes><id>div2</id><data-role>main</data-role><text>goodbye world</text></data>"#
                    .to_string()
            )
        );
        assert_eq!(generator.next(), None);
    }

    #[test]
    fn test_scraper_xml_generator_text_only() {
        let data = vec![
            "<div class='test' id='div1' data-role='main'>hello world</div>".to_string(),
            "<span class='test' id='span1' data-role='secondary'>hello rust</span>".to_string(),
            "<div class='test' id='div2' data-role='main'>goodbye world</div>".to_string(),
        ];
        let options = StorageOptions {
            file_format: Some(FileFormat::Xml),
            include_tag_content: Some(false),
            include_attributes: None,
            include_text_content: Some(true),
            include_tag_names: Some(true),
            pretty_print: Some(false),
            ..StorageOptions::new("test.xml".to_string())
        };
        let mut generator = ScraperXMLGenerator::new(&data, &options);
        assert_eq!(
            generator.next(),
            Some(r#"<?xml version="1.0" encoding="UTF-8"?>"#.to_string())
        );
        assert_eq!(
            generator.next(),
            Some(r#"<data><text>hello world</text></data>"#.to_string())
        );
        assert_eq!(
            generator.next(),
            Some(r#"<data><text>hello rust</text></data>"#.to_string())
        );
        assert_eq!(
            generator.next(),
            Some(r#"<data><text>goodbye world</text></data>"#.to_string())
        );
        assert_eq!(generator.next(), None);
    }

    #[test]
    fn test_scraper_xml_generator_pretty_print() {
        let data = vec![
            "<div class='test' id='div1' data-role='main'>hello world</div>".to_string(),
            "<span class='test' id='span1' data-role='secondary'>hello rust</span>".to_string(),
            "<div class='test' id='div2' data-role='main'>goodbye world</div>".to_string(),
        ];
        let options = StorageOptions {
            file_format: Some(FileFormat::Xml),
            include_tag_content: Some(true),
            include_attributes: Some(vec!["class".to_string(), "id".to_string(), "data-role".to_string()]),
            include_text_content: Some(true),
            include_tag_names: Some(true),
            pretty_print: Some(true),
            ..StorageOptions::new("test.xml".to_string())
        };
        let mut generator = ScraperXMLGenerator::new(&data, &options);
        let mut result = String::new();
        while let Some(line) = generator.next() {
            result.push_str(&line);
        }
        let expected = r#"<?xml version="1.0" encoding="UTF-8"?>
<data>
  <tag>div</tag>
  <classes>
    <class>test</class>
  </classes>
  <id>div1</id>
  <data-role>main</data-role>
  <text>hello world</text>
</data>
<data>
  <tag>span</tag>
  <classes>
    <class>test</class>
  </classes>
  <id>span1</id>
  <data-role>secondary</data-role>
  <text>hello rust</text>
</data>
<data>
  <tag>div</tag>
  <classes>
    <class>test</class>
  </classes>
  <id>div2</id>
  <data-role>main</data-role>
  <text>goodbye world</text>
</data>"#.to_string();
        assert_eq!(result, expected);
    }
}
