extern crate sdl2;


mod blocks;
mod pieces;

use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::surface::Surface;
use sdl2::render::{Texture, WindowCanvas, TextureCreator};
use std::borrow::{Borrow, BorrowMut};
use sdl2::video::WindowContext;
use blocks::block::Block;
use pieces::piece::Piece;
use sdl2::rect::Rect;

pub fn main() {

    //Initialisation
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    //Initialisation fenêtre
    let window = video_subsystem.window("Ugly Tetris", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();


    //LOAD TEXTURES
    let mut tcreator = canvas.texture_creator();
    // - background
    let mut bg_s = Surface::load_bmp("./res/bg.bmp").unwrap();
    let bg = bg_s.as_texture(tcreator.borrow()).unwrap();

    /*
     - Chargement de la texture "block"
     */
    let block_s = Surface::load_bmp("./res/block.bmp").unwrap();

    let mut blocks_t:[Texture; 7] = [
        block_s.as_texture(tcreator.borrow()).unwrap(),
        block_s.as_texture(tcreator.borrow()).unwrap(),
        block_s.as_texture(tcreator.borrow()).unwrap(),
        block_s.as_texture(tcreator.borrow()).unwrap(),
        block_s.as_texture(tcreator.borrow()).unwrap(),
        block_s.as_texture(tcreator.borrow()).unwrap(),
        block_s.as_texture(tcreator.borrow()).unwrap()
    ];

    blocks::block::init_textures(&mut blocks_t);

    let mut lastact = 0;


    let mut testpiece = Piece::newActive(0);



    'running: loop {
        //Boucle du jeu
        canvas.clear();
        canvas.copy(&bg,None, None).unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // Le reste de la boucle
        lastact += 1;
        if (lastact>=29) {
            lastact = 0;
            //Toutes les 0.5s (on descend les pièces)
            testpiece.go_down();
        }
        testpiece.draw(&mut canvas, &mut blocks_t);
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));// 60 FPS
    }
}

