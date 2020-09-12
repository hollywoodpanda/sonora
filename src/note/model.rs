use rodio::source::SineWave;
use rodio::Sink;
use std::{time::Duration, thread};

/**
 * Can play and stop something
 */
pub trait Playable {

    fn play(&mut self);
    fn stop(&mut self);

}
/**
 * Can have a sound (SineWave)
 */
pub trait Soundable {

    fn get_sound(&mut self) -> u32;
    fn get_sink(&mut self) -> &Sink;
    fn get_duration(&mut self) -> u64;

}

/**
 * A generic implementation of
 * playable sounds.
 */
impl<T> Playable for T where T : Soundable {

    fn play(&mut self) {

        let frequency = self.get_sound();
        let duration = self.get_duration();

        println!("playing sine frequency {} and duration {}", frequency, duration);

        self.get_sink().append(SineWave::new(frequency));

        block_for_sound(duration);

        self.stop();

    }

    fn stop(&mut self) {

        println!("playable sound stop");
        
        self.get_sink().stop();
        
    }

}

pub struct Note {
    
    pub sink: Sink,

    pub sound: u32,

    pub duration: u64

}

impl Soundable for Note {

    fn get_sink(&mut self) -> &Sink {

        return &self.sink;

    }

    fn get_sound(&mut self) -> u32 {

        return self.sound;

    }

    fn get_duration(&mut self) -> u64 {

        return self.duration;

    }

}

fn block_for_sound(time_in_millis : u64) {

    let period = Duration::from_millis(time_in_millis);
    thread::sleep(period);

}

