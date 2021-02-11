MATCH (a {service_name: "productpage-v1"})<-[:RESPONSE]-(b) RETURN a.response_size
