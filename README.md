# quilt

[![Rust CI](https://github.com/quilt-lang/quilt/actions/workflows/rust.yml/badge.svg)](https://github.com/quilt-lang/quilt/actions/workflows/rust.yml)

A programming language

## Usage

```
quilt examples/hello_world.png
quilt --pixel-size 20 examples/hello_world_x20.png
```

## Development

### Useful commands

- Run all checks: `make check`
- Scale images using ImageMagick: `convert -scale 500% foo.png foo_x5.png`
