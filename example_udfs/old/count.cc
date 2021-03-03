// udf_type: Scalar
// id: count
// return_type: int

class count : public user_func<int> {

 public:
    explicit count() {
        std::string count_key = "count";
        int tmp_count = 0;
        proxy_set_shared_data(count_key.c_str(), count_key.size(),
                              (const char *)&tmp_count, sizeof(tmp_count), 0);
    }
    int operator()(const trace_graph_t &graph) {
        int *tmp_count;
        size_t *size;
        WasmResult result;

        std::string count_key = "count";
        result = proxy_get_shared_data(count_key.c_str(), count_key.size(),
                                       (const char **)&tmp_count, size, 0);
        if (result != WasmResult::Ok) {
            LOG_ERROR("Failed to get shared data " + count_key + ".");
            return -1;
        }
        (*tmp_count)++;
        result = proxy_set_shared_data(count_key.c_str(), count_key.size(),
                                       (const char *)tmp_count,
                                       sizeof(*tmp_count), 0);
        if (result != WasmResult::Ok) {
            LOG_ERROR("Failed to set shared data " + count_key + ".");
        }
        return *tmp_count;
    }
};
