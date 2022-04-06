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
            coords: [0,0]
        };
        for i in 0..10{
            for j in 0..19{
                if self.blocks[i][j] != 10{
                    block.color = self.blocks[i][j] as usize;
                    block.coords = [i as u8,j as u8];
                    block.draw_grid(can,blockt);
                }

            }
        }
    }
    
    pub fn is_taken(&self, x:i16, y:i16) -> bool {
        if x>9 || y>18 || x<0 || y<0{ return true; }
        return self.blocks[x as usize][y as usize] < 10;
    }

    pub fn add(&mut self, x: usize, y: usize, color:u8) -> bool {
        if self.blocks[x][y] != 10 { return false; }
        self.blocks[x][y] = color;
        return true;

    }

    fn remove_line(&mut self, i:usize){
        for j in 0..i-1{
            //Ligne suivante = ligne actuelle au dessus de la ligne à supprimer (on décale)
            for k in 0..10{
                self.blocks[k][i-j] = self.blocks[k][i-j-1];
            }
        }
        //On vide la ligne 1
        for k in 0..10{
            self.blocks[k][0] = 10;
        }
    }

    pub fn verify_lines(&mut self, sum:i32) -> i32 {
        for i in 0..19{
            if self.line_full(i){
                self.remove_line(i);
                return self.verify_lines(sum+1);
            }
        }
        return sum;
    }

    fn line_full(&mut self, i:usize) -> bool {
        for j in 0..10{
            if self.blocks[j][i] == 10 { return false; }
        }
        return true;
    }

}