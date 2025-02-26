use std::collections::HashSet;

use super::storage::ScraperGenerator;
use super::storage::StorageOptions;
use html_parser::Dom;

pub struct ScraperCSVGenerator<'a>{
    tags: ScraperGenerator<'a>, 
    first: bool,
    order: Vec<String>,
}

impl<'a> ScraperCSVGenerator<'a> {
    pub fn new(data: &'a Vec<String>, options: &'a StorageOptions) -> Self {
        Self {
            tags: ScraperGenerator::new(data, options),
            first: true,
            order: vec![]
        }
    }
    
    fn first_gen(&mut self) -> Option<String> {
        let mut csv_order: HashSet<String> = HashSet::new();
        if !self.tags.options.include_tag_content.unwrap_or(false){
            csv_order.insert("text".to_string());
            return Some("text".to_string());
        }
        if self.tags.options.include_tag_names.unwrap_or(false){
            csv_order.insert("tag".to_string());
        }
        if let Some(attributes) = &self.tags.options.include_attributes{
            for attribute in attributes{
                csv_order.insert(attribute.to_string());
            }
        }
        else {
            for tag in self.tags.data{
                let tag = Dom::parse(&tag).unwrap().children[0].element().unwrap().clone();
                if !tag.id.is_none(){
                    csv_order.insert("id".to_string());
                }
                if tag.classes.len() > 0{
                    csv_order.insert("class".to_string());
                }
                for (key, _) in tag.attributes.iter(){
                    csv_order.insert(key.to_string());
                }
            }
        }
        for key in csv_order{
            self.order.push(key);
        }
        self.order.push("text".to_string());
        return Some(self.order.join(","));
    }
}

impl<'a> Iterator for ScraperCSVGenerator<'a> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        //TODO 
        if self.first{
            self.first = false;
            return self.first_gen();
        }
        None
    }    
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::storage::{FileFormat};

    #[test]
    fn test_scraper_csv_generator_empty() {
        let data = vec![];
        let options = StorageOptions {
            file_format: Some(FileFormat::Csv),
            include_tag_content: Some(false),
            ..StorageOptions::new("test.csv".to_string())
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
            file_format: Some(FileFormat::Csv),
            include_tag_content: Some(true),
            include_attributes: Some(vec!["class".to_string(), "id".to_string(), "data-role".to_string()]),
            delimiter: Some(",".to_string()),
            ..StorageOptions::new("test.csv".to_string())
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
            file_format: Some(FileFormat::Csv),
            include_tag_content: Some(true),
            include_attributes: Some(vec!["class".to_string(), "id".to_string(), "data-role".to_string()]),
            ..StorageOptions::new("test.csv".to_string())
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
            file_format: Some(FileFormat::Csv),
            include_tag_content: Some(false),
            ..StorageOptions::new("test.csv".to_string())
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
            file_format: Some(FileFormat::Csv),
            include_tag_content: Some(true),
            delimiter: Some(";".to_string()),
            ..StorageOptions::new("test.csv".to_string())
        };
        let mut generator = ScraperCSVGenerator::new(&data, &options);
        assert_eq!(generator.next(), Some("tag;class;id;data-role;text\n".to_string()));
        assert_eq!(generator.next(), Some("div;test;div1;main;hello world\n".to_string()));
        assert_eq!(generator.next(), Some("span;test;span1;secondary;hello rust\n".to_string()));
        assert_eq!(generator.next(), Some("div;test;div2;main;goodbye world\n".to_string()));
        assert_eq!(generator.next(), None);
    }
}