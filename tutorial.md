# Tutorial
There are below four important components you need to concern about. The tutorial only provides an overview. A precise usage is in [example/kv](https://github.com/blackredscarf/raftfwk/tree/master/examples/kv) which builds a distributed key-value storage system.

### Logger
Logging something in runtime of program.
```rust
use raftfwk::logger::create_logger;

let id = 1;
let logger = create_logger(id, String::from("info"));
```
That will create a [slog::Logger](https://docs.rs/slog).

### RaftService: 
Implementing your business in your struct which implements the trait RaftService.
```rust
use raftfwk::service::RaftService;

// Create your own service
pub struct KvService {
    // ...
}

// Impl RaftService
impl RaftService for KvService {
    // ...
}
```

### RaftStorage
It is a component to store raft logs. There are built-in MemoryStorage and LevelStorage. You can write your own Storage by impl the trait RaftStorage.
```rust
use raftfwk::level_storage::create_leveldb_storage;

let storage = create_leveldb_storage(format!("db_name"), logger.clone());
```

### RaftServer
The server of a raft node.
```rust
// The id of this node.
let id = 1;
// The RPC port
let port = 8010;
// Whether to join in another cluster
let cluster = None;

// Create logger
let logger = create_logger(id, String::from("info"));

// Create your service
let service = Arc::new(Mutex::new(KvService::new(logger.clone())));

// Create storage
let storage = create_leveldb_storage(format!("db{}", id), logger.clone());

// Make config
let config = RaftConfig::join(id, port, cluster);

// Create a server
let mut r = RaftServer::new(logger.clone(), config, storage, service.clone());

// Run server
r.run();
```