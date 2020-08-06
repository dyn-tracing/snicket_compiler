load("@rules_cc//cc:defs.bzl", "cc_library")

include_pattern = "boost/%s/"

hdrs_patterns = [
    "boost/%s.h",
    "boost/%s_fwd.h",
    "boost/%s.hpp",
    "boost/%s_fwd.hpp",
    "boost/%s/**/*.hpp",
    "boost/%s/**/*.ipp",
    "boost/%s/**/*.h",
]

def includes_list(library_name):
    return [".", include_pattern % library_name]

def hdr_list(library_name, exclude = []):
    return native.glob([p % (library_name,) for p in hdrs_patterns], exclude = exclude)

def boost_library(
        name,
        boost_name = None,
        defines = None,
        includes = None,
        hdrs = None,
        srcs = None,
        deps = None,
        copts = None,
        exclude_src = [],
        exclude_hdr = [],
        linkopts = None,
        linkstatic = None,
        visibility = ["//visibility:public"]):
    if boost_name == None:
        boost_name = name

    if defines == None:
        defines = []

    if includes == None:
        includes = []

    if hdrs == None:
        hdrs = []

    if srcs == None:
        srcs = []

    if deps == None:
        deps = []

    if copts == None:
        copts = []

    if linkopts == None:
        linkopts = []

    return cc_library(
        name = name,
        srcs = [],
        hdrs = hdr_list(boost_name, exclude_hdr) + hdrs,
        copts = copts,
        defines = defines,
        includes = includes_list(boost_name) + includes,
        licenses = [],
        linkopts = linkopts,
        linkstatic = linkstatic,
        visibility = visibility,
        deps = deps,
    )

