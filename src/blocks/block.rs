use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::Rect;
use std::borrow::Borrow;


const BCOLORS:[[u8; 3]; 7] = [
    [255,255,255],
    [255,0,255],
    [255,255,0],
    [0,255,0],
    [0,0,255],
    [0,255,255],
    [255, 128, 128]
];

const DECAL_X: i32 = 120;
const DECAL_Y: i32 = 3;



pub struct Block<T>{
    pub(crate) rect: Rect,
    pub(crate) color: usize,
    pub(crate) coords: T
}

pub fn init_textures(blocks_texture: &mut [Texture; 7]){
    for i in 0..7{
        blocks_texture[i].set_color_mod(BCOLORS[i][0], BCOLORS[i][1], BCOLORS[i][2])
    }
}

impl<T> Block<T>{


    pub fn draw(&self, can:&mut WindowCanvas, blockt:&mut [Texture; 7]){
            can.copy(blockt[self.color].borrow(), None, self.rect).unwrap();
    }

    /*pub fn set_x(&mut self, x:i32){
        self.rect.set_x(x);
    } // ! met à jour l'abscisse réelle, pas celle dans la grille

    pub fn get_x(&mut self, x:i32){
        self.rect.set_x(x);
    }

    pub fn set_y(&mut self, y:i32){
        self.rect.set_y(y);
    }

    pub fn get_y(&mut self, y:i32){
        self.rect.set_y(y);
    }*/
}

impl Block<[u8;2]>{
    pub fn draw_grid(&mut self, can:&mut WindowCanvas, blockt:&mut [Texture; 7]){
        self.rect.set_x(self.coords[0] as i32 * 28 + DECAL_X );
        self.rect.set_y(self.coords[1] as i32 * 28 + DECAL_Y );
        can.copy(blockt[self.color].borrow(), None, self.rect).unwrap();
    }
}
