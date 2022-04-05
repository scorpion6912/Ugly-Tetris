use sdl2::render::{Texture, WindowCanvas};
use sdl2::rect::Rect;
use crate::blocks::block::Block;
use crate::blocks::stack::Stack;
use rand::Rng;

pub(crate) struct PieceGen{
    pub(crate) next: [u8;4],
    remain: [u8;7]//Pieces restantes à tirer, 10 est considéré comme vide, sinon c'est entre 0 et 6 (incluse)
}

impl PieceGen{
    pub fn next_piece_nb(&mut self) -> u8{

        let to_return = self.next[0];
        self.next[0] = self.next[1];
        self.next[1] = self.next[2];
        self.next[2] = self.next[3];

        self.next[3] = self.gen_piece_nb();

        return to_return;
    }

    pub fn new() -> PieceGen{
        let mut piecegen = PieceGen{
            next: [0,0,0,0],
            remain: [0,1,2,3,4,5,6]
        };
        //On génère les 4 premières pièces
        piecegen.next_piece_nb();
        piecegen.next_piece_nb();
        piecegen.next_piece_nb();
        piecegen.next_piece_nb();

        return piecegen
    }

    fn gen_piece_nb(&mut self) -> u8{

        //Par la suite, on voudra que les pièces se génèrent en cycle (en gros on vide remain petit à petit et on la reremplit pour recommencer)
        let mut sommet =10;
        for i in 0..7{
            if self.remain[i]!=10 { sommet = i};
        }
        if sommet==10{
            for i in 0..7{
                self.remain[i]= rand::thread_rng().gen_range(0, 7-i) as u8;
            }
            sommet = 6;
        }
        let generated = self.remain[sommet] as u8;
        self.remain[sommet] = 10;
        return generated;
    }


}


pub(crate) struct Piece{
    typeinfo: u8,
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

    //Utilisé pour les tests unitaires
    pub fn get_blocks_pos(&mut self) -> Vec<[u8; 2]> {
        let mut arr:Vec<[u8;2]> = vec!();
        for b in &self.blocks{
            arr.push(b.coords);
        }
        return arr;
    }

    pub fn draw(&mut self, can:&mut WindowCanvas, blockt:&mut [Texture; 7]){
        for b in 0..4{
            self.blocks[b].draw_grid(can,blockt);
        }
    }

    pub fn go_down(&mut self, stack: &Stack) -> bool {
        for b in 0..4{
            if stack.is_taken(self.blocks[b].coords[0] as i16, self.blocks[b].coords[1] as i16+1) { return false};
        }

        for b in 0..4{
            self.blocks[b].coords[1] += 1;
        }
        self.y += 1.0;
        return true;
    }

    pub fn get_type(&mut self) -> u8 {
        return self.typeinfo;
    }

    /*
     * Le système de rotation complexe de notre Tetris fonctionne de la manière suivante
     (ce n'est pas exactement le même système que dans le Tetris classique car les
     "guidelines" pour ce dernier sont maintenues privées par la Tetris Company et surement brevetées):
     * - Si c'est une pièce en O, on ne fait rien
     * - On doit tester si la rotation est possible (pour tous les blocs, leur coordonnées ne sont pas prises)
     * - Si elle est prise, on doit tester prenant un axe de rotation différent, qui est:
     * - Pour la pièce T, les 3 autres blocs (celui du milieu étant le défaut), en commençant par celui qui disparaitrait en rotation normale
     * /EXEMPLE/
     *        #
     *  #     ##
     * ### => #
     * &&&    &&&
     * - Pour la pièce I, les 4 blocs (l'axe par défaut étant une valeur décimale, donc pas centré sur un bloc), en commençant par les extrémités,
     * la plus basse ET la plus à droite pour rotation anti-horaire ou la plus à gauche pour la rotation horaire
     * /EXEMPLE/
     * ####    #
     *  &&&    #&&&
     *  &&&    #&&&
     *  &&& => #&&&
     * - Pour les pièce Z et S, les 3 autres blocs que celui par défaut, en commençant par le plus bas, s'il y en a 2 au plus bas, on prend l'extrémité
     * /EXEMPLE/
     *  ##
     *   ##        #
     * &&   =>  &&##
     * &&       &&#
     * - Pour les pièce L et J, les 3 autres blocs que celui par défaut, en commençant par le plus bas, s'il y en a 2 au plus bas, on prend l'extrémité
     * /EXEMPLE/
     * ##&      &
     * #&&     &&
     * #   => ###
     * &&     &&#
     */


    pub fn rotate(&mut self, stack: &Stack, clockwise:bool){
        if self.typeinfo == 1 { return; }
        if self.test_rotation_and_rotate(stack, self.x, self.y, clockwise) { return ; }
        match self.typeinfo {
            0 => {
                /* POUR T */
                //On sélectionne le 1er bloc selon si c'est dans le sens horaire ou antihoraire
                if self.test_rotation_and_rotate(
                    stack,
                    self.blocks[3 - (clockwise as usize) * 3].coords[0] as f32,
                    self.blocks[3 - (clockwise as usize) * 3].coords[1] as f32,
                    clockwise) {
                    self.x = self.blocks[1].coords[0] as f32;
                    self.y = self.blocks[1].coords[1] as f32;
                    return ;
                }
                if self.test_rotation_and_rotate(
                    stack,
                    self.blocks[(clockwise as usize) * 3].coords[0] as f32,
                    self.blocks[(clockwise as usize) * 3].coords[1] as f32,
                    clockwise) {
                    self.x = self.blocks[1].coords[0] as f32;
                    self.y = self.blocks[1].coords[1] as f32;
                    return ;
                }
                if self.test_rotation_and_rotate(
                    stack,
                    self.blocks[2].coords[0] as f32,
                    self.blocks[2].coords[1] as f32,
                    clockwise) {
                    self.x = self.blocks[1].coords[0] as f32;
                    self.y = self.blocks[1].coords[1] as f32;
                    return ;
                }

            },
            2 => {
                /* POUR I */
                /* on va tester les 4 pièces dans l'ordre, il suffit de savoir si on commence par le début ou la fin */

                //On teste si la pièce est à la verticale & du plus bas au plus haut
                //ou si la pièce est à l'horizontale et que lacondition "la première pièce est plus à gauche" = clockwise
                if
                    (self.blocks[0].coords[0] == self.blocks[1].coords[0] && self.blocks[0].coords[1] > self.blocks[1].coords[1])||
                    (self.blocks[0].coords[1] == self.blocks[1].coords[1] && (self.blocks[0].coords[0] < self.blocks[1].coords[0]) == clockwise)
                {
                    for i in 0..4{
                        if self.test_rotation_and_rotate(
                            stack,
                            self.blocks[i].coords[0] as f32,
                            self.blocks[i].coords[1] as f32,
                            clockwise) {
                                //Remettre l'axe principal au bon endroit, (ce n'est pas très grave si il n'es pas sur la bonne longueur du rectangle, mais il doit être entre les 2 bons blocs)
                                let decal:bool = ((self.blocks[2].coords[0] as i16 - self.blocks[1].coords[0] as i16 )
                                + (self.blocks[2].coords[1] as i16 - self.blocks[1].coords[1] as i16))>0;
                                self.x = self.blocks[1].coords[0] as f32 + (decal as i16 * 2 - 1)as f32 *0.5;
                                self.y = self.blocks[1].coords[1] as f32 + (decal as i16 * 2 - 1)as f32 *0.5;
                            return ; }
                    }
                } else {
                    for i in 0..4{
                        if self.test_rotation_and_rotate(
                            stack,
                            self.blocks[3-i].coords[0] as f32,
                            self.blocks[3-i].coords[1] as f32,
                            clockwise) {
                            //Remettre l'axe principal au bon endroit, (ce n'est pas très grave si il n'es pas sur la bonne longueur du rectangle, mais il doit être entre les 2 bons blocs)
                            let decal:bool = ((self.blocks[2].coords[0] as i16 - self.blocks[1].coords[0] as i16 )
                                + (self.blocks[2].coords[1] as i16 - self.blocks[1].coords[1] as i16))>0;
                            self.x = self.blocks[1].coords[0] as f32 + (decal as i16 * 2 - 1)as f32 *0.5;
                            self.y = self.blocks[1].coords[1] as f32 + (decal as i16 * 2 - 1)as f32 *0.5;
                            return ; }
                    }
                }



            },
            3|4=>{
                //Savoir si le 1e bloc est en bas (true) ou pas (false)
                let fromdown:bool =  self.blocks[0].coords[1]>self.blocks[3].coords[1];
                if self.test_rotation_and_rotate(
                    stack,
                    self.blocks[3-(fromdown as usize*3)].coords[0] as f32,
                    self.blocks[3-(fromdown as usize*3)].coords[1] as f32,
                    clockwise) {
                    self.x = self.blocks[2].coords[0] as f32;
                    self.y = self.blocks[2].coords[1] as f32;
                    return ;
                }
                if self.test_rotation_and_rotate(
                    stack,
                    self.blocks[1].coords[0] as f32,
                    self.blocks[1].coords[1] as f32,
                    clockwise) {
                    self.x = self.blocks[2].coords[0] as f32;
                    self.y = self.blocks[2].coords[1] as f32;
                    return ;
                }
                if self.test_rotation_and_rotate(
                    stack,
                    self.blocks[fromdown as usize*3].coords[0] as f32,
                    self.blocks[fromdown as usize*3].coords[1] as f32,
                    clockwise) {
                    self.x = self.blocks[2].coords[0] as f32;
                    self.y = self.blocks[2].coords[1] as f32;
                    return ;
                }
            },
            5|6=>{
                if self.test_rotation_and_rotate(
                    stack,
                    self.blocks[0].coords[0] as f32,
                    self.blocks[0].coords[1] as f32,
                    clockwise) {
                    self.x = self.blocks[1].coords[0] as f32;
                    self.y = self.blocks[1].coords[1] as f32;
                    return ;
                }
                if self.test_rotation_and_rotate(
                    stack,
                    self.blocks[3].coords[0] as f32,
                    self.blocks[3].coords[1] as f32,
                    clockwise) {
                    self.x = self.blocks[1].coords[0] as f32;
                    self.y = self.blocks[1].coords[1] as f32;
                    return ;
                }
                if self.test_rotation_and_rotate(
                    stack,
                    self.blocks[2].coords[0] as f32,
                    self.blocks[2].coords[1] as f32,
                    clockwise) {
                    self.x = self.blocks[1].coords[0] as f32;
                    self.y = self.blocks[1].coords[1] as f32;
                    return ;
                }
            },
            _ => {}
        }
    }


    fn set_new_blocks_after_rotation(&mut self, bpos: &[[i16; 2]; 4], _axisx:f32, _axisy:f32){
        for b in 0..4{
            self.blocks[b].coords[0] = bpos[b][0] as u8;
            self.blocks[b].coords[1] = bpos[b][1] as u8;
        }
    }

    fn test_rotation_and_rotate(&mut self, stack:&Stack, axisx:f32, axisy:f32, clockwise:bool) -> bool {

        let mut bpos:[[i16;2];4] = [[0,0],[0,0],[0,0],[0,0]];
        for b in 0..4{
            bpos[b][0] = (-(clockwise as u8 as f32 *2.0-1.0)*(self.blocks[b].coords[1] as f32 - axisy) + axisx).ceil() as i16;
            bpos[b][1] = ((clockwise as u8 as f32 *2.0-1.0)*(self.blocks[b].coords[0] as f32 - axisx) + axisy).ceil() as i16;
        }
        for i in bpos{
            if stack.is_taken(i[0], i[1]) {

                return false
            };
        }

        self.set_new_blocks_after_rotation(&bpos, axisx, axisy);
        return true;
    }

    pub fn move_right(&mut self, stack: &Stack){
        for b in 0..4{
            if stack.is_taken(self.blocks[b].coords[0] as i16+1, self.blocks[b].coords[1] as i16) { return };
        }
        for b in 0..4{
            self.blocks[b].coords[0] += 1;
        }
        self.x += 1.0;
    }

    pub fn move_left(&mut self, stack: &Stack){
        for b in 0..4{
            if stack.is_taken(
                self.blocks[b].coords[0] as i16-1,
                self.blocks[b].coords[1] as i16
            ) { return };
        }
        for b in 0..4{
            self.blocks[b].coords[0] -= 1;
        }
        self.x -= 1.0;
    }

    pub fn pose(&self, stack:&mut Stack){
        for b in &self.blocks{
            stack.add(b.coords[0] as usize,b.coords[1] as usize,b.color as u8);
        }
    }

    pub fn new_active(piece_type:u8) -> Piece{
        match piece_type {
            0 => return Piece {
                typeinfo: 0,
                x: 4.0,
                y: 1.0,
                blocks:
                [
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 0,

                        coords: [4,0]
                    },
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 0,

                        coords: [4,1]
                    },
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 0,

                        coords: [5,1]
                    },
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 0,

                        coords: [4,2]
                    }
                ]
            },
            1 => return Piece {
                typeinfo: 1,
                x: 4.5,
                y: 0.5,
                blocks:
                [
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 1,

                        coords: [4,0]
                    },
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 1,

                        coords: [4,1]
                    },
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 1,

                        coords: [5,0]
                    },
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 1,

                        coords: [5,1]
                    }
                ]
            },
            2 => return Piece {
                typeinfo: 2,
                x: 4.5,
                y: 1.5,
                blocks:
                [
                    Block {
                        rect: Rect::new(0,0, 28, 28),
                        color: 2,

                        coords: [4,0]
                    },
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 2,

                        coords: [4,1]
                    },
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 2,

                        coords: [4,2]
                    },
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 2,

                        coords: [4,3]
                    }
                ]
            },
            3 => return Piece {
                typeinfo: 3,
                x: 5.0,
                y: 1.0,
                blocks:
                [
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 3,

                        coords: [4,0]
                    },
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 3,

                        coords: [4,1]
                    },
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 3,

                        coords: [5,1]
                    },
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 3,

                        coords: [5,2]
                    }
                ]
            },
            4 => return Piece {
                typeinfo: 4,
                x: 4.0,
                y: 1.0,
                blocks:
                [
                    Block {
                        rect: Rect::new(0,0, 28, 28),
                        color: 4,

                        coords: [5,0]
                    },
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 4,

                        coords: [5,1]
                    },
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 4,

                        coords: [4,1]
                    },
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 4,

                        coords: [4,2]
                    }
                ]
            },
            5 => return Piece {
                typeinfo: 5,
                x: 4.0,
                y: 1.0,
                blocks:
                [
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 5,

                        coords: [4,0]
                    },
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 5,

                        coords: [4,1]
                    },
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 5,

                        coords: [4,2]
                    },
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 5,

                        coords: [5,2]
                    }
                ]
            },
            _ => return Piece {
                typeinfo: 6,
                x: 5.0,
                y: 1.0,
                blocks:
                [
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 6,

                        coords: [5,0]
                    },
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 6,

                        coords: [5,1]
                    },
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 6,

                        coords: [5,2]
                    },
                    Block {
                        rect: Rect::new(0, 0, 28, 28),
                        color: 6,

                        coords: [4,2]
                    }
                ]
            }
        }
    }

}

