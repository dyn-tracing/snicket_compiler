// NOLINT(namespace-envoy)
#include <string>
#include <unordered_map>

#include "filter.pb.h"
#include "google/protobuf/util/json_util.h"
#include "proxy_wasm_intrinsics.h"

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

class BidiRootContext : public RootContext {
 public:
    explicit BidiRootContext(uint32_t id, std::string_view root_id)
        : RootContext(id, root_id) {
        std::string workload_name;
        if (getValue({"node", "metadata", "WORKLOAD_NAME"}, &workload_name)) {
            workload_name_ = workload_name;
            std::string warning = "initialized workload_name: ";
            warning = warning.append(workload_name_);
            LOG_WARN(warning);
        } else {
            LOG_WARN("Failed to set workload name");
        }
    }
    bool onConfigure(size_t /* configuration_size */) override;

    bool onStart(size_t) override;
    std::string_view getWorkloadName() { return workload_name_; }
    void incrementCount() { count_++; }
    void decrementCount() { count_--; }
    int getCount() { return count_; }

    std::string header_value_;

 private:
    std::string_view workload_name_;
    int count_ = 0;
};

class BidiContext : public Context {
 public:
    explicit BidiContext(uint32_t id, RootContext *root)
        : Context(id, root),
          root_(static_cast<BidiRootContext *>(static_cast<void *>(root))) {
        direction_ = getTrafficDirection();
        std::string warning = "Got traffic direction, is ";
        warning = warning.append(trafficDirectionToString(direction_));
        LOG_WARN(warning);
    }

    void onCreate() override;
    FilterHeadersStatus onRequestHeaders(uint32_t headers,
                                         bool end_of_stream) override;
    FilterHeadersStatus onRequestHeadersInbound();
    FilterHeadersStatus onRequestHeadersOutbound();
    FilterDataStatus onRequestBody(size_t body_buffer_length,
                                   bool end_of_stream) override;
    FilterHeadersStatus onResponseHeaders(uint32_t headers,
                                          bool end_of_stream) override;
    FilterHeadersStatus onResponseHeadersInbound();
    FilterHeadersStatus onResponseHeadersOutbound();
    void onDone() override;
    void onLog() override;
    void onDelete() override;

 private:
    BidiRootContext *root_;
    std::string b3_trace_id_;
    std::string b3_span_id_;
    std::string b3_parent_span_id_;
    TrafficDirection direction_;
};

static RegisterContextFactory
    register_BidiContext(CONTEXT_FACTORY(BidiContext),
                         ROOT_FACTORY(BidiRootContext), "add_header_root_id");

bool BidiRootContext::onConfigure(size_t config_buffer_length) {
    auto conf = getBufferBytes(WasmBufferType::PluginConfiguration, 0,
                               config_buffer_length);
    LOG_DEBUG("onConfigure " + conf->toString());
    header_value_ = conf->toString();
    return true;
}

bool BidiRootContext::onStart(size_t) {
    LOG_DEBUG("onStart");
    return true;
}

void BidiContext::onCreate() {
    LOG_DEBUG(std::string("onCreate " + std::to_string(id())));
}

FilterHeadersStatus BidiContext::onRequestHeaders(uint32_t, bool) {
    LOG_DEBUG(std::string("onRequestHeaders ") + std::to_string(id()));

    // Print all request headers
    auto result = getRequestHeaderPairs();
    auto pairs = result->pairs();
    LOG_WARN(std::string("request headers: ") + std::to_string(pairs.size()));
    for (auto &p : pairs) {
        LOG_WARN(std::string(p.first) + std::string(" -> ") +
                 std::string(p.second));
    }
    int THRESHOLD = 0;
    root_->incrementCount();
    std::string warning = "incrementing count ";
    warning = warning.append(std::to_string(root_->getCount()));
    LOG_WARN(warning);
    if (root_->getCount() > THRESHOLD) {
        LOG_INFO("above threshold");
    }

    replaceRequestHeader("x-envoy-force-trace", "true");
    if (direction_ == TrafficDirection::Inbound) {
        return onRequestHeadersInbound();
    } else if (direction_ == TrafficDirection::Outbound) {
        return onRequestHeadersOutbound();
    }
    LOG_WARN("didn't get direction in request header");
}

FilterHeadersStatus BidiContext::onRequestHeadersInbound() {
    LOG_WARN("in on request headers inbound");
    addResponseHeader("requestheaderINbound", "hi");
    return FilterHeadersStatus::Continue;
}

FilterHeadersStatus BidiContext::onRequestHeadersOutbound() {
    LOG_WARN("in on request headers outbound");
    addResponseHeader("requestheaderOUTbound", "hi");
    return FilterHeadersStatus::Continue;
}

FilterHeadersStatus BidiContext::onResponseHeadersInbound() {
    LOG_WARN("in on response headers inbound");
    addResponseHeader("responseheaderINbound", "hi");
    return FilterHeadersStatus::Continue;
}

FilterHeadersStatus BidiContext::onResponseHeadersOutbound() {
    LOG_WARN("in on response headers outbound");
    addResponseHeader("responseheaderOUTbound", "hi");
    return FilterHeadersStatus::Continue;
}

FilterHeadersStatus BidiContext::onResponseHeaders(uint32_t, bool) {
    LOG_DEBUG(std::string("onResponseHeaders ") + std::to_string(id()));
    // Print all headers
    auto result = getResponseHeaderPairs();
    auto pairs = result->pairs();
    LOG_WARN(std::string("response headers: ") + std::to_string(pairs.size()));
    for (auto &p : pairs) {
        LOG_WARN(std::string(p.first) + std::string(" -> ") +
                 std::string(p.second));
    }
    std::string warning = "decrementing count";
    warning = warning.append(std::to_string(root_->getCount()));
    LOG_WARN(warning);
    root_->decrementCount();
    replaceResponseHeader("location", "envoy-wasm");
    replaceResponseHeader("x-envoy-force-trace", "true");
    if (direction_ == TrafficDirection::Inbound) {
        return onResponseHeadersInbound();
    } else if (direction_ == TrafficDirection::Outbound) {
        return onResponseHeadersOutbound();
    }
    LOG_WARN("in response headers but no direction given");
}

FilterDataStatus BidiContext::onRequestBody(size_t body_buffer_length,
                                            bool end_of_stream) {
    return FilterDataStatus::Continue;
}

void BidiContext::onDone() {
    LOG_DEBUG(std::string("onDone " + std::to_string(id())));
}

void BidiContext::onLog() {
    LOG_DEBUG(std::string("onLog " + std::to_string(id())));
}

void BidiContext::onDelete() {
    LOG_DEBUG(std::string("onDelete " + std::to_string(id())));
}
