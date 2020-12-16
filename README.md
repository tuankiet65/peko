# peko

An [Ethereum](https://ethereum.org/) client written in
[Rust](https://www.rust-lang.org/).

The eventual goal is fully fledged client which supports sending and
receiving transactions, but for now the current goal is to connect and
synchronize the blockchain from other nodes in the network.

## Warning
This is strictly a learning project, where the goal is to learn more
about Rust and functionalities required to implement a Ethereum client.
As such, reimplementing existing cargos is encouraged when possible.
Because of this, this code should not be used in production.

## Naming
This project is not totally named after
[Usada Pekora](https://en.hololive.tv/portfolio/items/433585).
Subscribe to
[her channel](https://www.youtube.com/channel/UC1DCedRgGHBdm81E1llLhOQ)
anyway though.

## Components
* `peko-crypto`: collection of various cryptographic routines (Keccak,
  SHA-256, AES, ...) used by other components.
* `peko-evm`: library implementing the Ethereum Virtual Machine.
* `peko-blockchain`: library implementing the Ethereum blockchain.
* `peko-rlp`: library implementing the Recursive Length Prefix serialization
  method.
* `peko-p2p`: library implementing the P2P portion of Ethereum, which
  connects and synchronizes the blockchain from other nodes.
* `peko-rpc`: library implementing the Web3 portion of Ethereum,
  which allows applications to interact with the client via JSON-RPC.
* `peko`: the client binary.

## Resources

### General
* [Ethereum Yellow Paper](https://ethereum.github.io/yellowpaper/paper.pdf):
  formal specification of the Ethereum blockchain.
* [Ethereum Wiki](https://eth.wiki/)
* [Illustration of the Ethereum blockchain](https://i.stack.imgur.com/afWDt.jpg)

### Merkle-Patricia tree
* [Illustration of a Merkle-Patricia tree](https://i.stack.imgur.com/YZGxe.png)
* [Patricia Tree](https://eth.wiki/fundamentals/patricia-tree)
* [Merkling in Ethereum](https://blog.ethereum.org/2015/11/15/merkling-in-ethereum/)
* [Understanding the Ethereum trie](https://easythereentropy.wordpress.com/2014/06/04/understanding-the-ethereum-trie/)
* [Ethereum block architecture](https://ethereum.stackexchange.com/questions/268/ethereum-block-architecture)
* [State Tree Purning](https://blog.ethereum.org/2015/06/26/state-tree-pruning/)
* [Why Merkle-Patricia Trie has three types of nodes?](https://ethereum.stackexchange.com/questions/57486/why-merkle-patricia-trie-has-three-types-of-nodes)
* [Ethereum Merkle Tree Explanation](https://ethereum.stackexchange.com/questions/15288/ethereum-merkle-tree-explanation/15294)

### P2P protocol
* [ethereum/devp2p](https://github.com/ethereum/devp2p/) contains descriptions of
  P2P protocols used.
* [EIP 8](https://eips.ethereum.org/EIPS/eip-8)
* [geth's P2P implementation](https://github.com/ethereum/go-ethereum/tree/master/p2p)
* [ECIES encryption scheme](https://en.wikipedia.org/wiki/Integrated_Encryption_Scheme)

### Web3
* List of JSON-RPC endpoints: https://eth.wiki/json-rpc/api
* geth: [JSON-RPC server](https://geth.ethereum.org/docs/rpc/server)

### Ethash
* [A Prehistory of the Ethereum Protocol](https://vitalik.ca/general/2017/09/14/prehistory.html)
* [ethereum/ethash](https://github.com/ethereum/ethash)
* [ethash](https://eth.wiki/en/concepts/ethash/ethash)
* [Ethash DAG Disk Storage Format](https://eth.wiki/concepts/ethash/dag-disk-storage-format)
