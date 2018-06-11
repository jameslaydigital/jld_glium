#[macro_use]
extern crate glium;
extern crate image;
use glium::{glutin, Surface};
use std::io::Cursor;
mod teapot; //teapot.rs//

// DEFINE VERTEX STRUCT {{{
#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, tex_coords);
// }}}

fn main() {

    // INIT WINDOW {{{
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let mut events_loop = glutin::EventsLoop::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    // }}}

    // GET MODEL {{{
    let vertices = glium::VertexBuffer::new( &display,
                                              &teapot::VERTICES ).unwrap();
    let normals = glium::VertexBuffer::new( &display,
                                            &teapot::NORMALS ).unwrap();
    let indices = glium::IndexBuffer::new( &display,
                                           glium::index::PrimitiveType::TrianglesList,
                                           &teapot::INDICES ).unwrap();
    // }}}

    //// GET IMAGE TEXTURE {{{
    //let image = 
    //    image::load( Cursor::new(&include_bytes!("/home/james/Pictures/image.png")[..]), image::PNG)
    //    .unwrap()
    //    .to_rgba();
    //let image_dimensions =
    //    image.dimensions();
    //let image = 
    //    glium::texture::RawImage2d::from_raw_rgba_reversed( &image.into_raw(), image_dimensions);
    //let texture =
    //    glium::texture::Texture2d::new(&display, image)
    //    .unwrap();
    //// }}}

    // BUILD SHADERS {{{
    let vertex_shader_src = r#"
        #version 150

        in vec3 position;
        in vec3 normal;

        out vec3 v_normal;

        uniform mat4 matrix;

        void main() {
            v_normal = transpose(inverse(mat3(matrix))) * normal;
            gl_Position = matrix * vec4(position, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        in vec3 v_normal;
        out vec4 color;
        uniform vec3 u_light;

        void main() {
            float brightness = dot(normalize(v_normal), normalize(u_light));
            vec3 dark_color = vec3(0.6, 0.0, 0.0);
            vec3 regular_color = vec3(1.0, 0.0, 0.0);
            color = vec4(mix(dark_color, regular_color, brightness), 1.0);
        }
    "#;

    let program = glium::Program::from_source(
        &display,
        vertex_shader_src,
        fragment_shader_src,
        None
    ).unwrap();
    // }}}

    // RENDER LOOP {{{
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
        let matrix = [
            [0.01, 0.0, 0.0, 0.0],
            [0.0, 0.01, 0.0, 0.0],
            [0.0, 0.0, 0.01, 0.0],
            [0.0, 0.0, 0.0, 1.0f32],
        ];

        let light = [-1.0, 0.4, 0.9f32];
        let uniforms = uniform! {
            u_light: light,
            matrix: matrix,
        };

        target.draw(
            (&vertices, &normals),
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
    // }}}

}

