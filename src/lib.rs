use base64::{decode, encode};
use image::load_from_memory;
use image::ImageOutputFormat::Png;
use wasm_bindgen::prelude::*;
use web_sys::console::log_1 as log;

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
    log(&"Gray scale Called".into());

    let base64_to_vector = decode(encoded_file).unwrap();
    log(&"Image Decoded".into());

    let mut img = load_from_memory(&base64_to_vector).unwrap();
    log(&"Image loaded".into());

    img = img.grayscale();
    log(&"Gray Scale Effect Applied".into());

    let mut buffer = vec![];
    img.write_to(&mut buffer, Png).unwrap();
    log(&"New Image Written".into());

    let encode_img = encode(&buffer);
    let data_url = format!("data:image/png;base64,{}", encode_img);
    data_url
}
