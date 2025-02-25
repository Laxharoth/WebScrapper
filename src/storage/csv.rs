use super::storage::ScraperGenerator;
use super::storage::StorageOptions;

pub struct ScraperCSVGenerator<'a>(pub ScraperGenerator<'a>);

impl<'a> ScraperCSVGenerator<'a> {
    pub fn new(data: &'a Vec<String>, options: &'a StorageOptions) -> Self {
        Self {
            0: ScraperGenerator::new(data, options),
        }
    }
}

impl<'a> Iterator for ScraperCSVGenerator<'a> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        //TODO 
        None
    }    
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::storage::{FileFormat, Encoding};

    #[test]
    fn test_scraper_csv_generator_empty() {
        let data = vec![];
        let options = StorageOptions {
            file_name: "test.csv".to_string(),
            file_format: Some(FileFormat::Csv),
            include_tag_content: Some(false),
            include_attributes: Some(vec![]),
            include_text_content: Some(true),
            include_tag_names: Some(true),
            include_metadata: Some(false),
            pretty_print: Some(false),
            delimiter: Some(",".to_string()),
        };
        let mut generator = ScraperCSVGenerator::new(&data, &options);
        assert_eq!(generator.next(), None);
    }

    #[test]
    fn test_scraper_csv_generator_non_empty() {
        let data = vec![
            "<div class='test' id='div1' data-role='main'>hello world</div>".to_string(),
            "<span class='test' id='span1' data-role='secondary'>hello rust</span>".to_string(),
            "<div class='test' id='div2' data-role='main'>goodbye world</div>".to_string(),
        ];
        let options = StorageOptions {
            file_name: "test.csv".to_string(),
            file_format: Some(FileFormat::Csv),
            include_tag_content: Some(true),
            include_attributes: Some(vec!["class".to_string(), "id".to_string(), "data-role".to_string()]),
            include_text_content: Some(true),
            include_tag_names: Some(true),
            include_metadata: Some(false),
            pretty_print: Some(false),
            delimiter: Some(",".to_string()),
        };
        let mut generator = ScraperCSVGenerator::new(&data, &options);
        assert_eq!(generator.next(), Some("tag,class,id,data-role,text\n".to_string()));
        assert_eq!(generator.next(), Some("div,test,div1,main,hello world\n".to_string()));
        assert_eq!(generator.next(), Some("span,test,span1,secondary,hello rust\n".to_string()));
        assert_eq!(generator.next(), Some("div,test,div2,main,goodbye world\n".to_string()));
        assert_eq!(generator.next(), None);
    }

    #[test]
    fn test_scraper_csv_generator_missing_attributes() {
        let data = vec![
            "<div class='test' data-role='main'>hello world</div>".to_string(),
            "<span class='test' id='span1'>hello rust</span>".to_string(),
            "<div class='test' id='div2' data-role='main'>goodbye world</div>".to_string(),
        ];
        let options = StorageOptions {
            file_name: "test.csv".to_string(),
            file_format: Some(FileFormat::Csv),
            include_tag_content: Some(true),
            include_attributes: Some(vec!["class".to_string(), "id".to_string(), "data-role".to_string()]),
            include_text_content: Some(true),
            include_tag_names: Some(true),
            include_metadata: Some(false),
            pretty_print: Some(false),
            delimiter: Some(",".to_string()),
        };
        let mut generator = ScraperCSVGenerator::new(&data, &options);
        assert_eq!(generator.next(), Some("tag,class,id,data-role,text\n".to_string()));
        assert_eq!(generator.next(), Some("div,test,,main,hello world\n".to_string())); // Missing id
        assert_eq!(generator.next(), Some("span,test,span1,,hello rust\n".to_string())); // Missing data-role
        assert_eq!(generator.next(), Some("div,test,div2,main,goodbye world\n".to_string()));
        assert_eq!(generator.next(), None);
    }

    #[test]
    fn test_scraper_csv_generator_text_only() {
        let data = vec![
            "<div class='test' id='div1' data-role='main'>hello world</div>".to_string(),
            "<span class='test' id='span1' data-role='secondary'>hello rust</span>".to_string(),
            "<div class='test' id='div2' data-role='main'>goodbye world</div>".to_string(),
        ];
        let options = StorageOptions {
            file_name: "test.csv".to_string(),
            file_format: Some(FileFormat::Csv),
            include_tag_content: Some(false),
            include_attributes: None,
            include_text_content: Some(true),
            include_tag_names: Some(true),
            include_metadata: Some(false),
            pretty_print: Some(false),
            delimiter: Some(",".to_string()),
        };
        let mut generator = ScraperCSVGenerator::new(&data, &options);
        assert_eq!(generator.next(), Some("text\n".to_string()));
        assert_eq!(generator.next(), Some("hello world\n".to_string()));
        assert_eq!(generator.next(), Some("hello rust\n".to_string()));
        assert_eq!(generator.next(), Some("goodbye world\n".to_string()));
        assert_eq!(generator.next(), None);
    }

    #[test]
    fn test_scraper_csv_generator_custom_delimiter() {
        let data = vec![
            "<div class='test' id='div1' data-role='main'>hello world</div>".to_string(),
            "<span class='test' id='span1' data-role='secondary'>hello rust</span>".to_string(),
            "<div class='test' id='div2' data-role='main'>goodbye world</div>".to_string(),
        ];
        let options = StorageOptions {
            file_name: "test.csv".to_string(),
            file_format: Some(FileFormat::Csv),
            include_tag_content: Some(true),
            include_attributes: None,
            include_text_content: Some(true),
            include_tag_names: Some(true),
            include_metadata: Some(false),
            pretty_print: Some(false),
            delimiter: Some(";".to_string()),
        };
        let mut generator = ScraperCSVGenerator::new(&data, &options);
        assert_eq!(generator.next(), Some("tag;class;id;data-role;text\n".to_string()));
        assert_eq!(generator.next(), Some("div;test;div1;main;hello world\n".to_string()));
        assert_eq!(generator.next(), Some("span;test;span1;secondary;hello rust\n".to_string()));
        assert_eq!(generator.next(), Some("div;test;div2;main;goodbye world\n".to_string()));
        assert_eq!(generator.next(), None);
    }
}