MATCH (a) -[]-> (b {service_name: reviews-v1})-[]->(c) RETURN a.count
