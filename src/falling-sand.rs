use glium::{
    glutin,
    Surface,
    glutin::event_loop::ControlFlow,
    glutin::event::VirtualKeyCode,
};
use glam::{Mat4, Vec3, Quat};
use std::cmp::Ordering;
use rand::Rng;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Particle {
    AIR,
    SAND,
    WATER
}

#[derive(Debug, Copy, Clone)]
struct Move {
    src : (usize, usize),
    dst : (usize, usize)
}
impl PartialEq for Move {
    fn eq(&self, other: &Self) -> bool { self.dst.0 == other.dst.0 && self.dst.1 == other.dst.1 }
}
impl Eq for Move { }
impl Ord for Move {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.dst.0 == other.dst.0 { self.dst.1.cmp(&other.dst.1) } 
        else { self.dst.0.cmp(&other.dst.0) }
    }
}
impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone)]
struct Vertex {
    position : [f32; 2]
}
glium::implement_vertex!(Vertex, position);

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new()
        .with_title("Falling Sand")
        .with_inner_size(glutin::dpi::LogicalSize::new(600f32, 600f32));
    let context_builder = glutin::ContextBuilder::new();
    let display = glium::Display::new(window_builder, context_builder, &event_loop)
        .expect("Error creating display");

    let vertex_buffer = glium::VertexBuffer::new(&display, &[
        Vertex { position : [-1.0, -1.0] },
        Vertex { position : [1.0, -1.0] },
        Vertex { position : [1.0, 1.0] },
        Vertex { position : [1.0, 1.0] },
        Vertex { position : [-1.0, 1.0] },
        Vertex { position : [-1.0, -1.0] },
    ]).expect("Error creating vertex buffer");
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vert_src = r#"
        #version 140
        in vec2 position;
        uniform mat4 transform;
        void main() {
            gl_Position = transform * vec4(position, 0, 1);
        }
    "#;

    let frag_src = r#"
        #version 140
        out vec4 color;
        uniform vec3 pColor;
        void main() {
            color = vec4(pColor, 1);
        }
    "#;

    let program = glium::Program::from_source(&display, vert_src, frag_src, None)
        .expect("Error compiling shader program");

    let mut rng = rand::thread_rng();

    const GRID_WIDTH : usize = 100;
    const GRID_HEIGHT : usize = 100;
    
    let mut grid = [[Particle::AIR; GRID_HEIGHT]; GRID_WIDTH];

    let mut now_keys = [false; 255];
    let mut prev_keys = now_keys.clone();

    let mut acc = 0f32;
    let mut prev_t = std::time::Instant::now();
    const SIM_DT : f32 = 1.0 / 60.0;

    //let start_t = std::time::Instant::now();

    event_loop.run(move |event, _, control_flow| {
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

        // Update time accumulator
        let elapsed = prev_t.elapsed().as_secs_f32();
        acc += elapsed;
        prev_t = std::time::Instant::now();
        // If time for update, update and decrement accumulator
        while acc >= SIM_DT {
            // Handle key changes and update keys
            if now_keys[VirtualKeyCode::Q as usize] && !prev_keys[VirtualKeyCode::Q as usize] { *control_flow = ControlFlow::Exit; }
            if now_keys[VirtualKeyCode::R as usize] && !prev_keys[VirtualKeyCode::R as usize] { 
                for i in 0..GRID_WIDTH { for j in 0..GRID_HEIGHT { grid[i][j] = Particle::AIR; } }
            }
            prev_keys.copy_from_slice(&now_keys);

            // Update simulation
            // - Collect moves for all particles
            let mut moves : Vec<Move> = Vec::new();
            for i in 0..GRID_WIDTH {
                for j in 0..GRID_HEIGHT {
                    if grid[i][j] == Particle::SAND {
                        if j > 0 && (grid[i][j - 1] == Particle::AIR || grid[i][j - 1] == Particle::WATER) {
                            moves.push(Move { src: (i, j), dst: (i, j - 1) });
                        } else if i > 0 && j > 0 && (grid[i - 1][j - 1] == Particle::AIR || grid[i - 1][j - 1] == Particle::WATER) {
                            moves.push(Move { src: (i, j), dst: (i - 1, j - 1) });
                        } else if i < (GRID_WIDTH - 1) && j > 0 && (grid[i + 1][j - 1] == Particle::AIR || grid[i + 1][j - 1] == Particle::WATER) {
                            moves.push(Move { src: (i, j), dst: (i + 1, j - 1) });
                        }
                    } else if grid[i][j] == Particle::WATER {

                    }
                }
            }
            // - Sort moves by destination
            moves.sort_unstable();
            // - Iterate through moves, add to stack, and pick move from stack to execute on destination change
            let mut dst_prev = 0;
            moves.push(Move { src: (usize::MAX, usize::MAX), dst: (usize::MAX, usize::MAX) });
            for i in 0..(moves.len() - 1) {
                if moves[i + 1] != moves[i] {
                    let m = moves[rng.gen_range(dst_prev..(i + 1))];
                    let p = grid[m.src.0][m.src.1];
                    grid[m.src.0][m.src.1] = grid[m.dst.0][m.dst.1];
                    grid[m.dst.0][m.dst.1] = p;
                    dst_prev = i + 1;
                }
            }

            grid[rng.gen_range(0..GRID_WIDTH)][99] = Particle::SAND;
            //grid[50][99] = Particle::SAND;
            
            // Decrement accumulator
            acc -= SIM_DT;
        }


        let mut target = display.draw();
        target.clear_color(0.1, 0.1, 0.1, 1.0);
        for i in 0..GRID_WIDTH {
            for j in 0..GRID_HEIGHT {
                if !matches!(grid[i][j], Particle::AIR) {
                    let color = match grid[i][j] {
                        Particle::SAND => [1.0, 0.883, 0.617f32],
                        Particle::WATER => [0.176, 0.535, 0.938f32],
                        _ => [1.0, 0.0, 1.0f32]
                    };
                    let width = GRID_WIDTH as f32;
                    let height = GRID_HEIGHT as f32;
                    let x = i as f32;
                    let y = j as f32;
                    let transform = Mat4::from_scale_rotation_translation(
                        Vec3::new(1.0 / (GRID_WIDTH as f32), 1.0 / (GRID_HEIGHT as f32), 1.0),
                        Quat::IDENTITY,
                        Vec3::new(2.0 * ((x / width) - 0.5), 2.0 * ((y / height) - 0.5), 0.0)
                    );
                    let uniforms = glium::uniform!{
                        transform : transform.to_cols_array_2d(),
                        pColor : color
                    };
                    target.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default())
                        .expect("Error drawing particle");
                }
            }
        }
        target.finish().expect("Error finishing draw");
    });
}