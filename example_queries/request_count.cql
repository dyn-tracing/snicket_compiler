/* how many requests when through a particular trace A? */
MATCH n WHERE n.service_name == A RETURN request_count;