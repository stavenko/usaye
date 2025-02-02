This is USeless AsYnc sErver (USAYE).


To test this server you may:

```
cargo test
```

You also may run it (no dependencies required at the moment).

```
cargo run -- run --config ./configs/test-config.toml
```

Then do requests with curl:
```
curl -d "{\"task_url\":\"https://docs.rs/env_logger/latest/env_logger/\", \"delay\": \"10s\"}" -H"content-type:application/json" http://localhost:3001/public/add-task
```

To get result: 
```
curl -d "{\"id\": \"<uuid>\"}" -H"content-type:application/json" http://localhost:3001/public/get-task-result
```

To list running tasks:
```
curl -X POST  http://localhost:3001/public/list-tasks
```

to drop running or pending task:

```
curl -d "{\"id\": \"<uuid>\"}" -H"content-type:application/json" http://localhost:3001/public/drop-task
```


