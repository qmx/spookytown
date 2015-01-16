#![allow(unstable)]
extern crate handlebars;
extern crate rustdoc;
extern crate "rustc-serialize" as serialize;
extern crate getopts;

use std::os;
use getopts::{reqopt,optflag,getopts};
use std::io::fs;
use std::io::fs::PathExtensions;
use std::io::{File, USER_RWX};
use std::collections::BTreeMap;
use handlebars::{Handlebars};
use rustdoc::html::markdown::Markdown;

fn get_layout_template() -> String {
    let mut file = match File::open(&Path::new("_layouts/base.html.hbs")) {
        Err(why) => panic!("{} layout template", why.desc),
        Ok(file) => file
    };
    match file.read_to_string() {
        Err(why) => panic!("could not open layout template {}", why.desc),
        Ok(content) => content
    }
}

fn main() {
    let args: Vec<String> = os::args();


    let opts = &[
        reqopt("d", "", "set output dir", "NAME"),
        optflag("h", "help", "print this help menu")
            ];

    let matches = match getopts(args.tail(), opts) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string())
    };

    let outdir = matches.opt_str("d").unwrap();
    let dest_dir = Path::new(outdir);

    if dest_dir.exists() {
        panic!("destination dir already exists");
    } else {
        match fs::mkdir(&dest_dir, USER_RWX) {
            Err(why) => panic!("{}", why.desc),
            Ok(_) => {}
        }
    }


    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("layout", get_layout_template())
        .ok().expect("template creation failed");


    let mut pages: Vec<Path> = Vec::new();
    match fs::walk_dir(&Path::new(".")) {
        Err(why) => panic!("! {:?}", why.kind),
        Ok(paths) => for path in paths.filter(|ref p| p.is_file()) {
            let name = path.as_str().unwrap_or("");
            if !name.starts_with("_") {
                pages.push(path.clone());
            }
        }
    }

    for page in pages.iter() {
        let mut t = BTreeMap::new();
        let mut file = match File::open(page) {
            Err(why) => panic!("{}", why.desc),
            Ok(file) => file
        };
        let content = match file.read_to_string() {
            Err(why) => panic!("could not open layout template {}", why.desc),
            Ok(content) => content
        };

        t.insert("content".to_string(), content);
        let output = match handlebars.render("layout", &t) {
            Err(why) => panic!("{}", why.desc),
            Ok(o) => o
        };
        let target = dest_dir.join(page);
        match File::create(&target) {
            Err(why) => panic!("{}", why.desc),
            Ok(mut f) => f.write_str(output.as_slice())
        };
    }
    println!("{}", Markdown("I **hate** EJBs"));
}

