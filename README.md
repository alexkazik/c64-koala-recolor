# c64-koala-recolor

## Requirements

* rust (https://rustup.rs/)

## Running

The program expects four colors as an argument, and either reads a koala image from stdin
and writes the koala with adapted colors to stdout:

```shell
cargo run -- --colors 0,1,7,10 < input.koa > output.koa
```

Or replaces one or more images with a adapted version:

```shell
cargo run -- --colors 0,1,7,10 replace /path/to/image.koa [...]
```

Or maybe even many:

```shell
cargo run -- --colors 0,1,7,10 replace /path/to/*.koa
```
