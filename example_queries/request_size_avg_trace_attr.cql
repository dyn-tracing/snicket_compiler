MATCH (a) -[]-> (b {service_name: reviews-v1})-[]->(c) WHERE trace.request_size = 1 RETURN trace.request_size, avg(trace.request_size)
