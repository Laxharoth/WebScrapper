use html_parser;
use html_parser::Dom;
use html_parser::Element;

pub enum FilterType {
    And,
    Or,
}

pub struct TagFilter {
    pub filter: Vec<String>,
}

pub struct AttributeFilter {
    pub filter: Vec<(String, String)>,
    pub filter_type: FilterType,
}

pub struct IdFilter {
    pub filter: Vec<String>,
}

pub struct ClassFilter {
    pub filter: Vec<String>,
    pub filter_type: FilterType,
}


pub struct TextFilter {
    pub filter: Vec<String>,
    pub filter_type: FilterType,
}

pub struct ScrapeOptions {
    pub tags: TagFilter,
    pub id_filter: Option<IdFilter>,
    pub class_filter: Option<ClassFilter>,
    pub attributes_include: Option<AttributeFilter>,
    pub attributes_exclude: Option<AttributeFilter>,
    pub text_include: Option<TextFilter>,
    pub text_exclude: Option<TextFilter>,
}

pub fn scrape(raw_html: &str, options:ScrapeOptions) -> Vec<String> {
    let html = Dom::parse(raw_html).unwrap();
    let mut current:Vec<html_parser::Node> = Vec::new();
    let mut result:Vec<String> = Vec::new();
    for node in html.children {
        current.push(node);
        while !current.is_empty(){
            let node = current.pop().unwrap();
            if let Some(element) = node.element(){
                element.children.iter().for_each(|x| current.push(x.clone()));
            }
            if let Some(ref id_filter) = options.id_filter{
                if !has_id(node.element(), id_filter){
                    continue;
                }
            }
            if let Some(ref class_filter) = options.class_filter{
                if !has_class(node.element(), class_filter){
                    continue;
                }
            }
            if !has_tagname(node.element(), &options.tags){
                continue;
            }
            if let Some(ref attributes) = options.attributes_include{
                if !fulfill_attribute_filter(node.element(), attributes){
                    continue;
                }
            }
            if let Some(ref attributes) = options.attributes_exclude{
                if fulfill_attribute_filter(node.element(), attributes){
                    continue;
                }
            }
            if let Some(ref text_filters) = options.text_include{
                if !filter_by_text(node.element(), text_filters){
                    continue;
                }
            }
            if let Some(ref text_filters) = options.text_exclude{
                if filter_by_text(node.element(), text_filters){
                    continue;
                }
            }            
            result.push(node.element().unwrap().source_span.text.clone());
        }
    }
    return result;
}

fn has_tagname(element: Option<&Element>, tags: &TagFilter) -> bool {
    let element = match element {
        Some(e) => return tags.filter.iter().any(|x| x.eq(&e.name)),
        None => return false,
    };
}

fn has_id(element: Option<&Element>, id_filter: &IdFilter) -> bool {
    let element = match element {
        Some(e) => e,
        None => return false,
    };
    id_filter.filter.iter().any(|id| {
        element.id.clone().map_or(false, |v| v.eq(id))
    })
}

fn has_class(element: Option<&Element>, class_filter: &ClassFilter) -> bool {
    let element = match element {
        Some(e) => e,
        None => return false,
    };
    match class_filter.filter_type {
        FilterType::And => class_filter.filter.iter().all(|class| {
            element.classes.iter().any(|c| c.eq(class))
        }),
        FilterType::Or => class_filter.filter.iter().any(|class| {
            element.classes.iter().any(|c| c.eq(class))
        }),
    }
}

fn fulfill_attribute_filter(element: Option<&Element>, attributes: &AttributeFilter) -> bool {
    let element = match element {
        Some(e) => e,
        None => return false,
    };
    match attributes.filter_type {
        FilterType::And => attributes.filter.iter().all(|(key, value)| {
            match key.to_lowercase().as_str() {
                "class" => return element.classes.iter().any(|c| c.eq(value)),
                "id" => return element.id == Some(value.clone()),
                _ => element.attributes.get(key).map_or(false, |v| v.clone().unwrap().eq(value)),
            }
        }),
        FilterType::Or => attributes.filter.iter().any(|(key, value)| {
            element.attributes.get(key).map_or(false, |v| v.clone().unwrap().eq(value))
        }),
    }
}

fn filter_by_text(element: Option<&Element>, text_filters: &TextFilter) -> bool {
    let element = match element {
        Some(t) => t,
        None => return false,
    };
    let node_text = &element.source_span.text;
    match text_filters.filter_type{
        FilterType::And => text_filters.filter.iter().all(|fragment|{
            node_text.contains(fragment)
        }),
        FilterType::Or => text_filters.filter.iter().any(|fragment|{
            node_text.contains(fragment)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_dom(raw_html: &str) -> Dom {
        Dom::parse(raw_html).unwrap()
    }

    #[test]
    fn test_has_tagname() {
        let dom = init_dom("<div></div>");
        let element = dom.children.get(0).unwrap().element();
        let tags = TagFilter {
            filter: vec!["div".to_string()],
        };
        let tags_fail = TagFilter {
            filter: vec!["fail".to_string()],
        };
        // should return true
        assert!(has_tagname(element, &tags));
        // should return false
        assert!(!has_tagname(element, &tags_fail));
    }

    #[test]
    fn test_has_id() {
        let dom = init_dom("<div id='test'></div>");
        let element = dom.children.get(0).unwrap().element();
        let id_filter_success = IdFilter {
            filter: vec!["test".to_string()],
        };
        let id_filter_fail = IdFilter {
            filter: vec!["fail".to_string()],
        };
        // should return true
        assert!(has_id(element, &id_filter_success));
        // should return false
        assert!(!has_id(element, &id_filter_fail));
    }
    
    #[test]
    fn test_has_class_and() {
        let dom = init_dom("<div class='test city note logo animal fruit'></div>");
        let element = dom.children.get(0).unwrap().element();
        let class_filter_success = ClassFilter {
            filter: vec!["test".to_string(), "city".to_string(), "animal".to_string()],
            filter_type: FilterType::And,
        };
        let class_filter_fail = ClassFilter {
            filter: vec!["test".to_string(), "city".to_string(), "fail".to_string()],
            filter_type: FilterType::And,
        };
        // should return true
        assert!(has_class(element, &class_filter_success));
        // should return false
        assert!(!has_class(element, &class_filter_fail));
    }

    #[test]
    fn test_has_class_or() {
        let dom = init_dom("<div class='test city note logo animal fruit'></div>");
        let element = dom.children.get(0).unwrap().element();
        let class_filter_success = ClassFilter {
            filter: vec!["test".to_string(), "fail".to_string(), "animal".to_string()],
            filter_type: FilterType::Or,
        };
        let class_filter_fail = ClassFilter {
            filter: vec!["giorno".to_string(), "log".to_string(), "fail".to_string()],
            filter_type: FilterType::Or,
        };
        // should return true
        assert!(has_class(element, &class_filter_success), "should return true as at least one class matches");
        // should return false
        assert!(!has_class(element, &class_filter_fail), "should return false as no class matches");
    }

    #[test]
    fn test_fulfill_attribute_filter_and() {
        let dom = init_dom("<div height='test' width='test'></div>");
        let element = dom.children.get(0).unwrap().element();
        let attribute_filter = AttributeFilter {
            filter: vec![("height".to_string(), "test".to_string()), ("width".to_string(), "test".to_string())],
            filter_type: FilterType::And,
        };
        let attribute_filter_fail = AttributeFilter {
            filter: vec![("height".to_string(), "fail".to_string()), ("width".to_string(), "test".to_string())],
            filter_type: FilterType::And,
        };
        assert!(fulfill_attribute_filter(element, &attribute_filter), "should return true as all attributes match");
        assert!(!fulfill_attribute_filter(element, &attribute_filter_fail), "should return false as not all attributes match");
    }

    #[test]
    fn test_fulfill_attribute_filter_or() {
        let dom = init_dom("<div height='test' width='test'></div>");
        let element = dom.children.get(0).unwrap().element();
        let attribute_filter = AttributeFilter {
            filter: vec![("height".to_string(), "fail".to_string()), ("width".to_string(), "test".to_string())],
            filter_type: FilterType::Or,
        };
        let attribute_filter_fail = AttributeFilter {
            filter: vec![("height".to_string(), "null".to_string()), ("width".to_string(), "void".to_string())],
            filter_type: FilterType::Or,
        };
        assert!(fulfill_attribute_filter(element, &attribute_filter), "should return true as at least one attribute match");
        assert!(!fulfill_attribute_filter(element, &attribute_filter_fail), "should return false as no attribute match");
    }

    #[test]
    fn test_fulfill_attribute_filter_with_id_and_class() {
        let dom = init_dom("<div id='test_id' class='test_class'></div>");
        let element = dom.children.get(0).unwrap().element();

        // Test for id attribute
        let attribute_filter_id = AttributeFilter {
            filter: vec![("id".to_string(), "test_id".to_string())],
            filter_type: FilterType::And,
        };
        assert!(fulfill_attribute_filter(element, &attribute_filter_id), "should return true as id attribute matches");

        // Test for class attribute
        let attribute_filter_class = AttributeFilter {
            filter: vec![("class".to_string(), "test_class".to_string())],
            filter_type: FilterType::And,
        };
        assert!(fulfill_attribute_filter(element, &attribute_filter_class), "should return true as class attribute matches");

        // Test for non-matching id attribute
        let attribute_filter_id_fail = AttributeFilter {
            filter: vec![("id".to_string(), "wrong_id".to_string())],
            filter_type: FilterType::And,
        };
        assert!(!fulfill_attribute_filter(element, &attribute_filter_id_fail), "should return false as id attribute does not match");

        // Test for non-matching class attribute
        let attribute_filter_class_fail = AttributeFilter {
            filter: vec![("class".to_string(), "wrong_class".to_string())],
            filter_type: FilterType::And,
        };
        assert!(!fulfill_attribute_filter(element, &attribute_filter_class_fail), "should return false as class attribute does not match");
    }

    #[test]
    fn test_filter_by_text_and() {
        let dom = init_dom("<div>Occaecat ex minim tempor fugiat. Laborum consectetur ut et qui anim nostrud cupidatat tempor id sint eu cupidatat.</div>");
        let text = dom.children.get(0).unwrap().element();
        
        let text_filter = TextFilter {
            filter: vec!["qui anim".to_string(), "id sint eu cupidatat".to_string()],
            filter_type: FilterType::And,
        };
        let text_filter_fail = TextFilter {
            filter: vec!["minim".to_string(), "consetur".to_string()],
            filter_type: FilterType::And,
        };
        assert!(filter_by_text(text, &text_filter), "should return true as all text fragments are present");
        assert!(!filter_by_text(text, &text_filter_fail), "should return false as not all text fragments are present");
    }

    #[test]
    fn test_filter_by_text_or() {
        let dom = init_dom("<div>Occaecat ex minim tempor fugiat. Laborum consectetur ut et qui anim nostrud cupidatat tempor id sint eu cupidatat.</div>");
        let text = dom.children.get(0).unwrap().element();
        let text_filter = TextFilter {
            filter: vec!["adsfasdfasd".to_string(), "cupidatat tempor".to_string()],
            filter_type: FilterType::Or,
        };
        let text_filter_fail = TextFilter {
            filter: vec!["burip".to_string(), "consetur".to_string()],
            filter_type: FilterType::Or,
        };
        assert!(filter_by_text(text, &text_filter), "should return true as at least one text fragment is present");
        assert!(!filter_by_text(text, &text_filter_fail), "should return false as no text fragment is present");
    }

    // TODO ADD CASES FOR DIFFERENT FILTER OPTIONS
    #[test]
    fn test_scrape() {
        let raw_html = r#"
            <div class='test' id='div1' data-role='main'>hello world</div>
            <span class='test' id='span1' data-role='secondary'>hello rust</span>
            <div class='test' id='div2' data-role='main'>goodbye world</div>
            <div class='example' id='div3' data-role='main'>hello universe</div>
            <span class='example' id='span2' data-role='secondary'>goodbye rust</span>
        "#;

        // Case 1: Filter by tag and class
        let options1 = ScrapeOptions {
            tags: TagFilter {
                filter: vec!["div".to_string()],
            },
            id_filter: None,
            class_filter: Some(ClassFilter {
                filter: vec!["test".to_string()],
                filter_type: FilterType::And,
            }),
            attributes_include: None,
            attributes_exclude: None,
            text_include: None,
            text_exclude: None,
        };
        let result1 = scrape(raw_html, options1);
        assert_eq!(result1, vec!["<div class='test' id='div1' data-role='main'>hello world</div>", "<div class='test' id='div2' data-role='main'>goodbye world</div>"]);

        // Case 2: Filter by tag and id
        let options2 = ScrapeOptions {
            tags: TagFilter {
                filter: vec!["span".to_string()],
            },
            id_filter: Some(IdFilter {
                filter: vec!["span1".to_string()],
            }),
            class_filter: None,
            attributes_include: None,
            attributes_exclude: None,
            text_include: None,
            text_exclude: None,
        };
        let result2 = scrape(raw_html, options2);
        assert_eq!(result2, vec!["<span class='test' id='span1' data-role='secondary'>hello rust</span>"]);

        // Case 3: Filter by text include
        let options3 = ScrapeOptions {
            tags: TagFilter {
                filter: vec!["div".to_string(), "span".to_string()],
            },
            id_filter: None,
            class_filter: None,
            attributes_include: None,
            attributes_exclude: None,
            text_include: Some(TextFilter {
                filter: vec!["hello".to_string()],
                filter_type: FilterType::Or,
            }),
            text_exclude: None,
        };
        let result3 = scrape(raw_html, options3);
        assert_eq!(result3, vec![
            "<div class='test' id='div1' data-role='main'>hello world</div>",
            "<span class='test' id='span1' data-role='secondary'>hello rust</span>",
            "<div class='example' id='div3' data-role='main'>hello universe</div>"
        ]);

        // Case 4: Filter by attribute include
        let options4 = ScrapeOptions {
            tags: TagFilter {
                filter: vec!["div".to_string()],
            },
            id_filter: None,
            class_filter: None,
            attributes_include: Some(AttributeFilter {
                filter: vec![("data-role".to_string(), "main".to_string())],
                filter_type: FilterType::And,
            }),
            attributes_exclude: None,
            text_include: None,
            text_exclude: None,
        };
        let result4 = scrape(raw_html, options4);
        assert_eq!(result4, vec![
            "<div class='test' id='div1' data-role='main'>hello world</div>",
            "<div class='test' id='div2' data-role='main'>goodbye world</div>",
            "<div class='example' id='div3' data-role='main'>hello universe</div>"
        ]);

        // Case 5: Filter by multiple criteria
        let options5 = ScrapeOptions {
            tags: TagFilter {
                filter: vec!["div".to_string()],
            },
            id_filter: Some(IdFilter {
                filter: vec!["div1".to_string(), "div2".to_string()],
            }),
            class_filter: Some(ClassFilter {
                filter: vec!["test".to_string()],
                filter_type: FilterType::And,
            }),
            attributes_include: None,
            attributes_exclude: None,
            text_include: Some(TextFilter {
                filter: vec!["world".to_string()],
                filter_type: FilterType::Or,
            }),
            text_exclude: None,
        };
        let result5 = scrape(raw_html, options5);
        assert_eq!(result5, vec![
            "<div class='test' id='div1' data-role='main'>hello world</div>",
            "<div class='test' id='div2' data-role='main'>goodbye world</div>"
        ]);
    }
}
