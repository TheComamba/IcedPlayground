# iced-playground
Trying out several things in Rust's Iced GUI Framework.

## Rule on Scrollable
Placing a Rule inside a Scrollable makes the latter think that its width is extremely large. I suppose this is because Rule uses Length::Fill as its width. Is this by design, or is it a bug?

To test this, run:
```
git clone https://github.com/TheComamba/IcedPlayground.git;
cd IcedPlayground;
cargo run --bin rule_on_scrollable;
```

## Toggle with fixed width
Setting a fixed width for a Toggler with text breaks the apperance.

To test this, run:
```
git clone https://github.com/TheComamba/IcedPlayground.git;
cd IcedPlayground;
cargo run --bin toggler_with_fixed_width;
```