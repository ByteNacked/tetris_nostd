
#![cfg(test)]

use tetrislib::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::{Window, WindowContext};

use std::time::{SystemTime, UNIX_EPOCH};

use tetrislib as tetris;
use tetris::{BOARD_H, BOARD_W};

const SQUARE_SIZE: usize = 30;

#[test]
pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            "rust-sdl2 demo: Game of Life",
            (SQUARE_SIZE * BOARD_W) as u32,
            (SQUARE_SIZE * BOARD_H) as u32,
        )
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    println!("Using SDL_Renderer \"{}\"", canvas.info().name);
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Smth strange");
    let seed: u64 = since_the_epoch.as_millis() as u64;
    let mut game = tetris::Game::new(seed);

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        // get the inputs here
        let mut c = tetris::Control::default();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    c.fall = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    c.left = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    c.right = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    c.dash = true;
                }

                Event::MouseButtonDown {
                    x,
                    y,
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    //let x = (x as u32) / SQUARE_SIZE;
                    //let y = (y as u32) / SQUARE_SIZE;
                    //match game.get_mut(x as i32, y as i32) {
                    //    Some(square) => {*square = !(*square);},
                    //    None => unreachable!(),
                    //};
                }
                _ => {}
            }
        }

        game.control(&c);
        game.update();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for y in 0..BOARD_H {
            for x in 0..BOARD_W {
                let unit = game.get_draw_cell(x, y);
                /*if unit.dirty*/
                {
                    match unit.state {
                        tetris::CType::Empty => canvas.set_draw_color(Color::RGB(0, 0, 0)),
                        tetris::CType::Red => canvas.set_draw_color(Color::RGB(255, 0, 0)),
                        tetris::CType::Blue => canvas.set_draw_color(Color::RGB(0, 70, 180)),
                        tetris::CType::C1 => canvas.set_draw_color(Color::RGB(30, 220, 50)),
                        tetris::CType::C2 => canvas.set_draw_color(Color::RGB(255, 210, 0)),
                        tetris::CType::C3 => canvas.set_draw_color(Color::RGB(210, 210, 20)),
                        tetris::CType::C4 => canvas.set_draw_color(Color::RGB(0, 10, 210)),
                        _ => panic!("unsupported color"),
                    }

                    let x_calc: i32 = (x * SQUARE_SIZE as usize) as i32;
                    let y_calc: i32 = (y * SQUARE_SIZE as usize) as i32;
                    let _ = canvas.fill_rect(Rect::new(
                        x_calc,
                        y_calc,
                        SQUARE_SIZE as u32,
                        SQUARE_SIZE as u32,
                    ));
                    //unit.dirty = false;
                }
            }
        }

        canvas.present();
        std::thread::sleep(std::time::Duration::from_millis(15));
    }

    Ok(())
}
