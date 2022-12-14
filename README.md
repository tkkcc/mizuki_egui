UI for Mizuki

based on https://github.com/emilk/eframe_template/ 

```sh
# web
trunk serve
trunk build --release

# native
cargo run --release --target x86_64-unknown-linux-gnu

# test
cargo test --lib --target x86_64-unknown-linux-gnu -- --nocaptcha
```
