This package is a convenient starting point for building a rollup using the Sovereign SDK:

# The repo structure:
- `crates/stf`:  The `STF` is derived from the `Runtime` and is used in the `rollup` and `provers` crates.
- `crates/provers`: This crate is responsible for creating proofs for the `STF`.
- `crates/rollup`: This crate runs the `STF` and offers additional full-node functionalities.

# How to run the sov-rollup-starter:
#### 1. Change the working directory:

```shell,test-ci
$ cd crates/rollup/
```

#### 2. If you want to run a fresh rollup, clean the database:

```sh,test-ci
$ make clean-db
```

#### 3. Start the rollup node:

This will compile and start the rollup node:

```shell,test-ci,bashtestmd:long-running,bashtestmd:wait-until=RPC
$ cargo run --bin node
```

#### 4. Submit a token creation transaction to the `bank` module:

```sh,test-ci
$ make test-create-token
```

#### 5. Wait for the transaction to be submitted.
```sh,test-ci
$ make wait-ten-seconds
```


#### 6. Test if token creation succeeded:

```sh,test-ci
$ make test-bank-supply-of
```

#### 7. The output of the above script:

```bash,test-ci,bashtestmd:compare-output
$ curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","method":"bank_supplyOf","params":{"token_address":"sov1zdwj8thgev2u3yyrrlekmvtsz4av4tp3m7dm5mx5peejnesga27svq9m72"},"id":1}' http://127.0.0.1:12345
{"jsonrpc":"2.0","result":{"amount":1000},"id":1}
```

# How to run the sov-rollup-starter using celestia-da:
#### 1. Change the working directory:

```
$ cd crates/rollup/
```

#### 2. If you want to run a fresh rollup, clean the database:

```
$ make clean
```

#### 3. Start the Celestia local docker service:

```
$ make start
```

#### 4. Start the rollup node with the feature flag building with the celestia adapter:

This will compile and start the rollup node:

```
$ cargo run --bin node --no-default-features --features celestia_da
```

#### 5. Submit a token creation transaction to the `bank` module:

Using `CELESTIA=1` will enable the client to be built with Celestia support and submit the test token

```
$ CELESTIA=1 make test-create-token
```

#### 6. Test if token creation succeeded:


```
$ make test-bank-supply-of
```

#### 7. The output of the above script:

```
$ curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","method":"bank_supplyOf","params":{"token_address":"sov1zdwj8thgev2u3yyrrlekmvtsz4av4tp3m7dm5mx5peejnesga27svq9m72"},"id":1}' http://127.0.0.1:12345
{"jsonrpc":"2.0","result":{"amount":1000},"id":1}
```