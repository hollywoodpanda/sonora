use rodio::source::SineWave;
use rodio::Sink;

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

}

/**
 * A generic implementation of
 * playable sounds.
 */
impl<T> Playable for T where T : Soundable {

    fn play(&mut self) {

        let frequency = self.get_sound();

        println!("playing sine frequency: {}", frequency);

        self.get_sink().append(SineWave::new(frequency));

    }

    fn stop(&mut self) {

        println!("playable sound stop");
        
        self.get_sink().stop();

    }

}

pub struct Note {
    
    pub sink: Sink,

    pub sound: u32

}

impl Soundable for Note {

    fn get_sink(&mut self) -> &Sink {

        return &self.sink;

    }

    fn get_sound(&mut self) -> u32 {

        return self.sound;

    }

}