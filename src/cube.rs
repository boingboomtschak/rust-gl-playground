use glium::{
    glutin,
    Surface,
    index::PrimitiveType,
    glutin::event_loop::ControlFlow,
    glutin::event::VirtualKeyCode,
    uniform,
};
use glam::{Mat4, Vec3};

#[derive(Copy, Clone)]
struct Vertex {
    position : [f32; 3],
    normal : [f32; 3],
    color : [f32; 3],
}
glium::implement_vertex!(Vertex, position, normal, color);

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new()
        .with_title("Cube")
        .with_inner_size(glutin::dpi::LogicalSize::new(600_f32, 600_f32));
    let context_builder = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(window_builder, context_builder, &event_loop)
        .expect("Error creating display");
    
    let vertex_buffer = glium::VertexBuffer::new(&display, &[
        Vertex { position : [-1.0, -1.0,  1.0], normal : [ 0.0,  0.0,  1.0], color : [1.0, 0.0, 0.0] },
        Vertex { position : [ 1.0, -1.0,  1.0], normal : [ 0.0,  0.0,  1.0], color : [1.0, 0.0, 0.0] },
        Vertex { position : [ 1.0,  1.0,  1.0], normal : [ 0.0,  0.0,  1.0], color : [1.0, 0.0, 0.0] },
        Vertex { position : [-1.0,  1.0,  1.0], normal : [ 0.0,  0.0,  1.0], color : [1.0, 0.0, 0.0] },
        Vertex { position : [-1.0, -1.0, -1.0], normal : [ 0.0,  0.0, -1.0], color : [0.0, 1.0, 0.0] },
        Vertex { position : [ 1.0, -1.0, -1.0], normal : [ 0.0,  0.0, -1.0], color : [0.0, 1.0, 0.0] },
        Vertex { position : [ 1.0,  1.0, -1.0], normal : [ 0.0,  0.0, -1.0], color : [0.0, 1.0, 0.0] },
        Vertex { position : [-1.0,  1.0, -1.0], normal : [ 0.0,  0.0, -1.0], color : [0.0, 1.0, 0.0] },
        Vertex { position : [-1.0, -1.0,  1.0], normal : [-1.0,  0.0,  0.0], color : [0.0, 0.0, 1.0] },
        Vertex { position : [-1.0, -1.0, -1.0], normal : [-1.0,  0.0,  0.0], color : [0.0, 0.0, 1.0] },
        Vertex { position : [-1.0,  1.0, -1.0], normal : [-1.0,  0.0,  0.0], color : [0.0, 0.0, 1.0] },
        Vertex { position : [-1.0,  1.0,  1.0], normal : [-1.0,  0.0,  0.0], color : [0.0, 0.0, 1.0] },
        Vertex { position : [ 1.0, -1.0,  1.0], normal : [ 1.0,  0.0,  0.0], color : [1.0, 1.0, 0.0] },
        Vertex { position : [ 1.0, -1.0, -1.0], normal : [ 1.0,  0.0,  0.0], color : [1.0, 1.0, 0.0] },
        Vertex { position : [ 1.0,  1.0, -1.0], normal : [ 1.0,  0.0,  0.0], color : [1.0, 1.0, 0.0] },
        Vertex { position : [ 1.0,  1.0,  1.0], normal : [ 1.0,  0.0,  0.0], color : [1.0, 1.0, 0.0] },
        Vertex { position : [-1.0,  1.0,  1.0], normal : [ 0.0,  1.0,  0.0], color : [0.0, 1.0, 1.0] },
        Vertex { position : [ 1.0,  1.0,  1.0], normal : [ 0.0,  1.0,  0.0], color : [0.0, 1.0, 1.0] },
        Vertex { position : [ 1.0,  1.0, -1.0], normal : [ 0.0,  1.0,  0.0], color : [0.0, 1.0, 1.0] },
        Vertex { position : [-1.0,  1.0, -1.0], normal : [ 0.0,  1.0,  0.0], color : [0.0, 1.0, 1.0] },
        Vertex { position : [-1.0, -1.0,  1.0], normal : [ 0.0, -1.0,  0.0], color : [1.0, 0.0, 1.0] },
        Vertex { position : [ 1.0, -1.0,  1.0], normal : [ 0.0, -1.0,  0.0], color : [1.0, 0.0, 1.0] },
        Vertex { position : [ 1.0, -1.0, -1.0], normal : [ 0.0, -1.0,  0.0], color : [1.0, 0.0, 1.0] },
        Vertex { position : [-1.0, -1.0, -1.0], normal : [ 0.0, -1.0,  0.0], color : [1.0, 0.0, 1.0] }
    ]).expect("Error creating vertex buffer");

    let index_buffer = glium::IndexBuffer::new(&display, PrimitiveType::TrianglesList, &[
        0u16, 1, 2, 2, 3, 0u16, // front
        6, 5, 4, 4, 7, 6, // back
        10, 9, 8, 8, 11, 10, // left
        12, 13, 14, 14, 15, 12, // right
        18, 17, 16, 16, 19, 18, // top
        22, 21, 20, 20, 23, 22 // bottom
    ]).expect("Error creating index buffer");

    let vert_src = r#"
        #version 140
        in vec3 position;
        in vec3 normal;
        in vec3 color;
        out vec3 vPosition;
        out vec3 vNormal;
        out vec3 vColor;
        uniform mat4 model;
        uniform mat4 view;
        uniform mat4 persp;
        void main() {
            vPosition = (view * model * vec4(position, 1)).xyz;
            vNormal = (view * model * vec4(normal, 0)).xyz;
            vColor = color;
            gl_Position = persp * vec4(vPosition, 1);
        }
    "#;

    let frag_src = r#"
        #version 140
        in vec3 vPosition;
        in vec3 vNormal;
        in vec3 vColor;
        out vec4 color;
        uniform vec3 light = vec3(-3.0, 1.0, -2.0);
        uniform float amb = 0.4;
        uniform float dif = 0.4;
        uniform float spc = 0.4;
        void main() {
            vec3 N = normalize(vNormal);
            vec3 L = normalize(light - vPosition);
            vec3 E = normalize(vPosition);
            vec3 R = reflect(L, N);
            float d = dif*max(0, dot(N, L));
            float h = max(0, dot(R, E));
            float s = spc*pow(h, 100);
            float intensity = clamp(amb+d+s, 0, 1);
            color = vec4(intensity * vColor, 1);
        }
    "#;
    let program = glium::Program::from_source(&display, vert_src, frag_src, None)
        .expect("Error creating shader program");


    // Keeping track of key states
    let mut now_keys = [false; 255];
    let mut prev_keys = now_keys.clone();

    // Limiting simulation / key processing to specific framerate
    let mut acc = 0_f32;
    let mut prev_t = std::time::Instant::now();
    const SIM_DT : f32 = 1.0 / 60.0;

    let start_t = std::time::Instant::now();
    
    let persp = Mat4::perspective_rh_gl(45.0, 1.0, 0.1, 10.0);
    let view = Mat4::look_at_rh(Vec3::new(-4.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));

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
            if now_keys[VirtualKeyCode::Q as usize] && !prev_keys[VirtualKeyCode::Q as usize] { *control_flow = ControlFlow::Exit; }
            prev_keys.copy_from_slice(&now_keys);
            acc -= SIM_DT;
        }
        let model = Mat4::from_euler(glam::EulerRot::YZX, 
            start_t.elapsed().as_secs_f32() * 1.5, 
            start_t.elapsed().as_secs_f32() * 0.75,
            0.0);
        let uniforms = uniform!{ 
            persp : persp.to_cols_array_2d(), 
            view : view.to_cols_array_2d(), 
            model : model.to_cols_array_2d() 
        };
        let draw_params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 0.2), 1.0);
        target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &draw_params)
            .expect("Error drawing cube");
        target.finish()
            .expect("Error finishing draw");
    });
}