// NOLINT(namespace-envoy)
#include <map>
#include <numeric>
#include <regex>
#include <set>
#include <string>
#include <unordered_map>

#include "proxy_wasm_intrinsics.h"
#include "treenode.pb.h"

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

static unsigned char PROTO_BYTES [ 130 ] = {
    10,1,97,18,29,10,12,115,101,114,118,105,99,101,95,110,97,109,101,18,13,112,114,111,100,117,99,116,112,97,103,101,118,49,26,62,10,1,98,18,25,10,12,115,101,114,118,105,99,101,95,110,97,109,101,18,9,114,101,118,105,101,119,115,118,50,26,30,10,1,99,18,25,10,12,115,101,114,118,105,99,101,95,110,97,109,101,18,9,114,97,116,105,110,103,115,118,49,26,30,10,1,100,18,25,10,12,115,101,114,118,105,99,101,95,110,97,109,101,18,9,100,101,116,97,105,108,115,118,49,
};

class BidiRootContext : public RootContext {
public:
  explicit BidiRootContext(uint32_t id, StringView root_id)
      : RootContext(id, root_id) {
    std::string workload_name;
    if (getValue({"node", "metadata", "WORKLOAD_NAME"}, &workload_name)) {
      workload_name_ = workload_name;
      LOG_WARN("Intialized workload_name: " + workload_name_);

      if (workload_name_ == "productpagev1") {
        query_ = std::make_unique<TreeNode>();
        query_->ParseFromArray(&PROTO_BYTES, 130);
      }
    } else {
      LOG_WARN("Failed to set workload name");
    }
  }
  bool onConfigure(size_t /* configuration_size */) override;

  StringView getWorkloadName() { return workload_name_; }


private:
  std::string workload_name_;
  std::unique_ptr<TreeNode> query_;
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
  // auto header_pairs = getRequestHeaderPairs()->pairs();
  // for (const auto &pair : header_pairs) {
  //   LOG_WARN(trafficDirectionToString(direction_) + ": " +
  //            std::string(pair.first) + " --> " + std::string(pair.second));
  // }

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

FilterHeadersStatus BidiContext::onResponseHeaders(uint32_t) {
  // auto header_pairs = getResponseHeaderPairs()->pairs();
  // for (const auto &pair : header_pairs) {
  //   LOG_WARN(trafficDirectionToString(direction_) + ": " +
  //            std::string(pair.first) + " --> " + std::string(pair.second));
  // }

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
    // inbound response processing
    WasmDataPtr shared_data;
    WasmResult result = getSharedData(b3_span_id_, &shared_data);
    if (result == WasmResult::Ok && shared_data->data() != nullptr) {
      auto header_value = shared_data->toString();

      // Multiple paths could exist separated by commas.
      // split string using ','
      std::regex delimiter(",");
      std::sregex_token_iterator it{header_value.begin(), header_value.end(),
                                    delimiter, -1};
      std::vector<std::string> words{it, {}};
      for (auto &w : words) {
        w = std::string(workload_name) + "-" + w;
      }
      std::set<std::string> words_set{words.begin(), words.end()};

      if (workload_name == "productpagev1") {
          // TODO: Construct TreeNode graph using paths and properties returned
          // and check whether the query is subgraph isomorphic to the graph
          //generated from request trace.
      }

      // Now join them.
      std::string joined =
          std::accumulate(words.begin(), words.end(), std::string(),
                          [](const std::string &a,
                                          const std::string &b) -> std::string {
                            return a + (a.length() > 0 ? "," : "") + b;
                          });
      addResponseHeader("x-wasm", joined);
      LOG_WARN("inbound: x-wasm -> " + joined);
    } else {
      addResponseHeader("x-wasm", std::string(workload_name));
      LOG_WARN("inbound: x-wasm -> " + std::string(workload_name));
    }
  } else if (direction_ == TrafficDirection::Outbound) {
    // Received response from another service we called.
    // Collect x-wasm header value.
    auto header = getResponseHeader("x-wasm");
    if (header->data() != nullptr) {
      WasmDataPtr shared_data;
      WasmResult result = getSharedData(b3_parent_span_id_, &shared_data);
      if (result == WasmResult::Ok && shared_data->data() != nullptr) {
        LOG_WARN("outbound: x-wasm -> " + shared_data->toString() + "," +
                 header->toString());
        setSharedData(b3_parent_span_id_,
                      shared_data->toString() + "," + header->toString());
      } else {
        LOG_WARN("outbound: x-wasm -> " + header->toString());
        setSharedData(b3_parent_span_id_, header->toString());
      }
    } else {
      LOG_WARN("outbound: x-wasm not found");
    }
  }

  return FilterHeadersStatus::Continue;
}
