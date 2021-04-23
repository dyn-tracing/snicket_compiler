// udf_type: Scalar
// leaf_func: leaf_height
// mid_func: mid_height
// id: height

fn leaf_height(_graph: &Graph<(String, IndexMap<String, String>), ()>) -> u32 {
    return 0;
}

// TODO:  must children's responses always be in string form?  can we generalize?
fn mid_height(_graph: &Graph<(String, IndexMap<String, String>), ()>, children_responses: Vec<String>) -> u32 {
    let mut max = 0;
    for response in children_responses {
        let response_as_u32 = response.parse::<u32>();
            match response_as_u32 {
                Ok(num) => { if num > max { max = num; } }
                Err(e) => { print!("error: {0}\n", e); }
            }
    }
    return max + 1;
}
