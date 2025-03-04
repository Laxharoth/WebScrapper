use super::storage::ScraperGenerator;
use super::storage::StorageOptions;

pub struct ScraperJSONGenerator<'a>(pub ScraperGenerator<'a>, bool);

impl<'a> ScraperJSONGenerator<'a> {
    pub fn new(data: &'a Vec<String>, options: &'a StorageOptions) -> Self {
        Self {
            0: ScraperGenerator::new(data, options),
            1: true,
        }
    }
}

impl<'a> Iterator for ScraperJSONGenerator<'a> {
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
