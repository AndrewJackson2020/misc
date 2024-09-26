load("@rules_pkg//pkg:tar.bzl", "pkg_tar")
load("@rules_oci//oci:defs.bzl", "oci_image", "oci_tarball")

pkg_tar(
    name = "pgcat_tar",
    srcs = glob(["pgcat"])
)

oci_image(
    name = "container_image",
    base = "@rocky9_image",
    tars = [":pgcat_tar"],
    entrypoint = ["/pgcat"],
)

oci_tarball(
    name = "container",
    image = ":container_image",
    repo_tags = [
        "pgcat:latest",
    ],
)
