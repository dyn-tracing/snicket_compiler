extern crate handlebars;
extern crate serde;

use handlebars::Handlebars;
use serde::Serialize;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Serialize)]
pub enum Value {
    Float(f64),
    Bool(bool),
}

#[derive(Serialize)]
pub struct Data {
    value: Value,
}

fn main() {
    let template_path = Path::new("./examples/enum_template.handlebars");
    let display = template_path.display();
    let mut template_file = match File::open(&template_path) {
        Err(msg) => panic!("Failed to open {}: {}", display, msg),
        Ok(file) => file,
    };

    let mut template_str = String::new();
    match template_file.read_to_string(&mut template_str) {
        Err(msg) => panic!("Failed to read {}: {}", display, msg),
        Ok(_) => println!("Successfully read {}", display),
    }

    let handlebars = Handlebars::new();

    println!(
        "{}",
        handlebars
            .render_template(
                &template_str,
                &Data {
                    value: Value::Float(3.0)
                }
            )
            .expect("handlebar render failed")
    );

    println!(
        "{}",
        handlebars
            .render_template(
                &template_str,
                &Data {
                    value: Value::Bool(true)
                }
            )
            .expect("handlebar render failed")
    );
}
