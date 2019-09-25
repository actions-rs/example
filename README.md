# @actions-rs example

![CC0 licensed](https://img.shields.io/github/license/actions-rs/example)
[![Gitter](https://badges.gitter.im/actions-rs/community.svg)](https://gitter.im/actions-rs/community)
[![Coverage Status](https://coveralls.io/repos/github/actions-rs/example/badge.svg?branch=master)](https://coveralls.io/github/actions-rs/example?branch=master)

This is a Rust application example which is using GitHub Actions for CI.

## Workflows

All workflows here can be seen at the [Actions](https://github.com/actions-rs/example/actions) tab
and their configurations are [here](https://github.com/actions-rs/example/tree/master/.github/workflows).

**NOTE**: All jobs there are using [`continue-on-error: true`](https://help.github.com/en/articles/workflow-syntax-for-github-actions#jobsjob_idstepscontinue-on-error)
in order to execute the workflow completely even if there were any errors.\
Consider removing these lines in your own workflows.

### [Quickstart](https://github.com/actions-rs/example/blob/master/.github/workflows/quickstart.yml)

Based on the ["Quickstart" recipe](https://github.com/actions-rs/meta/blob/master/recipes/quickstart.md),
represents minimal reasonable CI suite for any Rust project.

### [MSRV](https://github.com/actions-rs/example/blob/master/.github/workflows/msrv.yml)

Based on the ["MSRV" recipe](https://github.com/actions-rs/meta/blob/master/recipes/msrv.md),
same as "Quickstart" workflow, but both for `stable` and MSRV (*Minimal Supported Rust version*) toolchains.

### [Nightly lints](https://github.com/actions-rs/example/blob/master/.github/workflows/nightly_lints.yml)

Based on the ["Nightly lints" recipe](https://github.com/actions-rs/meta/blob/master/recipes/nightly-lints.md),
searches for the most recent `nightly` toolchain with `clippy` and `rustfmt` available
in order to execute linters.

### [Cross compile](https://github.com/actions-rs/example/blob/master/.github/workflows/cross_compile.yml)

[`@actions-rs/cargo`](https://github.com/actions-rs/cargo) Action
can install [`cross`](https://github.com/rust-embedded/cross) on demand.

Workflow uses this "magic" option to build application for `armv7-unknown-linux-gnueabihf` and `powerpc64-unknown-linux-gnu` targets.

### [grcov](https://github.com/actions-rs/example/blob/master/.github/workflows/grcov.yml)

This workflow is using [`-Z profile`](https://github.com/rust-lang/rust/issues/42524) feature
and [`grcov`](https://github.com/mozilla/grcov) tool
to collect and aggregate code coverage data for multi-platform builds,
which is pushed to the [coveralls.io](https://coveralls.io/github/actions-rs/example) later.
