# Chronicle

**Light weight, blazing fast blockchain indexer**

### This is originally built for Educational purposes

[Run](#running) | [Developer Docs](./docs)

🚧 *Under active development, see [status](#status) below.* 🚧

## Overview
**Chronicle** in simple terms is an indexing software, indexing transactions and modules (Smart contract, Parachain, and so on) Events. Chronicle stores these data in a Postgress DB and renders these data using an GraphQL interface. Chronicle is designed to achieve high-performance and high-reliability in cloud deployments via a modular architecture, Developers can hand-pick modules to build whatever, seeing the components of the Chronicle as pieces of **Legos**.

![Chronicle architecture](https://github.com/developeruche/chronicle/blob/main/docs/assets/chronicles.png)


### Motivation

The motivation behind Chronicle lies in its mission to provide developers with a comprehensive, yet highly customizable toolset for indexing transactions and events across various blockchain networks. Whether it's monitoring smart contract interactions, tracking parachain activities, or analyzing decentralized finance (DeFi) protocols, Chronicle offers a unified solution that simplifies the complexities of data indexing.

## Status

Chronicle is under active development.
**Use in production at your own risk.**


### Chain Support
Chronicle is built to be very modular, add new chains is very frictionaless. Here are the categories of chains Chronicle is currently implemented for.

* EVM compatible chains
* Para chains


## Developers

### Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

### Running

TODO? on Docker implementation.

### Developing

Clone the repository and checkout submodules:
```
git clone https://github.com/developeruche/chronicle
cd chronicle
```

Add config file to the root of the project:
```toml

name = "chronicle_name"

[[indexer]]
event_name = "event_name"
state_machine = "EVM"
rpc_url = "wss://eth-mainnet.g.alchemy.com/v2/ALCHEYM_API_KEY"
address = "0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984" # UNI token
event_signature = "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef" # Transfer(address,address,uint256)
block_number = 19711184
db_url = "host=localhost user=postgres"

[server]
db_url = "host=localhost user=postgres"
server_url = "127.0.0.1:8010"

```

```
cargo build --release
./target/debug/chronicle --config-path .config.toml
```

Run unit tests:
```
cargo test
```


## License

The Chronicle library (i.e. all code outside of the `bin` directory) is licensed under the GNU Lesser General Public License v3.0.

The Chronicle binaries (i.e. all code inside of the `bin` directory) is licensed under the GNU Lesser General Public License v3.0.
