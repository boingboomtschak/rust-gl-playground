use glium::{
    Surface, Display,
    texture::{RawImage2d, Texture2d},
    glutin::window::WindowBuilder,
    glutin::ContextBuilder,
    glutin::event_loop::{EventLoop, ControlFlow},
    glutin::event::{Event, WindowEvent},
    glutin::dpi::LogicalSize,
};
use imgui::{Context, Window, Ui, ListBox, Selectable, Image, TextureId, MenuItem, ChildWindow};
use imgui_glium_renderer::{Renderer, Texture};
use imgui_winit_support::{WinitPlatform, HiDpiMode};
use std::time::Instant;
use std::rc::Rc;
use std::cell::RefCell;

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
            .build(ui, || {
                ChildWindow::new("Active Layers").build(ui, || {
                    ListBox::new("").build(ui, || {
                        for i in 0..64 {
                            Selectable::new(format!("Synthesis - ADD - {}", i)).build(ui);
                        }
                    });
                });
                ChildWindow::new("Layer Inspector").build(ui, || {
                    ui.text("Layer inspector text, lorem ipsum");
                })
            });
    }
}

fn show_render(opened: &mut bool, ui: &Ui, tex: TextureId) {
    if *opened {
        Window::new("Render").opened(opened).build(ui, || {
            Image::new(tex, [512.0, 512.0]).build(ui);
        });
    }
}

fn show_layer_inspector(opened: &mut bool, ui: &Ui) {
    if *opened {
        Window::new("Layers").opened(opened).build(ui, || {
            ChildWindow::new("test1").build(ui, || {
                
            });
            ChildWindow::new("test2").build(ui, || {
                
            });
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
    let display = Rc::new(Display::new(wb, cb, &event_loop).expect("Failed to initialize display"));

    // Create imgui context, platform renderer, and attach platform to window
    let mut imgui = Context::create();
    imgui.set_ini_filename(None);
    let mut platform = WinitPlatform::init(&mut imgui);
    platform.attach_window(imgui.io_mut(), display.gl_window().window(), HiDpiMode::Default);
    let mut renderer = Rc::new(RefCell::new(Renderer::init(&mut imgui, display.as_ref()).expect("Failed to initialize renderer")));

    let mut render_data = vec![0u8; 256];
    for i in (3..256).step_by(4) { render_data[i] = 255u8; }
    let render_tex = RawImage2d::from_raw_rgba(render_data, (8, 8));
    let render_tex = Texture2d::new(display.as_ref(), render_tex).expect("Failed to create render texture");
    let render_tex = renderer.borrow_mut().textures().insert(Texture {
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

                ui.show_demo_window(&mut true);

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
                renderer.borrow_mut().render(&mut target, draw_data).expect("Failed to render UI");

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

fn create_texture(renderer: Rc<RefCell<Renderer>>, display: Rc<Display>, size: (u32, u32)) -> TextureId {
    let tex_size = size.0 * size.1 * 4;
    let mut tex_data = vec![0u8; tex_size as usize];
    for i in (3..tex_size).step_by(4) { tex_data[i as usize] = 255u8 };
    let tex = RawImage2d::from_raw_rgba(tex_data, size);
    let tex = Texture2d::new(display.as_ref(), tex).expect("Failed to create layer texture");
    renderer.borrow_mut().textures().insert(Texture {
        texture : Rc::new(tex),
        sampler : Default::default()
    })
}

struct SynthesisLayer {
    texture_id: TextureId
}
struct ControlLayer {
    texture_id: TextureId
}
struct SourceLayer {
    texture_id: TextureId
}

trait Layer {
    //fn new(renderer: Rc<RefCell<Renderer>>, display: Rc<Display>, size: (u32, u32)) -> Box<Self>;
    //fn draw(&self, ui: &Ui);
}
impl SynthesisLayer {
    fn new(renderer: Rc<RefCell<Renderer>>, display: Rc<Display>, size: (u32, u32)) -> SynthesisLayer {
        let tex_id = create_texture(renderer, display, size);
        SynthesisLayer { texture_id : tex_id }
    }
}
impl ControlLayer {
    fn new(renderer: Rc<RefCell<Renderer>>, display: Rc<Display>, size: (u32, u32)) -> ControlLayer {
        let texture_id = create_texture(renderer, display, size);
        ControlLayer { texture_id : texture_id }
    }
}
impl SourceLayer {
    fn new(renderer: Rc<RefCell<Renderer>>, display: Rc<Display>, size: (u32, u32)) -> SourceLayer {
        let texture_id = create_texture(renderer, display, size);
        SourceLayer { texture_id : texture_id }
    }
}

struct Compositor {
    texture_id: TextureId,
    texture_size: [f32; 2],
    renderer: Rc<RefCell<Renderer>>,
    display: Rc<Display>,
    layers: Vec<Box<dyn Layer>>
}
impl Compositor {
    fn new(renderer: Rc<RefCell<Renderer>>, display: Rc<Display>, size: (u32, u32)) -> Compositor {
        let texture_id = create_texture(renderer.clone(), display.clone(), size);
        Compositor {
            texture_id: texture_id,
            texture_size: [size.0 as f32, size.1 as f32],
            renderer: renderer,
            display: display,
            layers: Vec::new()
        }
    }
    fn create_layer<L: Layer>(&self) {
        //L::new(self.renderer, self.display, (self.texture_size[0] as u32, self.texture_size[1] as u32));
    } // TODO
    fn delete_layer(i: usize) {} // TODO
    fn image(&self) -> Image { Image::new(self.texture_id, self.texture_size) }
}