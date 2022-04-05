use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
use crate::Block;

pub fn offgrid_piece(typepiece:usize, decalx:i32, decaly:i32) -> [Block<()>; 4] {
    /*
     * T - 0
     * O - 1
     * I - 2
     * S - 3
     * Z - 4
     * L - 5
     * J - 6
     */
    return match typepiece {
        0 => [
            Block{
                rect: Rect::new(decalx+14,decaly,14,14),
                color: typepiece,
                coords: ()
            },
            Block{
                rect: Rect::new(decalx,decaly,14,14),
                color: typepiece,
                coords: ()
            },Block{
                rect: Rect::new(decalx+14,decaly+14,14,14),
                color: typepiece,
                coords: ()
            },
            Block{
                rect: Rect::new(decalx+28,decaly,14,14),
                color: typepiece,
                coords: ()
            }
        ],
        1 => [
            Block{
                rect: Rect::new(decalx,decaly,14,14),
                color: typepiece,
                coords: ()
            },
            Block{
                rect: Rect::new(decalx+14,decaly,14,14),
                color: typepiece,
                coords: ()
            },Block{
                rect: Rect::new(decalx,decaly+14,14,14),
                color: typepiece,
                coords: ()
            },
            Block{
                rect: Rect::new(decalx+14,decaly+14,14,14),
                color: typepiece,
                coords: ()
            }
        ],
        2 => [
            Block{
                rect: Rect::new(decalx,decaly,14,14),
                color: typepiece,
                coords: ()
            },
            Block{
                rect: Rect::new(decalx+14,decaly,14,14),
                color: typepiece,
                coords: ()
            },Block{
                rect: Rect::new(decalx+28,decaly,14,14),
                color: typepiece,
                coords: ()
            },
            Block{
                rect: Rect::new(decalx+42,decaly,14,14),
                color: typepiece,
                coords: ()
            }
        ],
        3 => [
            Block{
                rect: Rect::new(decalx+28,decaly,14,14),
                color: typepiece,
                coords: ()
            },
            Block{
                rect: Rect::new(decalx+14,decaly,14,14),
                color: typepiece,
                coords: ()
            },Block{
                rect: Rect::new(decalx,decaly+14,14,14),
                color: typepiece,
                coords: ()
            },
            Block{
                rect: Rect::new(decalx+14,decaly+14,14,14),
                color: typepiece,
                coords: ()
            }
        ],
        4 => [
            Block{
                rect: Rect::new(decalx,decaly,14,14),
                color: typepiece,
                coords: ()
            },
            Block{
                rect: Rect::new(decalx+14,decaly,14,14),
                color: typepiece,
                coords: ()
            },Block{
                rect: Rect::new(decalx+28,decaly+14,14,14),
                color: typepiece,
                coords: ()
            },
            Block{
                rect: Rect::new(decalx+14,decaly+14,14,14),
                color: typepiece,
                coords: ()
            }
        ],
        5 => [
            Block{
                rect: Rect::new(decalx,decaly+14,14,14),
                color: typepiece,
                coords: ()
            },
            Block{
                rect: Rect::new(decalx+28,decaly,14,14),
                color: typepiece,
                coords: ()
            },Block{
                rect: Rect::new(decalx+28,decaly+14,14,14),
                color: typepiece,
                coords: ()
            },
            Block{
                rect: Rect::new(decalx+14,decaly+14,14,14),
                color: typepiece,
                coords: ()
            }
        ],
        _ => [
            Block{
                rect: Rect::new(decalx,decaly,14,14),
                color: typepiece,
                coords: ()
            },
            Block{
                rect: Rect::new(decalx+28,decaly+14,14,14),
                color: typepiece,
                coords: ()
            },Block{
                rect: Rect::new(decalx+28,decaly,14,14),
                color: typepiece,
                coords: ()
            },
            Block{
                rect: Rect::new(decalx+14,decaly,14,14),
                color: typepiece,
                coords: ()
            }
        ]
    }

}

pub fn hold_draw(piecetype:usize,can:&mut WindowCanvas, blockt:&mut [Texture; 7]){
    if piecetype >= 7 {return}
    for b in offgrid_piece(piecetype,30,50){
        b.draw(can,blockt);
    }
}


pub fn next_draw(piecetypes: [u8; 4], can:&mut WindowCanvas, blockt:&mut [Texture; 7]){
    for b in offgrid_piece(piecetypes[0] as usize, 480, 45){
        b.draw(can,blockt);
    }
    for b in offgrid_piece(piecetypes[1] as usize, 450, 125){
        b.draw(can,blockt);
    }
    for b in offgrid_piece(piecetypes[2] as usize, 550, 120){
        b.draw(can,blockt);
    }
    for b in offgrid_piece(piecetypes[3] as usize, 500, 275){
        b.draw(can,blockt);
    }
}