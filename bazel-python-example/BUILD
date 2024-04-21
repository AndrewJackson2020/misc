# TODO Figure out how to group sh and py binaries
py_library(
    name = "lib",
    srcs = ["lib.py"],
)

genrule(
    name = "terraform",
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

