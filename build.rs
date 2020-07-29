extern crate protoc_rust;

fn main() {
    protoc_rust::Codegen::new()
        .out_dir("src")
        .inputs(&["wasm/graph.proto"])
        .run()
        .expect("protoc");
}
