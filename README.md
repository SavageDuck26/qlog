# qlog (Quick Log)

Macro to append a line for logging.
``` toml
[dependencies]
qlog = { git = "https://github.com/SavageDuck26/qlog.git" }
```

```rust
use qlog::qlog;

let _ = qlog!("logs/app.log", "One does not simply write documentation.");
```
