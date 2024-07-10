This package is a convenient starting point for building a rollup using the Sovereign SDK:

# The repo structure:

- `crates/stf`: The `STF` is derived from the `Runtime` and is used in the `rollup` and `provers` crates.
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

#### Optional: Setup the gas constants for transaction execution.

Sovereign's SDK transactions should specify gas parameters in a similar way to Ethereum. When submitting a transaction, you need to specify a handful of gas parameters (that are stored in a structure called `TxDetails`) that depend on the rollup settings but also on the type of call message to execute. We also have to make sure that the sender holds enough gas tokens in its bank balance to make sure that the transaction is not rejected due to insufficient funds. Finally, sequencers need to stake enough tokens to pay for the transaction pre-execution checks (like signature verification, deserialization, etc.).

This can be quite overwhelming at first glance, hence here is a quick summary of the gas parameters with their respective blessed values (this should be enough to execute most transactions that are not compute/storage intensive),

First, let's look at the gas parameters that are required to submit a transaction (in the `TxDetails` structure):

- The `max_fee` parameter is the maximum amount of gas expressed in gas tokens that can be charged for the transaction execution.
- The `max_priority_fee` parameter is the maximum percentage (expressed in basis points) of the total gas consumed by the transaction execution that should be paid to reward the sequencer. This parameter can have any value because there is a safety mechanism that prevents the user from paying more than the `max_fee` in total.
- The `gas_limit` parameter is the maximum amount of gas (expressed in multidimensional gas units) that can be consumed by the transaction execution. This parameter is optional and can be left unspecified. In the future, we will add support for automatically computing this parameter from transaction simulation.
- The `user_balance` parameter is the balance of the sender's account (for the gas token) in the rollup's bank.
- The `sequencer_balance` parameter is the balance of the sequencer's account (for the gas token) in the rollup's bank.
- The `sequencer_stake` parameter is the staked amount of the sequencer in the `sequencer_registry` module.

_Blessed values:_

| Parameter         | Value                               |
| ----------------- | ----------------------------------- |
| max_fee           | 100_000_000                         |
| max_priority_fee  | any (50_000 is a reasonable choice) |
| gas_limit         | None                                |
| user_balance      | 1_000_000_000                       |
| sequencer_balance | 1_000_000_000                       |
| sequencer_stake   | 100_000_000                         |

Note also that:

- The `base_fee_per_gas` parameter (whose initial value `INITIAL_GAS_LIMIT` is set by the rollup in the `constants.toml` - see [here](https://github.com/Sovereign-Labs/sovereign-sdk-wip/blob/nightly/constants.toml)) roughtly corresponds to the rollup's gas price and is an internal parameter of the rollup.
- A batch can consume up to `INITIAL_GAS_LIMIT` gas units of gas, and the gas target is `1/ELASTICITY_MULTIPLIER` times that value (for each dimension).
- The `base_fee_per_gas` is dynamically adjusted based on the gas consumption of the batch. The adjustment follows the EIP-1559 which makes it goes down if the batch consumes more gas than the target (and respectively up if the batch consumes less gas than the target).

More details can be found in the Sovereign book [available here](https://github.com/Sovereign-Labs/sovereign-book).
