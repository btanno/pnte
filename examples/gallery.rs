use wiard::ToLogical;
use windows::Win32::System::Com::*;

fn main() -> anyhow::Result<()> {
    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED | COINIT_DISABLE_OLE1DDE)?;
    }
    let mut event_rx = wiard::EventReceiver::new();
    let window = wiard::Window::builder(&event_rx)
        .title("pnte gallery")
        .build()?;
    let size = window.inner_size().unwrap();
    let dpi = window.dpi().unwrap();
    let mut ctx = pnte::Context::new(pnte::Direct2D::new()?)?;
    ctx.set_scale_factor(dpi as f32 / 96.0);
    let target = ctx.create_render_target(&window, (size.width, size.height))?;
    let image = pnte::Image::from_file(&ctx, "./assets/ferris.png")?;
    let image_size = image.size();
    let image_size = pnte::Size::new(image_size.width / 8.0, image_size.height / 8.0);
    let white = pnte::SolidColorBrush::new(&ctx, (1.0, 1.0, 1.0, 1.0))?;
    let text_format = pnte::TextFormat::new(
        &ctx,
        pnte::Font::File(
            &std::path::Path::new(
                "./assets/Inconsolata/static/Inconsolata/Inconsolata-Regular.ttf",
            ),
            "Inconsolata",
        ),
        pnte::FontPoint(25.0),
        None,
        None,
    )?;
    let text_layout = pnte::TextLayout::new(
        &ctx,
        "abcdefghijklmnopqrstuvwxyz",
        &text_format,
        pnte::TextAlignment::Center,
        None,
    )?;
    let pt_text = pnte::Point::new(10.0, 530.0);
    let layout_size = text_layout.size();
    let mut hit_test_display: Option<(char, bool)> = None;
    loop {
        let Some((event, _)) = event_rx.recv() else {
            break;
        };
        match event {
            wiard::Event::MouseInput(m) => {
                let left_button = m.button == wiard::MouseButton::Left
                    && m.button_state == wiard::ButtonState::Released;
                if left_button {
                    let dpi = window.dpi().unwrap();
                    let mouse_position = m.mouse_state.position.to_logical(dpi as i32);
                    let mouse_position =
                        pnte::Point::new(mouse_position.x as f32, mouse_position.y as f32);
                    let inside = pt_text.x <= mouse_position.x
                        && pt_text.y <= mouse_position.y
                        && pt_text.x + layout_size.width >= mouse_position.x
                        && pt_text.y + layout_size.height >= mouse_position.y;
                    if inside {
                        let result = text_layout.hit_test((
                            mouse_position.x - pt_text.x,
                            mouse_position.y - pt_text.y,
                        ))?;
                        if result.inside {
                            hit_test_display = Some((
                                text_layout.chars()[result.text_position],
                                result.trailing_hit,
                            ));
                        }
                    }
                }
                window.redraw(None);
            }
            wiard::Event::Draw(_) => {
                ctx.draw(&target, |cmd| -> anyhow::Result<()> {
                    cmd.clear((0.0, 0.0, 0.3, 0.0));
                    let pt = pnte::Point::new(10.0, 0.0);
                    cmd.draw_text("image", pt, &white)?;
                    let pt = pnte::Point::new(10.0, 20.0);
                    cmd.draw_image(
                        &image,
                        None,
                        (
                            pt.x + 10.0,
                            pt.y,
                            pt.x + 10.0 + image_size.width,
                            pt.y + 10.0 + image_size.height,
                        ),
                        None,
                        pnte::Interpolation::HighQualityCubic,
                    );

                    let pt = pnte::Point::new(10.0, 20.0 + image_size.height + 10.0);
                    cmd.draw_text("image (opacity = 0.5)", pt, &white)?;
                    let pt = pnte::Point::new(pt.x, pt.y + 20.0);
                    cmd.draw_image(
                        &image,
                        None,
                        (
                            pt.x + 10.0,
                            pt.y,
                            pt.x + 10.0 + image_size.width,
                            pt.y + 10.0 + image_size.height,
                        ),
                        Some(0.5),
                        pnte::Interpolation::HighQualityCubic,
                    );

                    let pt = pnte::Point::new(image_size.width + 100.0, 0.0);
                    cmd.draw_text("stroke line (width = 2.0)", pt, &white)?;
                    cmd.stroke(
                        &pnte::Line::new((pt.x, 30.0), (pt.x + 200.0, 90.0)),
                        &white,
                        2.0,
                        None,
                    );

                    let pt = pnte::Point::new(pt.x, 90.0 + 10.0);
                    let line_style = pnte::StrokeStyle::new(
                        &ctx,
                        &pnte::StrokeStyleProperties {
                            start_cap: pnte::CapStyle::Round,
                            end_cap: pnte::CapStyle::Triangle,
                            line_join: pnte::LineJoin::Miter,
                            dash: Some(pnte::Dash {
                                cap: pnte::CapStyle::Flat,
                                style: pnte::DashStyle::DashDot,
                                offset: 0.0,
                            }),
                        },
                    )?;
                    cmd.draw_text("stroke line (styled)", pt, &white)?;
                    cmd.stroke(
                        &pnte::Line::new((pt.x, pt.y + 30.0), (pt.x + 200.0, pt.y + 90.0)),
                        &white,
                        2.0,
                        Some(&line_style),
                    );
                    let pt = pnte::Point::new(pt.x, pt.y + 90.0 + 10.0);
                    cmd.draw_text("stroke quadratic bezier", pt, &white)?;
                    let pt = pnte::Point::new(pt.x, pt.y + 30.0);
                    let path = pnte::Path::builder(&ctx, pt)?
                        .quadratic_bezier_to(
                            (pt.x + 20.0, pt.y + 90.0),
                            (pt.x + 200.0, pt.y + 90.0),
                        )
                        .build(pnte::PathEnd::Open)?;
                    cmd.stroke(&path, &white, 2.0, None);

                    let pt = pnte::Point::new(pt.x, pt.y + 90.0 + 10.0);
                    cmd.draw_text("stroke cubic bezier", pt, &white).unwrap();
                    let pt = pnte::Point::new(pt.x, pt.y + 30.0);
                    let path = pnte::Path::builder(&ctx, pt)?
                        .cubic_bezier_to(
                            (pt.x + 100.0, pt.y),
                            (pt.x + 100.0, pt.y + 90.0),
                            (pt.x + 200.0, pt.y + 90.0),
                        )
                        .build(pnte::PathEnd::Open)?;
                    cmd.stroke(&path, &white, 2.0, None);

                    let pt_text_caption = pnte::Point::new(pt_text.x, pt_text.y - 30.0);
                    cmd.draw_text("text", pt_text_caption, &white)?;
                    cmd.draw_text(&text_layout, pt_text, &white)?;
                    if let Some((c, trailing_hit)) = hit_test_display.as_ref() {
                        let pt_text = pnte::Point::new(pt_text.x, pt_text.y + 35.0);
                        cmd.draw_text(
                            &format!("{c}, trailing_hit = {trailing_hit}"),
                            pt_text,
                            &white,
                        )?;
                    }

                    let pt = pnte::Point::new(pt.x + 230.0, 0.0);
                    cmd.draw_text("fill rectangle", pt, &white)?;
                    let pt = pnte::Point::new(pt.x, pt.y + 30.0);
                    cmd.fill(&pnte::Rect::from_point_size(pt, (60.0, 60.0)), &white);

                    let pt = pnte::Point::new(pt.x, pt.y + 60.0 + 30.0);
                    cmd.draw_text("fill circle", pt, &white)?;
                    let pt_circle = pnte::Point::new(pt.x + 30.0, pt.y + 30.0 + 30.0);
                    cmd.fill(&pnte::Circle::new(pt_circle, 30.0), &white);

                    let pt = pnte::Point::new(pt.x, pt_circle.y + 60.0);
                    cmd.draw_text("fill ellipse", pt, &white)?;
                    let pt_ellipse = pnte::Point::new(pt.x + 30.0, pt.y + 30.0 + 30.0);
                    cmd.fill(&pnte::Ellipse::new(pt_ellipse, 30.0, 15.0), &white);

                    let pt = pnte::Point::new(pt.x + 150.0, 0.0);
                    let grad = pnte::LinearGradientBrush::new(
                        &ctx,
                        pt,
                        (pt.x + 60.0, pt.y),
                        pnte::Gamma::G22,
                        pnte::GradientMode::Clamp,
                        &[
                            pnte::GradientStop::new(0.0, (1.0, 0.0, 0.0, 1.0)),
                            pnte::GradientStop::new(0.5, (0.0, 1.0, 0.0, 1.0)),
                            pnte::GradientStop::new(1.0, (0.0, 0.0, 1.0, 1.0)),
                        ],
                    )?;
                    cmd.draw_text("line gradient", pt, &white)?;
                    let pt = pnte::Point::new(pt.x, pt.y + 30.0);
                    cmd.fill(&pnte::Rect::from_point_size(pt, (60.0, 60.0)), &grad);

                    let pt = pnte::Point::new(pt.x, pt.y + 60.0 + 30.0);
                    cmd.draw_text("radial gradient", pt, &white)?;
                    let pt = pnte::Point::new(pt.x, pt.y + 30.0);
                    let grad = pnte::RadialGradientBrush::new(
                        &ctx,
                        pnte::Circle::new((pt.x + 30.0, pt.y + 30.0), 30.0).to_ellipse(),
                        (0.0, 0.0),
                        pnte::Gamma::G22,
                        pnte::GradientMode::Clamp,
                        &[
                            pnte::GradientStop::new(0.0, (1.0, 0.0, 0.0, 1.0)),
                            pnte::GradientStop::new(0.5, (0.0, 1.0, 0.0, 1.0)),
                            pnte::GradientStop::new(1.0, (0.0, 0.0, 1.0, 1.0)),
                        ],
                    )?;
                    cmd.fill(&pnte::Rect::from_point_size(pt, (60.0, 60.0)), &grad);
                    Ok(())
                })??;
            }
            _ => {}
        }
    }
    Ok(())
}
