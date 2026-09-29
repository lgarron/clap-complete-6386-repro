#!/usr/bin/env -S fish --no-config --

function test_command
    cargo run -- $argv
end
test_command completions fish | source

echo ""
complete --do-complete "test_command boilerplate rust-toolchain add "

echo ""
complete --do-complete "test_command boilerplate rust-toolchain --"

echo ""
complete --do-complete "test_command boilerplate rust-toolchain add --"
