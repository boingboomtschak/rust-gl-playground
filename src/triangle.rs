use glium::{
    glutin,
    Surface,
    index::PrimitiveType,
    glutin::event_loop::ControlFlow,
    glutin::event::VirtualKeyCode,
};

#[derive(Copy, Clone)]
struct Vertex {
    position : [f32; 2],
    color: [f32; 3]
}
// Attaching vertex trait to Vertex
glium::implement_vertex!(Vertex, position, color);

fn main() {
    // Setup of event loop, window, GL context
    let event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new()
        .with_title("Triangle")
        .with_inner_size(glutin::dpi::LogicalSize::new(800f32, 800f32));
    let context_builder = glutin::ContextBuilder::new();
    let display = glium::Display::new(window_builder, context_builder, &event_loop)
        .expect("Error creating display");

    // Setting up vertex buffer
    let vertex_buffer = glium::VertexBuffer::new(&display, &[ 
        Vertex { position : [-0.5, -0.5], color : [0.0, 1.0, 0.0] }, 
        Vertex { position : [0.0, 0.5], color : [0.0, 0.0, 1.0] }, 
        Vertex { position : [0.5, -0.5], color : [1.0, 0.0, 0.0] } ])
        .expect("Error creating vertex buffer");

    let index_buffer = glium::IndexBuffer::new(&display, PrimitiveType::TrianglesList, &[0u16, 1, 2])
        .expect("Error creating index buffer");

    // Creating shader program
    let vert_src = r#"
        #version 140
        in vec2 position;
        in vec3 color;
        out vec3 vColor;
        void main() {
            vColor = color;
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;
    let frag_src = r#"
        #version 140
        in vec3 vColor;
        out vec4 color;
        void main() {
            color = vec4(vColor, 1.0);
        }
    "#;
    let program = glium::Program::from_source(&display, vert_src, frag_src, None)
        .expect("Error creating shader program");

    // Creating draw functions as a move closure (to access state of main fn context)
    let draw = move || {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.draw(&vertex_buffer, &index_buffer, &program, &glium::uniforms::EmptyUniforms, &Default::default())
            .expect("Error drawing triangle");
        target.finish()
            .expect("Error finishing draw");
    };

    // Keeping track of key states
    let mut now_keys = [false; 255];
    let mut prev_keys = now_keys.clone();
    
    // Limiting simulation / key processing to specific framerate
    let mut acc = 0_f32;
    let mut prev_t = std::time::Instant::now();
    const SIM_DT : f32 = 1.0 / 60.0;

    event_loop.run(move |event, _, control_flow| {
        // Updating control flow / other state according to events
        *control_flow = match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => ControlFlow::Exit,
                glutin::event::WindowEvent::KeyboardInput { 
                    input : glutin::event::KeyboardInput { virtual_keycode:Some(keycode), state, .. }, 
                    ..
                } => {
                    match state {
                        glutin::event::ElementState::Pressed => now_keys[keycode as usize] = true,
                        glutin::event::ElementState::Released => now_keys[keycode as usize] = false
                    };
                    ControlFlow::Poll
                },
                _ => ControlFlow::Poll
            },
            _ => ControlFlow::Poll
        };

        // Drawing whenever ready, simulating on specific time step
        let elapsed = prev_t.elapsed().as_secs_f32();
        acc += elapsed;
        prev_t = std::time::Instant::now();
        while acc >= SIM_DT {
            if now_keys[VirtualKeyCode::Q as usize] { *control_flow = ControlFlow::Exit; }
            prev_keys.copy_from_slice(&now_keys);
            acc -= SIM_DT;
        }
        draw();
    });
}
