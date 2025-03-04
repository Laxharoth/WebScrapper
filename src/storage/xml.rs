use super::storage::ScraperGenerator;
use super::storage::StorageOptions;

pub struct ScraperXMLGenerator<'a>(pub ScraperGenerator<'a>, bool);

impl<'a> ScraperXMLGenerator<'a> {
    pub fn new(data: &'a Vec<String>, options: &'a StorageOptions) -> Self {
        Self {
            0: ScraperGenerator::new(data, options), 1:true
        }
    }
}

impl<'a> Iterator for ScraperXMLGenerator<'a> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 {
            self.1 = false;
            return Some(r#"<?xml version="1.0" encoding="UTF-8"?>"#.to_string());
        }
        //TODO 
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
