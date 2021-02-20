use anyhow::Result;
use glow::HasContext;
use image as imageModule;
use imageModule::{DynamicImage, EncodableLayout};
use std::rc::Rc;

pub struct Renderer {
    pub ctx: Rc<glow::Context>,
}

impl Renderer {
    pub unsafe fn create_shader(&self, shader_type: u32, shader_source: &str) -> u32 {
        let gl = &self.ctx;

        let shader = gl.create_shader(shader_type).unwrap();
        gl.shader_source(shader, shader_source);
        gl.compile_shader(shader);
        if !gl.get_shader_compile_status(shader) {
            panic!("{}", gl.get_shader_info_log(shader));
        }

        shader
    }

    pub unsafe fn create_program(&self, vertex_shader: u32, fragment_shader: u32) -> u32 {
        let gl = &self.ctx;

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

    pub unsafe fn program_from_shaders(&self, vertex_shader: &str, fragment_shader: &str) -> u32 {
        let vertex_shader = self.create_shader(glow::VERTEX_SHADER, vertex_shader);
        let fragment_shader = self.create_shader(glow::FRAGMENT_SHADER, fragment_shader);

        self.create_program(vertex_shader, fragment_shader)
    }

    pub unsafe fn texture_2d_from_image(&self, image: &str, unit: u32) -> Result<u32> {
        let gl = &self.ctx;
        let image = imageModule::open(image)?;

        let texture = gl.create_texture().unwrap();
        gl.active_texture(unit);
        gl.bind_texture(glow::TEXTURE_2D, Some(texture));

        match image {
            DynamicImage::ImageRgb8(image) => {
                let (width, height) = image.dimensions();
                gl.tex_image_2d(
                    glow::TEXTURE_2D,
                    0,
                    glow::RGB as i32,
                    width as i32,
                    height as i32,
                    0,
                    glow::RGB,
                    glow::UNSIGNED_BYTE,
                    Some(image.as_bytes()),
                );
            }
            DynamicImage::ImageRgba8(image) => {
                let (width, height) = image.dimensions();
                gl.tex_image_2d(
                    glow::TEXTURE_2D,
                    0,
                    glow::RGBA as i32,
                    width as i32,
                    height as i32,
                    0,
                    glow::RGBA,
                    glow::UNSIGNED_BYTE,
                    Some(image.as_bytes()),
                );
            }
            _ => (),
        }

        gl.generate_mipmap(glow::TEXTURE_2D);

        Ok(texture)
    }
}
