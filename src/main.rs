extern crate select;
extern crate reqwest;

use select::document::Document;
use select::predicate::{Class, Name};

fn main() {
    allbud(vec!["jesus".to_string(), "og".to_string()])
}

fn allbud(search_terms: Vec<String>) {
    let base_url = "https://www.allbud.com";
    let search_url = format!("{}{}{}", base_url, "/marijuana-strains/search?q=", search_terms.join("+"));
    let search_resp = reqwest::get(&search_url).unwrap();
    assert!(search_resp.status().is_success());

    let strain_urls = Vec::new();
    Document::from_read(search_resp).unwrap().find(Class("object-title")).for_each(|node| {
        let a_tags = node.find(Name("a"));
        let search_strains = a_tags.map(|tag| format!("{}{}", base_url, tag.attr("href").unwrap()));
        let filtered_strain_urls = search_strains.filter(|strain| strain.contains("jesus")).filter(|strain| strain.contains("og"));
        filtered_strain_urls.for_each(|url| strain_urls.push(url)));
    })

    for url in strain_urls {
        let strain_resp = reqwest::get(&search_url).unwrap();
        assert!(strain_resp.status().is_success());

        Document::from_read(search_resp).unwrap().
    }
} 