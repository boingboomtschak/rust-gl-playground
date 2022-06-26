use glium::{
    Surface, Display, GlObject,
    texture::{RawImage2d, Texture2d},
    glutin::window::WindowBuilder,
    glutin::ContextBuilder,
    glutin::event_loop::{EventLoop, ControlFlow},
    glutin::event::{Event, WindowEvent},
    glutin::dpi::LogicalSize,
};
use imgui::{Context, Window, Ui, ListBox, Selectable, Image, TextureId, MenuItem};
use imgui_glium_renderer::{Renderer, Texture};
use imgui_winit_support::{WinitPlatform, HiDpiMode};
use std::time::Instant;
use std::rc::Rc;

fn show_main_menu_bar(ui : &Ui) -> bool {
    let mut close = false;
    ui.main_menu_bar(|| {
        ui.menu("vsynth", || {
            ui.text(format!("v{}", env!("CARGO_PKG_VERSION")));
            if MenuItem::new("Quit").shortcut("CTRL + Q").build(ui) { close = true; }
        });
        ui.menu("File", || {

        });
        ui.menu("Layers", || {

        });
        ui.menu("Compositor", || {

        });
    });
    return close;
}

fn show_layers(opened : &mut bool, ui : &Ui) {
    if *opened {
        Window::new("Layers")
            .opened(opened)
            .scroll_bar(false)
            .always_auto_resize(true)
            .build(ui, || {
                ListBox::new("").build(ui, || {
                    for i in 0..64 {
                        Selectable::new(format!("Synthesis - ADD - {}", i)).build(ui);
                    }
                });
            });
    }
}

fn show_render(opened : &mut bool, ui : &Ui, tex : TextureId) {
    if *opened {
        Window::new("Render").opened(opened).build(ui, || {
            Image::new(tex, [512.0, 512.0]).build(ui);
        });
    }
}

fn show_layer_inspector(opened : &mut bool, ui : &Ui) {
    if *opened {
        Window::new("Layer Inspector").opened(opened).build(ui, || {

        });
    }
}

fn main() {
    // Create event loop, window, context, and display for glium
    let event_loop = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_title("vsynth")
        .with_inner_size(LogicalSize::new(1000f32, 800f32));
    let cb = ContextBuilder::new().with_vsync(true);
    let display = Display::new(wb, cb, &event_loop).expect("Failed to initialize display");

    // Create imgui context, platform renderer, and attach platform to window
    let mut imgui = Context::create();
    imgui.set_ini_filename(None);
    let mut platform = WinitPlatform::init(&mut imgui);
    platform.attach_window(imgui.io_mut(), display.gl_window().window(), HiDpiMode::Default);
    let mut renderer = Renderer::init(&mut imgui, &display).expect("Failed to initialize renderer");

    let mut render_data = Vec::with_capacity(64 * 4);
    for i in 0..8 {
        for j in 0..8 {
            render_data.push(i * 32 as u8);
            render_data.push(j * 32 as u8);
            render_data.push((i + j) * 16 as u8);
            render_data.push(255 as u8);
        }
    }
    let render_tex = RawImage2d::from_raw_rgba(render_data, (8, 8));
    let render_tex = Texture2d::new(&display, render_tex).expect("Failed to create render texture");
    let render_tex = renderer.textures().insert(Texture {
        texture : Rc::new(render_tex),
        sampler : Default::default()
    });

    let mut frame_timer = Instant::now();

    let mut layers_open = true;
    let mut render_open = true;
    let mut layer_inspector_open = true;    

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { event : WindowEvent::CloseRequested, .. } => { *control_flow = ControlFlow::Exit; }
            Event::NewEvents(_) => {
                // Update imgui with elapsed time
                imgui.io_mut().update_delta_time(frame_timer.elapsed());
                frame_timer = Instant::now();
            }
            Event::MainEventsCleared => {
                // When input processing over, prepare imgui frame and request redraw of application
                let gl_window = display.gl_window();
                platform.prepare_frame(imgui.io_mut(), gl_window.window()).expect("Failed to prepare frame");
                gl_window.window().request_redraw();
            }
            Event::RedrawRequested(_) => {
                // When redraw requested, draw imgui and any glium rendering needed
                let ui = imgui.frame();
                let gl_window = display.gl_window();

                //ui.show_demo_window(&mut true);

                if show_main_menu_bar(&ui) { *control_flow = ControlFlow::Exit; }
                show_layers(&mut layers_open, &ui);
                show_render(&mut render_open, &ui, render_tex);
                show_layer_inspector(&mut layer_inspector_open, &ui);

                // Drawing
                let mut target = display.draw();
                target.clear_color(0.2, 0.2, 0.2, 1.0);
                
                // ImGui preparation
                platform.prepare_render(&ui, gl_window.window());
                let draw_data = ui.render();
                renderer.render(&mut target, draw_data).expect("Failed to render UI");

                target.finish().expect("Failed to finish render");
            }
            event => {
                // Pass all other events to imgui
                let gl_window = display.gl_window();
                platform.handle_event(imgui.io_mut(), gl_window.window(), &event);
            }
        }
    });
}