# Devtools test crate
Used as an MRE to show the off-by-one error for logging with
[`devtools`](https://github.com/crabnebula-dev/devtools), as described in
[issue 315](https://github.com/crabnebula-dev/devtools/issues/315).

## Off by one error
When logging using `devtools` log outputs are delayed by one event.
e.g.
```rust
tracing::debug!("1") // No log output.
...
tracing::debug!("2") // Logs "1".
...
tracing::debug!("3") // Logs "2". All subsequent logs are delayed by one logging event.
```

## Use
Switch between `devtools` and `tracing_subscriber` loggers in `src/main`.
Log events are triggered when a `greet` message is sent.
