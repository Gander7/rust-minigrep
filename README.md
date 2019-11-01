# rust-minigrep
A simple reimplemenation of grep that the rust docs use to help devs learn

## Usage
```
cargo run sun poem.txt
```
The above will look for, and print to stdout, all lines in poem.txt that contain sun.
The environment variable `CASE_INSENSITIVE` can be set to ignore case during searches.
The value of `CASE_INSENSITIVE` does not matter, it only matters that it is set.

## source
The lesson for it can be found [here](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
Reading previous lessons is recommended.
