# cargo doc

Difference between `cargo doc` and `cargo rustdoc`:

Their relationship is like between cargo build and cargo rustc: cargo doc performs all the usual work, for an entire workspace, including dependencies (by default). cargo rustdoc allows you to pass flags directly to rustdoc, and only works for a single crate.

Here is the execution code for cargo rustdoc. Here is the code for cargo doc. The only differences is that cargo rustdoc always specify to not check dependencies while cargo doc allows you to choose (by default it does, but you can specify the flag --no-deps), and that cargo rustc allows you to pass flags directly to rustdoc with the flags after the --.

For avoid adding third-party documents:
To remember it: 

```shell
cargo doc --no-deps
```
or
```shell
cargo rustdoc
```



## future
It is for Personal knowledge.

***To remember it:***
cargo doc --no-deps

Open
```
#![warn(missing_docs)]
#![deny(missing_docs)]
#![feature(rustdoc_missing_doc_code_examples)]
#![warn(rustdoc::missing_doc_code_examples)]
```
It was very hard!