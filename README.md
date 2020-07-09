Repository for dynamic tracing queries.

Use cypher patterns as a basis for specifying desired trace attributes: https://neo4j.com/docs/2.0/cypher-refcard/

Simply run `cargo run` to see what's generated for query `MATCH a-->b : x,b-->c : y, a-->d: z,`
