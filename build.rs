extern crate protobuf_codegen_pure;

fn main() {
    protobuf_codegen_pure::Codegen::new()
        .out_dir("src/proto")
        .inputs(&["wasm/treenode.proto"])
        .includes(&["wasm"])
        .run()
        .expect("protoc");
}
