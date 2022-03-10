use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
use crate::Block;

pub(crate) struct Stack{
    blocks:[[u8;19];10]
}


impl Stack{
    pub fn init_stack() -> Stack{
        return Stack{
            // n=10 -> non occupé
            // n= [0-6] -> occupé par couleur n
            blocks:[
                [10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10],
                [10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10],
                [10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10],
                [10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10],
                [10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10],
                [10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10],
                [10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10],
                [10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10],
                [10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10],
                [10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10]
            ]
        }
    }

    pub fn draw(&self, can: &mut WindowCanvas, blockt: &mut [Texture; 7]) {
        let mut block:Block<[u8;2]> = Block{
            rect: Rect::new(0,0,28,28),
            color: 0,
            posed: true,
            coords: [0,0]
        };
        for mut i in 0..10{
            for mut j in 0..19{
                if (self.blocks[i][j] != 10){
                    block.color = self.blocks[i][j] as usize;
                    block.coords = [i as u8,j as u8];
                    block.draw_grid(can,blockt);
                }

            }
        }
    }
    
    pub fn isTaken(&self, x:i16, y:i16) -> bool {
        if (x>9 || y>18 || x<0 || y<0){ return true; }
        return self.blocks[x as usize][y as usize] < 10;
    }

    pub fn add(&mut self, x: usize, y: usize, color:u8){
        self.blocks[x][y] = color;
    }
}