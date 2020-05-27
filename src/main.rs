extern crate select;
extern crate url;
extern crate reqwest;

use std::env;
use url::Url;
use std::fs::OpenOptions;
use std::io;

mod parser;
mod booru;
mod gelbooru;

// ffs I'm duplicating a project: https://bionus.github.io/imgbrd-grabber/
// Just use that instead. *cries in spagett code*

fn main() {
    let input = env::args().last().unwrap();
    let url = Url::parse(input.as_str()).unwrap();

    let links = match url.domain().unwrap() {
        "www.steamcardexchange.net" => parser::get_image_links(url, "element-link-right"),
        //"imgur.com" => imgur::get_image_links(url), // imgur does some really janky hotloading, it's gonna get fancy...
        "www.coedcherry.com" => parser::get_image_links(url, "track"),
        "neko-booru.com" => booru::get_image_links(url, "shm-main-image", "thumb", "src"),
        "gelbooru.com" => gelbooru::get_image_links(url),
        //"derpibooru.org" => derpi::get_image_links(url), // going to need a custom one for derpi too
        "flandre.moe" => parser::get_image_links(url, "directlink"), // Also lolibooru.moe?
        "lolibooru.moe" => parser::get_image_links(url, "directlink"), // These two are a bit odd, one seems to direct to the other
        "konachan.com" => parser::get_image_links(url, "directlink"), // kona's another easy one
        "konachan.net" => parser::get_image_links(url, "directlink"),
        //"kusubooru.com" => kusu::get_image_links(url), // woo custom parsers
        "evbooru.com" => parser::get_image_links(url, "directlink"),
        //"behoimi.org" => behoimi::get_image_links(url), // :eyeroll: moar custom parsers
        //"www.bittersweetcandybowl.com" => candy::get_image_links(url), // I need to find a way to genericify these, they're mostly the same...
        "e-shuushuu.net" => parser::get_image_links(url, "thumb_image"), // Dunno what's with the naming, it's a directlink
        //"www.furaffinity.net" => furaffinity::get_image_links(url), // bugger, thought this one was going to be easy.
        //"safebooru.org" => safebooru::get_image_links(url), // Literally runs gelbooru, like most of the customs, but with slightly different tags/url scheming
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