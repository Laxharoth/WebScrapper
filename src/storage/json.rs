use super::storage::ScraperGenerator;
use super::storage::StorageOptions;

pub struct ScraperJSONGenerator<'a>(pub ScraperGenerator<'a>);

impl<'a> ScraperJSONGenerator<'a> {
    pub fn new(data: &'a Vec<String>, options: &'a StorageOptions) -> Self {
        Self {
            0: ScraperGenerator::new(data, options),
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
