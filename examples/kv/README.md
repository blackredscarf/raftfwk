# A key-value storage example

Building.
```
cargo build
```

Running node 1:
```
kv -i 1 -p 8010 -l info -s leveldb
```
Waiting for node 1 becoming a leader, then running node 2:
```
kv -i 2 -p 8020 -c 127.0.0.1:8010 -l info -s leveldb
```
After node 2 becoming a follower, adding node 3:
```
kv -i 3 -p 8030 -c 127.0.0.1:8010 -l info -s leveldb
```

The program includes a user client in terminal. You can try to execute commands like
```
> set name John
Set: Success

> get name
Value: John
```
You are allowed to "set" in the leader and "get" in all peers.