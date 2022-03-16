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
    let audio_subsystem = context.audio().unwrap();
    let music = audio::AudioSpecWAV::load_wav("./res/ugly_music.wav").unwrap();


    let audiospec:AudioSpecDesired = AudioSpecDesired{ freq: Some(44100), channels: Some(1), samples: None };



    let format:AudioFormat = music.format.clone();

    let cvt = audio::AudioCVT::new(
        music.format,music.channels,music.freq,
        music.format,audiospec.channels.unwrap(),audiospec.freq.unwrap()).unwrap().convert(music.buffer().to_vec());

    let wav_copy = CopyWav{ bits: cvt, pos: 0 };
    let audio_device = audio_subsystem.open_playback(None, &audiospec, move |_spec| {
        wav_copy
    }).unwrap();

    audio_device.resume();

    std::thread::sleep_ms(5000);
}
*/


use std::borrow::Borrow;

pub fn play_music(context: &sdl2::Sdl){
    let mcontext = sdl2::mixer::init(sdl2::mixer::InitFlag::OGG).unwrap();
    mcontext.borrow();
    sdl2::mixer::Music::from_file("./res/Menu_8BSD.ogg").unwrap().play(-1);
}