# Rust

All of my notes and projects from my Rust learning journey

## Instalation Linux/Mac OS

---

Run the following commands in order.

This command downloads a script and starts the installation of the rustup tool, which installs the latest stable version of Rust:

```bash
$ curl https://sh.rustup.rs -sSf | sh
```

Run the following to add Rust to your path:

```bash
$ source $HOME/.cargo/env
```

Run the following to update Rust to the latest version:

```bash
$ rustup update
```

Check ther version of Rust by running:

```bash
$ rustc --version
```

## General Notes

---

- Rust files always end with the .rs extension. If you’re using more than one word in your filename,use an underscore to separate them. For example, use hello_world.rs rather than helloworld.rs.

- The "main" function is the first line of code that will run in ever Rust program:

```rust
fn main() {}
```

> ## Note:
>
> ### Formatting Code
>
> Run the following command to format your code:
>
> ```bash
> $ rustfmt example.rs
> ```

## Compiling and Running

- Before running a Rust program, you must compile it using the Rust compiler byt entering the "rustc" command.

example:

```bash
$ rustc example.rs
```

you should see two files after this runs. The "example.rs" and the "example" file.

## Cargo

see cargo.md file

## Prelude
 - The [prelude](https://doc.rust-lang.org/std/prelude/index.html) is the list of things that Rust automatically import into every Rust program. 