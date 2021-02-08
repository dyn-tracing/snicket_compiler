MATCH a-->b : x,
WHERE a.service_name == "productpage-v1",
GROUP a.response_size BY avg,
