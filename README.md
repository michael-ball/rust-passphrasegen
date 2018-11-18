# rust-passphrasegen
A passphrase generator written in Rust.

## Usage
Build by running `cargo build --release`.

Generate a passphrase by running `passphrasegen`.

Required arguments:
* Path to words file (eg. /usr/share/dict/words)

### Example

Generate a passphrase of 5 words using the american english wordlist:

    $ passphrasegen -w 5 /usr/share/dict/american-english
