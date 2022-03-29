extern crate sdl2;


mod blocks;
mod pieces;
mod music;

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
use rand::Rng;
use sdl2::{audio, TimerSubsystem};
use crate::blocks::stack::Stack;
use crate::music::play_music;
use sdl2::mixer::Music;

pub fn main() {
    //Initialisation
    let sdl_context = sdl2::init().unwrap();

    let mut music:Option<Music> = None;
    play_music(&sdl_context, &mut music);
    music.unwrap().play(2).unwrap();

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

    let mut lastact:i32 = 0;

    let mut rng = rand::thread_rng(); // Random generator
    let mut currentpiece:Piece = Piece::new_active(rng.gen_range(0, 7));
    let mut blockstack:Stack = Stack::init_stack();

    let mut rotating:bool = false;
    let mut goleft:bool = false;
    let mut goright:bool = false;
    let mut godown:bool = false;

    let mut timer:TimerSubsystem = sdl_context.timer().unwrap();


    'running: loop {
        //Boucle du jeu

        rotating = false;
        goright = false;
        goleft = false;
        godown = false;


        canvas.clear();
        canvas.copy(&bg,None, None).unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown {keycode: Some(Keycode::Space),..} =>{
                    rotating = true;
                },
                Event::KeyDown {keycode: Some(Keycode::Left), .. } =>{
                    goleft = true;
                },
                Event::KeyDown {keycode: Some(Keycode::Right), .. } =>{
                    goright = true;
                },
                Event::KeyDown {keycode: Some(Keycode::Down), .. } =>{
                    godown = true;
                }
                _ => {}
            }
        }
        // Le reste de la boucle

        if (goright){ currentpiece.move_right(&blockstack); }
        if (goleft){ currentpiece.move_left(&blockstack); }

        if (rotating){ currentpiece.rotate(&blockstack,true); }

        if (godown){
            if (!currentpiece.go_down(&blockstack)){
                currentpiece.pose(&mut blockstack);
                currentpiece = Piece::new_active(rng.gen_range(0, 7))
            }
        }

        lastact += 1;
        if (lastact>=29) {
            lastact = 0;
            //Toutes les 0.5s (on descend les pièces)
            if (!currentpiece.go_down(&blockstack)){
                currentpiece.pose(&mut blockstack);
                currentpiece = Piece::new_active(rng.gen_range(0, 7))
            }
        }

        currentpiece.draw(&mut canvas, &mut blocks_t);

        blockstack.draw(&mut canvas, &mut blocks_t);
        canvas.present();
        timer.delay(17);//16,6 = 1000/60 donc 60FPS
    }
}


