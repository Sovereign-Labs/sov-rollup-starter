
# This is still work in progress. 

This package is a convenient starting point for building a rollup using the Sovereign SDK:


# How to run the sov-rollup-starter:
1. `cd crates/rollup/`

2. Starting the node:
If you want to run a fresh rollup remove the `rollup-starter-data` folder.
This will compile and start the rollup node:

```shell
cargo run --bin node
```


3. In another shell run:

```shell
make test-create-token
```

4. Test if token creation succeeded

```shell
make test-bank-supply-of
```