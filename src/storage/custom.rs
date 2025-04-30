use super::storage::StorageOptions;

pub struct CustomDataGenerator<'a> {
    pub data: &'a Vec<String>,
    pub options: &'a StorageOptions,
    pub index: usize,
}

impl<'a> CustomDataGenerator<'a> {
    pub fn new(data: &'a Vec<String>, options: &'a StorageOptions) -> Self {
        Self {
            data,
            options,
            index: 0,
        }
    }

}

impl Iterator for CustomDataGenerator<'_> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.data.len() {
            if let Some(custom_data_storage) = self.options.custom_data_storage {
                custom_data_storage(&self.data[self.index]);
            }
            self.index += 1;
            Some("".to_string())
        } else {
            None
        }
    }
}