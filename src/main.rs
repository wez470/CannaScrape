extern crate select;
extern crate reqwest;

use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

fn main() {
    println!("ALLBUD\n----------");
    allbud(vec!["jesus".to_string(), "og".to_string()]);
    println!("\nLEAFLY\n----------");
    leafly(vec!["jesus".to_string(), "og".to_string()]);
}

fn allbud(search_terms: Vec<String>) {
    let base_url = "https://www.allbud.com";
    let search_url = format!("{}{}{}", base_url, "/marijuana-strains/search?q=", search_terms.join("+"));
    let search_resp = reqwest::get(&search_url).unwrap();
    assert!(search_resp.status().is_success());

    let mut strain_urls = Vec::new();
    Document::from_read(search_resp).unwrap().find(Class("object-title")).for_each(|node| {
        let a_tags = node.find(Name("a"));
        let search_strains = a_tags.map(|tag| format!("{}{}", base_url, tag.attr("href").unwrap()));
        let filtered_strain_urls = search_strains.filter(|strain| {
            let mut contains_terms = true;
            search_terms.iter().for_each(|term| {
                contains_terms &= strain.contains(term);
            });
            return contains_terms;
        });
        filtered_strain_urls.for_each(|url| strain_urls.push(url));
    });

    for url in strain_urls {
        let strain_resp = reqwest::get(&url).unwrap();
        assert!(strain_resp.status().is_success());

        let doc = Document::from_read(strain_resp).unwrap();
        let rating = doc.find(Class("rating-num")).next().unwrap();
        let num_ratings = doc.find(Attr("id", "product-rating-votes")).next().unwrap();
        let split_url: Vec<&str> = url.split('/').collect();
        let name = split_url.last().unwrap().replace("-", " ");
        println!("{}:\trating: {}\tnum_ratings: {}", name, rating.inner_html().trim(), num_ratings.inner_html().trim());
    }
}

fn leafly(search_terms: Vec<String>) {
    let base_url = "https://www.leafly.com";
    let search_url = format!("{}{}{}{}", base_url, "/search?q=", search_terms.join("+"), "&typefilter=strain");
    let search_resp = reqwest::get(&search_url).unwrap();
    assert!(search_resp.status().is_success());

    let doc = Document::from_read(search_resp).unwrap();
    let mut names = Vec::new();
    let mut num_reviews = Vec::new();
    let mut ratings = Vec::new();
    doc.find(Name("li").descendant(Class("padding-rowItem")).descendant(Class("copy--bold"))).for_each(|item| {
        let name = item.text().trim().to_lowercase();
        names.push(name);
    });
    doc.find(Name("li").descendant(Class("padding-rowItem")).descendant(Class("color--light"))).for_each(|item| {
        let match_chars: &[_] = &['(', ')', ' '];
        let num_revs = item.text().trim_matches(match_chars).split_whitespace().next().unwrap().to_string();
        num_reviews.push(num_revs);
    });
    doc.find(Name("li").descendant(Class("padding-rowItem")).descendant(Name("img"))).for_each(|item| {
        let rating = item.attr("src").unwrap().split('/').nth(2).unwrap();
        ratings.push(rating);
    });

    for review in names.iter().zip(num_reviews.iter().zip(ratings.iter())) {
        let mut contains_terms = true;
        search_terms.iter().for_each(|term| {
            contains_terms &= review.0.contains(term);
        });
        if contains_terms {
            println!("{}:\trating: {}\tnum_ratings: {}", review.0, (review.1).1, (review.1).0);
        }
    }
}