pub enum FilterType {
    And,
    Or,
}

pub struct tagFilter {
    pub filter: Vec<String>,
}

pub struct attributeFilter {
    pub filter: Vec<(String, String)>,
    pub filterType: FilterType,
}

pub struct textFilter {
    pub filter: Vec<String>,
    pub filterType: FilterType,
}

pub struct ScrapeOptions {
    pub tags: tagFilter,
    pub attributesInclude: Option<attributeFilter>,
    pub attributesExclude: Option<attributeFilter>,
    pub textInclude: Option<(Vec<textFilter>, FilterType)>,
    pub textExclude: Option<(Vec<textFilter>, FilterType)>,
}

pub fn scrape(raw_html: &str, options:ScrapeOptions) -> Vec<String> {
    // Function implementation
    vec![]
}

fn scrape_tags(raw_html: &str, tags: &tagFilter) -> Vec<String> {
    // Function implementation
    vec![]
}

fn filter_by_attributes(html_tags: &Vec<String>, attributes: &attributeFilter) -> Vec<String> {
    // Function implementation
    vec![]
}

fn filter_by_text(html_tags: &Vec<String>, text_filters: &Vec<textFilter>) -> Vec<String> {
    // Function implementation
    vec![]
}