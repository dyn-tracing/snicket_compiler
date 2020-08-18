// NOLINT(namespace-envoy)
#include <map>
#include <numeric>
#include <regex>
#include <set>
#include <string>
#include <unordered_map>

#include "absl/strings/str_join.h"
#include "absl/strings/str_split.h"
#include "proxy_wasm_intrinsics.h"

#include "graph_utils.h"

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
  // These will be used to generate trace graphs.
  std::string paths_joined;
  std::string properties_joined;

  // Collect paths
  WasmDataPtr shared_data;
  WasmResult result = getSharedData(b3_span_id_ + "path", &shared_data);
  if (result == WasmResult::Ok && shared_data->data() != nullptr) {
    auto header_value = shared_data->toString();

    // Multiple paths could exist separated by commas.
    // split string using ','
    std::vector<std::string> paths =
        absl::StrSplit(header_value, ",", absl::SkipEmpty());

    // Prepend current workload name to paths.
    for (auto &w : paths) {
      w = std::string(root_->getWorkloadName()) + "-" + w;
    }

    // Join them all to a single string.
    paths_joined = absl::StrJoin(paths, ",");
  }

  // When this service is a leaf node
  if (paths_joined.empty()) {
    paths_joined = root_->getWorkloadName();
  }

  addResponseHeader("x-wasm", paths_joined);
  LOG_WARN("x-wasm: " + paths_joined);

  LOG_WARN("Proceed to collect properties.");
  // Collect properties
  std::vector<std::string> properties;

  // From rust code, we'll pass down, a vector of vector of strings.
  // and generate following snippet for each of the inner vector.
  {
    std::string value;
    if (getValue(
            {
                "node",
                "metadata",
                "WORKLOAD_NAME",
            },
            &value)) {
      std::string result = std::string(root_->getWorkloadName());
      for (auto p : {
               "node",
               "metadata",
               "WORKLOAD_NAME",
           }) {
        result += "." + std::string(p);
      }
      result += "==";
      result += value;

      properties.push_back(result);
    } else {
      LOG_WARN("failed to get property");
    }
  }
  {
    int64_t value;
    if (getValue(
            {
                "response",
                "total_size",
            },
            &value)) {
      std::string result = std::string(root_->getWorkloadName());
      for (auto p : {
               "response",
               "total_size",
           }) {
        result += "." + std::string(p);
      }
      result += "==";
      result += std::to_string(value);

      properties.push_back(result);
    } else {
      LOG_WARN("failed to get property");
    }
  }

  LOG_WARN("number of properties collected " +
           std::to_string(properties.size()));

  properties_joined = absl::StrJoin(properties, ",");

  LOG_WARN("properties_joined:" + properties_joined);

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
    LOG_WARN("x-wasm-property: " + properties_joined);
  }

  if (root_->getWorkloadName() == "productpagev1") {
    // TODO: Construct TreeNode graph using paths and properties returned
    // and check whether the query is subgraph isomorphic to the graph
    // generated from request trace.

    std::set<std::string> vertices = {
        "d",
        "a",
        "b",
        "c",
    };

    std::vector<std::pair<std::string, std::string>> edges = {
        {
            "a",
            "b",
        },
        {
            "b",
            "c",
        },
        {
            "a",
            "d",
        },
    };

    std::map<std::string, std::map<std::vector<std::string>, std::string>>
        ids_to_properties;
    ids_to_properties["a"][{
        "node",
        "metadata",
        "WORKLOAD_NAME",
    }] = "productpagev1";
    ids_to_properties["b"][{
        "node",
        "metadata",
        "WORKLOAD_NAME",
    }] = "reviewsv2";
    ids_to_properties["c"][{
        "node",
        "metadata",
        "WORKLOAD_NAME",
    }] = "ratingsv1";
    ids_to_properties["d"][{
        "node",
        "metadata",
        "WORKLOAD_NAME",
    }] = "detailsv1";

    trace_graph_t pattern =
        generate_trace_graph(vertices, edges, ids_to_properties);
    trace_graph_t target =
        generate_trace_graph_from_headers(paths_joined, properties_joined);

    auto mapping = get_sub_graph_mapping(pattern, target);
    if (mapping == nullptr || mapping->find("") == mapping->end()) {
      LOG_WARN("No mapping found");
      return;
    }

    std::string to_store;

    const Node *node_ptr = get_node_with_id(target, mapping->at("a"));
    if (node_ptr == nullptr || node_ptr->properties.find({
                                   "response",
                                   "total_size",
                               }) == node_ptr->properties.end()) {
      LOG_WARN("no node found");
      return;
    }

    to_store = node_ptr->properties.at({
        "response",
        "total_size",
    });

    LOG_WARN("Value to store: " + to_store);

    auto context_id = id();
    auto callback = [context_id](uint32_t, size_t body_size, uint32_t) {
      getContext(context_id)->setEffectiveContext();
      auto body =
          getBufferBytes(BufferType::HttpCallResponseBody, 0, body_size);
      LOG_WARN(std::string(body->view()));
    };

    auto result = root()->httpCall("storage-upstream",
                                   {{":method", "GET"},
                                    {":path", "/store"},
                                    {":authority", "storage-upstream"},
                                    {"key", b3_trace_id_},
                                    {"value", to_store}},
                                   "", {}, 1000, callback);
    if (result != WasmResult::Ok) {
      LOG_WARN("Failed to make a call to storage-upstream: " +
               toString(result));
    }
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
      LOG_WARN("x-wasm: " + shared_data->toString() + "," + header->toString());
      setSharedData(b3_parent_span_id_ + "path",
                    shared_data->toString() + "," + header->toString());
    } else {
      LOG_WARN("x-wasm: " + header->toString());
      setSharedData(b3_parent_span_id_ + "path", header->toString());
    }
  } else {
    LOG_WARN("x-wasm not found");
  }

  auto property_header = getResponseHeader("x-wasm-property");
  if (property_header->data() != nullptr) {
    WasmDataPtr shared_data;
    WasmResult result =
        getSharedData(b3_parent_span_id_ + "property", &shared_data);
    if (result == WasmResult::Ok && shared_data->data() != nullptr) {
      LOG_WARN("x-wasm-property: " + shared_data->toString() + "," +
               property_header->toString());
      setSharedData(b3_parent_span_id_ + "property",
                    shared_data->toString() + "," +
                        property_header->toString());
    } else {
      LOG_WARN("x-wasm-property: " + header->toString());
      setSharedData(b3_parent_span_id_ + "property",
                    property_header->toString());
    }
  } else {
    LOG_WARN("x-wasm-property not found");
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

  if (direction_ == TrafficDirection::Inbound) {
    onResponseHeadersInbound();
  } else if (direction_ == TrafficDirection::Outbound) {
    onResponseHeadersOutbound();
  }

  return FilterHeadersStatus::Continue;
}
