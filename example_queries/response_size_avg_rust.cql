MATCH a-->b : x,
WHERE a.service_name == "frontend-v1_plugin",
GROUP a.response_size BY avg,
