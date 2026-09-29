# Repro for https://github.com/clap-rs/clap/issues/6386

Repro based on https://github.com/lgarron/repo . To repro:

```shell
git clone --depth 1 https://github.com/lgarron/clap-complete-6386-repro && cd ./clap-complete-6386-repro/

fish ./test.fish
```

- Expected: The printed completions show the `--override` option.
- Expected: The printed completions repeat the subcommands of the previous level (`add`, `edit`, `help`, `reveal`)
