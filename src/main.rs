extern crate select;
extern crate url;
extern crate reqwest;

use std::env;
use url::Url;
use std::fs::OpenOptions;
use std::io;

mod parser;

fn main() {
    let input = env::args().last().unwrap();
    let url = Url::parse(input.as_str()).unwrap();

    let links = match url.domain().unwrap() {
        "www.steamcardexchange.net" => parser::get_image_links(url, "element-link-right"),
        _ => panic!("Unsupported site"),
    };

    for link in links {
        let filename = link.path_segments().unwrap().last().unwrap();
        println!("{}", filename);

        let mut file = match OpenOptions::new().write(true).create_new(true).open(filename) {
            Err(_) => continue,
            Ok(f) => f,
        };

        let mut image = reqwest::blocking::get(link.as_str()).unwrap();
        io::copy(&mut image, &mut file).unwrap();
    }
}