use winit::{
    application::ApplicationHandler, dpi::LogicalSize, event::WindowEvent, event_loop::EventLoop,
    window::Window,
};

struct App {
    window: Option<Window>,
    ctx: Option<pnte::Context<pnte::Direct2D>>,
    render_target: Option<pnte::d2d1::RenderTarget>,
}

impl App {
    fn new() -> anyhow::Result<Self> {
        Ok(Self {
            window: None,
            ctx: None,
            render_target: None,
        })
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            let attr = Window::default_attributes()
                .with_title("pnte winit")
                .with_inner_size(LogicalSize::new(1024, 768));
            self.window = Some(event_loop.create_window(attr).unwrap());
            self.ctx = Some(pnte::Context::new(pnte::Direct2D::new().unwrap()).unwrap());
            let window = self.window.as_ref().unwrap();
            let ctx = self.ctx.as_mut().unwrap();
            let size = window.inner_size();
            let dpi = window.scale_factor();
            ctx.set_scale_factor(dpi as f32);
            self.render_target = Some(
                ctx.create_render_target(&window, (size.width, size.height))
                    .unwrap(),
            );
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::RedrawRequested => {
                let ctx = self.ctx.as_ref().unwrap();
                let render_target = self.render_target.as_ref().unwrap();
                ctx.draw(render_target, |cmd| {
                    let white = pnte::SolidColorBrush::new(&ctx, (1.0, 1.0, 1.0, 1.0)).unwrap();
                    cmd.clear((0.0, 0.0, 0.3, 0.0));
                    cmd.draw_text("hello!!!", (10.0, 10.0), &white).unwrap();
                })
                .unwrap();
            }
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            _ => {}
        }
    }
}

fn main() -> anyhow::Result<()> {
    let event_loop = EventLoop::new()?;
    let mut app = App::new()?;
    event_loop.run_app(&mut app)?;
    Ok(())
}
