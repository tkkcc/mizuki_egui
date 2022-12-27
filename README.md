# UI for Mizuki

final demo: https://tkkcc.github.io/mizuki_egui

## conclusion:
1. lag on text editing
1. font blur
1. hard text editing, selection/copy/paste, native build has no keyboard support
1. some bugs, table bottom empty
1. large size, 7M

## development

```sh
# web
trunk serve
trunk build --release
trunk build --release -d docs --public-url /mizuki_egui

# native
cargo run --release --target x86_64-unknown-linux-gnu

# test api
cargo test --lib --target x86_64-unknown-linux-gnu -- --nocaptcha
```

based on https://github.com/emilk/eframe_template/
