# TODO Figure out how to group sh and py binaries
load("@rules_pyvenv//:venv.bzl", "py_venv")
load("@@example_deps//:requirements.bzl", "requirement", "data_requirement")
load("@rules_python//python/pip_install:requirements.bzl", "compile_pip_requirements")

py_library(
    name = "lib",
    srcs = ["lib.py"],
)

genrule(
    name = "terraform_bin",
    outs = ["terraform"],
    cmd = r"""
	wget https://releases.hashicorp.com/terraform/1.8.1/terraform_1.8.1_linux_amd64.zip
	unzip terraform_1.8.1_linux_amd64.zip
    	cp terraform $(OUTS)	
    """,
    executable=True,
)

genrule(
    name = "bash",
    srcs = ["bash/whatever.sh"],
    outs = ["whatever.sh"],
    cmd = r"""
    	cp $(SRCS) $(OUTS)	
    """,
    executable=True,
)

py_binary(
    name = "bin",
    srcs = ["bin.py"],
    deps = [
    	":lib",
    ],
)

py_venv(
    name = "venv",
    deps = [
        ":lib",
        "@@example_deps_polars//:pkg",
    ],
)

compile_pip_requirements(
    name = "requirements",
    extra_args = ["--allow-unsafe"],
    requirements_in = "requirements.in",
    requirements_txt = "requirements.txt",
)
