#[macro_use]
extern crate glium;
extern crate image;

use glium::{glutin, Surface};
use std::io::Cursor;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

fn main() {

    // INIT WINDOW //
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let mut events_loop = glutin::EventsLoop::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    // END INIT WINDOW //

    // BUILD VERTICES //
    let v1 = Vertex { position: [-0.5, -0.5 ], tex_coords: [0.0, 0.0] };
    let v2 = Vertex { position: [ 0.0,  0.5 ], tex_coords: [0.0, 1.0] };
    let v3 = Vertex { position: [ 0.5, -0.25], tex_coords: [1.0, 0.0] };
    let shape = vec![v1, v2, v3];
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    // END BUILD VERTICES //

    // GET IMAGE TEXTURE //
    let image = 
        image::load( Cursor::new(&include_bytes!("/home/james/Pictures/image.png")[..]), image::PNG)
        .unwrap()
        .to_rgba();
    let image_dimensions =
        image.dimensions();
    let image = 
        glium::texture::RawImage2d::from_raw_rgba_reversed( &image.into_raw(), image_dimensions);
    let texture =
        glium::texture::Texture2d::new(&display, image)
        .unwrap();
    // END GET IMAGE TEXTURE //


    // BUILD SHADERS //
    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        in vec2 tex_coords;
        out vec2 v_tex_coords;

        uniform mat4 matrix;

        void main() {
            v_tex_coords = tex_coords;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        in vec2 v_tex_coords;
        out vec4 color;

        uniform sampler2D tex;

        void main() {
            color = texture(tex, v_tex_coords);
        }
    "#;

    let program = glium::Program::from_source(
        &display,
        vertex_shader_src,
        fragment_shader_src,
        None
    ).unwrap();
    // END BUILD SHADERS //


    let mut t: f32 = -0.5;
    let mut closed = false;
    while closed == false {

        t = if t >= 0.5 {
            -0.5
        } else {
            t + 0.001
        };

        let mut target = display.draw();
        target.clear_color(0.9, 0.3, 0.1, 1.0);
        // DRAW HERE //
        let uniforms = uniform! {
            matrix: [
                [t.cos(),   t.sin(), 0.0, 0.0],
                [-t.sin(),  t.cos(), 0.0, 0.0],
                [0.0,       0.0,     1.0, 0.0],
                [  t,       0.0,     0.0, 1.0f32],
            ],
            tex: &texture,
        };
        target.draw(
            &vertex_buffer,
            &indices,
            &program,
            &uniforms,
            &Default::default()
        ).unwrap();
        target.finish().unwrap();

        events_loop.poll_events(|ev| {
            match ev {
                glutin::Event::WindowEvent{event, ..} => match event {
                    glutin::WindowEvent::Closed => {
                        closed = true;
                        println!("Closed");
                    }
                    _ => (),
                },
                _ => (),
            }
        });
    }

}
