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

class BidiRootContext : public RootContext {
public:
  explicit BidiRootContext(uint32_t id, StringView root_id)
      : RootContext(id, root_id) {
    std::string workload_name;
    if (getValue({"node", "metadata", "WORKLOAD_NAME"}, &workload_name)) {
      workload_name_ = workload_name;
      LOG_WARN("Intialized workload_name: " + workload_name_);
    } else {
      LOG_WARN("Failed to set workload name");
    }
  }
  bool onConfigure(size_t /* configuration_size */) override;

  StringView getWorkloadName() { return workload_name_; }

private:
  std::string workload_name_;
};

class BidiContext : public Context {
public:
  explicit BidiContext(uint32_t id, RootContext *root)
      : Context(id, root),
        root_(static_cast<BidiRootContext *>(static_cast<void *>(root))),
        b3_trace_id_(""), b3_span_id_(""), b3_parent_span_id_("") {
    direction_ = getTrafficDirection();
  }

  FilterHeadersStatus onRequestHeaders(uint32_t headers) override;
  FilterHeadersStatus onResponseHeaders(uint32_t headers) override;

  void onResponseHeadersInbound();
  void onResponseHeadersOutbound();

private:
  BidiRootContext *root_;
  std::string b3_trace_id_;
  std::string b3_span_id_;
  std::string b3_parent_span_id_;
  TrafficDirection direction_;
};

static RegisterContextFactory
    register_BidiContext(CONTEXT_FACTORY(BidiContext),
                         ROOT_FACTORY(BidiRootContext), "bidi_root_id");

bool BidiRootContext::onConfigure(size_t) { return true; }

FilterHeadersStatus BidiContext::onRequestHeaders(uint32_t) {
  auto trace_id = getRequestHeader("x-b3-traceid");
  if (trace_id->data() == nullptr) {
    LOG_WARN(trafficDirectionToString(direction_) + " " +
             "x-b3-traceid not found!");
  } else {
    b3_trace_id_ = trace_id->toString();
    LOG_WARN(trafficDirectionToString(direction_) + " trace_id_ " +
             b3_trace_id_);
  }

  auto span_id = getRequestHeader("x-b3-spanid");
  if (span_id->data() == nullptr) {
    LOG_WARN(trafficDirectionToString(direction_) + " " +
             "x-b3-spanid not found!");
  } else {
    b3_span_id_ = span_id->toString();
    LOG_WARN(trafficDirectionToString(direction_) + " span_id " + b3_span_id_);
  }

  auto parent_span_id = getRequestHeader("x-b3-parentspanid");
  if (parent_span_id->data() == nullptr) {
    LOG_WARN(trafficDirectionToString(direction_) + " " +
             "x-b3-parentspanid not found!");
  } else {
    b3_parent_span_id_ = parent_span_id->toString();
    LOG_WARN(trafficDirectionToString(direction_) + " parent_span_id " +
             b3_parent_span_id_);
  }

  return FilterHeadersStatus::Continue;
}

void BidiContext::onResponseHeadersInbound() {
  // inbound response processing
  WasmDataPtr shared_data;
  WasmResult result = getSharedData(b3_span_id_ + "path", &shared_data);
  if (result == WasmResult::Ok && shared_data->data() != nullptr) {
    auto header_value = shared_data->toString();

    // Multiple paths could exist separated by commas.
    // split string using ','
    std::regex delimiter(",");
    std::sregex_token_iterator it{header_value.begin(), header_value.end(),
                                  delimiter, -1};
    std::vector<std::string> paths{it, {}};
    for (auto &w : paths) {
      w = std::string(root_->getWorkloadName()) + "-" + w;
    }
    std::set<std::string> paths_set{paths.begin(), paths.end()};

    // Now join them.
    std::string joined = std::accumulate(
        paths.begin(), paths.end(), std::string(),
        [](const std::string &a, const std::string &b) -> std::string {
          return a + (a.length() > 0 ? "," : "") + b;
        });
    addResponseHeader("x-wasm", joined);
    LOG_WARN("inbound: x-wasm -> " + joined);

    if (root_->getWorkloadName() == "productpagev1") {
      // TODO: Construct TreeNode graph using paths and properties returned
      // and check whether the query is subgraph isomorphic to the graph
      // generated from request trace.
    }

  } else {
    addResponseHeader("x-wasm", std::string(root_->getWorkloadName()));
    LOG_WARN("inbound: x-wasm -> " + std::string(root_->getWorkloadName()));
  }

  LOG_WARN("inbound: proceed to collect properties.");
  // Collect properties
  std::vector<std::string> properties;

  // From rust code, we'll pass down, a vector of vector of strings.
  // and generate following snippet for each of the inner vector.
  std::string value;
  if (getValue({"node", "metadata", "WORKLOAD_NAME"}, &value)) {
    std::string result = std::string(root_->getWorkloadName());
    for (auto p : {"node", "metadata", "WORKLOAD_NAME"}) {
      result += "." + std::string(p);
    }
    result += "==";
    result += value;

    properties.push_back(result);
  } else {
    LOG_WARN("failed to get property");
  }

  LOG_WARN("inbound: number of properties collected " +
           std::to_string(properties.size()));

  std::string properties_joined = std::accumulate(
      properties.begin(), properties.end(), std::string(),
      [](const std::string &a, const std::string &b) -> std::string {
        return a + (a.length() > 0 ? "," : "") + b;
      });

  LOG_WARN("inbound: properties_joined:" + properties_joined);

  WasmDataPtr property_shared_data;
  result = getSharedData(b3_span_id_ + "property", &property_shared_data);
  if (result == WasmResult::Ok && property_shared_data->data() != nullptr) {
    auto received = property_shared_data->toString();

    if (properties_joined.length() > 0) {
      properties_joined += ",";
    }
    properties_joined += received;
  }

  if (properties_joined.length() > 0) {
    addResponseHeader("x-wasm-property", properties_joined);
    LOG_WARN("inbound: x-wasm -> " + properties_joined);
  }

  if (root_->getWorkloadName() == "productpagev1") {
    LOG_WARN("Collected properties: " + properties_joined);
  }
}

void BidiContext::onResponseHeadersOutbound() {
  // Received response from another service we called.
  // Collect x-wasm header value.
  auto header = getResponseHeader("x-wasm");
  if (header->data() != nullptr) {
    WasmDataPtr shared_data;
    WasmResult result =
        getSharedData(b3_parent_span_id_ + "path", &shared_data);
    if (result == WasmResult::Ok && shared_data->data() != nullptr) {
      LOG_WARN("outbound: x-wasm -> " + shared_data->toString() + "," +
               header->toString());
      setSharedData(b3_parent_span_id_ + "path",
                    shared_data->toString() + "," + header->toString());
    } else {
      LOG_WARN("outbound: x-wasm -> " + header->toString());
      setSharedData(b3_parent_span_id_ + "path", header->toString());
    }
  } else {
    LOG_WARN("outbound: x-wasm not found");
  }

  auto property_header = getResponseHeader("x-wasm-property");
  if (property_header->data() != nullptr) {
    WasmDataPtr shared_data;
    WasmResult result =
        getSharedData(b3_parent_span_id_ + "property", &shared_data);
    if (result == WasmResult::Ok && shared_data->data() != nullptr) {
      LOG_WARN("outbound: x-wasm -> " + shared_data->toString() + "," +
               property_header->toString());
      setSharedData(b3_parent_span_id_ + "property",
                    shared_data->toString() + "," +
                        property_header->toString());
    } else {
      LOG_WARN("outbound: x-wasm -> " + header->toString());
      setSharedData(b3_parent_span_id_ + "property",
                    property_header->toString());
    }
  }
}

FilterHeadersStatus BidiContext::onResponseHeaders(uint32_t) {
  if (b3_trace_id_ == "") {
    LOG_WARN(trafficDirectionToString(direction_) + " " +
             "x-b3-traceid not set");
  } else {
    LOG_WARN(trafficDirectionToString(direction_) + " trace_id " +
             b3_trace_id_);
  }

  if (b3_span_id_ == "") {
    LOG_WARN(trafficDirectionToString(direction_) + " " +
             "x-b3-spanid not set");
  } else {
    LOG_WARN(trafficDirectionToString(direction_) + " span_id " + b3_span_id_);
  }

  if (b3_parent_span_id_ == "") {
    LOG_WARN(trafficDirectionToString(direction_) + " " +
             "x-b3-parentspanid not set");
  } else {
    LOG_WARN(trafficDirectionToString(direction_) + " parent_span_id " +
             b3_parent_span_id_);
  }

  StringView workload_name = root_->getWorkloadName();
  if (direction_ == TrafficDirection::Inbound) {
    onResponseHeadersInbound();
  } else if (direction_ == TrafficDirection::Outbound) {
    onResponseHeadersOutbound();
  }

  return FilterHeadersStatus::Continue;
}
