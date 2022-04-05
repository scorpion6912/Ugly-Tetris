use pieces::piece::Piece;
use crate::pieces;

#[cfg(test)]
mod tests {
    use crate::{PieceGen, Stack};
    use super::*;

    #[test]
    fn test_rotation_simple() {
        let test_stack = Stack::init_stack();
        let mut piece = Piece::new_active(2);//Pièce I
        piece.rotate(&test_stack, true);
        let positions = piece.get_blocks_pos();
        assert!(
            positions.get(0).unwrap() == &[6,1]&&
            positions.get(1).unwrap() == &[5,1]&&
            positions.get(2).unwrap() == &[4,1]&&
            positions.get(3).unwrap() == &[3,1]
        );//positions attendues après rotation
    }

    #[test]
    fn test_rotation_contre_mur() {
        let test_stack = Stack::init_stack();
        let mut piece = Piece::new_active(5);
        for _i in 0..8{ piece.move_left(&test_stack);}
        piece.rotate(&test_stack, false);
        let positions = piece.get_blocks_pos();
        assert!(
            positions.get(0).unwrap() == &[0,0]&&
            positions.get(1).unwrap() == &[0,1]&&
            positions.get(2).unwrap() == &[0,2]&&
            positions.get(3).unwrap() == &[1,2]
        );//Positions attendues après rotation contre un mur
    }

    #[test]
    fn test_piece_generation() {
        //On vérifie que les pièces générées soient toutes différentes
        //[!] Ce test est basé sur l'aléatoire, mais la probabilité d'un résultat faux positif est de 1/720

        let mut generator = PieceGen::new();
        let mut pieces:[u8;7] = [10,11,12,13,14,15,16];
        for p in 0..6{
            pieces[p] = generator.next_piece_nb();
        }
        for p in 0..6{
            for q in 0..6{
                if p!=q && pieces[p] == pieces[q]{
                    assert!(false);
                }
            }
        }

        assert!(true);
    }


    #[test]
    fn test_stack_score() {
        //On vérifie que 3 points soient assignés en remplissant 3 lignes

        let mut stack = Stack::init_stack();

        for i in 0..10{stack.add(i, 18, 1);}
        for i in 1..10{stack.add(i, 17, 1);}
        for i in 0..10{stack.add(i, 16, 1);}
        for i in 0..9{stack.add(i, 15, 1);}

        assert!(stack.verify_lines(0)==2);
    }

}