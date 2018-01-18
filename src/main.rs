#[macro_use]
extern crate glium;
extern crate image;

fn main() {
	use glium::{glutin, Surface};
	let mut events_loop = glium::glutin::EventsLoop::new();

	let window = glium::glutin::WindowBuilder::new()
		.with_dimensions(1024, 768)
		.with_title("Hello World!");

	let context = glium::glutin::ContextBuilder::new();
	let display = glium::Display::new(window, context, &events_loop).unwrap();
	let image = image::load(std::io::Cursor::new(&include_bytes!("../Cat-PNG-HD.png")[..]), image::PNG).unwrap().to_rgba();
	let image_dimensions = image.dimensions();
	let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
	let texture = glium::texture::Texture2d::new(&display, image).unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
        tex_coords: [f32; 2],
    }
    implement_vertex!(Vertex, position, tex_coords);

    let vertex1 = Vertex { position: [-0.5, -0.5], tex_coords: [0.0, 0.0] };
    let vertex2 = Vertex { position: [ 0.0,  0.5], tex_coords: [0.0, 1.0], };
    let vertex3 = Vertex { position: [ 0.5, -0.25], tex_coords: [1.0, 0.0] };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

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

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut t: f32 = -2.0;
	let mut closed = false;
	while !closed {
		t += 0.0002;
		if t > 2.0 {
			t = -2.0;
		}

		let mut frame = display.draw();
		frame.clear_color(0.0, 0.0, 1.0, 1.0);
        let uniforms = uniform! {
            matrix: [
                [t.cos(), -t.sin(), 0.0, 0.0],
                [t.sin(), t.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [ t , 0.0, 0.0, 1.0],
            ],
            tex: &texture,
        };

		frame.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();
		frame.finish().unwrap();

		events_loop.poll_events(|ev| {
			match ev {
				glutin::Event::WindowEvent { event, .. } => match event {
					glutin::WindowEvent::Closed => closed = true,
					_ => (),
				},
				_ => (),
			}
		});
	}
}

