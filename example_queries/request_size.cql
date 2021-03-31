MATCH (a) -[]-> (b {})-[]->(c) WHERE b.node.metadata.WORKLOAD_NAME = 'reviews-v1' RETURN a.request.total_size
