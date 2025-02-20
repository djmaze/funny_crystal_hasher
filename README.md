# Funny crystal hasher

Reimplementation of the [funny_hash algorithm](https://github.com/funny-falcon/funny_hash), output-compatible with [the version used in the Crystal language](https://github.com/crystal-lang/crystal/blob/master/src/crystal/hasher.cr).

## Command line testing

Run the following to calculate a hash at the commandline:

```bash
cargo run --release <SEED 1> <SEED 2> <STRING TO HASH>
```

E.g.

```bash
cargo run --release 11111 22222 "foobar"
```
