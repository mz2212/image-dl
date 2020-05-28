use super::Url;
use std::io::Read;
use select::document::Document;
use select::predicate::{Class, Attr};


pub fn get_image_links(url: Url, client: reqwest::blocking::Client) -> Vec<Url> {
	let mut response = client.get(url.as_str()).send().unwrap();
	let mut body = String::new();
	response.read_to_string(&mut body).unwrap();
	let dom = Document::from(body.as_str());
	let mut links: Vec<Url> = Vec::new();

	for node in dom.find(Class("shm-thumb")) {
		let mut post_body = String::new();
		let mut post_url = url.clone();
		post_url.set_path(node.attr("href").unwrap());
		println!("{}", post_url.as_str());
		response = client.get(post_url.as_str()).send().unwrap();
		// for boorus this should only be one element...
		response.read_to_string(&mut post_body).unwrap();
		let post_dom = Document::from(post_body.as_str());
		let post_image = post_dom.find(Attr("id", "main_image")).next().unwrap();
		
		let image_path = post_image.attr("src").unwrap();

		let image_link = Url::parse(image_path).unwrap_or_else(|_| {
			post_url.set_path(image_path);
			post_url
		});
		links.push(image_link);
	}

	links
}
