#![allow(dead_code)]

use glium::{glutin::{event, event_loop}, Display};
use crate::constants::{DROP_INTERVAL_LEVEL, BASE_INTERVAL};

pub enum Request<'a, 'b: 'a> {
    Event {
        event: &'a event::Event<'b, ()>,
        should_update_ui: &'a mut bool,
        should_exit: &'a mut bool,
        drop_interval: &'a mut usize,
    },
    SetUi {
        needs_redraw: &'a mut bool,
        drop_interval: &'a mut usize,
    },
    Redraw,
}

/// In most of the examples the `glutin` crate is used for providing the window context and
/// events while the `glium` crate is used for displaying `conrod_core::render::Primitives` to the
/// screen.
///
/// This function simplifies some of the boilerplate involved in limiting the redraw rate in the
/// glutin+glium event loop.
pub fn run_loop<F>(display: Display, event_loop: event_loop::EventLoop<()>, mut callback: F) -> !
    where
        F: 'static + FnMut(Request, &Display),
{
    let mut drop_interval: usize = BASE_INTERVAL + (DROP_INTERVAL_LEVEL / 5);
    let drop_interval_ms = std::time::Duration::from_millis(drop_interval as u64);
    let mut next_update = None;
    let mut ui_update_needed = false;
    event_loop.run(move |event, _, control_flow| {
        {
            let mut should_update_ui = false;
            let mut should_exit = false;
            callback(
                Request::Event {
                    event: &event,
                    should_update_ui: &mut should_update_ui,
                    should_exit: &mut should_exit,
                    drop_interval: &mut drop_interval,
                },
                &display,
            );
            ui_update_needed |= should_update_ui;
            if should_exit {
                *control_flow = event_loop::ControlFlow::Exit;
                return;
            }
        }

        // We don't want to draw any faster than 60 FPS, so set the UI only on every 16ms, unless:
        // - this is the very first event, or
        // - we didn't request update on the last event and new events have arrived since then.
        let should_set_ui_on_main_events_cleared = next_update.is_none() && ui_update_needed;
        match (&event, should_set_ui_on_main_events_cleared) {
            (event::Event::WindowEvent {
                event: glium::glutin::event::WindowEvent::KeyboardInput {
                    input:
                    glium::glutin::event::KeyboardInput {
                        virtual_keycode: Some(glium::glutin::event::VirtualKeyCode::Pause),
                        state: glium::glutin::event::ElementState::Pressed,
                        ..
                    },
                    ..
                }, ..
            }, _) => {
                if next_update.is_some() {
                    next_update = None;
                } else {
                    next_update = Some(std::time::Instant::now() + drop_interval_ms);
                }
            }
            (event::Event::WindowEvent {
                event: glium::glutin::event::WindowEvent::KeyboardInput {
                    input:
                    glium::glutin::event::KeyboardInput {
                        virtual_keycode: Some(glium::glutin::event::VirtualKeyCode::R),
                        state: glium::glutin::event::ElementState::Pressed,
                        ..
                    },
                    ..
                }, ..
            }, _) => {
                next_update = Some(std::time::Instant::now() + drop_interval_ms);
            }
            (event::Event::NewEvents(event::StartCause::Init { .. }), _)
            | (event::Event::NewEvents(event::StartCause::ResumeTimeReached { .. }), _)
            //| (event::Event::MainEventsCleared, true)
            => {
                let drop_interval_ms = std::time::Duration::from_millis(drop_interval as u64);
                next_update = Some(std::time::Instant::now() + drop_interval_ms);
                ui_update_needed = false;

                let mut needs_redraw = false;
                callback(
                    Request::SetUi {
                        needs_redraw: &mut needs_redraw,
                        drop_interval: &mut drop_interval,
                    },
                    &display,
                );
                if needs_redraw {
                    display.gl_window().window().request_redraw();
                } else {
                    // We don't need to redraw anymore until more events arrives.
                    next_update = None;
                }
            }
            _ => {}
        }
        if let Some(next_update) = next_update {
            *control_flow = event_loop::ControlFlow::WaitUntil(next_update);
        } else {
            *control_flow = event_loop::ControlFlow::Wait;
        }

        // Request redraw if needed.
        match &event {
            event::Event::RedrawRequested(_) => {
                callback(Request::Redraw, &display);
            }
            _ => {}
        }
    })
}

// Conversion functions for converting between types from glium's version of `winit` and
// `conrod_core`.
conrod_winit::v023_conversion_fns!();
