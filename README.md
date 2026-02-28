# merkle_tree_rust

A minimal Rust implementation of a **Merkle Tree** — a cryptographic data structure used to efficiently and securely verify the contents of large data sets.  
This project builds a Merkle Tree from a list of data elements and computes root hashes that can be used to prove integrity.

---

## Features

- Construct a Merkle Tree from arbitrary input data
- Compute and retrieve the root hash

---

## What Is a Merkle Tree?

- In cryptography and computer science, a hash tree or Merkle tree is a tree in which every "leaf" node is labelled with the cryptographic hash of a data block, and every node that is not a leaf (called a branch, inner node, or inode) is labelled with the cryptographic hash of the labels of its child nodes. A hash tree allows efficient and secure verification of the contents of a large data structure. A hash tree is a generalization of a hash list and a hash chain. (Source: https://en.wikipedia.org/wiki/Merkle_tree)
---

## Installation

Build the project with Cargo:

```bash
cargo build --release 
```


## Example

```use merkle_tree_rust::MerkleTree;

let items = vec![1,2,3];

let tree = MerkleTree::new(items);
println!("Computed Merkle Root: {:?}", tree.root());```