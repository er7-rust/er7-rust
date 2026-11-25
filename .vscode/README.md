# VS Code workspace

Open the repository root. Install the recommended extensions
(`rust-lang.rust-analyzer`, `vadimcn.vscode-lldb`).

- **Fixtures are read-only.** `er7/samples/*.er7` and
  `er7-redact/samples/*.er7` hold bare-CR wire bytes; a normal editor save
  can rewrite them as CRLF or add a final newline, and the round-trip tests
  then fail. Change them intentionally with a byte-aware editor.
- **Tasks** (*Tasks: Run Task*; *Run Test Task* runs the workspace tests):
  workspace tests, the parser round-trip contract, strict Clippy, fmt, the
  three `-D missing-docs` rustdoc checks, and the site check/build.
- **Debugging.** *Debug parser with synthetic message* (F5, CodeLLDB on
  macOS/Linux) builds `er7` with `profile.dev.debug="full"` so `parse`
  shows its locals, and runs it on `er7/samples/multilingual.er7`. Windows
  MSVC debugging is not configured or verified.
- **Fuzzing** is a separate workspace: open `er7/fuzz` for it.
