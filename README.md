# pnte

2D Graphics library for Windows in Rust

pnte is a Direct2D and DreictWrite wrapper.

## Requirement

To use this library, `CoInitializeEx` must be called for each thread.

## How to use simply

1. Create a `pnte::Context`.

```rust
let ctx = pnte::Context::new(pnte::Direct2D::new()?)?;
```

2. Create a render target.

```rust
let render_target = ctx.create_render_target(&window, (size.width, size.height))?;
```

3. Draw.

```rust
ctx.draw(&render_target, |cmd| {
    let white = pnte::SolidColorBrush::new(&ctx, (1.0, 1.0, 1.0, 1.0))?;
    cmd.clear((0.0, 0.0, 0.0, 0.0));
    cmd.draw_text("pnte", (10.0, 10.0), &white)?;
})?;
```

## License

This library is licensed under the [MIT license](LICENSE).
