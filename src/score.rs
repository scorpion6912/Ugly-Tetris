use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};

pub(crate) fn display_score(mut score:i32, numt:&Texture, can: &mut WindowCanvas){
    let mut i = 0;
    let mut x:i64;
    while score > 0{
        x = 745-(i*30);
        can.copy(numt,Rect::new(100*(score%10),0,100,100),Rect::new(x as i32,100,30,30)).unwrap();
        i+= 1;
        score = score.div_euclid(10);
    }
    return;
}