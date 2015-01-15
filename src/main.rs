#![allow(unstable)]
extern crate handlebars;
extern crate "rustc-serialize" as serialize;

use std::io::File;
use std::collections::BTreeMap;
use serialize::json::{Json, ToJson};
use handlebars::{Handlebars};
fn main() {
    let mut handlebars = Handlebars::new();
    let path = Path::new("template.hbs");
    let mut file = match File::open(&path) {
        Err(why) => panic!("meh {}", why.desc),
        Ok(file) => file,
    };

    let t = match file.read_to_string() {
        Err(why) => panic!("meh {}", why.desc),
        Ok(string) => string
    };
    handlebars.register_template_string("hello", t)
        .ok().expect("template creation failed");

    let mut t = BTreeMap::new();
    t.insert("name".to_string(), "whoa2".to_json());

    println!("{}", handlebars.render("hello", &t).unwrap());
}
