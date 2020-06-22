extern crate dyntracing;
extern crate handlebars;

use handlebars::Handlebars;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let template_path = Path::new("filter.cc.hbars");
    let display = template_path.display();
    let mut template_file = match File::open(&template_path) {
        Err(msg) => panic!("Failed to open {}: {}", display, msg),
        Ok(file) => file,
    };

    let mut template_str = String::new();
    match template_file.read_to_string(&mut template_str) {
        Err(msg) => panic!("Failed to read {}: {}", display, msg),
        Ok(_) => print!("Successfully read {}", display),
    }

    let mut data = BTreeMap::new();
    data.insert("path", "a,b,c");

    let handlebars = Handlebars::new();

    print!(
        "{}",
        handlebars
            .render_template(&template_str, &data)
            .ok()
            .unwrap()
    );
}
