MATCH (a {service_name: "productpage-v1"}) -[]-> (b {service_name: "reviews-v1"}) RETURN a.request_size
UNION
MATCH (a {service_name: "productpage-v1"}) -[]-> (b {service_name: "reviews-v2"}) RETURN a.request_size

