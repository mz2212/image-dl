use super::Url;
use std::io::Read;
use select::document::Document;
use select::predicate::{Class, Name};

// Find all the image links for a page
// Need a different parser for boorus. 
// They don't host links to the full size image in search results.
// node_attr might be src or href, depending on how the booru is set up

pub fn get_image_links(url: Url, element: &str, thumb_element: &str, node_attr: &str) -> Vec<Url> {
	let mut response = reqwest::blocking::get(url.as_str()).unwrap();
	let mut body = String::new();
	response.read_to_string(&mut body).unwrap();
	let dom = Document::from(body.as_str());
	let mut links: Vec<Url> = Vec::new();

	for node in dom.find(Class(thumb_element)) {
		let mut post_url = url.clone();
		let mut post_body = String::new();
		post_url.set_path(node.attr("href").unwrap());
		println!("{}", post_url.as_str());
		response = reqwest::blocking::get(post_url.as_str()).unwrap();
		// for boorus this should only be one element...
		response.read_to_string(&mut post_body).unwrap();
		let post_dom = Document::from(post_body.as_str());
		let post_image = post_dom.find(Class(element))
			.next().unwrap();
		
		// A workaround for video posts... It might not work in all cases.
		let image_path = post_image.attr(node_attr).unwrap_or_else(|| {
			post_dom.find(Name("source")).next().unwrap()
				.attr("src").unwrap()
		});

		let image_link = Url::parse(image_path).unwrap_or_else(|_| {
			post_url.set_path(image_path);
			post_url
		});
		links.push(image_link);
	}

	links
}