MATCH (a) -[]-> (b)-[]->(c) WHERE b.node.metadata.WORKLOAD_NAME = 'reviews-v1' AND trace.request.total_size = 1 RETURN trace.request.total_size, avg(trace.request.total_size)
