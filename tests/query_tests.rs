use assert_cmd::prelude::*; // Add methods on commands
use diffy;
use std::fs;
use std::path::Path; // Directory management
use std::process::Command; // Run programs
use test_case::test_case; // Parametrized tests

#[test_case("breadth_histogram.cql", vec!["histogram.cc"]; "breadth_histogram")]
#[test_case("breadth.cql", vec![]; "breadth")]
#[test_case("height_histogram.cql", vec!["histogram.cc"]; "height_histogram")]
#[test_case("count.cql", vec!["count.cc"]; "count")]
#[test_case("response_code_count.cql", vec!["count.cc"]; "response_code_count")]
#[test_case("response_size_avg.cql", vec!["avg.cc"]; "response_size_avg")]
#[test_case("return.cql", vec![]; "return_query")]
#[test_case("return_height.cql", vec![]; "return_height")]
fn check_compilation(
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

#[test_case("breadth_histogram.cql", vec!["histogram.rs"];
                                        "inconclusive - breadth_histogram")]
#[test_case("breadth.cql", vec![]; "breadth")]
#[test_case("height_histogram.cql" , vec!["histogram.rs"];
                                         "inconclusive - height_histogram" )]
#[test_case("count.cql", vec!["count.rs"]; "count")]
#[test_case("response_code_count.cql", vec!["count.rs"]; "response_code_count")]
#[test_case("response_size_avg.cql", vec!["avg.rs"];
                                         "inconclusive - response_size_avg")]
#[test_case("return.cql", vec![]; "return_query")]
#[test_case("return_height.cql", vec![]; "return_height")]
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
    args.extend(vec!["--root-node", "0"]);
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
