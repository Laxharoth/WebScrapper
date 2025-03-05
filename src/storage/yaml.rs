use super::storage::ScraperGenerator;
use super::storage::StorageOptions;

pub struct ScraperYAMLGenerator<'a>(pub ScraperGenerator<'a>);

impl<'a> ScraperYAMLGenerator<'a> {
    pub fn new(data: &'a Vec<String>, options: &'a StorageOptions) -> Self {
        Self {
            0: ScraperGenerator::new(data, options),
        }
    }
}

impl<'a> Iterator for ScraperYAMLGenerator<'a> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        //TODO 
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
  tag: div
  class: 
    - test
  id: div1
  data-role: main
  text: hello world
data:
  tag: span
  class: 
    - test
  id: span1
  data-role: secondary
  text: hello rust
data:
  tag: div
  class: 
    - test
  id: div2
  data-role: main
  text: goodbye world
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
  tag: div
  class: 
    - test
  data-role: main
  text: hello world
data:
  tag: span
  class: 
    - test
  id: span1
  text: hello rust
data:
  tag: div
  class: 
    - test
  id: div2
  data-role: main
  text: goodbye world
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
  text: hello world
data:
  text: hello rust
data:
  text: goodbye world
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
  tag: div
  class: 
    - test
    - example
  id: div1
  data-role: main
  text: hello world
data:
  tag: span
  class: 
    - test
    - example
  id: span1
  data-role: secondary
  text: hello rust
data:
  tag: div
  class: 
    - test
    - example
  id: div2
  data-role: main
  text: goodbye world
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
  tag: div
  id: div1
  data-role: main
  text: hello world
data:
  tag: span
  id: span1
  data-role: secondary
  text: hello rust
data:
  tag: div
  id: div2
  data-role: main
  text: goodbye world
"#.to_string();
        assert_eq!(result, expected);
    }
}
