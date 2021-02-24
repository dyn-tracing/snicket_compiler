extern crate clap;
extern crate dyntracing;
extern crate handlebars;
extern crate input_stream;
extern crate serde;
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::token_factory::CommonTokenFactory;
use antlr_rust::InputStream;
use clap::{App, Arg};
use dyntracing::lexer::CypherLexer;
use dyntracing::parser::CypherParser;

use std::fs;
use std::path::Path;

fn main() {
    // Set up logging
    let mut builder = env_logger::Builder::from_default_env();
    // do not want timestamp for now
    builder.default_format_timestamp(false);
    // Set default log level to info
    builder.filter_level(log::LevelFilter::Trace);
    builder.init();

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
    let tf = CommonTokenFactory::default();
    // Read query from file specified by command line argument.
    let query_file = matches.value_of("query").unwrap();
    let query: String = fs::read_to_string(query_file).unwrap();
    let query_stream = InputStream::new_owned(query.into_boxed_str());
    let lexer = CypherLexer::new_with_token_factory(query_stream, &tf);
    let token_source = CommonTokenStream::new(lexer);
    let mut parser = CypherParser::new(token_source);
    let result = parser.oC_Cypher();
    match result {
        Err(e) => {
            log::error!("Error parsing query: {:?}", e);
        }
        Ok(v) => {
            dyntracing::to_ir::visit_result(v);
        }
    }
}
