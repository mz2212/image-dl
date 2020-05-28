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
mod derpi;
mod kusu;
mod behoimi;

static USER_AGENT: &str = "Mozilla/5.0 (Windows NT 6.1; WOW64; rv:77.0) Gecko/20190101 Firefox/77.0";

// ffs I'm duplicating a project: https://bionus.github.io/imgbrd-grabber/
// Just use that instead. *cries in spagett code*

fn main() {
    let input = env::args().last().unwrap();
    let url = Url::parse(input.as_str()).unwrap();

    let client = reqwest::blocking::Client::builder()
        .user_agent(USER_AGENT)
        .build().unwrap();

    let links = match url.domain().unwrap() {
        "www.steamcardexchange.net" => parser::get_image_links(url, "element-link-right", client),
        "www.coedcherry.com" => parser::get_image_links(url, "track", client),
        //moebooru based sites are the BEST https://github.com/moebooru/moebooru
        "flandre.moe" => parser::get_image_links(url, "directlink", client),
        "lolibooru.moe" => parser::get_image_links(url, "directlink", client),
        "konachan.com" => parser::get_image_links(url, "directlink", client),
        "konachan.net" => parser::get_image_links(url, "directlink", client),
        "evbooru.com" => parser::get_image_links(url, "directlink", client),
        "www.sakugabooru.com" => parser::get_image_links(url, "directlink", client),
        "iibooru.org" => parser::get_image_links(url, "directlink", client),
        "yande.re" => parser::get_image_links(url, "directlink", client),
        "img.genshiken-itb.org" => parser::get_image_links(url, "directlink", client),
        "e-shuushuu.net" => parser::get_image_links(url, "thumb_image", client),
        "neko-booru.com" => booru::get_image_links(url, "shm-main-image", "thumb", "src"),
        "gelbooru.com" => gelbooru::get_image_links(url, client),
        "derpibooru.org" => derpi::get_image_links(url, client),
        "kusubooru.com" => kusu::get_image_links(url, client),
        "behoimi.org" => behoimi::get_image_links(url, client),
        //"imgur.com" => imgur::get_image_links(url), // imgur does some really janky hotloading, it's gonna get fancy...
        //"www.bittersweetcandybowl.com" => candy::get_image_links(url, client), // I need to find a way to genericify these, they're mostly the same...
        //"www.furaffinity.net" => furaffinity::get_image_links(url, client), // bugger, thought this one was going to be easy.
        //"safebooru.org" => safebooru::get_image_links(url, client), // Literally runs gelbooru, like most of the customs, but with slightly different tags/url scheming
        _ => panic!("Unsupported site"),
    };

    for link in links {
        let filename = link.path_segments().unwrap().last().unwrap();
        println!("{}", filename);
/*
        let mut file = match OpenOptions::new().write(true).create_new(true).open(filename) {
            Err(_) => continue,
            Ok(f) => f,
        };

        let mut image = client.get(link.as_str()).send().unwrap();
        io::copy(&mut image, &mut file).unwrap();*/
    }
}