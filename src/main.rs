extern crate reqwest;
extern crate select;

use colored::*;
use select::document::Document;
use select::predicate::Class;
use webbrowser;

fn get_html(url: &str) -> reqwest::Response {
    let resp = reqwest::get(url).unwrap();
    assert!(resp.status().is_success());

    resp
}

fn get_links(doc: reqwest::Response) -> select::document::Document {
    let document = Document::from_read(doc).unwrap();

    document
}

fn main() {
    let doc = get_links(get_html("http://www.vodovod-pula.hr"));

    let mut links = Vec::new();

    let mut n = 0;

    for node in doc.find(Class("mod-articles-category-title")) {
        links.push(node.attr("href").unwrap());
    }

    println!("\nHitne intervencije i obavijesti");
    for link in &links {
        match link.contains("jadreski") {
            true => {
                println!("{}", link.yellow());
                n += 1;
            }
            false => println!("{}", link),
        }
    }

    println!();

    if n != 0 {
        webbrowser::open("http://www.vodovod-pula.hr").unwrap();
    }
}
