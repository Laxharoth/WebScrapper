mod fetcher;
mod scrapper;
mod storage;

use fetcher::fetch::fetch;
use scrapper::scrap::{scrape, ScrapeOptions, TagFilter};
use storage::storage::{store, StorageOptions, FileFormat};

fn main() {
    // Fetch HTML content
    let url = "https://scholar.google.com/scholar?hl=es&as_sdt=0%2C5&q=random+number+generator+for+cryptography&btnG=&oq=random+number+generator+for+cryptogra";
    let raw_html = fetch(url)
        .expect(format!("error fetching url:{}", url).as_str());

    // Define scraping options
    let scrape_options = ScrapeOptions {
        tags: TagFilter {
            filter: vec!["div".to_string(), "span".to_string()],
        },
        id_filter: None,
        class_filter: None,
        attributes_include: None,
        attributes_exclude: None,
        text_include: None,
        text_exclude: None,
    };

    // Scrape the HTML content
    let scraped_data = scrape(&raw_html, scrape_options);

    // Define storage options
    let storage_options = StorageOptions {
        file_name: "output.json".to_string(),
        file_format: Some(FileFormat::Json),
        include_tag_content: Some(true),
        include_attributes: Some(vec!["class".to_string(), "id".to_string(), "data-role".to_string()]),
        include_text_content: Some(true),
        include_tag_names: Some(true),
        pretty_print: Some(true),
        delimiter: None,
    };

    // Store the scraped data
    store(&scraped_data, &storage_options).expect("Failed to store data");
}
