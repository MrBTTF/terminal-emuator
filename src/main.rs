pub mod graphics;
pub mod processor;
pub mod render_gl;
pub mod resources;
pub mod shell;
pub mod ui;

use glutin::config::{Config, ConfigTemplateBuilder, GlConfig};
use glutin::context::{ContextAttributesBuilder, GlProfile, NotCurrentGlContext, Version};
use glutin::context::{NotCurrentContext, PossiblyCurrentContext};
use glutin::display::{GetGlDisplay, GlDisplay};
use glutin::surface::{GlSurface, WindowSurface};
use glutin::surface::{Surface, SwapInterval};
use glutin_winit::{DisplayBuilder, GlWindow};
use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, KeyEvent, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopBuilder};

use nalgebra::Vector3;
use raw_window_handle::HasRawWindowHandle;
use render_gl::{ColorBuffer, Viewport};
use resources::Resources;
use shell::Shell;
use ui::Ui;
use winit::window::{Window, WindowBuilder};
use winit::{keyboard, window};

use std::env;
use std::ffi::{CStr, CString};
use std::num::NonZeroU32;
use std::path::Path;

use anyhow::{Ok, Result};

fn setup_gl() -> Result<(
    gl::Gl,
    EventLoop<()>,
    (PossiblyCurrentContext, Surface<WindowSurface>, Window),
)> {
    let el = EventLoopBuilder::new().build()?;

    let window_builder = WindowBuilder::new()
        .with_title("Terminal")
        .with_transparent(false)
        .with_inner_size(LogicalSize::new(1024_i32, 768_i32));

    let template = ConfigTemplateBuilder::default();

    let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));
    let (window, gl_config) = display_builder
        .build(&el, template, |configs| {
            // Find the config with the maximum number of samples, so our triangle will
            // be smooth.
            configs
                .reduce(|accum, config| {
                    let transparency_check = config.supports_transparency().unwrap_or(false)
                        & !accum.supports_transparency().unwrap_or(false);

                    if transparency_check || config.num_samples() > accum.num_samples() {
                        config
                    } else {
                        accum
                    }
                })
                .unwrap()
        })
        .unwrap();

    let raw_window_handle = window.as_ref().map(|window| window.raw_window_handle());

    let context_attributes = ContextAttributesBuilder::new()
        .with_profile(GlProfile::Core)
        .with_context_api(glutin::context::ContextApi::OpenGl(Some(Version::new(
            4, 6,
        ))))
        .build(raw_window_handle);

    let gl_display = gl_config.display();
    let not_current = unsafe {
        gl_display
            .create_context(&gl_config, &context_attributes)
            .expect("failed to create context")
    };

    let window = window.unwrap();
    let attrs = window.build_surface_attributes(Default::default());
    let gl_surface = unsafe {
        gl_config
            .display()
            .create_window_surface(&gl_config, &attrs)
            .unwrap()
    };

    let gl_context = not_current.make_current(&gl_surface).unwrap();

    let gl = unsafe {
        let gl = gl::Gl::load_with(|ptr| {
            let s = CString::new(ptr).expect("CString::new failed");
            gl_display.get_proc_address(&s) as *const _
        });

        gl.Enable(gl::BLEND);
        gl.BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl
    };
    Ok((gl, el, (gl_context, gl_surface, window)))
}

fn run() -> Result<()> {
    env::set_var("PYTHONPATH", "venv:scripts");

    let (gl, el, state) = setup_gl()?;

    let mut viewport = Viewport::for_window(1024, 768);
    viewport.set_used(&gl);
    let color_buffer = ColorBuffer::from_color(Vector3::new(0.0, 0.0, 0.0));
    color_buffer.set_used(&gl);

    let res = Resources::from_relative_exe_path(Path::new("assets"))?;

    let ui = Ui::new(&res, &gl, viewport.w as u32, viewport.h as u32)?;

    let mut shell = Shell::new(ui)?;
    let (gl_context, gl_surface, window) = &state;

    el.run(move |event, elwt| {
        // println!("{:#?}", event);

        match event {
            Event::AboutToWait => {
                window.request_redraw();
            }
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::RedrawRequested => {
                        color_buffer.clear(&gl);
                        shell.update();

                        gl_surface.swap_buffers(gl_context).unwrap();
                    }
                    WindowEvent::Resized(physical_size) => {
                        viewport
                            .update_size(physical_size.width as i32, physical_size.height as i32);
                        viewport.set_used(&gl);

                        gl_surface.resize(
                            gl_context,
                            NonZeroU32::new(physical_size.width).unwrap(),
                            NonZeroU32::new(physical_size.height).unwrap(),
                        );

                        shell.handle_event(shell::Event::Resized(
                            physical_size.width,
                            physical_size.height,
                        ));
                        window.request_redraw();
                    }
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::KeyboardInput {
                        device_id: _,
                        event:
                            KeyEvent {
                                state: ElementState::Pressed,
                                logical_key: keyboard::Key::Character(c),
                                ..
                            },
                        is_synthetic: _,
                    } => {
                        println!("{:#?}", c);

                        let c = c.chars().nth(0);
                        match c {
                            Some('\u{8}' | '\r') => (), //backspace
                            None => (),
                            _ => {
                                shell.handle_event(shell::Event::ReceivedCharacter(c.unwrap()));
                                // window.request_redraw();
                            }
                        }
                    }
                    WindowEvent::KeyboardInput {
                        device_id: _,
                        event:
                            KeyEvent {
                                state: ElementState::Pressed,
                                logical_key: keyboard::Key::Named(keycode),
                                ..
                            },
                        is_synthetic: _,
                    } => {
                        match keycode {
                            keyboard::NamedKey::Backspace => {
                                shell.handle_event(shell::Event::Backspace);
                                window.request_redraw();
                            }

                            keyboard::NamedKey::Space => {
                                shell.handle_event(shell::Event::ReceivedCharacter(' '));
                                window.request_redraw();
                            }
                            keyboard::NamedKey::Enter => {
                                shell.handle_event(shell::Event::Enter);
                                window.request_redraw();
                            }
                            keyboard::NamedKey::ArrowLeft => {
                                shell.handle_event(shell::Event::Left);
                                window.request_redraw();
                            }
                            keyboard::NamedKey::ArrowRight => {
                                shell.handle_event(shell::Event::Right);
                                window.request_redraw();
                            }
                            keyboard::NamedKey::ArrowUp => {
                                shell.handle_event(shell::Event::Previous);
                                window.request_redraw();
                            }
                            keyboard::NamedKey::ArrowDown => {
                                shell.handle_event(shell::Event::Next);
                                window.request_redraw();
                            }
                            _ => (),
                        };
                    }
                    WindowEvent::KeyboardInput {
                        device_id: _,
                        event:
                            KeyEvent {
                                state: ElementState::Released,
                                // virtual_keycode: Some(keycode),
                                ..
                            },
                        is_synthetic: _,
                    } => {
                        shell.handle_event(shell::Event::Release);
                        window.request_redraw();
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    })?;
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
    }
}
