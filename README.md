# rustimizer

Little helper for decision making in Rust.

_Disclaimer: The project's name was chosen with the help of an LLM._

## Quickstart

To be done.

## Developers

### Local environment

You need:

- [Rust](https://www.rust-lang.org/).
- [GNU make](https://www.gnu.org/software/make/manual/make.html).
- [pre-commit](https://pre-commit.com/).

Set-up:

```shell
make install
```

Build and run tests:

```shell
make
```

Build for release:

```shell
make release
```

All available targets:

```shell
make help
```

### Release process

We use [release-plz](https://release-plz.dev/). Merging a PR to main automatically generates a new PR with the relevant
changes.

`release-plz` runs in the ci/cd and is not part of the local enviroment. In case of issues, you may need to install it
in your local env with `cargo install --locked release-plz`.
