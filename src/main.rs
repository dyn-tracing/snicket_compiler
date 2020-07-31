extern crate dyntracing;
extern crate handlebars;
extern crate protobuf;
extern crate serde;

use dyntracing::{code_gen, lexer, parser, tree_fold::TreeFold};
use handlebars::Handlebars;
use protobuf::Message;
use serde::Serialize;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// Data passed to handlebars template to generate CC file
#[derive(Serialize)]
struct HandlebarsData {
    // The name of root service node in the service mesh.
    root: String,
    // Length of the following proto_bytes vector.
    proto_len: usize,
    // The byte representation of TreeNode proto specified by the user query.
    proto_bytes: Vec<u8>,
    // The property to return matching the user query represented by above TreeNode proto.
    return_action: Vec<String>,
}

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
        Ok(_) => println!("Successfully read {}", display),
    }

    let query = "MATCH a-->b : x, b-->c : y, a-->d: z, \
                    WHERE a.service_name == productpagev1, \
                            b.service_name == reviewsv2, \
                            c.service_name == ratingsv1, \
                            d.service_name == detailsv1, \
                    RETURN a.service_name,";
    let tokens = lexer::get_tokens(query);
    let mut token_iter = tokens.iter().peekable();
    let parse_tree = parser::parse_prog(&mut token_iter);

    let mut code_gen = code_gen::CodeGen::new();
    code_gen.visit_prog(&parse_tree);

    assert!(code_gen.nodes.len() == 1, "only support tree pattern.");
    let mut root_id = "";
    for (k, _v) in code_gen.nodes.iter() {
        root_id = k;
    }
    let root = code_gen.nodes.get(root_id).unwrap();

    let proto_bytes = root.write_to_bytes().unwrap();
    let proto_len = proto_bytes.len();

    let data = HandlebarsData {
        // NOTE: we assume that user knows the app/service which is the root of their service graph.
        root: String::from("productpagev1"),
        proto_len,
        proto_bytes,
        return_action: code_gen
            .return_action
            .into_iter()
            .map(|x| x.to_string())
            .collect(),
    };

    let handlebars = Handlebars::new();

    let output = handlebars
        .render_template(&template_str, &data)
        .expect("handlebar render failed");

    let mut file = File::create("./wasm/filter.cc").expect("file create failed.");
    file.write_all(output.as_bytes()).expect("write failed");
}
