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


use sdl2::mixer::{AUDIO_S16LSB, DEFAULT_CHANNELS, Music, Sdl2MixerContext};

pub struct MusicPlayer{
    pub(crate) _music: Music<'static>,
    pub(crate) _context: Sdl2MixerContext
}

pub fn play_music(_context: &sdl2::Sdl) -> MusicPlayer {


    let frequency = 44_100;
    let format = AUDIO_S16LSB;
    let channels = DEFAULT_CHANNELS;
    let chunk_size = 1_024;

    sdl2::mixer::open_audio(frequency, format, channels, chunk_size).unwrap();

    let mcontext = sdl2::mixer::init(sdl2::mixer::InitFlag::OGG).unwrap();

    sdl2::mixer::allocate_channels(4);

    fn hook_finished_music(){
        println!("Musique finie !\n");
    }

    sdl2::mixer::Music::set_volume(32);

    sdl2::mixer::Music::hook_finished(hook_finished_music);

    println!("[MUSIC]Chunk decoders : {}" , sdl2::mixer::get_chunk_decoders_number());
    println!("[MUSIC]Music decoders : {}" , sdl2::mixer::get_music_decoders_number());
    println!("[MUSIC]Paused channels : {}" , sdl2::mixer::get_paused_channels_number());
    println!("[MUSIC]Volume : {}" , sdl2::mixer::Music::get_volume());
    println!("[MUSIC]Playing channels : {}" , sdl2::mixer::get_playing_channels_number());


    let music =  sdl2::mixer::Music::from_file("./res/Menu_8BSD.ogg").unwrap();


    music.play(-1).unwrap();

    let player = MusicPlayer{_music:music, _context:mcontext};

    return player;











}