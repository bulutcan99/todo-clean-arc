#![allow(unused)]

mod settings;
mod model;
mod security;
mod web;

use settings::Settings;

fn main() {
    let settings = Settings::new();
    println!("Hello, world!");
}
