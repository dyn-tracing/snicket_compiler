// NOLINT(namespace-envoy)
#include <string>
#include <unordered_map>

#include "google/protobuf/util/json_util.h"
#include "proxy_wasm_intrinsics.h"
#include "filter.pb.h"

#define REQUEST_THRESHOLD 10
#define STORAGE_NAME "storage-upstream"

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
  explicit AddHeaderRootContext(uint32_t id, std::string_view root_id) : RootContext(id, root_id) {
    std::string workload_name;
    if (getValue({"node", "metadata", "WORKLOAD_NAME"}, &workload_name)) {
            workload_name_ = workload_name;
            std::string warning = "initialized workload_name: ";
            warning = warning.append(workload_name_);
            LOG_WARN(warning);
    } else {
            LOG_WARN("Failed to set workload name");
    }
    direction_ = getTrafficDirection();
    count_ = 0;
    int tmp_count = 0;
    std::string count_key = "count";
    proxy_set_shared_data(count_key.c_str(), count_key.size(),
                              (const char *)&tmp_count, sizeof(tmp_count), 0);
  }
  bool onConfigure(size_t /* configuration_size */) override;

  bool onStart(size_t) override;
  std::string getWorkloadName() { return workload_name_; }
  //void incrementCount() { count_++; }
  //void decrementCount() { count_--; }
  //int getCount() { return count_; }
  void incrementCount() {
        int *tmp_count;
        size_t *size;
        WasmResult result;

        std::string count_key = "count";
        result = proxy_get_shared_data(count_key.c_str(), count_key.size(),
                                       (const char **)&tmp_count, size, 0);
        if (result != WasmResult::Ok) {
            LOG_ERROR("Failed to get shared data " + count_key + ".");
            return;
        }
        (*tmp_count)++;
        result = proxy_set_shared_data(count_key.c_str(), count_key.size(),
                                       (const char *)tmp_count,
                                       sizeof(*tmp_count), 0);
        if (result != WasmResult::Ok) {
            LOG_ERROR("Failed to set shared data " + count_key + ".");
        }
    }
    void decrementCount() {
        WasmResult result;
        int *tmp_count;
        size_t *size;
        std::string count_key = "count";
        result = proxy_get_shared_data(count_key.c_str(), count_key.size(),
                                       (const char **)&tmp_count, size, 0);
        if (result != WasmResult::Ok) {
            LOG_ERROR("Failed to get shared data " + count_key + ".");
            return;
        }
        (*tmp_count)--;
        result = proxy_set_shared_data(count_key.c_str(), count_key.size(),
                                       (const char *)tmp_count,
                                       sizeof(*tmp_count), 0);
        if (result != WasmResult::Ok) {
            LOG_ERROR("Failed to set shared data " + count_key + ".");
        }
    }
    int getCount() {
        WasmResult result;
        int *tmp_count;
        size_t *size;
        std::string count_key = "count";
        result = proxy_get_shared_data(count_key.c_str(), count_key.size(),
                                       (const char **)&tmp_count, size, 0);
        if (result != WasmResult::Ok) {
            LOG_ERROR("Failed to get shared data " + count_key + ".");
            return -1;
        }
        return *tmp_count;
    }

  std::string header_value_;

private:
  std::string workload_name_;
  int count_;
  TrafficDirection direction_;
};

class AddHeaderContext : public Context {
public:
  explicit AddHeaderContext(uint32_t id, RootContext* root) : Context(id, root), root_(static_cast<AddHeaderRootContext*>(static_cast<void*>(root))) {
    direction_ = getTrafficDirection();
  }

  void onCreate() override;
  FilterHeadersStatus onRequestHeaders(uint32_t headers, bool end_of_stream) override;
  FilterHeadersStatus onRequestHeadersInbound();
  FilterHeadersStatus onRequestHeadersOutbound();
  FilterDataStatus onRequestBody(size_t body_buffer_length, bool end_of_stream) override;
  FilterHeadersStatus onResponseHeaders(uint32_t headers, bool end_of_stream) override;
  FilterHeadersStatus onResponseHeadersInbound();
  FilterHeadersStatus onResponseHeadersOutbound();
  void onDone() override;
  void onLog() override;
  void onDelete() override;
private:
  void store_warning(std::string count);
  TrafficDirection direction_;
  AddHeaderRootContext* root_;
};
static RegisterContextFactory register_AddHeaderContext(CONTEXT_FACTORY(AddHeaderContext),
                                                      ROOT_FACTORY(AddHeaderRootContext),
                                                      "add_header_root_id");

void AddHeaderContext::store_warning(std::string count) {
    std::size_t am_review = root_->getWorkloadName().find("review");
    if (am_review == std::string::npos) {
        LOG_WARN("i am not reviews, so I do not store my count");
	return;
    }
    std::string key = toString(getCurrentTimeNanoseconds());
    std::string value = root_->getWorkloadName() + ":" + count;
    //std::string value = root_->getWorkloadName();

    LOG_WARN("Storing timestamp " + key + " as node " + value + ".");
    auto context_id = id();
    auto callback = [context_id](uint32_t, size_t body_size, uint32_t) {
        getContext(context_id)->setEffectiveContext();
        auto body =
            getBufferBytes(WasmBufferType::HttpCallResponseBody, 0, body_size);
        LOG_WARN(std::string(body->view()));
    };
    auto result = root()->httpCall(STORAGE_NAME,
                                   {{":method", "GET"},
                                    {":path", "/store"},
                                    {":authority", STORAGE_NAME},
                                    {"key", key},
                                    {"value", value}},
                                   "", {}, 1000, callback);
    if (result != WasmResult::Ok) {
        LOG_ERROR("Failed to make a call to " + STORAGE_NAME + ": " +
                  toString(result));
    }
    return;
}

bool AddHeaderRootContext::onConfigure(size_t config_buffer_length) {
  auto conf = getBufferBytes(WasmBufferType::PluginConfiguration, 0, config_buffer_length);
  LOG_DEBUG("onConfigure " + conf->toString());
  header_value_ = conf->toString();
  return true; 
}

bool AddHeaderRootContext::onStart(size_t) { LOG_DEBUG("onStart"); return true;}

void AddHeaderContext::onCreate() { LOG_DEBUG(std::string("onCreate " + std::to_string(id()))); }

FilterHeadersStatus AddHeaderContext::onRequestHeaders(uint32_t, bool) {
  LOG_WARN(std::string("onRequestHeaders ") + std::to_string(id()));
  LOG_WARN("REQUEST BEGIN #######################################");
  FilterHeadersStatus status;
  
  if (direction_ == TrafficDirection::Inbound) {
        status = onRequestHeadersInbound();
    } else if (direction_ == TrafficDirection::Outbound) {
        status = onRequestHeadersOutbound();
    } else {
        LOG_ERROR("Missing request header direction.");
        status = FilterHeadersStatus::Continue;
    }
  LOG_WARN("REQUEST END ############################################");
  return status;
}
FilterHeadersStatus AddHeaderContext::onRequestHeadersInbound() {
  root_->incrementCount();
  std::string count_as_string = std::to_string(root_->getCount());
  store_warning(count_as_string);
  return FilterHeadersStatus::Continue;
}
FilterHeadersStatus AddHeaderContext::onRequestHeadersOutbound() {
  return FilterHeadersStatus::Continue;
}

FilterHeadersStatus AddHeaderContext::onResponseHeaders(uint32_t, bool) {
  LOG_DEBUG(std::string("onResponseHeaders ") + std::to_string(id()));
  LOG_WARN(std::string("onResponseHeaders ") + std::to_string(id()));
  //addResponseHeader("newheader", root_->header_value_);
  addResponseHeader("hi", "hello");

  LOG_WARN("RESPONSE BEGIN #######################################");
  FilterHeadersStatus status;
  if (direction_ == TrafficDirection::Inbound) {
        status = onResponseHeadersInbound();
    } else if (direction_ == TrafficDirection::Outbound) {
        status = onResponseHeadersOutbound();
    } else {
        LOG_ERROR("Missing request header direction.");
        status = FilterHeadersStatus::Continue;
    }
  LOG_WARN("RESPONSE END ############################################");
  return status;
}

FilterHeadersStatus AddHeaderContext::onResponseHeadersInbound() {
  LOG_WARN("response headers inbound");
  return FilterHeadersStatus::Continue;
}

FilterHeadersStatus AddHeaderContext::onResponseHeadersOutbound() {
  LOG_WARN("response headers outbound");
  root_->decrementCount();
  std::string count_as_string = std::to_string(root_->getCount());
  LOG_WARN("count is " + count_as_string);
  LOG_WARN("workload name is " + root_->getWorkloadName());
  replaceResponseHeader("location", "envoy-wasm");
  store_warning(count_as_string);
  return FilterHeadersStatus::Continue;
}

FilterDataStatus AddHeaderContext::onRequestBody(size_t body_buffer_length, bool end_of_stream) {
  return FilterDataStatus::Continue;
}

void AddHeaderContext::onDone() { LOG_DEBUG(std::string("onDone " + std::to_string(id()))); }

void AddHeaderContext::onLog() { LOG_DEBUG(std::string("onLog " + std::to_string(id()))); }

void AddHeaderContext::onDelete() { LOG_DEBUG(std::string("onDelete " + std::to_string(id()))); }
