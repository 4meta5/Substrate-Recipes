# <a href="https://substrate.dev/recipes">Substrate Recipes</a> 🍴😋🍴

![Build Status](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Factions-badge.atrox.dev%2Fsubstrate-developer-hub%2Frecipes%2Fbadge%3Fref%3Dmaster&style=flat)
<!-- markdown-link-check-disable-next-line -->
![Lines of Code](https://tokei.rs/b1/github/substrate-developer-hub/recipes)
[![Try on playground](https://img.shields.io/badge/Playground-Recipes-brightgreen?logo=Parity%20Substrate)](https://playground.substrate.dev/?deploy=recipes)

_A Hands-On Cookbook for Aspiring Blockchain Chefs_

## Get Started

Ready to roll up your sleeves and cook some blockchain? Read the book online at
[substrate.dev/recipes](https://substrate.dev/recipes) 😋

## Repository Structure

There are five primary directories in this repository:

-   **Text**: Source of [the book](https://substrate.dev/recipes) written in markdown. This text
	describes the code in the other three directories.
-   **Pallets**: Pallets for use in FRAME-based runtimes.
-   **Runtimes**: Runtimes for use in Substrate nodes.
-   **Consensus**: Consensus engines for use in Substrate nodes.
-   **Nodes**: Complete Substrate nodes ready to run.

The book is built with [mdbook](https://github.com/rust-lang/mdBook) and deployed via
[github pages](https://pages.github.com/).

## Building This Book Locally

Building the book requires [mdBook], ideally the same version that
rust-lang/rust uses in [this file][rust-mdbook]. To get it:

[mdBook]: https://github.com/rust-lang-nursery/mdBook
[rust-mdbook]: https://github.com/rust-lang/rust/blob/master/src/tools/rustbook/Cargo.toml

```bash
$ cargo install mdbook --vers [version-num]
```
To build the book, type:

```bash
$ mdbook build
```

The output will be in the `book` subdirectory. To check it out, open up `book/index.html` in
a web browser, or to serve the book locally, type:

```bash
$ mdbook serve
```

The default address to view the book will be located at [http://localhost:3000](http://localhost:3000) .

## License

The Substrate Recipes are [GPL 3.0 Licensed](LICENSE) It is open source and
[open for contributions](./CONTRIBUTING.md).

## Using Recipes in External Projects

The pallets and runtimes provided here are tested and ready to be used in other Substrate-based
blockchains. The big caveat is that you must use the same upstream Substrate version throughout the
project.
