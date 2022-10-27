pub mod graphics;
pub mod render_gl;
pub mod resources;
pub mod ui;
pub mod processor;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;
use glutin::window::WindowBuilder;
use glutin::{Api, ContextBuilder};

use nalgebra::Vector3;
use processor::Processor;
use render_gl::{ColorBuffer, Viewport};
use resources::Resources;
use ui::state::State;

use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use ui::textfield::TextField;

use anyhow::Result;

use crate::ui::state::Component;

fn setup_gl() -> Result<(
    gl::Gl,
    glutin::event_loop::EventLoop<()>,
    glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>,
)> {
    let el = glutin::event_loop::EventLoop::new();
    let wb = WindowBuilder::new()
        .with_title("Terminal")
        .with_inner_size(glutin::dpi::LogicalSize::new(1024_i32, 768_i32));
    let windowed_context = ContextBuilder::new()
        .with_gl_profile(glutin::GlProfile::Core)
        .with_gl(glutin::GlRequest::Specific(Api::OpenGl, (4, 6)))
        .build_windowed(wb, &el)?;

    let gl_context = unsafe { windowed_context.make_current().unwrap() };
    let gl = gl::Gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

    unsafe {
        gl.Enable(gl::BLEND);
        gl.BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }
    Ok((gl, el, gl_context))
}

fn run() -> Result<()> {
    let (gl, el, gl_context) = setup_gl()?;

    let mut viewport = Viewport::for_window(1024, 768);
    viewport.set_used(&gl);

    let color_buffer = ColorBuffer::from_color(Vector3::new(0.0, 0.0, 0.0));
    color_buffer.set_used(&gl);

    let res = Resources::from_relative_exe_path(Path::new("assets"))?;

    let processor = Processor::new();
    let text_field = TextField::new(&res, &gl, viewport.w as u32, viewport.h as u32)?;
    let mut state = State::new(text_field, processor);

    // setup_listeners(&mut text_field, &processor);
    state.on_enter(Component::Processor);
    state.on_print(Component::TextField);

    state.run();

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => (),
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    viewport.update_size(physical_size.width as i32, physical_size.height as i32);
                    viewport.set_used(&gl);
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::RedrawRequested(_) => {}
            _ => (),
        }
        state.update(&event);
        color_buffer.clear(&gl);
        state.render(&gl);

        gl_context.swap_buffers().unwrap();
        // processor.update();
    });
}

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
    }
}
