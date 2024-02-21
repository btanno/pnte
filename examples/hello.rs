use windows::Win32::System::Com::*;

fn main() -> anyhow::Result<()> {
    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED | COINIT_DISABLE_OLE1DDE).unwrap();
    }
    let mut event_rx = wiard::EventReceiver::new();
    let window = wiard::Window::builder(&event_rx)
        .title("pnte hello")
        .build()?;
    let size = window.inner_size().unwrap();
    let dpi = window.dpi().unwrap() as f32;
    let mut ctx = pnte::Context::new(pnte::Direct2D::new()?)?;
    ctx.set_dpi(dpi, dpi);
    let render_target = ctx.create_render_target(&window, (size.width, size.height))?;
    let text_format = pnte::TextFormat::new(&ctx)
        .font(pnte::Font::System("Yu Gothic UI"))
        .size(pnte::FontPoint(32.0))
        .build()?;
    let text_layout = pnte::TextLayout::new(&ctx)
        .text("hello! 🚀")
        .format(&text_format)
        .build()?;
    let white = pnte::SolidColorBrush::new(&ctx, (1.0, 1.0, 1.0, 1.0))?;
    loop {
        let Some((event, _)) = event_rx.recv() else {
            break;
        };
        match event {
            wiard::Event::Draw(_) => {
                ctx.draw(&render_target, |cmd| {
                    cmd.clear((0.0, 0.0, 0.3, 0.0));
                    cmd.draw_text(&text_layout, (10.0, 10.0), &white).ok();
                })
                .ok();
            }
            _ => {}
        }
    }
    Ok(())
}
