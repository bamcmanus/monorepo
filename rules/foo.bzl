def _foo_binary_impl(ctx):
    file = ctx.actions.declare_file(ctx.label.name)
    ctx.actions.write(
        output = file,
        content = "Hello, {}!\n".format(ctx.attr.username),
    )
    return [DefaultInfo(files = depset([file]))]

foo_binary = rule(
    implementation = _foo_binary_impl,
    attrs = {
        "username": attr.string(),
    },
)

print("bzl file evaluation")
