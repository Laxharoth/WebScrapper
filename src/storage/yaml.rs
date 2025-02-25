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
