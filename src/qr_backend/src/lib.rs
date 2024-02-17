use candid::{CandidType, Deserialize};
use std::include_bytes;
mod core;

const IMAGE_SIZE_IN_PIXEL: usize = 1024;
const LOGO_TRANSPARENT: &[u8] = include_bytes!("./assets/logo_transparent.png");
const LOGO_WHITE: &[u8] = include_bytes!("./assets/logo_white.png");
#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}


#[derive(CandidType, Deserialize)]
struct Options
    {
        add_logo:bool,
        add_gradient:bool,
        add_transparency: Option<bool>,
    }

#[derive(CandidType, Deserialize)]
struct QRError
    {
        message: String,
    }

#[derive(CandidType, Deserialize)]
enum QRResult
    {
        Image(Vec<u8>),
        Err(QRError),
    }

fn qrcode(input: String, options: Options) -> QRResult
    {
        let logo = if options.add_transparency == Some(true)
            {
                LOGO_TRANSPARENT
            }
        else
            {
                LOGO_WHITE
            };
        let result = match core::generate(input, options, logo, IMAGE_SIZE_IN_PIXEL)
            {
                Ok(blob) => QRResult::Image(blob),
                Err(err) => QRResult::Err(QRError{message: err.to_string()}),
            };
        ic_cdk::println!("Executed Instructions: {}", ic_cdk::api::performance_counter(0));
        result
    }

#[ic_cdk::update]
fn qrcode_update(input: String, options: Options) -> QRResult
    {
        qrcode(input, options)
    }

#[ic_cdk::query]
fn qrcode_query(input: String, options: Options) -> QRResult
    {
        qrcode(input, options)
    }