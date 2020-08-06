load("@rules_proto//proto:defs.bzl", "proto_library")
load("//bazel/wasm:wasm.bzl", "wasm_cc_binary")
load("@rules_cc//cc:defs.bzl", "cc_library", "cc_proto_library", "cc_test")

proto_library(
    name = "treenode_proto",
    srcs = [
        "treenode.proto",
    ],
)

cc_proto_library(
    name = "treenode_cc_proto",
    deps = [":treenode_proto"],
)

wasm_cc_binary(
    name = "filter.wasm",
    srcs = [
        "filter.cc",
    ],
    deps = [
        ":graph_utils",
        ":str_utils",
        "@envoy_wasm_api//:proxy_wasm_intrinsics",
    ],
)

cc_test(
    name = "filter_test",
    srcs = [
        "filter_test.cc",
    ],
    deps = [
        "@com_google_protobuf//:protobuf_lite",
        "@gtest",
        "@gtest//:gtest_main",
    ],
)

cc_library(
    name = "str_utils",
    srcs = [
        "str_utils.cc",
    ],
    hdrs = [
        "str_utils.h",
    ],
)

cc_test(
    name = "str_utils_test",
    srcs = [
        "str_utils_test.cc",
    ],
    deps = [
        ":str_utils",
        "@gtest",
        "@gtest//:gtest_main",
    ],
)

cc_library(
    name = "graph_utils",
    srcs = [
        "graph_utils.cc",
    ],
    hdrs = [
        "graph_utils.h",
    ],
    deps = [
        ":str_utils",
        "@boost//:graph",
    ],
)

cc_test(
    name = "graph_utils_test",
    srcs = [
        "graph_utils_test.cc",
    ],
    deps = [
        ":graph_utils",
        "@gtest",
        "@gtest//:gtest_main",
    ],
)
