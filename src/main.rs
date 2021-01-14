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
use std::path::PathBuf;
use std::collections::BTreeMap;

static TEMPLATE_FOLDER : &str = "templates_for_code_gen";
static FILTER_FOLDER : &str = "templates_for_code_gen/filter_folder/";

/* Generates code using templates formatted by handlebars
 * (see https://docs.rs/handlebars/3.5.2/handlebars/)
 * The handlebars templates can use any information found in the codegen object.
 * The formatted output is written to the file in output_filename.
 * Arguments:
 * @code_gen:  a code_gen object that contains information that can be formatted nicely by the handlebars
 * @template_path_name: the path leading to a handlebars template
 * @output_filename: where the output is written
 */
fn generate_code_from_codegen_with_handlebars(code_gen: &code_gen::CodeGen, template_path: PathBuf, output_filename: String) {
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

/* Generates code using templates formatted by handlebars
 * (see https://docs.rs/handlebars/3.5.2/handlebars/)
 * The handlebars templates are given filter_name as input;  it is their only piece of information.
 * The formatted output is written to the file in output_filename.
 * Arguments:
 * @filter_name:  the name of the filter;  also the only information used by the handlebars formatter
 * @template_path_name: the path leading to a handlebars template
 * @output_filename: where the output is written
 */
fn generate_code_from_filter_name_with_handlebars(filter_name: String, template_path: PathBuf, output_filename: String) {
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

    let mut data = BTreeMap::new();
    data.insert("filter_name".to_string(), filter_name);
    let output = handlebars
        .render_template(&template_str, &data)
        .expect("handlebar render failed");

    let mut file = File::create(output_filename).expect("file create failed.");
    file.write_all(output.as_bytes()).expect("write failed");
}


fn main() {
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
                 .long("root_node")
                 .value_name("ROOT_NODE")
                 .takes_value(true)
                 .default_value("0")
                 .help("Sets the root node of a query"),
         )
        .arg(
             Arg::with_name("compilation_mode")
                 .short("c")
                 .long("compilation_mode")
                 .value_name("COMPILATION_MODE")
                 .takes_value(true)
                 .possible_values(&compile_vals)
                 .default_value("cpp")
                 .help("Sets what to compile to:  the simulator (sim) or cpp wasm filter (cpp)"),
         )
        .arg(
             Arg::with_name("filter_name")
                 .short("fn")
                 .long("filter_name")
                 .value_name("FILTER_NAME")
                 .takes_value(true)
                 .default_value("my_filter")
                 .help("Sets the name of the filter that is produced"),
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
    let filter_name = matches.value_of("filter_name").unwrap().to_string();
    if c_mode == "sim" {
        // Because we are making a library, not just one file, we need to copy over an example library.  Then,
        // we have to write to three files:  Cargo.toml to edit the filter name, lib.rs to edit the filter name,
        // and <filter_name>.rs for the actual filter implementation
        
        // Making new library folder
        let mut lib_src_folder = filter_name.to_string();
        lib_src_folder.push_str("/src/");
        fs::create_dir_all(&lib_src_folder).unwrap();

        // Cargo.toml
        let mut cargo_file_name = filter_name.clone();
        cargo_file_name.push_str("/");
        cargo_file_name.push_str("/Cargo.toml");
        let cargo_handlebars_name : PathBuf = [FILTER_FOLDER, "Cargo.toml.handlebars"].iter().collect();
        generate_code_from_filter_name_with_handlebars(filter_name.clone(), cargo_handlebars_name, cargo_file_name);

        // Filter types
        let mut filter_types_file = filter_name.clone();
        filter_types_file.push_str("/");
        filter_types_file.push_str(&filter_name);
        filter_types_file.push_str("_types.rs");
        let filter_types_handlebars : PathBuf = [FILTER_FOLDER, "src", "filter_types.rs.handlebars"].iter().collect();
        generate_code_from_codegen_with_handlebars(&code_gen, filter_types_handlebars, filter_types_file);

        // The lib file
        let mut lib_file_name = filter_name.clone();
        lib_file_name.push_str("/src/lib.rs");
        let lib_file_handlebars : PathBuf = [FILTER_FOLDER, "src", "lib.rs.handlebars"].iter().collect();
        generate_code_from_filter_name_with_handlebars(filter_name.clone(), lib_file_handlebars, lib_file_name);
        
        // The filter itself
        let mut filter_file_name = filter_name.clone();
        filter_file_name.push_str("/src/");
        filter_file_name.push_str(&filter_name);
        filter_file_name.push_str(".rs");
        let filter_name_handlebars : PathBuf = [FILTER_FOLDER, "src", "filter.rs.handlebars"].iter().collect();
        generate_code_from_codegen_with_handlebars(&code_gen, filter_name_handlebars, filter_file_name);

        // The graph_utils file
        let mut graph_utils_new_file_name = filter_name.clone();
        graph_utils_new_file_name.push_str("/src/graph_utils.rs");
        let mut graph_utils_old_file_name = FILTER_FOLDER.to_string();
        graph_utils_old_file_name.push_str("src/graph_utils.rs");
        fs::copy(&graph_utils_old_file_name, &graph_utils_new_file_name).unwrap();


    }
    else {
        let mut filter_name_cc = filter_name.to_string().clone();
        filter_name_cc.push_str(".cc");
        let filter_handlebars_cc : PathBuf = [TEMPLATE_FOLDER, "filter.cc.handlebars"].iter().collect();
        generate_code_from_codegen_with_handlebars(&code_gen, filter_handlebars_cc, filter_name_cc);
    }
}
