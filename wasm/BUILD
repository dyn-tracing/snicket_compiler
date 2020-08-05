load("@rules_proto//proto:defs.bzl", "proto_library")
load("//bazel/wasm:wasm.bzl", "wasm_cc_binary")

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
