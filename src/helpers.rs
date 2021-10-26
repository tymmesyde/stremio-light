use std::borrow::Cow;

use rust_embed::RustEmbed;
use mime_guess::from_path;
use wry::{Error, application::window::Icon, http::{Response, ResponseBuilder}};

#[derive(RustEmbed)]
#[folder = "dist/"]
#[prefix = "/"]
struct Dist;

pub fn load_icon(bytes: &[u8]) -> Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load_from_memory(bytes)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}

pub fn handle_file_request(path: &String) -> Result<Response, Error> {
    match Dist::get(path) {
        Some(content) => {
            let body: Vec<u8> = match content.data {
                Cow::Borrowed(bytes) => bytes.into(),
                Cow::Owned(bytes) => bytes.into(),
            };
            ResponseBuilder::new().mimetype(from_path(path).first_or_octet_stream().as_ref()).body(body)
        }
        None => ResponseBuilder::new().status("404").body(vec![0])
    }
}