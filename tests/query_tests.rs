use assert_cmd::prelude::*; // Add methods on commands
use diffy;
use std::fs;
use std::path::Path; // Directory management
use std::process::Command; // Run programs
use test_case::test_case; // Parametrized tests

#[test_case("get_service_name.cql", vec![]; "inconclusive - get_service_name")]
#[test_case("height.cql", vec!["height.rs"]; "inconclusive - height")]
#[test_case("histogram.cql", vec!["histogram.rs"]; "inconclusive - histogram")]
#[test_case("request_size.cql", vec![]; "inconclusive - request_size")]
#[test_case("request_size_avg.cql", vec![]; "inconclusive - request_size_avg")]
#[test_case("latency.cql", vec!["latency.rs"]; "inconclusive - latency")]
fn check_compilation_cc(
    query_name: &str,
    udf_names: Vec<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Static folders
    let proj_dir = Path::new("");
    let query_dir = proj_dir.join("example_queries");
    let udf_dir = proj_dir.join("example_udfs");

    // The input query in the folder is provided as test case
    let query_file = query_dir.join(query_name);
    // This is the binary to compile a query
    let mut cmd = Command::new(proj_dir.join("target/debug/dtc"));
    // Assemble the args, first the input query
    let mut args = vec!["-q", query_file.to_str().unwrap()];
    // Append every udf that is provided
    let mut str_builder = String::new();
    for udf_name in udf_names {
        let udf_file = udf_dir.join(udf_name);
        str_builder.push_str("-u");
        str_builder.push_str(udf_file.to_str().unwrap());
    }
    if !str_builder.is_empty() {
        args.push(&str_builder);
    }
    let mut out_file = query_dir.join(query_name);
    out_file.set_extension("cc");
    args.extend(vec!["-o", out_file.to_str().unwrap()]);
    args.extend(vec!["--root-node", "productpage-v1"]);
    cmd.args(args);
    cmd.assert().success();

    let mut ref_file = query_dir.join(query_name);
    ref_file.set_extension("cc.ref");
    let out_file_str = fs::read_to_string(out_file).unwrap();
    let ref_file_str = fs::read_to_string(ref_file).unwrap();
    if out_file_str != ref_file_str {
        let diff = diffy::create_patch(&out_file_str, &ref_file_str);
        let diff_color = diffy::PatchFormatter::new().with_color();
        panic!(
            "Files differ in the following way:\n{}",
            diff_color.fmt_patch(&diff)
        );
    } else {
        Ok(())
    }
}

#[test_case("get_service_name.cql", vec![]; "get_service_name")]
#[test_case("height.cql", vec!["height.rs"]; "height")]
#[test_case("histogram.cql", vec!["histogram.rs"]; "inconclusive - histogram")]
#[test_case("request_size.cql", vec![]; "request_size")]
#[test_case("request_size_avg.cql", vec![]; "request_size_avg")]
#[test_case("request_size_avg_trace_attr.cql", vec![]; "request_size_avg_trace_attr")]
#[test_case("latency.cql", vec!["latency.rs"]; "inconclusive - latency")]
fn check_compilation_rust(
    query_name: &str,
    udf_names: Vec<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Static folders
    let proj_dir = Path::new("");
    let query_dir = proj_dir.join("example_queries");
    let udf_dir = proj_dir.join("example_udfs");

    // The input query in the folder is provided as test case
    let query_file = query_dir.join(query_name);
    // This is the binary to compile a query
    let mut cmd = Command::new(proj_dir.join("target/debug/dtc"));
    // Assemble the args, first the input query
    let mut args = vec!["-q", query_file.to_str().unwrap()];
    // Append every udf that is provided
    let mut str_builder = String::new();
    for udf_name in udf_names {
        let udf_file = udf_dir.join(udf_name);
        str_builder.push_str("-u");
        str_builder.push_str(udf_file.to_str().unwrap());
    }
    if !str_builder.is_empty() {
        args.push(&str_builder);
    }
    let mut out_file = query_dir.join(query_name);
    out_file.set_extension("rs");
    args.extend(vec!["-o", out_file.to_str().unwrap()]);
    args.extend(vec!["-c", "sim"]);
    args.extend(vec!["--root-node", "productpage-v1"]);
    cmd.args(args);
    cmd.assert().success();

    let mut ref_file = query_dir.join(query_name);
    ref_file.set_extension("rs.ref");
    let out_file_str = fs::read_to_string(out_file).unwrap();
    let ref_file_str = fs::read_to_string(ref_file).unwrap();
    if out_file_str != ref_file_str {
        let diff = diffy::create_patch(&out_file_str, &ref_file_str);
        let diff_color = diffy::PatchFormatter::new().with_color();
        panic!(
            "Files differ in the following way:\n{}",
            diff_color.fmt_patch(&diff)
        );
    } else {
        Ok(())
    }
}
