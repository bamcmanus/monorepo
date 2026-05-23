# tools/gosec.bzl

"""Starlark definition for the go_sec rule."""
def _go_sec_impl(ctx):
    """Implementation for the go_sec rule."""
    # Declare the output file for the report.
    output_report = ctx.actions.declare_file(ctx.label.name + "_report.txt")

    # Collect all input files from the 'srcs' attribute.
    go_files = ctx.files.srcs

    # Prepare the arguments for the gosec command.
    # The command will be: gosec -out=<report_file> [list of go files]
    args = ctx.actions.args()
    args.add("-out=" + output_report.path)
    args.add_all(go_files)

    # Define the action to run the gosec executable.
    ctx.actions.run(
        outputs = [output_report],
        inputs = go_files,
        executable = ctx.executable._gosec,
        arguments = [args],
        mnemonic = "GoSec",
        progress_message = "🕵️ Running gosec security scan on " + ctx.label.name,
    )

    # Return the output file so other rules can use it.
    return [DefaultInfo(files = depset([output_report]))]

go_sec = rule(
    implementation = _go_sec_impl,
    attrs = {
        "srcs": attr.label_list(
            doc = "List of .go files to scan.",
            allow_files = [".go"],
            mandatory = True,
        ),
        # Define an implicit dependency on the gosec tool.
        # The 'cfg = "exec"' flag ensures it's built for the machine running Bazel.
        "_gosec": attr.label(
            doc = "The gosec tool.",
            default = Label("//tools:gosec"),
            executable = True,
            cfg = "exec",
        ),
    },
    doc = "Runs gosec security scanner on a set of Go files.",
)
