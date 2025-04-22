# WebScrapper

WebScrapper is a Rust-based library for fetching, scraping, and storing web content. It provides modular components for downloading HTML content, extracting specific elements using filters, and saving the results in various formats such as JSON, XML, CSV, YAML, and plain text.

## Features

- **Fetcher Module**: Fetch HTML content from a given URL using `reqwest`.
- **Scrapper Module**: Extract specific elements from the HTML using customizable filters.
- **Storage Module**: Save the scraped data in multiple formats (JSON, XML, CSV, YAML, or plain text).

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/WebScrapper.git
   cd WebScrapper
   ```

2. Ensure you have Rust installed. If not, install it from [rustup.rs](https://rustup.rs/).

3. Build the project:
   ```bash
   cargo build
   ```

4. Run the project:
   ```bash
   cargo run
   ```

## Usage

### Fetching HTML Content

The `fetcher` module provides a function to fetch HTML content from a URL:
```rust
let url = "https://example.com";
let raw_html = fetcher::fetch::fetch(url).expect("Failed to fetch HTML");
```

### Scraping HTML Content

The `scrapper` module allows you to extract specific elements using filters:
```rust
use scrapper::scrap::{scrape, ScrapeOptions, TagFilter};

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

let scraped_data = scrape(&raw_html, scrape_options);
```

### Storing Scraped Data

The `storage` module provides functionality to save the scraped data in various formats:
```rust
use storage::storage::{store, StorageOptions, FileFormat};

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

store(&scraped_data, &storage_options).expect("Failed to store data");
```

### Example

Here is a complete example that uses all three modules:
```rust
mod fetcher;
mod scrapper;
mod storage;

use fetcher::fetch::fetch;
use scrapper::scrap::{scrape, ScrapeOptions, TagFilter};
use storage::storage::{store, StorageOptions, FileFormat};

fn main() {
    let url = "https://example.com";
    let raw_html = fetch(url).expect("Failed to fetch HTML");

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

    let scraped_data = scrape(&raw_html, scrape_options);

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

    store(&scraped_data, &storage_options).expect("Failed to store data");
}
```

## Supported Formats

- **JSON**: Pretty-printed or compact.
- **XML**: With optional pretty-printing.
- **CSV**: Customizable delimiter.
- **YAML**: Indented output.
- **Plain Text**: Raw HTML content.

## Dependencies

- [html_parser](https://crates.io/crates/html_parser): For parsing HTML content.
- [reqwest](https://crates.io/crates/reqwest): For HTTP requests.
- [tokio](https://crates.io/crates/tokio): For asynchronous runtime.

## Testing

Run the tests using:
```bash
cargo test
```

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
