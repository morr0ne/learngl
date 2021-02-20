use anyhow::{Context as anyhowContext, Result};
use bytemuck::bytes_of;
use glfw::Context as glfwContext;
use glow::HasContext;
use std::{mem::size_of, rc::Rc};

const WINDOW_TITLE: &str = "Triangle: Draw Arrays";
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

const VERTEX_SHADER_SOURCE: &str = include_str!("triangle.vert");
const FRAGMENT_SHADER_SOURCE: &str = include_str!("triangle.frag");

mod renderer;
use renderer::Renderer;

fn main() -> Result<()> {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)?;

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw
        .create_window(WIDTH, HEIGHT, WINDOW_TITLE, glfw::WindowMode::Windowed)
        .context("Failed to create glfw windows")?;

    // Make the window's context current
    window.make_current();
    window.set_key_polling(true);

    unsafe {
        let gl = Rc::new(glow::Context::from_loader_function(|s| {
            window.get_proc_address(s)
        }));

        let renderer = Renderer { ctx: gl.clone() };
        gl.clear_color(0.2, 0.3, 0.3, 1.0);

        let shader_program =
            renderer.program_from_shaders(VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE);
        gl.use_program(Some(shader_program));

        type Vertex = [f32; 8];

        let vertices: [Vertex; 4] = [
            // top right
            [0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0],
            // bottom right
            [0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0],
            // bottom left
            [-0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0],
            // top left
            [-0.5, 0.5, 0.0, 0.2, 0.3, 0.4, 0.0, 1.0],
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

        let _texture0 = renderer.texture_2d_from_image("src/container.jpg", glow::TEXTURE0)?;
        let _texture1 = renderer.texture_2d_from_image("src/awesomeface.png", glow::TEXTURE1)?;

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
        gl.vertex_attrib_pointer_f32(
            2,
            2,
            glow::FLOAT,
            false,
            size_of::<Vertex>() as i32,
            size_of::<[f32; 6]>() as i32,
        );
        gl.enable_vertex_attrib_array(2);

        gl.uniform_1_i32(
            Some(
                &gl.get_uniform_location(shader_program, "texture0")
                    .context("No uniform found at location: texture0")?,
            ),
            0,
        );
        gl.uniform_1_i32(
            Some(
                &gl.get_uniform_location(shader_program, "texture1")
                    .context("No uniform found at locatiom: texture1")?,
            ),
            1,
        );

        // gl.polygon_mode(glow::FRONT_AND_BACK, glow::LINE);

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

            let mut trans = glam::Mat4::from_rotation_x(glfw.get_time() as f32);
            trans = trans * glam::Mat4::from_rotation_y(glfw.get_time() as f32);
            trans = trans * glam::Mat4::from_rotation_z(glfw.get_time() as f32);

            gl.uniform_matrix_4_f32_slice(
                Some(
                    &gl.get_uniform_location(shader_program, "transform")
                        .context("No uniform found at locatiom: transform")?,
                ),
                true,
                &trans.to_cols_array(),
            );

            gl.clear(glow::COLOR_BUFFER_BIT);

            gl.draw_elements(glow::TRIANGLES, 6, glow::UNSIGNED_INT, 0);

            // Swap front and back buffers
            window.swap_buffers();
            // Poll for and process events
            glfw.poll_events();
        }
    }

    Ok(())
}
