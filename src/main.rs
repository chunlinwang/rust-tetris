#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_patterns)]

#[macro_use]
extern crate conrod_core;
extern crate conrod_glium;
#[macro_use]
extern crate conrod_winit;
extern crate find_folder;
extern crate glium;
extern crate image;

use glium::Surface;
use conrod_core::{Borderable, Dimensions, Labelable, Sizeable, Positionable, Widget};
use crate::schema::{Tetris, MoveAction};
use crate::constants::{APP_NAME, TETRIS_CONTAINER_HEIGHT, TETRIS_CONTAINER_WIDTH, SCREEN_WIDTH, SCREEN_HEIGHT, DROP_INTERVAL_LEVEL, APP_AUTHOR, MATRIX_WIDTH, ADJUST_MARGIN, MATRIX_HEIGHT, RUSH_DOWN, MIN_LEVEL, MAX_LEVEL};
use crate::game_controller::{draw_play_ground, check_clear_line, move_action, game_over_checker};
use conrod_core::widget::matrix::Element;

mod constants;
mod game_controller;
mod support;
mod schema;

fn main() {
    // Build the window.
    let event_loop = glium::glutin::event_loop::EventLoop::new();
    let window = glium::glutin::window::WindowBuilder::new()
        .with_resizable(false)
        .with_title(APP_NAME)
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(SCREEN_WIDTH, SCREEN_HEIGHT));
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &event_loop).unwrap();

    // construct our `Ui`.
    let mut ui = conrod_core::UiBuilder::new([SCREEN_WIDTH as f64, SCREEN_HEIGHT as f64]).build();

    // Add a `Font` to the `Ui`'s `font::Map` from file.
    let assets = find_folder::Search::KidsThenParents(3, 5)
        .for_folder("assets")
        .unwrap();
    let font_path = assets.join("fonts/OpenSansEmoji.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();

    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = conrod_core::image::Map::<glium::texture::Texture2d>::new();

    // A type used for converting `conrod_core::render::Primitives` into `Command`s that can be used
    // for drawing to the glium `Surface`.
    let mut renderer = conrod_glium::Renderer::new(&display).unwrap();

    // Instantiate the generated list of widget identifiers.
    let mut ids = Ids::new(ui.widget_id_generator());

    let mut tetris = Tetris::new();

    // Poll events from the window.
    support::run_loop(display, event_loop, move |request, display| {
        match request {
            support::Request::Event {
                event,
                should_update_ui,
                should_exit,
                drop_interval,
            } => {
                // Use the `winit` backend feature to convert the winit event to a conrod one.
                if let Some(event) = support::convert_event(&event, &display.gl_window().window()) {
                    ui.handle_event(event);
                    *should_update_ui = true;
                }

                match event {
                    glium::glutin::event::Event::WindowEvent { event, .. } => match event {
                        // Break from the loop upon `Escape`.
                        glium::glutin::event::WindowEvent::CloseRequested
                        | glium::glutin::event::WindowEvent::KeyboardInput {
                            input:
                            glium::glutin::event::KeyboardInput {
                                virtual_keycode:
                                Some(glium::glutin::event::VirtualKeyCode::Escape),
                                ..
                            },
                            ..
                        } => *should_exit = true,
                        glium::glutin::event::WindowEvent::KeyboardInput {
                            input:
                            glium::glutin::event::KeyboardInput {
                                virtual_keycode: Some(glium::glutin::event::VirtualKeyCode::Up),
                                state: glium::glutin::event::ElementState::Pressed,
                                ..
                            },
                            ..
                        } => {
                            if tetris.block.block_type != "O" {
                                if tetris.pos_y < 0 {
                                    tetris.pos_y = 0;
                                }

                                if tetris.pos_y as usize + MATRIX_WIDTH > TETRIS_CONTAINER_WIDTH {
                                    if tetris.block.block_type == "I" {
                                        tetris.pos_y = (TETRIS_CONTAINER_WIDTH - MATRIX_WIDTH) as isize;
                                    } else {
                                        tetris.pos_y = (TETRIS_CONTAINER_WIDTH - ADJUST_MARGIN) as isize;
                                    }
                                }
                            }

                            tetris.direction = (tetris.direction + 1) % 4;

                            tetris.reset_speed();
                            *drop_interval = tetris.speed;

                            *should_update_ui = true;
                        }
                        glium::glutin::event::WindowEvent::KeyboardInput {
                            input:
                            glium::glutin::event::KeyboardInput {
                                virtual_keycode: Some(glium::glutin::event::VirtualKeyCode::Left),
                                state: glium::glutin::event::ElementState::Pressed,
                                ..
                            },
                            ..
                        } => {
                            move_action(&mut tetris, MoveAction::Left);

                            tetris.reset_speed();
                            *drop_interval = tetris.speed;

                            *should_update_ui = true;
                        }
                        glium::glutin::event::WindowEvent::KeyboardInput {
                            input:
                            glium::glutin::event::KeyboardInput {
                                virtual_keycode: Some(glium::glutin::event::VirtualKeyCode::Right),
                                state: glium::glutin::event::ElementState::Pressed,
                                ..
                            },
                            ..
                        } => {
                            move_action(&mut tetris, MoveAction::Right);

                            tetris.reset_speed();
                            *drop_interval = tetris.speed;

                            *should_update_ui = true;
                        }
                        glium::glutin::event::WindowEvent::KeyboardInput {
                            input:
                            glium::glutin::event::KeyboardInput {
                                virtual_keycode: Some(glium::glutin::event::VirtualKeyCode::Down),
                                state: glium::glutin::event::ElementState::Pressed,
                                ..
                            },
                            ..
                        } => {
                            tetris.speed = RUSH_DOWN;

                            *drop_interval = RUSH_DOWN;

                            *should_update_ui = true;
                        }
                        glium::glutin::event::WindowEvent::KeyboardInput {
                            input:
                            glium::glutin::event::KeyboardInput {
                                virtual_keycode: Some(glium::glutin::event::VirtualKeyCode::R),
                                state: glium::glutin::event::ElementState::Pressed,
                                ..
                            },
                            ..
                        } => {
                            tetris = Tetris::new();

                            *should_update_ui = true;
                        }
                        glium::glutin::event::WindowEvent::KeyboardInput {
                            input:
                            glium::glutin::event::KeyboardInput {
                                virtual_keycode: Some(glium::glutin::event::VirtualKeyCode::P),
                                state: glium::glutin::event::ElementState::Pressed,
                                ..
                            },
                            ..
                        } => {
                            tetris.increase_level();

                            draw_play_ground(&mut tetris);

                            *should_update_ui = true;
                        }
                        glium::glutin::event::WindowEvent::KeyboardInput {
                            input:
                            glium::glutin::event::KeyboardInput {
                                virtual_keycode: Some(glium::glutin::event::VirtualKeyCode::M),
                                state: glium::glutin::event::ElementState::Pressed,
                                ..
                            },
                            ..
                        } => {
                            tetris.decrease_level();
                            draw_play_ground(&mut tetris);
                            *should_update_ui = true;
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
            support::Request::SetUi { needs_redraw, drop_interval } => {
                if tetris.is_game_over == false {
                    // down automatically
                    if move_action(&mut tetris, MoveAction::Down) == false {
                        check_clear_line(&mut tetris);
                        tetris.create_new_block();
                        *drop_interval = tetris.speed;

                        if tetris.round == 0 {
                            game_over_checker(&mut tetris);
                        }
                    }

                    if tetris.is_game_over == false {
                        draw_play_ground(&mut tetris);
                    }

                    // Instantiate all widgets in the GUI.
                    set_widgets(ui.set_widgets(), &mut ids, &tetris);

                    *needs_redraw = ui.has_changed();
                }
            }
            support::Request::Redraw => {
                // Render the `Ui` and then display it on the screen.
                let primitives = ui.draw();

                renderer.fill(display, primitives, &image_map);
                let mut target = display.draw();
                target.clear_color(0.0, 0.0, 0.0, 1.0);
                renderer.draw(display, &mut target, &image_map).unwrap();
                target.finish().unwrap();
            }
        }
    })
}

// Draw the Ui.
fn set_widgets(ref mut ui: conrod_core::UiCell, ids: &mut Ids, tetris: &Tetris) {
    use conrod_core::{color, widget, Colorable, Positionable, Sizeable, Widget};

    let back_ground: [[usize; TETRIS_CONTAINER_HEIGHT]; TETRIS_CONTAINER_WIDTH] = tetris.play_ground;

    // Construct our main `Canvas` tree.
    widget::Canvas::new()
        .flow_down(&[
            (
                ids.header,
                widget::Canvas::new().color(color::BLUE).length_weight(0.1),
            ),
            (
                ids.body,
                widget::Canvas::new().flow_right(&[
                    (
                        ids.back_ground,
                        widget::Canvas::new().color(color::LIGHT_YELLOW).length_weight(2.0),
                    ),
                    (
                        ids.game_panel,
                        widget::Canvas::new().color(color::BLUE).flow_down(&[
                            (
                                ids.level_panel,
                                widget::Canvas::new().color(color::LIGHT_BLUE).title_bar("Level").title_bar_color(color::LIGHT_ORANGE),
                            ),
                            (
                                ids.score_panel,
                                widget::Canvas::new().color(color::LIGHT_BLUE).title_bar("Score").title_bar_color(color::LIGHT_ORANGE),
                            ),
                            (
                                ids.next_block_panel,
                                widget::Canvas::new().color(color::DARK_BLUE).title_bar("Next").title_bar_color(color::LIGHT_ORANGE),
                            ),
                            (
                                ids.blocks,
                                widget::Canvas::new().color(color::DARK_BLUE).title_bar("Blocks").title_bar_color(color::LIGHT_ORANGE),
                            ),
                        ]),
                    ),
                ]),
            ),
            (
                ids.footer,
                widget::Canvas::new().color(color::BLUE).length_weight(0.1),
            ),
        ])
        .set(ids.master, ui);

    if tetris.is_game_over {
        let floating = widget::Canvas::new()
            .floating(true)
            .w_h(200.0, 300.0)
            .label_color(color::WHITE);
        floating
            .middle_of(ids.body)
            .title_bar("GAME OVER 😖")
            .color(color::LIGHT_ORANGE.with_luminance(0.5))
            .set(ids.floating_game_over, ui);
    }

    widget::Text::new("Press R to retry")
        .color(color::WHITE)
        .font_size(20)
        .middle_of(ids.floating_game_over)
        .set(ids.retry, ui);

    widget::Text::new(APP_AUTHOR)
        .color(color::LIGHT_ORANGE.with_luminance(0.5))
        .font_size(12)
        .middle_of(ids.footer)
        .set(ids.author, ui);

    widget::Text::new(APP_NAME)
        .color(color::LIGHT_ORANGE)
        .font_size(24)
        .middle_of(ids.header)
        .set(ids.title, ui);

    widget::Text::new(tetris.level.to_string().as_str())
        .color(color::LIGHT_ORANGE)
        .font_size(24)
        .middle_of(ids.level_panel)
        .set(ids.level, ui);

    widget::Text::new(tetris.score.to_string().as_str())
        .color(color::LIGHT_ORANGE)
        .font_size(24)
        .middle_of(ids.score_panel)
        .set(ids.score, ui);

    widget::Text::new(tetris.blocks.to_string().as_str())
        .color(color::LIGHT_ORANGE)
        .font_size(24)
        .middle_of(ids.blocks)
        .set(ids.num_block, ui);

    let back_ground_wh = ui.wh_of(ids.back_ground).unwrap();
    let mut elements = widget::Matrix::new(TETRIS_CONTAINER_WIDTH, TETRIS_CONTAINER_HEIGHT)
        .w_h(back_ground_wh[0], back_ground_wh[1])
        .mid_top_of(ids.back_ground)
        .set(ids.matrix, ui);

    let field_w = back_ground_wh[0] / TETRIS_CONTAINER_WIDTH as f64;
    let field_h = back_ground_wh[1] / TETRIS_CONTAINER_HEIGHT as f64;
    let field_dimension: Dimensions = [field_w, field_h];

    let next_block = tetris.next_block.matrix_data[tetris.direction];
    let mut next_block_matrix = widget::Matrix::new(MATRIX_WIDTH, MATRIX_HEIGHT)
        .w_h(field_w * MATRIX_WIDTH as f64, field_h * MATRIX_HEIGHT as f64)
        .middle_of(ids.next_block_panel)
        .set(ids.next_block, ui);

    while let Some(mut elem) = next_block_matrix.next(ui) {
        let field = next_block.data[elem.row][elem.col];
        update_block(ui, &mut elem, field, field_dimension);
    }

    while let Some(mut elem) = elements.next(ui) {
        let field = back_ground[elem.col][elem.row];
        update_block(ui, &mut elem, field, field_dimension);
    }

    fn update_block(ui: &mut conrod_core::UiCell, elem: &mut Element, field: usize, field_dimension: Dimensions) {
        match field {
            0 => {
                elem.set(widget::BorderedRectangle::new(field_dimension).color(color::LIGHT_YELLOW.with_luminance(0.6))
                             .border(0.2).border_color(color::LIGHT_BROWN), ui);
            }
            1 => {
                elem.set(widget::BorderedRectangle::new(field_dimension).color(color::PURPLE.with_luminance(0.6))
                             .border(0.5).border_color(color::WHITE), ui);
            }
            _ => panic!("Error to generate back ground."),
        };
    }
}

// Generate a unique `WidgetId` for each widget.
widget_ids! {
    struct Ids {
        master,
        back_ground,
        matrix,
        game_panel,
        score,
        score_panel,
        next_block_panel,
        next_block,
        blocks,
        num_block,
        header,
        body,
        footer,
        title,
        author,
        block,
        floating_game_over,
        retry,
        level_panel,
        level,
    }
}