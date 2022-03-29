/*use std::borrow::Borrow;
use sdl2::audio;
use sdl2::audio::{AudioCallback, AudioFormat, AudioSpecDesired};

struct CopyWav {
    bits: Vec<u8>,
    pos: usize
}

impl AudioCallback for CopyWav {
    type Channel = u8;

    fn callback(&mut self, data: &mut [u8]) {
        let (start, end) = (self.pos, self.pos + data.len());
        self.pos += data.len();

        let audio_data = &self.bits[start..end];
        for (src, dst) in audio_data.iter().zip(data.iter_mut()) {
            *dst = *src;
        }
    }
}


pub fn play_music(context: &sdl2::Sdl){

}
*/


use std::borrow::{Borrow, BorrowMut};
use sdl2::mixer::{AUDIO_S16LSB, DEFAULT_CHANNELS, InitFlag, Music};
use std::thread;
use sdl2::TimerSubsystem;
use std::time::Duration;

pub fn play_music(context: &sdl2::Sdl, music: &mut Option<Music>){
    let _audio = context.audio().unwrap();

    let frequency = 44_100;
    let format = AUDIO_S16LSB;
    let channels = DEFAULT_CHANNELS;
    let chunk_size = 1_024;

    sdl2::mixer::open_audio(frequency, format, channels, chunk_size).unwrap();

    let _mcontext = sdl2::mixer::init(sdl2::mixer::InitFlag::OGG).unwrap();

    sdl2::mixer::allocate_channels(1);

    fn hook_finished_music(){
        println!("Musique finie !\n");
    }

    sdl2::mixer::Music::set_volume(16);

    sdl2::mixer::Music::hook_finished(hook_finished_music);

    let mtimer =  context.timer().unwrap().clone();



    *music = Some(sdl2::mixer::Music::from_file("./res/Menu_8BSD.ogg").unwrap());












}