# TODO Figure out how to group sh and py binaries
py_library(
    name = "lib",
    srcs = ["lib.py"],
    srcs_version = "PY2AND3",
)

sh_binary(
    name = "whatever.sh",
    srcs = ["bash/whatever.sh"],
)

py_binary(
    name = "bin",
    srcs = ["bin.py"],
    srcs_version = "PY2AND3",
    deps = [":lib"],
)

filegroup(
    name = "srcs",
    srcs = ["BUILD"] + glob(["**/*.py"]) + glob(["**/*.sh"]),
)
