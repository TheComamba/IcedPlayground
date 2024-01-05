# iced-playground
Trying out several things in Rust's Iced GUI Framework.

## Rule on Scrollable
Placing a Rule inside a Scrollable makes the latter think that its width is extremely large. I suppose this is because Rule uses Length::Fill as its width. Is this by design, or is it a bug?

To test this, run:
```bash
git clone https://github.com/TheComamba/IcedPlayground.git;
cd IcedPlayground;
cargo run --bin rule_on_scrollable;
```