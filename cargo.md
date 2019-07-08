# Cargo

- Cargo is Rust's build system and package manager.

## Getting Started

- Enter the following command to create a cargo project called "example"

```bash
$ cargo new example
```

This will create a new directory called "example" and add a "Cargo.toml" file a "src" directory and a "main.rs" file in the src directory.

- The "Cargo.toml" file is a config file with all of the config info about the project. (Think of it as a package.json file in React.)

## Building a Project

- Once you have created a project you can use the following command to build your project using cargo:

```bash
$ cargo build
```

(be sure to run this code in the directory of your project)

This command will create multiple files related to your project.

The executable for this project is placed within the /target/debug directory.

- To simply compile and run the code you can run the following withing the project root directory:

```bash
$ cargo run
```

- The "cargo check" command will check to see if your code will compile.
  - this is used to speed up the development process because it does not need to compile the code.

```bash
$ cargo check
```

## Release Build

- To build your project for release, run the following:

```bash
$ cargo build --release
```

This will build a project ready for release with the best optimizations possible.

> ### Note:
>
> This process takes longer as the code will be optimized for relese.
> If you simply want to build for testing us the "cargo build" command instead.
