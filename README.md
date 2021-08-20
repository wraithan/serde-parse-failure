This repository shows the test `thing_numby` failing when `tracing-bunyan-formatter` is added to the dependencies

If you go into the `Cargo.toml` and remove `tracing-bunyan-formatter` from the deps then the tests pass.

Test output from `cargo test`

```
$ cargo test
    Finished test [unoptimized + debuginfo] target(s) in 0.02s
     Running unittests (target/debug/deps/serde_parse_failure-6e794418de97ed7d)

running 5 tests
test tests::struct_numby ... ok
test tests::thing_booly ... ok
test tests::thing_stringy ... ok
test tests::thing_enumy ... ok
test tests::thing_numby ... FAILED

failures:

---- tests::thing_numby stdout ----
thread 'tests::thing_numby' panicked at 'couldn't parse: Error("invalid type: map, expected u8", line: 0, column: 0)', src/lib.rs:25:73
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::thing_numby

test result: FAILED. 4 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'
```

It is only fields that are supposed to be numbers that fail to parse.