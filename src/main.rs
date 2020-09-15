extern crate clap;
extern crate dyntracing;
extern crate handlebars;
extern crate serde;

use clap::{App, Arg};
use dyntracing::{code_gen, lexer, parser, tree_fold::TreeFold};
use handlebars::Handlebars;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let matches = App::new("Dynamic Tracing")
        .arg(
            Arg::with_name("query")
                .short("q")
                .long("query")
                .required(true)
                .value_name("FILE")
                .help("Sets the .cql query file to use"),
        )
        .arg(
            Arg::with_name("udf")
                .short("u")
                .long("udf")
                .value_name("UDF_FILE")
                .help("Optionally sets .cc user defined function file to use"),
        )
        .get_matches();

    // Read query from file specified by command line argument.
    let query_file = matches.value_of("query").unwrap();
    let query = fs::read_to_string(query_file)
        .unwrap_or_else(|_| panic!("failed to read file {}", query_file));

    let mut config = code_gen::CodeGenConfig::new();

    // Parse udf if any
    if let Some(udf_file) = matches.value_of("udf") {
        let udf = std::fs::read_to_string(udf_file)
            .unwrap_or_else(|_| panic!("failed to read file {}", udf_file));
        config.parse_udf(udf);
    }

    // Run parsing and code generation.
    let tokens = lexer::get_tokens(&query);
    let mut token_iter = tokens.iter().peekable();
    let parse_tree = parser::parse_prog(&mut token_iter);

    let mut code_gen = code_gen::CodeGen::new_with_config(config);
    // Get the full original command.
    code_gen.cmd = std::env::args().collect::<Vec<String>>().join(" ");

    // TODO(taegyunkim): Specify the root service name from commandline
    code_gen.root_id = "productpagev1";
    code_gen.visit_prog(&parse_tree);

    // Open filter template.
    let template_path = Path::new("filter.cc.handlebars");
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

    let output = handlebars
        .render_template(&template_str, &code_gen)
        .expect("handlebar render failed");

    let mut file = File::create("./wasm/filter.cc").expect("file create failed.");
    file.write_all(output.as_bytes()).expect("write failed");
}
