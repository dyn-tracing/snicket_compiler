MATCH (a {service_name: "productpage-v1"}) RETURN agg(a.request_size)
