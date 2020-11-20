// NOLINT(namespace-envoy)
#include <string>
#include <unordered_map>

#include "filter.pb.h"
#include "google/protobuf/util/json_util.h"
#include "proxy_wasm_intrinsics.h"

// TrafficDirection is a mirror of envoy xDS traffic direction.
// As defined in istio/proxy/extensions/common/context.h
enum class TrafficDirection : int64_t {
    Unspecified = 0,
    Inbound = 1,
    Outbound = 2,
};

// Retrieves the traffic direction from the configuration context.
TrafficDirection getTrafficDirection() {
    int64_t direction;
    if (getValue({"listener_direction"}, &direction)) {
        return static_cast<TrafficDirection>(direction);
    }
    return TrafficDirection::Unspecified;
}

std::string trafficDirectionToString(TrafficDirection dir) {
    if (dir == TrafficDirection::Unspecified) {
        return "unspecified";
    } else if (dir == TrafficDirection::Inbound) {
        return "inbound";
    } else {
        return "outbound";
    }
}

class AddHeaderRootContext : public RootContext {
 public:
    explicit AddHeaderRootContext(uint32_t id, std::string_view root_id)
        : RootContext(id, root_id) {}
    bool onConfigure(size_t /* configuration_size */) override;

    bool onStart(size_t) override;

    std::string header_value_;
};

class CustomContext : public Context {
 public:
    explicit CustomContext(uint32_t id, RootContext *root)
        : Context(id, root), root_(static_cast<AddHeaderRootContext *>(
                                 static_cast<void *>(root))) {}

    void onCreate() override;
    FilterHeadersStatus onRequestHeaders(uint32_t headers,
                                         bool end_of_stream) override;
    FilterDataStatus onRequestBody(size_t body_buffer_length,
                                   bool end_of_stream) override;
    FilterDataStatus onResponseBody(size_t body_buffer_length,
                                   bool end_of_stream) override;
    FilterHeadersStatus onResponseHeaders(uint32_t headers,
                                          bool end_of_stream) override;
    void onDone() override;
    void onLog() override;
    void onDelete() override;

 private:
    AddHeaderRootContext *root_;
};
static RegisterContextFactory
    register_CustomContext(CONTEXT_FACTORY(CustomContext),
                              ROOT_FACTORY(AddHeaderRootContext),
                              "add_header_root_id");

bool AddHeaderRootContext::onConfigure(size_t config_buffer_length) {
    auto conf = getBufferBytes(WasmBufferType::PluginConfiguration, 0,
                               config_buffer_length);
    LOG_DEBUG("onConfigure " + conf->toString());
    header_value_ = conf->toString();
    return true;
}

bool AddHeaderRootContext::onStart(size_t) {
    LOG_DEBUG("onStart");
    return true;
}

void CustomContext::onCreate() {
    LOG_DEBUG(std::string("onCreate " + std::to_string(id())));
}

WasmDataPtr get_trace_id(TrafficDirection direction) {
    auto trace_id = getRequestHeader("x-b3-traceid");
    if (trace_id->data() == nullptr) {
        LOG_WARN(trafficDirectionToString(direction) + " " +
                 "x-b3-traceid not found!");
    } else {
        LOG_WARN(trafficDirectionToString(direction) + " trace_id_ " +
                 trace_id->toString());
    }
    return trace_id;
}

FilterHeadersStatus CustomContext::onRequestHeaders(uint32_t, bool) {
    auto result = getRequestHeaderPairs();
    auto pairs = result->pairs();
    LOG_WARN(std::string("headers: ") + std::to_string(pairs.size()));
    for (auto &p : pairs) {
        LOG_WARN(std::string(p.first) + std::string(" -> ") +
                 std::string(p.second));
    }
    TrafficDirection direction = getTrafficDirection();
    auto service_src = getRequestHeader("service_source");
    if (direction == TrafficDirection::Inbound) {
        auto trace_id = get_trace_id(direction);
        if (service_src->data()) {
            LOG_WARN("INCOMING REQUEST FROM  " + service_src->toString());
        } else {
            LOG_WARN("INCOMING REQUEST SERVICE SOURCE MISSING");
        }
        removeRequestHeader("service_source");
    } else if (direction == TrafficDirection::Outbound) {
        auto trace_id = get_trace_id(direction);
        LOG_WARN("OUTGOING REQUEST");
        std::string workload_name;
        if (getValue({"node", "metadata", "WORKLOAD_NAME"}, &workload_name)) {
            LOG_WARN("Initialized workload_name: " + workload_name);
            addRequestHeader("service_source", workload_name);
        } else {
            LOG_WARN("Failed to set workload name");
        }
    } else {
        LOG_WARN("Weird direction.");
    }
    return FilterHeadersStatus::Continue;
}

FilterHeadersStatus CustomContext::onResponseHeaders(uint32_t, bool) {
    auto result = getResponseHeaderPairs();
    auto pairs = result->pairs();
    LOG_WARN(std::string("headers: ") + std::to_string(pairs.size()));
    for (auto &p : pairs) {
        LOG_WARN(std::string(p.first) + std::string(" -> ") +
                 std::string(p.second));
    }
    TrafficDirection direction = getTrafficDirection();
    auto service_src = getRequestHeader("service_source");
    if (direction == TrafficDirection::Inbound) {
        auto trace_id = get_trace_id(direction);
        if (service_src->data()) {
            LOG_WARN("INCOMING RESPONSE FROM  " + service_src->toString());
        } else {
            LOG_WARN("INCOMING RESPONSE SERVICE SOURCE MISSING");
        }
        removeResponseHeader("service_source");
    } else if (direction == TrafficDirection::Outbound) {
        auto trace_id = get_trace_id(direction);
        LOG_WARN("OUTGOING RESPONSE");
        std::string workload_name;
        if (getValue({"node", "metadata", "WORKLOAD_NAME"}, &workload_name)) {
            LOG_WARN("Initialized workload_name: " + workload_name);
            replaceResponseHeader("service_source", workload_name);
        } else {
            LOG_WARN("Failed to set workload name");
        }
    } else {
        LOG_WARN("Weird direction.");
    }
    return FilterHeadersStatus::Continue;
}

FilterDataStatus CustomContext::onRequestBody(size_t body_buffer_length,
                                               bool end_of_stream) {
    size_t size;
    uint32_t flags;
    auto body =
        getBufferBytes(WasmBufferType::HttpRequestBody, 0, body_buffer_length);
    logError("onRequestBody " + std::string(body->view()));

    return FilterDataStatus::Continue;
}

FilterDataStatus CustomContext::onResponseBody(size_t body_buffer_length,
                                                bool end_of_stream) {
    size_t size;
    uint32_t flags;
    auto body =
        getBufferBytes(WasmBufferType::HttpResponseBody, 0, body_buffer_length);
    logError("onResponseBody " + std::string(body->view()));
    return FilterDataStatus::Continue;
}

void CustomContext::onDone() {
    LOG_DEBUG(std::string("onDone " + std::to_string(id())));
}

void CustomContext::onLog() {
    LOG_DEBUG(std::string("onLog " + std::to_string(id())));
}

void CustomContext::onDelete() {
    LOG_DEBUG(std::string("onDelete " + std::to_string(id())));
}
