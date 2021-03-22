MATCH (a {service_name: "productpage-v1"}) RETURN a.request_size, hist
