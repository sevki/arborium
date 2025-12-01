# Starlark (Bazel BUILD file)
load("@rules_python//python:defs.bzl", "py_binary", "py_library")

py_library(
    name = "my_lib",
    srcs = ["lib.py"],
    deps = [
        "//third_party:requests",
        "@pip//numpy",
    ],
    visibility = ["//visibility:public"],
)

py_binary(
    name = "main",
    srcs = ["main.py"],
    deps = [":my_lib"],
    data = glob(["data/*.json"]),
)

# Custom rule
def _impl(ctx):
    output = ctx.actions.declare_file(ctx.label.name + ".out")
    ctx.actions.run(
        outputs = [output],
        inputs = ctx.files.srcs,
        executable = ctx.executable._tool,
    )
    return [DefaultInfo(files = depset([output]))]
