MATCH (a)-[]-(b) WHERE a.service_name = "productpage-v1"
RETURN a.response_size
