/* how many requests went through a particular trace A? */
MATCH n WHERE n.service_name == A RETURN request_count;

/* how many requests went through microservices A, B and C in that order? */
MATCH x->y, y->z,
WHERE x.service_name == A, y.service_name == B, z.service_name,
RETURN request_count;