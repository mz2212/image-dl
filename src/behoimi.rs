use super::Url;
use std::io::Read;
use select::document::Document;
use select::predicate::{Class, Attr};

// This booru is obnoxious in my attempts to automatically download.
// Imma go all out with this one and see if it's a sample image, then download the full thing.


pub fn get_image_links(url: Url, client: reqwest::blocking::Client) -> Vec<Url> {
	let mut response = client.get(url.as_str()).send().unwrap();
	let mut body = String::new();
	response.read_to_string(&mut body).unwrap();
	let dom = Document::from(body.as_str());
	let mut links: Vec<Url> = Vec::new();

	for node in dom.find(Class("preview")) {
		let mut post_body = String::new();
		let mut post_url = url.clone();
		post_url.set_path(node.parent().unwrap().attr("href").unwrap());
		println!("{}", post_url.as_str());
		response = client.get(post_url.as_str()).send().unwrap();
		response.read_to_string(&mut post_body).unwrap();
		let post_dom = Document::from(post_body.as_str());
		let post_image = post_dom.find(Attr("id", "image")).next().unwrap();
		
		let image_path = post_image.attr("src").unwrap();

		let image_link = Url::parse(image_path).unwrap_or_else(|_| {
			post_url.set_path(image_path);
			post_url
		});

		let mut path_segments = image_link.path_segments().unwrap();
		let filename = path_segments.clone().last().unwrap().trim_start_matches("sample");
		let mut image_link_mod = image_link.clone(); // This RETURNS a Url instead of modifying the Url
		
		if path_segments.nth(1).unwrap() == "sample" {
			// not sure how to remove the /data/sample/whatever/image.jpg from this... might be able to push path_segments
			image_link_mod.path_segments_mut().unwrap().clear().push("data").extend(path_segments);
		}

		image_link_mod = image_link_mod.join(filename).unwrap();

		links.push(image_link_mod);
	}

	links
}
