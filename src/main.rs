#![feature(duration_checked_ops)]

extern crate rand;

#[macro_use]
extern crate glium;

use rand::random;
use std::time::{Duration, SystemTime};
use std::thread::sleep;

fn game_loop() {
    sleep(Duration::from_millis(random::<u64>() % 50));
}

fn main() {
    use glium::DisplayBuild;
    use glium::Surface;
    use glium::glutin::Event;

    let fps = 60;
    let time_for_frame = Duration::from_millis(1000 / fps);

    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);

    let vertex1 = Vertex { position: [-0.25, -0.5] };
    let vertex2 = Vertex { position: [ 0.0,  0.5] };
    let vertex3 = Vertex { position: [ 0.25, -0.5] };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        uniform float x;
        uniform float y;
        void main() {
            vec2 pos = position;
            pos.x += x;
            pos.y += y;
            gl_Position = vec4(pos, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let speed = 0.01;
    let speed_decay = 0.9;

    let mut x: f32 = -0.5;
    let mut y: f32 = -0.5;

    let mut x_speed: f32 = 0.0;
    let mut y_speed: f32 = 0.0;

    let mut moving_left = false;
    let mut moving_right = false;
    let mut moving_up = false;
    let mut moving_down = false;

    loop {
        let now = SystemTime::now();

        x_speed *= speed_decay;
        y_speed *= speed_decay;

        if moving_left { x_speed = -speed; }
        if moving_right { x_speed = speed; }

        if moving_down { y_speed = -speed; }
        if moving_up { y_speed = speed; }

        x += x_speed;
        y += y_speed;

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.2, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &uniform! { x: x, y: y },
                    &Default::default()).unwrap();
        target.finish().unwrap();

        // listing the events produced by the window and waiting to be received
        for ev in display.poll_events() {
            use glium::glutin::ElementState::{Pressed, Released};
            use glium::glutin::VirtualKeyCode::{Left, Right, Down, Up};
            match ev {
                Event::Closed => return,   // the window has been closed by the user
                Event::KeyboardInput(Pressed, _, Some(Left)) => moving_left = true,
                Event::KeyboardInput(Released, _, Some(Left)) => moving_left = false,
                Event::KeyboardInput(Pressed, _, Some(Right)) => moving_right = true,
                Event::KeyboardInput(Released, _, Some(Right)) => moving_right = false,
                Event::KeyboardInput(Pressed, _, Some(Down)) => moving_down = true,
                Event::KeyboardInput(Released, _, Some(Down)) => moving_down = false,
                Event::KeyboardInput(Pressed, _, Some(Up)) => moving_up = true,
                Event::KeyboardInput(Released, _, Some(Up)) => moving_up = false,
                _ => ()
            }
        }

        match now.elapsed() {
            Ok(elapsed) => {
                match time_for_frame.checked_sub(elapsed) {
                    Some(duration) => sleep(duration),
                    None => ()
                }
            }
            Err(e) => {
                println!("Error {:?}", e)
            }
        }
    }
}
