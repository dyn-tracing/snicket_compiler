from prometheus_api_client import PrometheusConnect
prom = PrometheusConnect(url="http://localhost:9090", disable_ssl=True)

# Get the list of all the metrics that the Prometheus host scrapes
# print(prom.all_metrics())
# prom = PrometheusConnect()
# my_label_config = {'cluster': 'my_cluster_id', 'label_2': 'label_2_value'}
# prom.get_current_metric_value(metric_name='up', label_config=my_label_config)

# # Here, we are fetching the values of a particular metric name
# prom.custom_query(query="prometheus_http_requests_total")

# Now, lets try to fetch the `sum` of the metrics
query = prom.custom_query(query="histogram_quantile(0.95, sum(rate(istio_request_duration_seconds_bucket{reporter=\"source\",source_workload=\"productpage-v1\",destination_service_name=\"reviews\",response_code=\"200\"}[60m])) by (le,source_workload,source_version,destination_service_name,destination_workload,destination_version,request_protocol))")
print(query)

# rate(istio_request_duration_milliseconds_sum{reporter="source"}[60m]) / rate(istio_request_duration_milliseconds_count{reporter="source"}[60m])
# rate(istio_request_duration_milliseconds_sum{reporter="source", destination_app="ratings", source_app="reviews",source_canonical_revision="v3"}[1m]) / rate(istio_request_duration_milliseconds_count{reporter="source", destination_app="ratings", source_app="reviews",source_canonical_revision="v3"}[1m])
