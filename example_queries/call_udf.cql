MATCH a-->b : x, b-->c : y, a-->d: z,
WHERE a.service_name == productpagev1,
        b.service_name == reviewsv2,
        c.service_name == ratingsv1,
        d.service_name == detailsv1,
RETURN aggr_func,


// Assume following implementation for aggr_func
//
// class aggr_func : public user_func<int> {
// public:
//     int operator()(const trace_graph_t &graph) {
//         num_vertices += graph.num_vertices();
//         return num_vertices;
//     }

// private:
//     int num_vertices = 0;
// };