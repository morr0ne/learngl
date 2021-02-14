use bytemuck::bytes_of;
use glfw::Context;
use glow::HasContext;
use std::mem::size_of;

const WINDOW_TITLE: &str = "Triangle: Draw Arrays";
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

const VERTEX_SHADER_SOURCE: &str = include_str!("triangle.vert");
const FRAGMENT_SHADER_SOURCE: &str = include_str!("triangle.frag");

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
        let gl = glow::Context::from_loader_function(|s| window.get_proc_address(s));

        gl.clear_color(0.2, 0.3, 0.3, 1.0);

        type Vertex = [f32; 3];

        let vertices: [Vertex; 3] = [[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0]];

        let vao = gl.create_vertex_array().unwrap();
        gl.bind_vertex_array(Some(vao));

        let vbo = gl.create_buffer().unwrap();
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, bytes_of(&vertices), glow::STATIC_DRAW);

        gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, size_of::<Vertex>() as i32, 0);
        gl.enable_vertex_attrib_array(0);

        let vertex_shader = gl.create_shader(glow::VERTEX_SHADER).unwrap();
        gl.shader_source(vertex_shader, VERTEX_SHADER_SOURCE);
        gl.compile_shader(vertex_shader);
        if !gl.get_shader_compile_status(vertex_shader) {
            panic!("{}", gl.get_shader_info_log(vertex_shader));
        }

        let fragment_shader = gl.create_shader(glow::FRAGMENT_SHADER).unwrap();
        gl.shader_source(fragment_shader, FRAGMENT_SHADER_SOURCE);
        gl.compile_shader(fragment_shader);
        if !gl.get_shader_compile_status(fragment_shader) {
            panic!("{}", gl.get_shader_info_log(fragment_shader));
        }

        let shader_program = gl.create_program().unwrap();
        gl.attach_shader(shader_program, vertex_shader);
        gl.attach_shader(shader_program, fragment_shader);
        gl.link_program(shader_program);
        if !gl.get_program_link_status(shader_program) {
            panic!("{}", gl.get_program_info_log(shader_program));
        }
        gl.delete_shader(vertex_shader);
        gl.delete_shader(fragment_shader);

        gl.use_program(Some(shader_program));

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
            gl.draw_arrays(glow::TRIANGLES, 0, 3);

            // Swap front and back buffers
            window.swap_buffers();
            // Poll for and process events
            glfw.poll_events();
        }
    }
}
