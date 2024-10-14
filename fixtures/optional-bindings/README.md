# A test for uniffi feature enabled functions

This tests that it is impossible to test the uniffi feature enabled functions.

To test, please run the following test will fail:

```sh
cargo test
```

However, if you uncomment the `default = ["optional"]` line in the `Cargo.toml` file, the test will pass.

This is because it is the only way to test the uniffi feature enabled functions.