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
        if self.0.index == 0 {
            self.0.index += 1;
            return Some(self.0.data[self.0.index - 1].clone());
        }
        None
    }    
}
