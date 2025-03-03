# MIRAI  [![Build Status](https://travis-ci.com/facebookexperimental/MIRAI.svg?token=uaX9rExVwSVz5FfMFphz&branch=master)](https://travis-ci.com/facebookexperimental/MIRAI) [![codecov](https://codecov.io/gh/facebookexperimental/MIRAI/branch/master/graph/badge.svg?token=q4jzL09Ahl)](https://codecov.io/gh/facebookexperimental/MIRAI)
MIRAI is an abstract interpreter for the [Rust](https://www.rust-lang.org/) compiler's [mid-level intermediate
representation](https://github.com/rust-lang/rfcs/blob/master/text/1211-mir.md) (MIR).
It is intended to become a widely used static analysis tool for Rust.
The initial focus will be on taint analysis.

## Using MIRAI

You'll need to install MIRAI as described [here](https://github.com/facebookexperimental/MIRAI/blob/master/documentation/InstallationGuide.md).

To run mirai, use cargo with `RUSTC_WRAPPER` set to `mirai`.
Use `rustup override` to make Cargo use the same version of Rust as MIRAI as
described in the [Installation Guide](https://github.com/facebookexperimental/MIRAI/blob/master/documentation/InstallationGuide.md).

The easiest way to get started is to first build your project in the normal way.
Refer [this link](https://doc.rust-lang.org/1.30.0/book/first-edition/getting-started.html) for details
on compiling a cargo project.
When there are no compile errors,
no lint errors and no test failures, you can proceed to the next step and run MIRAI. For example:
```
cargo clean -p my_crate_name
RUSTC_WRAPPER=mirai cargo check
```

This will likely produce a lot of warnings, which you can then fix by adding annotations using this
 [crate](https://crates.io/crates/mirai-annotations). Keep running cargo check as above until there are no more warnings.

At this stage your code will be better documented and more readable. Perhaps you'll also have found and fixed a few bugs.

Set the verbosity level of output from MIRAI by setting the environment variable `MIRAI_LOG` to one of
`info`, `warn`, `debug`, or `trace`.

To go beyond this, super lint + better documentation phase, you'll need to be aware of a lot more things.

## Developing MIRAI
See the [developer guide](https://github.com/facebookexperimental/MIRAI/blob/master/documentation//DeveloperGuide.md)
for instructions on how to build, run and debug MIRAI.

## Full documentation
* [Overview of project](https://github.com/facebookexperimental/MIRAI/blob/master/documentation/Overview.md).
* [Architecture](https://github.com/facebookexperimental/MIRAI/blob/master/documentation/Architecture.md).
* [Design discussions](https://github.com/facebookexperimental/MIRAI/blob/master/documentation/DesignDiscussions.md).
* [Further reading](https://github.com/facebookexperimental/MIRAI/blob/master/documentation/FurtherReading.md).

## Road map
* Set up visitor infrastructure for MIR (Early December)
* Provide a way to store and retrieve function summaries (Late December)
* State tracking and memory operations (January)
* Design Abstract Domain abstraction and implement some domains (February)
* Full scale Abstract Interpreter (March, April)
* Expression simplifier
* Hook up SMT solver
* Work on scalability
* Deploy MIRAI in the build system of a large project
* More domains and refined diagnostics

## Join the MIRAI community
<!-- * Website:
* Facebook page:
* Mailing list
* irc:  -->
See the [CONTRIBUTING](https://github.com/facebookexperimental/MIRAI/blob/master/CONTRIBUTING.md) file for how to help out.

## License
MIRAI is MIT licensed, as found in the [LICENSE](https://github.com/facebookexperimental/MIRAI/blob/master/LICENSE) file.
