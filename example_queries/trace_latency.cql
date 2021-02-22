MATCH (a {type: "trace", customer: "xyz"})-[]->(c) RETURN avg(trace.latency)
