# TODO Figure out how to group sh and py binaries
load("@rules_pyvenv//:venv.bzl", "py_venv")
load("@@example_deps//:requirements.bzl", "requirement", "data_requirement")
load("@rules_python//python/pip_install:requirements.bzl", "compile_pip_requirements")
# load("@aspect_rules_py//py:defs.bzl", "py_binary", "py_library")
# load("@aspect_rules_py//py/private:py_venv.bzl", "py_venv")


# py_venv(
#     name = "py_venv_aspect",
#     deps = [":lib"],
# )

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

# TODO Not hermetic, maybe deadend
genrule(
    name = "build_venv",
    outs = ["venv.tar"],
    cmd = """
	export BUILD_ENV_INPUT="$(location :_build_venv_bin_deps.json)"
	./$(location :build_venv_bin) venv
	tar czf venv.tar --directory=venv .
	cp -r venv.tar $(OUTS)
    """,
    tools = [":build_venv_bin", "//:_build_venv_bin_deps.json"],
)

py_binary(
    name = "bin",
    srcs = ["bin.py"],
    deps = [
    	":lib",
        "@@example_deps_polars//:pkg",
    ],
)

py_venv(
    name = "build_venv_bin",
    deps = [
        ":lib",
    ],
)

compile_pip_requirements(
    name = "requirements",
    extra_args = ["--allow-unsafe"],
    requirements_in = "requirements.in",
    requirements_txt = "requirements.txt",
)
