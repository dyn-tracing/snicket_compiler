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

class AddHeaderRootContext : public RootContext {
public:
  explicit AddHeaderRootContext(uint32_t id, std::string_view root_id)
      : RootContext(id, root_id) {}
  bool onConfigure(size_t /* configuration_size */) override;

  bool onStart(size_t) override;

  std::string header_value_;
};

class AddHeaderContext : public Context {
public:
  explicit AddHeaderContext(uint32_t id, RootContext *root)
      : Context(id, root),
        root_(static_cast<AddHeaderRootContext *>(static_cast<void *>(root))) {}

  void onCreate() override;
  FilterHeadersStatus onRequestHeaders(uint32_t headers,
                                       bool end_of_stream) override;
  FilterDataStatus onRequestBody(size_t body_buffer_length,
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
    register_AddHeaderContext(CONTEXT_FACTORY(AddHeaderContext),
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

void AddHeaderContext::onCreate() {
  LOG_DEBUG(std::string("onCreate " + std::to_string(id())));
}

FilterHeadersStatus AddHeaderContext::onRequestHeaders(uint32_t, bool) {
  TrafficDirection direction = getTrafficDirection();
  if (direction == TrafficDirection::Inbound) {
    WasmDataPtr hdr = getRequestHeader("test_key");
    LOG_WARN("INCOMING REQUEST FROM  " + hdr->toString());
    removeRequestHeader("test_key");
  } else if (direction == TrafficDirection::Outbound) {
    LOG_WARN("OUTGOING REQUEST");
    std::string workload_name;
    if (getValue({"node", "metadata", "WORKLOAD_NAME"}, &workload_name)) {
      LOG_WARN("Initialized workload_name: " + workload_name);
      addRequestHeader("test_key", workload_name);
    } else {
      LOG_WARN("Failed to set workload name");
    }
  } else {
    LOG_WARN("Weird direction.");
  }
  return FilterHeadersStatus::Continue;
}

FilterHeadersStatus AddHeaderContext::onResponseHeaders(uint32_t, bool) {
  TrafficDirection direction = getTrafficDirection();
  if (direction == TrafficDirection::Inbound) {
    WasmDataPtr hdr = getResponseHeader("test_key");
    LOG_WARN("INCOMING RESPONSE FROM  " + hdr->toString());
    removeResponseHeader("test_key");
  } else if (direction == TrafficDirection::Outbound) {
    LOG_WARN("OUTGOING RESPONSE");
    std::string workload_name;
    if (getValue({"node", "metadata", "WORKLOAD_NAME"}, &workload_name)) {
      LOG_WARN("Initialized workload_name: " + workload_name);
      addResponseHeader("test_key", workload_name);
    } else {
      LOG_WARN("Failed to set workload name");
    }
  } else {
    LOG_WARN("Weird direction.");
  }
  return FilterHeadersStatus::Continue;
}
FilterDataStatus AddHeaderContext::onRequestBody(size_t body_buffer_length,
                                                 bool end_of_stream) {
  return FilterDataStatus::Continue;
}

void AddHeaderContext::onDone() {
  LOG_DEBUG(std::string("onDone " + std::to_string(id())));
}

void AddHeaderContext::onLog() {
  LOG_DEBUG(std::string("onLog " + std::to_string(id())));
}

void AddHeaderContext::onDelete() {
  LOG_DEBUG(std::string("onDelete " + std::to_string(id())));
}
