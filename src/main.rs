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
use std::path::{Path, PathBuf};

/* Generates code using templates formatted by handlebars
 * (see https://docs.rs/handlebars/3.5.2/handlebars/)
 * The handlebars templates can use any information found in the codegen object.
 * The formatted output is written to the file in output_filename.
 * Arguments:
 * @code_gen:  a code_gen object that contains information that can be formatted nicely by the handlebars
 * @template_path_name: the path leading to a handlebars template
 * @output_filename: where the output is written
 */
fn generate_code_from_codegen_with_handlebars(
    code_gen: &code_gen::CodeGen,
    template_path: PathBuf,
    output_filename: PathBuf,
) {
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

    let mut file = File::create(output_filename).expect("file create failed.");
    file.write_all(output.as_bytes()).expect("write failed");
}

fn main() {
    let bin_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let def_filter_dir = bin_dir.join("cpp_filter/filter.cc");
    let compile_vals = ["sim", "cpp"];
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
                .help("Optionally sets user defined function file to use"),
        )
        .arg(
            Arg::with_name("root_node")
                .short("r")
                .long("root-node")
                .value_name("ROOT_NODE")
                .required(true)
                .help("Sets the root node of a query"),
        )
        .arg(
            Arg::with_name("compilation_mode")
                .short("c")
                .long("compilation-mode")
                .value_name("COMPILATION_MODE")
                .takes_value(true)
                .possible_values(&compile_vals)
                .default_value("cpp")
                .help("Sets what to compile to:  the simulator (sim) or cpp wasm filter (cpp)"),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("out-file")
                .value_name("OUT_FILE")
                .default_value(def_filter_dir.to_str().unwrap())
                .help("Location and name of the output file."),
        )
        .get_matches();

    // Read query from file specified by command line argument.
    let query_file = matches.value_of("query").unwrap();
    let query = fs::read_to_string(query_file)
        .unwrap_or_else(|_| panic!("failed to read file {}", query_file));

    let mut config = code_gen::CodeGenConfig::new();
    let c_mode = matches.value_of("compilation_mode").unwrap();
    if c_mode == "sim" {
        config.am_cpp = false;
    }

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
    code_gen.cmd = std::env::args().collect::<Vec<String>>().join(" ");

    code_gen.root_id = matches.value_of("root_node").unwrap();
    code_gen.visit_prog(&parse_tree);

    // Use the information in the code generator code_gen and format it, using handlebars
    // to a template with all the basic filter information enclosed
    let c_mode = matches.value_of("compilation_mode").unwrap();
    let mut output_name = PathBuf::from(matches.value_of("output").unwrap());
    if c_mode == "sim" {
        // Because we are making a library, not just one file, we need to copy over an example library.  Then,
        // we have to write to three files:  Cargo.toml to edit the filter name, lib.rs to edit the filter name,
        // and <output_name>.rs for the actual filter implementation
        let rust_dir = bin_dir.join("rust_filter");
        // The filter itself
        let mut filter_file_name = rust_dir.join(output_name);
        filter_file_name.set_extension("rs");
        let filter_name_handlebars = bin_dir.join("filter.rs.handlebars");
        print!("writing filter to {0}", filter_file_name.to_str().unwrap());
        generate_code_from_codegen_with_handlebars(
            &code_gen,
            filter_name_handlebars,
            filter_file_name,
        );
    } else {
        let filter_handlebars_cc = bin_dir.join("filter.cc.handlebars");
        output_name.set_extension("cc");
        generate_code_from_codegen_with_handlebars(&code_gen, filter_handlebars_cc, output_name);
    }
}
