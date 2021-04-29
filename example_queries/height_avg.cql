MATCH (a) -[]-> (b)-[]->(c) WHERE c.node.metadata.WORKLOAD_NAME = 'ratings-v1' RETURN height(a), avg(height(a))
