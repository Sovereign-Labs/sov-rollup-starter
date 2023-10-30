This package is a convenient starting point for building a rollup using the Sovereign SDK:

# How to run the sov-rollup-starter:
#### 1. Change the working directory:

```shell,test-ci
$ cd crates/rollup/
```

#### 2. Cleanup database:
```sh,test-ci
$ make clean-db
```

#### 3. Starting the node:
If you want to run a fresh rollup remove the `rollup-starter-data` folder.
This will compile and start the rollup node:

```shell,test-ci,bashtestmd:long-running,bashtestmd:wait-until=RPC
$ cargo run --bin node
```

#### 4. In another shell run:

```sh,test-ci
$ make test-create-token
```

#### 5. Test if token creation succeeded

```sh,test-ci
$ make test-bank-supply-of
```

#### 6. The output of the above script:

```bash,test-ci,bashtestmd:compare-output
$ curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","method":"bank_supplyOf","params":["sov1zdwj8thgev2u3yyrrlekmvtsz4av4tp3m7dm5mx5peejnesga27svq9m72"],"id":1}' http://127.0.0.1:12345
{"jsonrpc":"2.0","result":{"amount":1000},"id":1}
```