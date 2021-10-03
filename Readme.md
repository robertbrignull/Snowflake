Also see https://github.com/robertbrignull/Snowflake-c for another implementation of this same problem.

# Snowflake

Generates snowflakes by simulating random walks of individual particles.

## Use

### Compiling

To compile, run
```bash
cargo build --release
```

### Running

#### Generating a snowflake

To generate a snowflake, run
```bash
cargo run --release generate --flake-file output.flake --num-particles <NUM>
```

This will generate a flake to `output.flake`. This file can then later be rendered to produce an image, movie, or other statistics.

You can also continue an existing flake by settings the `--flake-file` argument to an existing flake file.

To see all arguments, run
```bash
cargo run --release generate --help
```

#### Rendering a snowflake

To render a snowflake as an image, run
```bash
cargo run --release render --flake-file output.flake --output output.png
```

This will render the flake as a png image.

To see all arguments, run
```bash
cargo run --release render --help
```

### Running tests

To run all fast unit tests, run
```bash
cargo test --release
```

To run performance tests, run
```bash
cargo test --release perf -- --ignored --show-output
```
