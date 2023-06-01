# xkcd-cli

A simple CLI to download xkcd comics.

## Note

-   I'm exploring the Rust language + libraries, so wanted to build something while learning.
-   So you might -

    -   see - `unwrap`'s, `expect`'s and `panic`'s instead of proper error handling in some places.
    -   not see - clean code or design patterns or full test coverage.

-   If you fancy one-liners, try this instead -

    ```sh
    curl -sSL https://xkcd.com/info.0.json | jq '.img' | xargs curl -sSL --output "xkcd_$(date +%F).png"
    ```

## Dependencies

-   clap
-   colored
-   rand
-   rand_chacha
-   reqwest
-   serde

### Testing Dependencies

-   assert_cmd
-   insta
-   predicates

## Installation Instructions

```sh
  # Clone the repository
  git clone https://github.com/krish-r/xkcd-cli.git

  # Switch to the cloned directory
  cd xkcd-cli

  # Try it without installing
  cargo run -- [OPTIONS]
  # For Ex.
  #   cargo run -- --latest
  #   cargo run -- --random
  #   cargo run -- --output="/tmp/xkcd.png"

  # or

  # To install (the cargo bin directory `~/.cargo/bin` should be in your `$PATH`)
  cargo install --path .

```

## Uninstall instructions

```sh
rm -i $(which xkcd-cli)
```

## Usage

```sh
xkcd-cli --help
```

```sh
Usage: xkcd-cli [OPTIONS]

Options:
-l, --latest           latest
-r, --random           random
-o, --output <OUTPUT>  output
-h, --help             Print help
```

## Credits

-   [xkcd][xkcd] - for the api.

[xkcd]: https://xkcd.com/
