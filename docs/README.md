# Chronicle
-------------------------

**Chronicle** in simple terms is an indexing software, indexing transactions and modules (Smart contract, Parachain, and so on) Events. Chronicle stores these data in a Postgress DB and renders these data using an GraphQL interface. Chronicle is designed to achieve high-performance and high-reliability in cloud deployments via a modular architecture, Developers can hand-pick modules to build whatever, seeing the components of the Chronicle as pieces of **Legos**.


The core reasons for building **Chronicle** are:
1. Exploring the possiblity for building a high-performance and high-reliability blockchain indexer.
2. Building a modular architecture that allows developers to hand-pick modules to build whatever they want.
3. Building an advanced software that intermidate rust engineers and learn from and contribute to.

This is nice to note that a good chuck of the development pratices and rust centered design patterns are inspired by the [Substrate](https://github.com/paritytech/polkadot-sdk), [Hyperbridge](https://github.com/polytope-labs/hyperbridge), and [Rundler](https://github.com/alchemyplatform/rundler).


Chronicle is Divided into two main components:
1. **Chronicle-Indexer**: This is the core of the Chronicle, it is responsible for indexing transactions and modules (Smart contract, Parachain, and so on) Events.
2. **Chronicle-GraphQL**: This is the interface of the Chronicle, it is responsible for rendering the indexed data in a GraphQL interface. This is refered to as `chronicle-server`.


The code base is classified into:
1. Binary
    - Chronicle: this is the only binary crate in the code base, it is responsible for running the Chronicle. it is the entry point of the Chronicle, the `main.rs` file is located in the `bin` directory. It runs the `chronicle-indexer` and `chronicle-server` concurrently, Empowered with a CLI interference to control the Chronicle.

2. Crates
  - Chronicle-Indexer: This is the core of the Chronicle, it is responsible for indexing transactions and modules (Smart contract, Parachain, and so on) Events.
  - Chronicle-GraphQL: This is the interface of the Chronicle, it is responsible for rendering the indexed data in a GraphQL interface. This is refered to as `chronicle-server`.
  - Chronicle-Primitives: This is a shared library that contains common utilities and data structures used across the Chronicle.
  - Chronicle-Task: This implements a rules on how concurrent tasks should be executed in the Chronicle.
