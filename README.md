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

We use [release-plz](https://release-plz.dev/).

1. When you are ready for a new release, manually trigger the "[Release](./github/workflows/release.yaml)"" workflow.
2. This creates a new unpublished package and generates a new GitHub release, as well as the PR with the updated
   changelog.
3. Review, approve and merge the PR.

Every merge to main (including any commit) runs the
"[Publish Unreleased Packages](./github/workflows/publish-unreleased.yaml)" workflow that takes care of publishing.
Therefore, after step 3, this will run and automatically publish the new release.

`release-plz` runs in the ci/cd and is not part of the local enviroment. In case of issues, you may need to install it
in your local env with:
```shell
cargo install --locked cargo-semver-checks release-plz
```
