use sdl2::render::{Texture, WindowCanvas};
use sdl2::rect::Rect;
use crate::blocks::block::Block;
use std::borrow::BorrowMut;


pub(crate) struct Piece{
    x: f32,
    y: f32,
    blocks: [Block<[u8;2]>;4] //Les blocs ont des coordonnées relatives
}

/*
 * T - 0
 * O - 1
 * I - 2
 * S - 3
 * Z - 4
 * L - 5
 * J - 6
 */
impl Piece{

    pub fn newActive(piece_type:u8) -> Piece{
        match piece_type {
            0 => return Piece {
                x: 4.0,
                y: 1.0,
                blocks:
                [
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 0,
                        posed: false,
                        coords: [4,0]
                    },
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 0,
                        posed: false,
                        coords: [4,1]
                    },
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 0,
                        posed: false,
                        coords: [4,2]
                    },
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 0,
                        posed: false,
                        coords: [5,1]
                    }
                ]
            },
            1 => return Piece {
                x: 4.5,
                y: 0.5,
                blocks:
                [
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 1,
                        posed: false,
                        coords: [4,0]
                    },
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 1,
                        posed: false,
                        coords: [4,1]
                    },
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 1,
                        posed: false,
                        coords: [5,0]
                    },
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 1,
                        posed: false,
                        coords: [5,1]
                    }
                ]
            },
            2 => return Piece {
                x: 4.5,
                y: 1.5,
                blocks:
                [
                    Block {
                        rect: Rect::new(0,0, 28, 28),
                        color: 2,
                        posed: false,
                        coords: [4,0]
                    },
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 2,
                        posed: false,
                        coords: [4,1]
                    },
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 2,
                        posed: false,
                        coords: [4,2]
                    },
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 2,
                        posed: false,
                        coords: [4,3]
                    }
                ]
            },
            3 => return Piece {
                x: 5.0,
                y: 1.0,
                blocks:
                [
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 3,
                        posed: false,
                        coords: [4,0]
                    },
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 3,
                        posed: false,
                        coords: [4,1]
                    },
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 3,
                        posed: false,
                        coords: [5,1]
                    },
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 3,
                        posed: false,
                        coords: [5,2]
                    }
                ]
            },
            4 => return Piece {
                x: 4.0,
                y: 1.0,
                blocks:
                [
                    Block {
                        rect: Rect::new(0,0, 28, 28),
                        color: 4,
                        posed: false,
                        coords: [5,0]
                    },
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 4,
                        posed: false,
                        coords: [5,1]
                    },
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 4,
                        posed: false,
                        coords: [4,1]
                    },
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 4,
                        posed: false,
                        coords: [4,2]
                    }
                ]
            },
            5 => return Piece {
                x: 4.0,
                y: 1.0,
                blocks:
                [
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 5,
                        posed: false,
                        coords: [4,0]
                    },
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 5,
                        posed: false,
                        coords: [4,1]
                    },
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 5,
                        posed: false,
                        coords: [4,2]
                    },
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 5,
                        posed: false,
                        coords: [5,2]
                    }
                ]
            },
            _ => return Piece {
                x: 5.0,
                y: 1.0,
                blocks:
                [
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 6,
                        posed: false,
                        coords: [5,0]
                    },
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 6,
                        posed: false,
                        coords: [5,1]
                    },
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 6,
                        posed: false,
                        coords: [5,2]
                    },
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 6,
                        posed: false,
                        coords: [4,2]
                    }
                ]
            }
        }
    }

    pub fn draw(&mut self, can:&mut WindowCanvas, blockt:&mut [Texture; 7]){
        for mut b in 0..3{
            self.blocks[b].draw_grid(can,blockt);
        }
    }

    pub fn go_down(&mut self){
        for mut b in 0..3{
            self.blocks[b].coords[1] += 1;
        }
        self.y += 1.0;
    }
}

