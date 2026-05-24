load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_test")

rust_binary(
    name = "hello-world-rust",
    srcs = ["src/main.rs"],
)

rust_test(
    name = "test",
    crate = ":hello-world-rust",
)
