use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use std::ffi::CStr;
use std::mem::size_of;
use widgets::draw::{DrawQueue, Primitive, Vertex};
use widgets::event::Event as Event1;
use widgets::geometry::{Pointi, Rect, Size, SizeRequest};
use widgets::widget::Widget;

mod gl;
use gl::types::{GLchar, GLenum, GLfloat, GLint, GLsizei, GLsizeiptr, GLuint, GLvoid};

struct TestWidget(Rect);

impl Widget for TestWidget {
    fn get_position(&self) -> Pointi {
        self.0.pos
    }

    fn set_position(&mut self, position: Pointi) {
        self.0.pos = position
    }

    fn get_size(&self) -> Size {
        self.0.size
    }

    fn update_size(&mut self, size_request: SizeRequest) {
        self.0.size = size_request.unwrap_or(self.0.size)
    }

    fn draw(&self, dq: &mut DrawQueue) {
        let mut c = dq.begin(self);
        let r = self.0.at_origin();
        c.draw(r, 0);
        c.draw_triangle([10, 10], [190, 10], [95, 90], [0.2, 0.4, 0.6, 1.0]);
        c.draw_line(r.top_right(), r.bottom_left(), [1.0, 0.0, 0.0, 1.0]);
        c.draw_rect([0, 0], [10, 10], [1.0, 0.0, 0.0, 1.0], None);
        c.draw_rect([300, 100], [10, 10], [0.0, 0.0, 1.0, 1.0], None);
    }

    fn push_event(&mut self, _event: Event1) {}
}

fn main() {
    let event_loop = EventLoop::new();
    let wb = WindowBuilder::new().with_title("widgets");
    //.with_transparent(true);
    let win_ctx = ContextBuilder::new().build_windowed(wb, &event_loop).unwrap();
    let win_ctx = unsafe { win_ctx.make_current() }.unwrap();

    let gl = gl::Gl::load_with(|ptr| win_ctx.get_proc_address(ptr));
    let gl_vars = unsafe { init_gl(&gl) };
    //println!("vars: {:?}", gl_vars);

    let mut widget = TestWidget(Rect::new([20, 10], [200, 100]));
    let mut drawq = DrawQueue::default();

    event_loop.run(move |event, _, cf| {
        *cf = ControlFlow::Wait;
        //println!("{:?}", event);

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(size) => {
                    win_ctx.resize(size);
                    unsafe {
                        gl.Viewport(0, 0, size.width as GLsizei, size.height as GLsizei);
                        gl.Uniform2f(gl_vars.u_viewport, size.width as GLfloat, size.height as GLfloat);
                    }
                }
                WindowEvent::CloseRequested => {
                    *cf = ControlFlow::Exit;
                }
                _ => (),
            },
            Event::MainEventsCleared => {
                widget.update_size([None, None].into());
                //win_ctx.window().request_redraw();
            }
            Event::RedrawRequested(_) => {
                drawq.clear();
                widget.draw(&mut drawq);

                unsafe {
                    gl.Clear(gl::COLOR_BUFFER_BIT);
                    draw_elements(&gl, &drawq);
                }

                win_ctx.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}


const BUFFER_SIZE: GLsizeiptr = 1024;

pub struct GlVars {
    //gl: gl::Gl,
    u_viewport: GLint,
    u_tex: GLint,
    t_white: GLuint,
}

unsafe fn init_gl(gl: &gl::Gl) -> GlVars {
    use std::ptr::null;

    gl.Enable(gl::DEBUG_OUTPUT);
    gl.DebugMessageCallback(Some(debug_callback), null());

    let vert_src = include_str!("widgets.vert.glsl");
    let frag_src = include_str!("widgets.frag.glsl");

    let gl_ver = CStr::from_ptr(gl.GetString(gl::VERSION) as *const _);
    eprintln!("OpenGL version {:?}", gl_ver);

    let vert_sh = gl.CreateShader(gl::VERTEX_SHADER);
    let len = vert_src.len() as GLint;
    gl.ShaderSource(vert_sh, 1, [vert_src.as_ptr() as *const _].as_ptr(), &len);
    gl.CompileShader(vert_sh);

    let frag_sh = gl.CreateShader(gl::FRAGMENT_SHADER);
    let len = frag_src.len() as GLint;
    gl.ShaderSource(frag_sh, 1, [frag_src.as_ptr() as *const _].as_ptr(), &len);
    gl.CompileShader(frag_sh);

    let program = gl.CreateProgram();
    gl.AttachShader(program, vert_sh);
    gl.AttachShader(program, frag_sh);
    gl.LinkProgram(program);
    gl.UseProgram(program);

    let u_viewport = gl.GetUniformLocation(program, b"viewport\0".as_ptr() as *const _);
    let u_tex = gl.GetUniformLocation(program, b"tex\0".as_ptr() as *const _);

    let mut vao = 0;
    gl.GenVertexArrays(1, &mut vao);
    gl.BindVertexArray(vao);

    let mut vert_buf = 0;
    gl.GenBuffers(1, &mut vert_buf);
    gl.BindBuffer(gl::ARRAY_BUFFER, vert_buf);
    gl.BufferData(gl::ARRAY_BUFFER, BUFFER_SIZE, null(), gl::DYNAMIC_DRAW);

    let mut idx_buf = 0;
    gl.GenBuffers(1, &mut idx_buf);
    gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, idx_buf);
    gl.BufferData(gl::ELEMENT_ARRAY_BUFFER, BUFFER_SIZE, null(), gl::DYNAMIC_DRAW);

    let pos_attrib = gl.GetAttribLocation(program, b"pos\0".as_ptr() as *const _);
    let color_attrib = gl.GetAttribLocation(program, b"color\0".as_ptr() as *const _);
    let texc_attrib = gl.GetAttribLocation(program, b"texc\0".as_ptr() as *const _);

    gl.VertexAttribPointer(
        pos_attrib as GLuint,
        2,
        gl::FLOAT,
        gl::FALSE,
        size_of::<Vertex>() as GLsizei,
        Vertex::pos_offset() as *const _,
    );
    gl.EnableVertexAttribArray(pos_attrib as GLuint);

    gl.VertexAttribPointer(
        color_attrib as GLuint,
        4,
        gl::FLOAT,
        gl::FALSE,
        size_of::<Vertex>() as GLsizei,
        Vertex::color_offset() as *const _,
    );
    gl.EnableVertexAttribArray(color_attrib as GLuint);

    gl.VertexAttribPointer(
        texc_attrib as GLuint,
        2,
        gl::FLOAT,
        gl::FALSE,
        size_of::<Vertex>() as GLsizei,
        Vertex::texc_offset() as *const _,
    );
    gl.EnableVertexAttribArray(texc_attrib as GLuint);

    let mut t_white = 0;
    let pixel = [-1i32];
    gl.GenTextures(1, &mut t_white);
    gl.BindTexture(gl::TEXTURE_2D, t_white);
    gl.TexImage2D(
        gl::TEXTURE_2D,
        0,
        gl::RGBA as GLint,
        1,
        1,
        0,
        gl::RGBA,
        gl::UNSIGNED_BYTE,
        pixel.as_ptr() as *const _,
    );
    gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
    gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
    //gl.Enable(gl::TEXTURE_2D);
    gl.Uniform1i(u_tex, 0);

    gl.ClearColor(0.1, 0.1, 0.1, 1.0);

    GlVars {
        u_viewport,
        u_tex,
        t_white,
    }
}

#[allow(unused_variables)]
extern "system" fn debug_callback(
    source: GLenum, gltype: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *const GLchar,
    user_param: *mut GLvoid,
) {
    let ty_desc = match gltype {
        gl::DEBUG_TYPE_ERROR => "ERROR",
        gl::DEBUG_TYPE_MARKER => "MARKER",
        gl::DEBUG_TYPE_OTHER => "OTHER",
        gl::DEBUG_TYPE_PERFORMANCE => "PERFORMANCE",
        gl::DEBUG_TYPE_POP_GROUP => "POP_GROUP",
        gl::DEBUG_TYPE_PORTABILITY => "PORTABILITY",
        gl::DEBUG_TYPE_PUSH_GROUP => "PUSH_GROUP",
        gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => "UNDEFINED_BEHAVIOR",
        _ => "(unk)",
    };
    let sev_desc = match severity {
        gl::DEBUG_SEVERITY_HIGH => "HIGH",
        gl::DEBUG_SEVERITY_LOW => "LOW",
        gl::DEBUG_SEVERITY_MEDIUM => "MEDIUM",
        gl::DEBUG_SEVERITY_NOTIFICATION => "NOTIFICATION",
        _ => "(unk)",
    };
    let src_desc = match source {
        gl::DEBUG_SOURCE_API => "API",
        gl::DEBUG_SOURCE_APPLICATION => "APPLICATION",
        gl::DEBUG_SOURCE_OTHER => "OTHER",
        gl::DEBUG_SOURCE_SHADER_COMPILER => "SHADER_COMPILER",
        gl::DEBUG_SOURCE_THIRD_PARTY => "THIRD_PARTY",
        gl::DEBUG_SOURCE_WINDOW_SYSTEM => "WINDOW_SYSTEM",
        _ => "(unk)",
    };
    let msg = unsafe { CStr::from_ptr(message) };
    eprintln!(
        "GL CALLBACK: type={}, severity={} source={} message={:?}",
        ty_desc, sev_desc, src_desc, msg
    );
}

unsafe fn draw_elements(gl: &gl::Gl, draw_queue: &DrawQueue) {
    gl.BufferSubData(
        gl::ARRAY_BUFFER,
        0,
        (draw_queue.vertices.len() * size_of::<Vertex>()) as GLsizeiptr,
        draw_queue.vertices.as_ptr() as *const _,
    );
    gl.BufferSubData(
        gl::ELEMENT_ARRAY_BUFFER,
        0,
        (draw_queue.indices.len() * size_of::<u32>()) as GLsizeiptr,
        draw_queue.indices.as_ptr() as *const _,
    );

    for cmd in &draw_queue.commands {
        let mode = prim_to_glenum(cmd.primitive);
        gl.DrawElements(
            mode,
            cmd.idx_len as GLsizei,
            gl::UNSIGNED_INT,
            (cmd.idx_offset * size_of::<u32>()) as *const _,
        );
    }
}

fn prim_to_glenum(primitive: Primitive) -> GLenum {
    match primitive {
        Primitive::Points => gl::POINTS,
        Primitive::Lines => gl::LINES,
        Primitive::LineStrip => gl::LINE_STRIP,
        Primitive::Triangles => gl::TRIANGLES,
        Primitive::TriangleStrip => gl::TRIANGLE_STRIP,
        Primitive::TriangleFan => gl::TRIANGLE_FAN,
    }
}
