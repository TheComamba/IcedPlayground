# iced-playground
Trying out several things in Rust's Iced GUI Framework.

## Canvas Coordinates
The bounds.contains check of the Canvas widget does not seem to work properly. If the canvas top-left does not coincide with the window top-left then the check interprets the point as if it lived in the window frame of reference, not the canvas frame of reference.

To test this, run:
```
git clone https://github.com/TheComamba/IcedPlayground.git;
cd IcedPlayground;
cargo run --bin canvas_coordinates;
```

## Rule on Scrollable
Placing a Rule inside a Scrollable makes the latter think that its width is extremely large. I suppose this is because Rule uses Length::Fill as its width. Is this by design, or is it a bug?

To test this, run:
```
git clone https://github.com/TheComamba/IcedPlayground.git;
cd IcedPlayground;
cargo run --bin rule_on_scrollable;
```

## Toggler with fixed width
Setting a fixed width for a Toggler with text breaks the apperance.

To test this, run:
```
git clone https://github.com/TheComamba/IcedPlayground.git;
cd IcedPlayground;
cargo run --bin toggler_with_fixed_width;
```