use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use glutin::dpi::PhysicalSize;
use takeable_option::Takeable;
use std::ffi::{CStr, CString, c_void};

use gles30::*;

pub fn main() {
    let (raw_context, el) = {
        let el = EventLoop::new();
        let wb = WindowBuilder::new().with_title("A fantastic window!")
        .with_inner_size(PhysicalSize::new(2200, 512));

        let raw_context =
            ContextBuilder::new().build_windowed(wb, &el).unwrap();

        (raw_context, el)
    };

    let raw_context = unsafe { raw_context.make_current().unwrap() };

    println!(
        "Pixel format of the window's GL context: {:?}",
        raw_context.get_pixel_format()
    );

    let gl = GlFns::load_with(|c_char_ptr| {
        let cstr = unsafe { CStr::from_ptr(c_char_ptr) };
        raw_context.get_proc_address(cstr.to_str().unwrap()) as *mut std::ffi::c_void
    });
    assert!(gl.ClearColor_is_loaded());
    assert!(gl.Clear_is_loaded());

    let program = init_shaders(&gl);
    println!("program: {}", program);
    let mut degree = 0.0;
    let mut x: i32 = 2200;
    let mut y: i32 = 512;

    let mut raw_context = Takeable::new(raw_context);
    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::MainEventsCleared => {
                //raw_context.window().request_redraw();
            },
            Event::LoopDestroyed => {
                Takeable::take(&mut raw_context); // Make sure it drops first
                return;
            },
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    raw_context.resize(physical_size);
                    unsafe { gl.Viewport(0, 0, physical_size.width as i32, physical_size.height as i32); }
                    x = physical_size.width as i32;
                    y = physical_size.height as i32;
                    println!("sizes: {}, {}", x, y);
                }
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit
                }
                _ => (),
            },
            Event::RedrawRequested(_) => {
                println!("el {:?}", event);
                render(&gl, program, x, y);
                raw_context.swap_buffers().unwrap();
                degree += 1.0f32;
            }
            _ => (),
        }
    });
}

const VERTEX_SRC: &str = include_str!("vertex_shader.glsl");
const FRAGMENT_SRC: &str = include_str!("fragment_shader.glsl");
const TRI_VERTS: [gles30::GLfloat; 6] = [40.0f32, 16.0f32, 104.0f32, 16.0f32, 104.0f32, 80.0f32];
const TRI_COLS: [gles30::GLfloat; 12] = [0.0f32, 1.0f32, 0.0f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32, 0.0f32, 1.0f32, 1.0f32, 0.0f32, 0.0f32];

fn init_shaders(gl: &GlFns) -> gles30::GLuint {
    unsafe {
    let vshader = gl.CreateShader(gles30::GL_VERTEX_SHADER);
    let vsrc_ptr: &CStr = &CString::new(VERTEX_SRC).unwrap();
    gl.ShaderSource(vshader, 1, &vsrc_ptr.as_ptr(), std::ptr::null());
    gl.CompileShader(vshader);
    let mut compiled = 0;
    gl.GetShaderiv(vshader,gles30::GL_COMPILE_STATUS,&mut compiled);
    println!("vshader compilation: {}", compiled);
        if compiled == 0 {
            let mut info_len = 0;
      gl.GetShaderiv(vshader,gles30::GL_INFO_LOG_LENGTH, &mut info_len);
       if info_len > 0 {
           let mut buf: Vec<i8> = std::iter::repeat(0).take((info_len) as usize).collect();
           gl.GetShaderInfoLog(vshader, info_len, std::ptr::null_mut(), buf.as_mut_ptr());
           let ubuf: Vec<u8> = buf.iter().map(|&x| x as u8).take_while(|&x| x != 0).collect();
           println!("Could not compile shader: {}", CString::new(ubuf).unwrap()
            .to_str().unwrap());
         }
        }

    let fshader = gl.CreateShader(gles30::GL_FRAGMENT_SHADER);
    let fsrc_ptr: &CStr = &CString::new(FRAGMENT_SRC).unwrap();
    gl.ShaderSource(fshader, 1, &fsrc_ptr.as_ptr(), std::ptr::null());
    gl.CompileShader(fshader);    let mut compiled = 0;
    gl.GetShaderiv(fshader,gles30::GL_COMPILE_STATUS,&mut compiled);
    println!("fshader compilation: {}", compiled);
        if compiled == 0 {
            let mut info_len = 0;
      gl.GetShaderiv(fshader,gles30::GL_INFO_LOG_LENGTH, &mut info_len);
       if info_len > 0 {
           let mut buf: Vec<i8> = std::iter::repeat(0).take((info_len) as usize).collect();
           gl.GetShaderInfoLog(fshader, info_len, std::ptr::null_mut(), buf.as_mut_ptr());
           let ubuf: Vec<u8> = buf.iter().map(|&x| x as u8).take_while(|&x| x != 0).collect();
           println!("Could not compile shader: {}", CString::new(ubuf).unwrap()
            .to_str().unwrap());
         }
        }
    println!("shaders: {}, {}", vshader, fshader);
    // currently assuming this compiles correctly

    let program = gl.CreateProgram();
    gl.AttachShader(program, vshader);
    gl.AttachShader(program, fshader);
    gl.LinkProgram(program);
    return program;
    }
}

// this is Some Shenanigans to deal with std140 layouts - next time use the shader_types crate
const BITSTRING: [[u32; 4]; 4] = [[0xAAAAAAAA; 4], [0x55555555; 4], [0x0000FFFF; 4], [0x1425e1fe; 4]];

fn render(gl: &GlFns, program: gles30::GLuint, x: i32, y: i32) {
    unsafe {
        gl.ClearColor(0.2, 0.3, 0.3, 1.0);
        gl.Clear(gles30::GL_COLOR_BUFFER_BIT);
        gl.UseProgram(program);
        // again, assuming this works

        let pos_handle: gles30::GLuint = gl.GetAttribLocation(program, CString::new("VertexPosition").unwrap().as_ptr()) as gles30::GLuint;

        println!("handle: {}", pos_handle);

        let dims = gl.GetUniformLocation(program, CString::new("Dimensions").unwrap().as_ptr());
        println!("uniform: {}", dims);
        gl.Uniform2f(dims, x as f32, y as f32);

        gl.VertexAttribPointer(pos_handle, 2, gles30::GL_FLOAT,
            gles30::GL_FALSE as u8, 0, TRI_VERTS.as_ptr() as *const c_void);
            
        gl.EnableVertexAttribArray(pos_handle);

        let mut block = 0;
        gl.GenBuffers(1, &mut block);
        gl.BindBuffer(gles30::GL_UNIFORM_BUFFER, block);
        gl.BufferData(gles30::GL_UNIFORM_BUFFER, 16*4, BITSTRING.as_ptr() as *const c_void, gles30::GL_STATIC_DRAW);
        println!("buffer object: {}", block);

        let block_handle = gl.GetUniformBlockIndex(program, CString::new("bitstring").unwrap().as_ptr());
        println!("uniform buffer: {}", block_handle);
        gl.UniformBlockBinding(program, block_handle, 2);
        gl.BindBufferBase(GL_UNIFORM_BUFFER, 2, block);

        gl.DrawArraysInstanced(gles30::GL_TRIANGLES, 0, 3, 4*32);
    }
}