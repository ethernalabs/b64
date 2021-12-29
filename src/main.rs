#[macro_use]
extern crate clap;
use clap::App;

extern crate base64;
//extern crate copypasta-ext;

use base64::{encode_config, decode_config};

use copypasta_ext::prelude::*;
use copypasta_ext::x11_fork::ClipboardContext;

fn main() {
    let yaml = load_yaml!("args.yml");
    let operation = App::from_yaml(yaml).get_matches();
    let mut ctx = ClipboardContext::new().unwrap();

    if let Some(d) = operation.value_of("decode") {
        let value = decode_config(d, base64::URL_SAFE).unwrap();
        let decoded_value = std::str::from_utf8(&value).unwrap();
        ctx.set_contents(decoded_value.into()).unwrap();
    }

    if let Some(e) = operation.value_of("encode") {
        let encoded_string = encode_config(e, base64::URL_SAFE);
        ctx.set_contents(encoded_string.into()).unwrap();
    }
}
