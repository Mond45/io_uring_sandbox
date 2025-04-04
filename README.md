My learning project for integrating `io_uring` async I/O API into `virtiofsd`.

To run test client (will create Unix domain socket `socket`, and `test.txt` in cwd):

```bash
cargo run --bin io_uring_sandbox
cargo run --bin client
```

Currently need to manually unlink `socket` after running.
