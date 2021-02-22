MATCH (a) -[]-> (b {service_name: reviews-v1})-[]->(c) RETURN a.request_size, avg(a.request_size)
