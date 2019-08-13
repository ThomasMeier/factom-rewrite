# Building Factom

To build Factom daemon, you'll need to install rust. You will need to clone this repository and checkout the latest release tag. After which, you must run:

```
./scripts/init.sh
./scripts/build.sh
caro build --release
```

This will compile the runtime and build it into an executible you will find inside `./target`.

Subsequent builds simply omit the `./scripts/init.sh` step.

## Shell Completion

Shell completion scripts are generated when you build Factom. You will find them in `target/release/completion-scripts/factom.bash`. In addition to `bash` there are also scripts for `fish`, `zsh`, and `powershell`.