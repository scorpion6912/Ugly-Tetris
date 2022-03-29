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
use pieces::piece::{Piece,PieceGen};
use sdl2::rect::Rect;
use sdl2::{audio, TimerSubsystem};
use crate::blocks::stack::Stack;
use crate::music::play_music;
use sdl2::mixer::Music;
use crate::pieces::offgrid::{hold_draw, next_draw};

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
    let tcreator = canvas.texture_creator();
    // - background
    let bg_s = Surface::load_bmp("./res/bg.bmp").unwrap();
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


    let mut blockstack:Stack = Stack::init_stack();

    //Evenements (input utilisateur)
    let mut rotatingright:bool = false;//Rotation horaire
    let mut rotatingleft:bool = false;//Rotation anti-horaire
    let mut goleft:bool = false;
    let mut goright:bool = false;
    let mut godown:bool = false;//Aller en bas (lentement)
    let mut rushdown:bool = false;//Aller en bas d'un coup
    let mut switching:bool = false;//Inverser avec la piece "Hold"


    let mut piecegen = PieceGen::new();
    let mut currentpiece = Piece::new_active(piecegen.next_piece_nb());

    let mut switchable = true;
    let mut switchtype = 10;

    let mut timer:TimerSubsystem = sdl_context.timer().unwrap();

    let _audio = sdl_context.audio().unwrap();
    let _music_player = play_music(&sdl_context);

    'running: loop {
        //Boucle du jeu

        rotatingright = false;
        rotatingleft = false;
        goright = false;
        goleft = false;
        godown = false;
        rushdown = false;
        switching = false;


        canvas.clear();
        canvas.copy(&bg,None, None).unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown {keycode: Some(Keycode::Space),..} =>{
                    rotatingright = true;
                },
                Event::KeyDown {keycode: Some(Keycode::X),..} =>{
                    rotatingleft = true;
                },
                Event::KeyDown {keycode: Some(Keycode::Left), .. } =>{
                    goleft = true;
                },
                Event::KeyDown {keycode: Some(Keycode::Right), .. } =>{
                    goright = true;
                },
                Event::KeyDown {keycode: Some(Keycode::Down), .. } =>{
                    godown = true;
                },
                Event::KeyDown {keycode: Some(Keycode::Up), .. } =>{
                    rushdown = true;
                },
                Event::KeyDown {keycode: Some(Keycode::C), .. } =>{
                    switching = true;
                }
                _ => {}
            }
        }
        // Le reste de la boucle

        if goright{ currentpiece.move_right(&blockstack); }
        if goleft{ currentpiece.move_left(&blockstack); }

        if rotatingright{ currentpiece.rotate(&blockstack,true); }
        if rotatingleft{ currentpiece.rotate(&blockstack,false); }


        if godown{
            if !currentpiece.go_down(&blockstack){
                currentpiece.pose(&mut blockstack);
                currentpiece = Piece::new_active(piecegen.next_piece_nb());
                switchable = true;
            }
        }

        if rushdown{
            while currentpiece.go_down(&blockstack){ ; };
            currentpiece.pose(&mut blockstack);
            currentpiece = Piece::new_active(piecegen.next_piece_nb());
            switchable = true;
        }

        if (switchable && switching){
            //On inverse le hold et la currentpiece
            switchable = false;
            let tmptype = switchtype;
            switchtype = currentpiece.get_type();
            if tmptype == 10{
                currentpiece = Piece::new_active(piecegen.next_piece_nb())
            }else{
                currentpiece = Piece::new_active(tmptype)
            }
        }

        lastact += 1;
        if lastact>=29 {
            lastact = 0;
            //Toutes les 0.5s (on descend les pièces)
            if !currentpiece.go_down(&blockstack){
                currentpiece.pose(&mut blockstack);
                currentpiece = Piece::new_active(piecegen.next_piece_nb());
                switchable = true;
            }
        }


        currentpiece.draw(&mut canvas, &mut blocks_t);

        hold_draw(switchtype as usize, &mut canvas, &mut blocks_t);
        next_draw(piecegen.next,&mut canvas, &mut blocks_t);

        blockstack.verify_lines();
        blockstack.draw(&mut canvas, &mut blocks_t);
        canvas.present();

        timer.delay(17);//16,6 = 1000/60 donc 60FPS
    }
}


