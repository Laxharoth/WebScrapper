use std::fs::File;
use std::io;
use std::io::Write;

use super::txt;
use super::json;
use super::csv;
use super::xml;
use super::yaml;
use super::custom;


pub enum FileFormat {
    Txt,
    Json,
    Csv,
    Xml,
    Yaml,
    Custom,
}

/// Options for configuring storage behavior. Some file 
/// 
/// # Fields
/// 
/// * `file_name` - The name of the file where data will be stored.
/// * `file_format` - The format of the file (e.g., CSV, JSON). Defaults to `FileFormat::Txt`.
/// * `include_tag_content` - Whether to include the content of HTML tags. Defaults to `false`.
/// * `include_attributes` - A list of attribute names to include in the output. Defaults to an empty list.
/// * `include_text_content` - Whether to include the text content of HTML elements. Defaults to `true`.
/// * `include_tag_names` - Whether to include the names of HTML tags. Defaults to `true`.
/// * `include_metadata` - Whether to include metadata in the output. Defaults to `false`.
/// * `pretty_print` - Whether to pretty-print the output (e.g., for JSON). Defaults to `false`.
/// * `delimiter` - The delimiter to use for CSV format. Defaults to `,`.
/// * `append` - Whether to append to the file if it already exists. Defaults to `false`.
/// * `compress` - Whether to compress the output file. Defaults to `false`.
/// * `encoding` - The encoding to use for the output file (e.g., "UTF-8", "ASCII"). Defaults to `Encoding::Utf8`.

pub struct StorageOptions {
    pub file_name: String,
    pub file_format: Option<FileFormat>,
    pub include_tag_content: Option<bool>, // include_attribures and include_tag_names will be ignored if this is false
    pub include_attributes: Option<Vec<String>>, // List of attribute names to include, None means all attributes
    pub include_text_content: Option<bool>,
    pub include_tag_names: Option<bool>,
    // pub include_metadata: Option<bool>,
    // pub custom_data_patterns: Option<Vec<String>>, // List of custom data extraction patterns
    pub pretty_print: Option<bool>, // For JSON and XML formats
    pub delimiter: Option<String>, // For CSV format
    pub custom_data_storage: Option<fn(&String)>, // Only for custom file formats
}

impl StorageOptions  {
    pub fn new(filename:String) -> Self {
        Self {
            file_name: filename,
            file_format: None,
            include_tag_content: None,
            include_attributes: None,
            include_text_content: None,
            include_tag_names: None,
            // include_metadata: None,
            pretty_print: None,
            delimiter: None,
            custom_data_storage: None,
        }
    } 
}

pub struct ScraperGenerator<'a> {
    pub data: &'a Vec<String>,
    pub options: &'a StorageOptions,
    pub index: usize,
}

impl<'a> ScraperGenerator<'a> {
    pub fn new(data: &'a Vec<String>, options: &'a StorageOptions) -> Self {
        Self {
            data,
            options,
            index: 0,
        }
    }
}

pub fn store(data: &Vec<String>, options: &StorageOptions) -> Result<(), io::Error> {
    let content_iter: Box<dyn Iterator<Item = String>> = match options.file_format.as_ref().unwrap_or(&FileFormat::Txt) {
        FileFormat::Txt => Box::new(txt::ScraperTxtGenerator::new(data, &options)),
        FileFormat::Json => Box::new(json::ScraperJSONGenerator::new(data, &options)),
        FileFormat::Csv => Box::new(csv::ScraperCSVGenerator::new(data, &options)),
        FileFormat::Xml => Box::new(xml::ScraperXMLGenerator::new(data, &options)),
        FileFormat::Yaml => Box::new(yaml::ScraperYAMLGenerator::new(data, &options)),
        FileFormat::Custom => Box::new(custom::CustomDataGenerator::new(data, &options)),
    };

    match options.file_format.as_ref().unwrap_or(&FileFormat::Txt) {
        FileFormat::Custom =>{
            for _ in content_iter{}
        },
        _ =>{
            let mut file = File::create(&options.file_name)?;
        
            for line in content_iter {
                file.write_all(line.as_bytes())?;
            }
        }
    }

    Ok(())
}