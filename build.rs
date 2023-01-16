use dotenvy::dotenv;
use std::env::var;

fn main() {
    dotenv().ok();

    if var("OPENAI_KEY").is_err() {
        println!("cargo:rustc-cfg=no_key");
    } else if var("OPENAI_KEY").unwrap() == "" {
        println!("cargo:rustc-cfg=no_key");
    }
}
