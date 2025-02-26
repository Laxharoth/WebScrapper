use super::storage::ScraperGenerator;
use super::storage::StorageOptions;

pub struct ScraperTxtGenerator<'a>(pub ScraperGenerator<'a>);

impl<'a> ScraperTxtGenerator<'a> {
    pub fn new(data: &'a Vec<String>, options: &'a StorageOptions) -> Self {
        Self {
            0: ScraperGenerator::new(data, options),
        }
    }
}

impl<'a> Iterator for ScraperTxtGenerator<'a> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.index < self.0.data.len() {
            self.0.index += 1;
            return Some(self.0.data[self.0.index - 1].clone());
        }
        None
    }    
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::storage::{FileFormat};

    #[test]
    fn test_scraper_txt_generator() {
        let data = vec![
            "<div class='test' id='div1' data-role='main'>hello world</div>".to_string(),
            "<span class='test' id='span1' data-role='secondary'>hello rust</span>".to_string(),
            "<div class='test' id='div2' data-role='main'>goodbye world</div>".to_string(),
        ];
        let options = StorageOptions {
            file_format: Some(FileFormat::Txt),
            ..StorageOptions::new("test.txt".to_string())
        };
        let mut generator = ScraperTxtGenerator::new(&data, &options);
        assert_eq!(generator.next(), Some("<div class='test' id='div1' data-role='main'>hello world</div>".to_string()));
        assert_eq!(generator.next(), Some("<span class='test' id='span1' data-role='secondary'>hello rust</span>".to_string()));
        assert_eq!(generator.next(), Some("<div class='test' id='div2' data-role='main'>goodbye world</div>".to_string()));
        assert_eq!(generator.next(), None);
    }
}
