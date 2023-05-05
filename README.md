# ZAMA TFHE-rs Sandbox

![ZAMA](https://801304998-files.gitbook.io/~/files/v0/b/gitbook-x-prod.appspot.com/o/spaces%2FZTH0QvT3qRVMHlneX3h5%2Fuploads%2Fgit-blob-38a3bd421919d4950aa21f138757da7b54532f87%2Ftfhe-rs-doc-home.png?alt=media "ZAMA")

## Background

**TFHE-rs** is a pure Rust implementation of **TFHE** for Boolean and integer arithmetics over encrypted data. It includes a Rust and C API, as well as a client-side WASM API.
**TFHE-rs** is meant for developers and researchers who want full control over what they can do with **TFHE**, while not worrying about the low level implementation.
The goal is to have a stable, simple, high-performance, and production-ready library for all the advanced features of **TFHE**.

## Description

This sandbox relies on [ZAMA](https://www.zama.ai/) technology for the **Fully Homomorphic Encryption** Rust library called **TFHE-rs**.
You can consider this repository as a boilerplate in order to easily bootstrap your experiments with this technology.

There are 3 **circuits** available:

* **boolean**
* **short int**
* **integer**

## Installation

```console
cargo build
```

## Usage

```console
cargo run --release
```
