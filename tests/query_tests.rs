use assert_cmd::prelude::*; // Add methods on commands
use std::path::Path; // Directory management
use std::process::Command; // Run programs
use test_case::test_case; // Parametrized tests

#[test_case("breadth_histogram.cql", vec!["histogram.cc"]; "breadth_histogram")]
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
    let mut args = vec!["-q".to_string(), format!("{}", query_file.display()).into()];
    // Append every udf that is provided
    for udf_name in udf_names {
        let udf_file = udf_dir.join(udf_name);
        args.push("-u".to_string());
        args.push(format!("{}", udf_file.display()).to_string());
    }
    let mut out_file = query_dir.join(query_name);
    out_file.set_extension("cc");
    args.extend(vec!["-o".to_string(), out_file.to_str().unwrap().into()]);
    cmd.args(args);
    cmd.assert().success();

    Ok(())
}

#[test_case("count.cql", vec!["count.rs"]; "count")]
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
    let mut args = vec!["-q".to_string(), format!("{}", query_file.display()).into()];
    // Append every udf that is provided
    for udf_name in udf_names {
        let udf_file = udf_dir.join(udf_name);
        args.push("-u".to_string());
        args.push(format!("{}", udf_file.display()).to_string());
    }
    let out_file = String::from("filter");
    args.extend(vec!["-o".to_string(), out_file]);
    args.extend(vec!["-c".to_string(), String::from("sim")]);
    cmd.args(args);
    cmd.assert().success();

    Ok(())
}
