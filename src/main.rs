use bytemuck::bytes_of;
use glfw::Context;
use glow::HasContext;
use std::{mem::size_of, rc::Rc};

const WINDOW_TITLE: &str = "Triangle: Draw Arrays";
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

const VERTEX_SHADER_SOURCE: &str = include_str!("triangle.vert");
const FRAGMENT_SHADER_SOURCE: &str = include_str!("triangle.frag");

struct Renderer {
    gl: Rc<glow::Context>,
}

impl Renderer {
    unsafe fn create_shader(&self, shader_type: u32, shader_source: &str) -> u32 {
        let gl = &self.gl;

        let shader = gl.create_shader(shader_type).unwrap();
        gl.shader_source(shader, shader_source);
        gl.compile_shader(shader);
        if !gl.get_shader_compile_status(shader) {
            panic!("{}", gl.get_shader_info_log(shader));
        }

        shader
    }

    unsafe fn create_program(&self, vertex_shader: u32, fragment_shader: u32) -> u32 {
        let gl = &self.gl;

        let program = gl.create_program().unwrap();
        gl.attach_shader(program, vertex_shader);
        gl.attach_shader(program, fragment_shader);
        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            panic!("{}", gl.get_program_info_log(program));
        }
        gl.delete_shader(vertex_shader);
        gl.delete_shader(fragment_shader);

        program
    }

    unsafe fn program_from_shaders(&self, vertex_shader: &str, fragment_shader: &str) -> u32 {
        let vertex_shader = self.create_shader(glow::VERTEX_SHADER, vertex_shader);
        let fragment_shader = self.create_shader(glow::FRAGMENT_SHADER, fragment_shader);

        self.create_program(vertex_shader, fragment_shader)
    }
}

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw
        .create_window(WIDTH, HEIGHT, WINDOW_TITLE, glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // Make the window's context current
    window.make_current();
    window.set_key_polling(true);

    unsafe {
        let gl = Rc::new(glow::Context::from_loader_function(|s| {
            window.get_proc_address(s)
        }));

        let renderer = Renderer { gl: gl.clone() };
        gl.clear_color(0.2, 0.3, 0.3, 1.0);

        let program = renderer.program_from_shaders(VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE);

        type Vertex = [f32; 6];

        let vertices: [Vertex; 3] = [
            [0.5, -0.5, 0.0, 1.0, 0.0, 1.0],
            [-0.5, -0.5, 0.0, 0.0, 1.0, 0.0],
            [0.0, 0.5, 0.0, 0.0, 0.0, 1.0],
        ];

        let indices: [u32; 6] = [0, 1, 3, 1, 2, 3];

        let vao = gl.create_vertex_array().unwrap();
        gl.bind_vertex_array(Some(vao));

        let vbo = gl.create_buffer().unwrap();
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, bytes_of(&vertices), glow::STATIC_DRAW);

        let ebo = gl.create_buffer().unwrap();
        gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
        gl.buffer_data_u8_slice(
            glow::ELEMENT_ARRAY_BUFFER,
            bytes_of(&indices),
            glow::STATIC_DRAW,
        );

        gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, size_of::<Vertex>() as i32, 0);
        gl.enable_vertex_attrib_array(0);

        gl.vertex_attrib_pointer_f32(
            1,
            3,
            glow::FLOAT,
            false,
            size_of::<Vertex>() as i32,
            size_of::<[f32; 3]>() as i32,
        );
        gl.enable_vertex_attrib_array(1);

        // gl.polygon_mode(glow::FRONT_AND_BACK, glow::LINE);

        let attr = gl.get_parameter_i32(glow::MAX_VERTEX_ATTRIBS);
        println!("{}", attr);

        // Loop until the user closes the window
        while !window.should_close() {
            for (_, event) in glfw::flush_messages(&events) {
                println!("{:?}", event);
                match event {
                    glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                        window.set_should_close(true)
                    }
                    glfw::WindowEvent::FramebufferSize(width, height) => {
                        gl.viewport(0, 0, width, height)
                    }
                    _ => {}
                }
            }
            gl.clear(glow::COLOR_BUFFER_BIT);

            gl.use_program(Some(program));

            gl.draw_arrays(glow::TRIANGLES, 0, 3);
            // gl.draw_elements(glow::TRIANGLES, 6, glow::UNSIGNED_INT, 0);

            // Swap front and back buffers
            window.swap_buffers();
            // Poll for and process events
            glfw.poll_events();
        }
    }
}
