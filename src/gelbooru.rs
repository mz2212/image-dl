use super::Url;
use std::io::Read;
use select::document::Document;
use select::predicate::{Class, Name, Attr, And};

// Find all the image links for a page
// Need a different parser for boorus. 
// This one's only for gelbooru, where they do all sorts of strange stuff >-<

pub fn get_image_links(url: Url, client: reqwest::blocking::Client) -> Vec<Url> {
	let mut response = client.get(url.as_str()).send().unwrap();
	let mut body = String::new();
	response.read_to_string(&mut body).unwrap();
	let dom = Document::from(body.as_str());
	let mut links: Vec<Url> = Vec::new();

	for node in dom.find(And(Name("img"), Class("thumbnail-preview"))) {
		let mut post_body = String::new(); // This next line is nasty, but for some reason they don't include the protocol in their hrefs
		let mut post_url = Url::parse(format!("{}{}", "https:", node.parent().unwrap().attr("href").unwrap()).as_str()).unwrap();
		println!("{}", post_url.as_str());
		response = client.get(post_url.as_str()).send().unwrap();
		// for boorus this should only be one element...
		response.read_to_string(&mut post_body).unwrap();
		let post_dom = Document::from(post_body.as_str());
		let post_image = post_dom.find(Attr("id", "image"))
			.next().unwrap();
		
		// A workaround for video posts... It might not work in all cases.
		let image_path = post_image.attr("src").unwrap_or_else(|| {
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