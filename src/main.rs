pub mod graphics;
pub mod render_gl;
pub mod resources;
pub mod ui;
pub mod processor;
pub mod shell;

use glutin::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use glutin::event_loop::ControlFlow;
use glutin::window::WindowBuilder;
use glutin::{Api, ContextBuilder};

use nalgebra::Vector3;
use render_gl::{ColorBuffer, Viewport};
use resources::Resources;
use shell::Shell;
use ui::Ui;

use std::env;
use std::path::Path;

use anyhow::Result;

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
    env::set_var("PYTHONPATH", "venv:scripts");

    let (gl, el, gl_context) = setup_gl()?;

    let mut viewport = Viewport::for_window(1024, 768);
    viewport.set_used(&gl);

    let color_buffer = ColorBuffer::from_color(Vector3::new(0.0, 0.0, 0.0));
    color_buffer.set_used(&gl);

    let res = Resources::from_relative_exe_path(Path::new("assets"))?;

    let ui = Ui::new(&res, &gl, viewport.w as u32, viewport.h as u32)?;

    let mut shell = Shell::new(ui)?;

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => (),
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    viewport.update_size(physical_size.width as i32, physical_size.height as i32);
                    viewport.set_used(&gl);

                    shell.handle_event(shell::Event::Resized(
                        physical_size.width,
                        physical_size.height,
                    ));
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::ReceivedCharacter(c) => {
                    // println!("{}, {:#?}", c, c);
                    match c {
                        '\u{8}' | '\r' => (), //backspace
                        _ => {
                            shell.handle_event(shell::Event::ReceivedCharacter(*c));
                        }
                    }
                }
                WindowEvent::KeyboardInput {
                    device_id: _,
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(keycode),
                            ..
                        },
                    is_synthetic: _,
                } => {
                    match keycode {
                        VirtualKeyCode::Back => {
                            shell.handle_event(shell::Event::Backspace);
                        }
                        VirtualKeyCode::Return | VirtualKeyCode::NumpadEnter => {
                            shell.handle_event(shell::Event::Enter);
                        }
                        VirtualKeyCode::Left => {
                            shell.handle_event(shell::Event::Left);
                        }
                        VirtualKeyCode::Right => {
                            shell.handle_event(shell::Event::Right);
                        }
                        VirtualKeyCode::Up => {
                            shell.handle_event(shell::Event::Previous);
                        }
                        VirtualKeyCode::Down => {
                            shell.handle_event(shell::Event::Next);
                        }
                        _ => (),
                    };
                }
                WindowEvent::KeyboardInput {
                    device_id: _,
                    input:
                        KeyboardInput {
                            state: ElementState::Released,
                            // virtual_keycode: Some(keycode),
                            ..
                        },
                    is_synthetic: _,
                } => {
                    shell.handle_event(shell::Event::Release);
                }
                _ => (),
            },
            _ => (),
        }
        color_buffer.clear(&gl);
        shell.update();

        gl_context.swap_buffers().unwrap();
    });
}

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
    }
}
