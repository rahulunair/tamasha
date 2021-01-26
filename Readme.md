## Tamasha

A cli client to fetch jokes using JokeAPI in Rust.

### Clone and Install

```bash
git clone https://github.com/rahulunair/tamasha.git
cd tamasha && cargo build --release
```

The binary will be in `./target/release/`

### How to use

The simplest way to use `tamasha` is:

```bash
tamasha --fetch
```
This will fetch a joke from one of the categories:

```bash
- Misc
- Programming
- Dark
- Pun 
- Spooky
- Christmas
```
For example, 

```bash
./target/release/tamasha --fetch
Why did the Python programmer not respond to the foreign mails he got?

Because his interpreter was busy collecting garbage.
```


To fetch a joke from a specific category, use `-c` or `--category` flag with `--fetch`

For example,

```bash
./target/release/tamasha -c programming --fetch
Knock knock.
Who's there?
Recursion.
Recursion who?
Knock knock.
```

### Cli options

```bash
tamasha 0.1.0
Fetch me a joke!

USAGE:
    tamasha [FLAGS] [OPTIONS]

FLAGS:
        --fetch
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --category <category>    [default: any]
    -f, --format <format>        [default: txt]
