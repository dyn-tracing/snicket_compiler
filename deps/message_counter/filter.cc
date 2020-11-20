// NOLINT(namespace-envoy)
#include <string>
#include <unordered_map>

#include "google/protobuf/util/json_util.h"
#include "proxy_wasm_intrinsics.h"
#include "filter.pb.h"

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
  explicit BidiRootContext(uint32_t id, std::string_view root_id) : RootContext(id, root_id) {
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
  explicit BidiContext(uint32_t id, RootContext* root) : Context(id, root), root_(static_cast<BidiRootContext*>(static_cast<void*>(root))) {
    direction_ = getTrafficDirection();
    std::string warning = "Got traffic direction, is ";
    warning = warning.append(trafficDirectionToString(direction_));
    LOG_WARN(warning);
  }

  void onCreate() override;
  FilterHeadersStatus onRequestHeaders(uint32_t headers, bool end_of_stream) override;
  FilterDataStatus onRequestBody(size_t body_buffer_length, bool end_of_stream) override;
  FilterHeadersStatus onResponseHeaders(uint32_t headers, bool end_of_stream) override;
  void onResponseHeadersInbound();
  void onResponseHeadersOutbound();
  void onDone() override;
  void onLog() override;
  void onDelete() override;
private:

  BidiRootContext* root_;
  std::string b3_trace_id_;
  std::string b3_span_id_;
  std::string b3_parent_span_id_;
  TrafficDirection direction_;
};

static RegisterContextFactory register_BidiContext(CONTEXT_FACTORY(BidiContext),
                                                      ROOT_FACTORY(BidiRootContext),
                                                      "add_header_root_id");

bool BidiRootContext::onConfigure(size_t config_buffer_length) {
  auto conf = getBufferBytes(WasmBufferType::PluginConfiguration, 0, config_buffer_length);
  LOG_DEBUG("onConfigure " + conf->toString());
  header_value_ = conf->toString();
  return true; 
}

bool BidiRootContext::onStart(size_t) { LOG_DEBUG("onStart"); return true;}

void BidiContext::onCreate() { LOG_DEBUG(std::string("onCreate " + std::to_string(id()))); }

FilterHeadersStatus BidiContext::onRequestHeaders(uint32_t, bool) {
  LOG_DEBUG(std::string("onRequestHeaders ") + std::to_string(id()));
  return FilterHeadersStatus::Continue;
}

void BidiContext::onResponseHeadersInbound() {
  LOG_DEBUG("in on response headers inbound");
  addResponseHeader("responseheaderINbound", "hi");
}

void BidiContext::onResponseHeadersOutbound() {
  LOG_DEBUG("in on response headers outbound");
  addResponseHeader("responseheaderOUTbound", "hi");
}


FilterHeadersStatus BidiContext::onResponseHeaders(uint32_t, bool) {
  LOG_DEBUG(std::string("onResponseHeaders ") + std::to_string(id()));
  addResponseHeader("newheader", root_->header_value_);
  replaceResponseHeader("location", "envoy-wasm");

  if (direction_ == TrafficDirection::Inbound) {
    onResponseHeadersInbound();
  } else if (direction_ == TrafficDirection::Outbound) {
    onResponseHeadersOutbound();
  }
  return FilterHeadersStatus::Continue;
}

FilterDataStatus BidiContext::onRequestBody(size_t body_buffer_length, bool end_of_stream) {
  return FilterDataStatus::Continue;
}

void BidiContext::onDone() { LOG_DEBUG(std::string("onDone " + std::to_string(id()))); }

void BidiContext::onLog() { LOG_DEBUG(std::string("onLog " + std::to_string(id()))); }

void BidiContext::onDelete() { LOG_DEBUG(std::string("onDelete " + std::to_string(id()))); }
