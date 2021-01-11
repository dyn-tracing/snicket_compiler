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
    let proj_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let query_dir = proj_dir.join("example_queries/");
    let udf_dir = proj_dir.join("example_udfs/");
    let query_file = query_dir.join(query_name);
    let mut args = vec!["-q".to_string(), format!("{}", query_file.display()).into()];
    for udf_name in udf_names {
        let udf_file = udf_dir.join(udf_name);
        args.push("-u".to_string());
        args.push(format!("{}", udf_file.display()).to_string());
    }
    let mut cmd = Command::cargo_bin("dyntracing")?;
    cmd.args(args);
    cmd.assert().success();

    Ok(())
}
