GPT-CLI
===

A Simple CLI client for using OpenAI GPT API.

This project is written in Rust.

## Usage 

```shell
gpt-cli config # configure the client
gpt-cli chat <your api key>
>>> Hello!

AI: How can I assist you with excellence today?

>>> Tell everybody I like Rust!

AI: Sure thing! Let everyone know - "Rust is their absolute favorite!"

>>> ::exit
```

## Build & Install

If you have Rust environment, just use `cargo install --path .` to build and 
install it from the source code.

Otherwise, you can directly use a prepared binary to run it.