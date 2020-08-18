extern crate dyntracing;
extern crate handlebars;
extern crate serde;

use dyntracing::{code_gen, lexer, parser, tree_fold::TreeFold};
use handlebars::Handlebars;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
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

    let query = "MATCH a-->b : x, b-->c : y, a-->d: z, \
                    WHERE a.service_name == productpagev1, \
                            b.service_name == reviewsv2, \
                            c.service_name == ratingsv1, \
                            d.service_name == detailsv1, \
                    RETURN dfs_max_value_visitor(response_size),";
    let tokens = lexer::get_tokens(query);
    let mut token_iter = tokens.iter().peekable();
    let parse_tree = parser::parse_prog(&mut token_iter);

    let mut code_gen = code_gen::CodeGen::new();

    code_gen.udf_table.insert(
        "dfs_max_value_visitor",
        code_gen::Udf {
            udf_type: code_gen::UdfType::Scalar,
            id: "dfs_max_value_visitor",
            func_impl: r#"
            class dfs_max_value_visitor : public boost::default_dfs_visitor {
                public:
                  dfs_max_value_visitor(std::initializer_list<std::string> key, int *max) {
                    key_{key.begin(), key.end()};
                    max_ = max;
                  }

                  template <typename Vertex, typename Graph>
                  void discover_vertex(Vertex u, const Graph &g) {
                    auto map = g[u].properties;

                    int value = std::atoi(map.at(key_).c_str());
                    LOG_WARN(g[u].id + " " + std::to_string(value));

                    if (value > *max_) {
                      *max_ = value;
                    }
                  }

                  std::vector<std::string> key_;
                  int *max_;
                };
            "#,
            return_type: "int",
            ..Default::default()
        },
    );
    code_gen.root_id = "productpagev1";
    code_gen.visit_prog(&parse_tree);

    let handlebars = Handlebars::new();

    let output = handlebars
        .render_template(&template_str, &code_gen)
        .expect("handlebar render failed");

    let mut file = File::create("./wasm/filter.cc").expect("file create failed.");
    file.write_all(output.as_bytes()).expect("write failed");
}
