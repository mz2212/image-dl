use super::Url;
use std::io::Read;
use select::document::Document;
use select::predicate::Class;

// Find all the image links for a page
// This could end up being a lot more complex... 
// But this generic implementation should work in most cases

pub fn get_image_links(url: Url, element: &str, client: reqwest::blocking::Client) -> Vec<Url> {
	let mut response = client.get(url.as_str()).send().unwrap();
	let mut body = String::new();
	response.read_to_string(&mut body).unwrap();
	let dom = Document::from(body.as_str());
	let mut links: Vec<Url> = Vec::new();
	for node in dom.find(Class(element)) {
		links.push(Url::parse(node.attr("href").unwrap()).unwrap_or_else(|_| {
			let mut temp_url = url.clone();
			temp_url.set_path(node.attr("href").unwrap());
			temp_url
		}));
	}

	links
}