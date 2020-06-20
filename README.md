# RaftFwk
A framework of distribution consensus algorithm [Raft](https://raft.github.io/raft.pdf) written in Rust. Implementing a sequence of tools based on library [tikv/raft-rs](https://github.com/tikv/raft-rs), including necessary user interfaces of a raft server, the network library of [grpc](https://github.com/tikv/grpc-rs) and the log storage of [leveldb](https://docs.rs/rusty-leveldb).


## Installation
You need to pay attention on the requirements of [tikv/grpc-rs](https://github.com/tikv/grpc-rs) which is a wrapper of [gRPC Core](https://github.com/grpc/grpc) written in C. You can see the installation guide in its [README](https://github.com/tikv/grpc-rs#prerequisites). It seen that it on Windows only works for msvc compile version of Rust.

Adding the dependencies in Cargo.toml.
```
[dependencies]
raftfwk = { git = "github.com/blackredscarf/raftfwk" }
```

## Usage
See [tutorial](https://github.com/blackredscarf/raftfwk/tree/master/tutorial.md) and [examples/kv](https://github.com/blackredscarf/raftfwk/tree/master/examples/kv).


## References
- [Raft paper](https://raft.github.io/raft.pdf)
- [tikv/raft-rs](https://github.com/tikv/raft-rs)
- [tikv/grpc-rs](https://github.com/tikv/grpc-rs)
- [gRPC Core](https://github.com/grpc/grpc)
- [rusty-leveldb](https://docs.rs/rusty-leveldb)


