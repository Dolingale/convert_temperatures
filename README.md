# Convert temperatures

The convert temperatures is a small program that runs in the terminal and has been developed in Rust. This program is one of three small exercises proposed at the end of module 3

This program has been realized within the framework of the self-training, realized with the [Rust Book](https://doc.rust-lang.org/book/title-page.html).

## Installation

### Windows

Install [Visual Studio 2022](https://visualstudio.microsoft.com/downloads/) with:

- “Desktop Development with C++”
- The Windows 10 or 11 SDK
- The English language pack component, along with any other language pack of your choosing

Then install [rustup](https://www.rust-lang.org/tools/install)

Then check if rustup is well installed

```shell
$ rustc --version
```
### Linux and macOs

If you’re using Linux or macOS, open a terminal and enter the following command:

```shell
$ curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
```

You will also need a linker, which is a program that Rust uses to join its compiled outputs into one file. It is likely you already have one. If you get linker errors, you should install a C compiler, which will typically include a linker. A C compiler is also useful because some common Rust packages depend on C code and will need a C compiler.

On macOS, you can get a C compiler by running:

```shell
$ xcode-select --install
```

Linux users should generally install GCC or Clang, according to their distribution’s documentation. For example, if you use Ubuntu, you can install the ```build-essential ```package.

## Usage

In the terminal run to install the dependencies:

```shell
cargo install
```

And then run the program with :

```shell
cargo run
```

Then enter in the terminal the conversion you want (Fahrenheit to Celsius or Celsius to Fahrenheit), then the initial temperature.

## License
No license