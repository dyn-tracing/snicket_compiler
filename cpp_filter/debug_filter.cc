// Auto generated Envoy WASM filter from following command:
// /storage/Projekte/tracing_env/tracing_compiler/target/debug/dtc -q
// /storage/Projekte/tracing_env/tracing_compiler/example_queries/count.cql -u
// /storage/Projekte/tracing_env/tracing_compiler/example_udfs/count.cc -r
// productpage-v1

// NOLINT(namespace-envoy)
#include <map>
#include <numeric>
#include <regex>
#include <set>
#include <string>
#include <unordered_map>

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

// udf_type: Scalar
// id: count
// return_type: int

class BidiRootContext : public RootContext {
 public:
    explicit BidiRootContext(uint32_t id, std::string_view root_id)
        : RootContext(id, root_id) {
        std::string workload_name;
        if (getValue({"node", "metadata", "WORKLOAD_NAME"}, &workload_name)) {
            workload_name_ = workload_name;
            LOG_WARN("Initialized workload_name: " + workload_name_);
        } else {
            LOG_WARN("Failed to set workload name");
        }
    }
    bool onConfigure(size_t /* configuration_size */) override;

    std::string_view getWorkloadName() { return workload_name_; }

 private:
    std::string workload_name_;
};

class BidiContext : public Context {
 public:
    explicit BidiContext(uint32_t id, RootContext *root)
        : Context(id, root),
          root_(static_cast<BidiRootContext *>(static_cast<void *>(root))) {
        direction_ = getTrafficDirection();
    }

    FilterHeadersStatus onRequestHeaders(uint32_t headers,
                                         bool end_of_stream) override;
    FilterHeadersStatus onRequestHeadersInbound();
    FilterHeadersStatus onRequestHeadersOutbound();
    FilterHeadersStatus onResponseHeaders(uint32_t headers,
                                          bool end_of_stream) override;
    FilterHeadersStatus onResponseHeadersInbound();
    FilterHeadersStatus onResponseHeadersOutbound();
    void print_headers(WasmHeaderMapType type);

 private:
    BidiRootContext *root_;
    TrafficDirection direction_;
};

static RegisterContextFactory
    register_BidiContext(CONTEXT_FACTORY(BidiContext),
                         ROOT_FACTORY(BidiRootContext), "bidi_root_id");

bool BidiRootContext::onConfigure(size_t) { return true; }

void BidiContext::print_headers(WasmHeaderMapType type) {
    if (type == WasmHeaderMapType::RequestHeaders) {
        auto result = getRequestHeaderPairs();
        auto pairs = result->pairs();
        LOG_WARN("Request headers: " + toString(pairs.size()));
        for (auto &p : pairs) {
            LOG_WARN(std::string(p.first) + " -> " + std::string(p.second));
        }
    } else if (type == WasmHeaderMapType::ResponseHeaders) {
        auto result = getResponseHeaderPairs();
        auto pairs = result->pairs();
        LOG_WARN("Response headers: " + toString(pairs.size()));
        for (auto &p : pairs) {
            LOG_WARN(std::string(p.first) + " -> " + std::string(p.second));
        }
    }
}

FilterHeadersStatus BidiContext::onRequestHeaders(uint32_t, bool) {
    print_headers(WasmHeaderMapType::RequestHeaders);
    if (direction_ == TrafficDirection::Inbound) {
        return onRequestHeadersInbound();
    } else if (direction_ == TrafficDirection::Outbound) {
        return onRequestHeadersOutbound();
    }
}

FilterHeadersStatus BidiContext::onRequestHeadersInbound() {
    LOG_WARN("Traversed.");
    return FilterHeadersStatus::Continue;
}

FilterHeadersStatus BidiContext::onRequestHeadersOutbound() {
    LOG_WARN("Traversed.");
    return FilterHeadersStatus::Continue;
}

FilterHeadersStatus BidiContext::onResponseHeaders(uint32_t, bool) {
    print_headers(WasmHeaderMapType::ResponseHeaders);
    if (direction_ == TrafficDirection::Inbound) {
        return onResponseHeadersInbound();
    } else if (direction_ == TrafficDirection::Outbound) {
        return onResponseHeadersOutbound();
    }
}
FilterHeadersStatus BidiContext::onResponseHeadersInbound() {
    LOG_WARN("Traversed.");
    return FilterHeadersStatus::Continue;
}
FilterHeadersStatus BidiContext::onResponseHeadersOutbound() {
    LOG_WARN("Traversed.");
    return FilterHeadersStatus::Continue;
}
