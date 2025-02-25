use super::storage::ScraperGenerator;
use super::storage::StorageOptions;

pub struct ScraperXMLGenerator<'a>(pub ScraperGenerator<'a>);

impl<'a> ScraperXMLGenerator<'a> {
    pub fn new(data: &'a Vec<String>, options: &'a StorageOptions) -> Self {
        Self {
            0: ScraperGenerator::new(data, options),
        }
    }
}

impl<'a> Iterator for ScraperXMLGenerator<'a> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        //TODO 
        None
    }    
}
