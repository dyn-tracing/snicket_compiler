MATCH (a) -[]-> (b {service_name: reviews-v1})-[]->(c) WHERE trace.request_size = 1 RETURN a.request_size, avg(a.request_size)
