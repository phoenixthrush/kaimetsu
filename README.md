# kaimetsu

## Building

```bash
RUSTFLAGS="-Zunstable-options -Cpanic=immediate-abort" cargo +nightly build \
    -Z build-std=std,panic_abort \
    -Z build-std-features= \
    --target x86_64-apple-darwin --release

du -sh target/x86_64-apple-darwin/release/kaimetsu
```
