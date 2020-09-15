# Call User Defined Function

filter.cc is obtained by running following command from project root directory.

`cargo run -- -q ./example_queries/call_udf/query.cql -u ./example_queries/call_udf/max_response_size.cc`

Note that this filter can be compiled, but it doesn't work as intended, because
response size property is not fetched.
