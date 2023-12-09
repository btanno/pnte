use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn main() -> anyhow::Result<()> {
    let event_loop = EventLoop::new()?;
    // When a window is built, winit call `CoInitializeEx`.
    let window = WindowBuilder::new()
        .with_title("pnte winit")
        .with_inner_size(LogicalSize::new(1024, 768))
        .build(&event_loop)?;
    let size = window.inner_size();
    let mut ctx = pnte::Context::new(pnte::Direct2D::new()?)?;
    ctx.set_scale_factor(window.scale_factor() as f32);
    let render_target = ctx.create_render_target(&window, (size.width, size.height))?;
    event_loop.run(move |event, elwt| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::RedrawRequested => {
                ctx.draw(&render_target, |cmd| {
                    let white = pnte::SolidColorBrush::new(&ctx, (1.0, 1.0, 1.0, 1.0)).unwrap();
                    cmd.clear((0.0, 0.0, 0.3, 0.0));
                    cmd.draw_text("hello!!!", (10.0, 10.0), &white).unwrap();
                })
                .unwrap();
            }
            WindowEvent::CloseRequested => elwt.exit(),
            _ => {}
        },
        _ => {}
    })?;
    Ok(())
}
