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
    texcoord : [f32; 2]
}
glium::implement_vertex!(Vertex, position, normal, texcoord);

struct Mesh {
    vertex_buffer : glium::VertexBuffer<Vertex>,
    index_buffer : glium::IndexBuffer<u32>
}
impl Mesh {
    fn new(display : &glium::Display, mesh : &tobj::Mesh) -> Mesh {
        let mut vertices = Vec::new();
        for i in 0..(mesh.positions.len() / 3) {
            let pos : [f32; 3] = mesh.positions[(i*3)..(i*3)+3].try_into().unwrap();
            let norm : [f32; 3] = mesh.normals[(i*3)..(i*3)+3].try_into().unwrap();
            let uv : [f32; 2] = mesh.texcoords[(i*2)..(i*2)+2].try_into().unwrap();
            vertices.push(Vertex { position : pos, normal : norm, texcoord : uv });
        }
        let v_buf = glium::VertexBuffer::new(display, &vertices)
            .expect("Error creating vertex buffer for mesh");
        let i_buf = glium::IndexBuffer::new(display, PrimitiveType::TrianglesList, &mesh.indices)
            .expect("Error creating index buffer for mesh");
        Mesh { vertex_buffer : v_buf, index_buffer : i_buf }
    }
    fn create_meshes(display : &glium::Display, filename : &str) -> Vec<Mesh> {
        let (mdls, _mtls) = tobj::load_obj(filename, &tobj::GPU_LOAD_OPTIONS)
            .expect("Error loading obj file");
        let mut meshes = Vec::new();
        for mdl in mdls {
            meshes.push(Mesh::new(display, &mdl.mesh));
        }
        meshes
    }
}

fn main() {
    // Preamble
    let event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new()
        .with_title("Model Display")
        .with_inner_size(glutin::dpi::LogicalSize::new(600f32, 600f32));
    let context_builder = glutin::ContextBuilder::new()
        .with_depth_buffer(24);
    let display = glium::Display::new(window_builder, context_builder, &event_loop)
        .expect("Error creating display");

    // Load obj, create meshes
    let obj_filename = std::env::args()
        .skip(1)
        .next()
        .expect("Path to obj file is required as first arg");
    let meshes = Mesh::create_meshes(&display, &obj_filename);

    // Creating display program
    let vert_src = r#"
        #version 140
        in vec3 position;
        in vec3 normal;
        in vec3 _texcoord; // haven't set up textures yet
        out vec3 vPosition;
        out vec3 vNormal;
        uniform mat4 model;
        uniform mat4 view;
        uniform mat4 persp;
        void main() {
            vPosition = (view * model * vec4(position, 1)).xyz;
            vNormal = (view * model * vec4(normal, 0)).xyz;
            gl_Position = persp * vec4(vPosition, 1);
        }
    "#;

    let frag_src = r#"
        #version 140
        in vec3 vPosition;
        in vec3 vNormal;
        out vec4 color;
        uniform vec3 light = vec3(-3.0, 1.0, -2.0);
        uniform float amb = 0.1;
        uniform float dif = 0.5;
        uniform float spc = 0.7;
        void main() {
            vec3 N = normalize(vNormal);
            vec3 L = normalize(light - vPosition);
            vec3 E = normalize(vPosition);
            vec3 R = reflect(L, N);
            float d = dif*max(0, dot(N, L));
            float h = max(0, dot(R, E));
            float s = spc*pow(h, 50);
            float intensity = clamp(amb+d+s, 0, 1);
            color = vec4(intensity * vec3(1.0, 0.0, 0.0), 1); // COLOR SET TO RED UNTIL TEXTURES SETUP
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
        let t = start_t.elapsed().as_secs_f32();
        let (w, h) = display.get_framebuffer_dimensions();
        let persp = Mat4::perspective_rh_gl(45.0, (w as f32) / (h as f32), 0.1, 10.0);
        let view = Mat4::look_at_rh(Vec3::new(-4.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
        let model = Mat4::from_euler(glam::EulerRot::YZX, t * 1.5, t * 0.75, 0.0);
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
        target.clear_color_and_depth((0.2, 0.2, 0.2, 1.0), 1.0);
        for mesh in meshes.iter() {
            target.draw(&mesh.vertex_buffer, &mesh.index_buffer, &program, &uniforms, &draw_params)
                .expect("Error drawing mesh");
        }
        target.finish().expect("Error finishing draw");
    });
}