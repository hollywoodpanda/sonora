use rodio::Sink;

mod note;
use note::model::{Note, Playable};
use std::{thread,time};

fn main() {
    
    note::say_hi();

    let device = rodio::default_output_device().unwrap();
    
    let mut sound = Note { 
        sound: 300, 
        sink: Sink::new(&device)
    };

    sound.play();
    let period = time::Duration::from_millis(2000);
    thread::sleep(period);
    sound.stop();
    thread::sleep(period);
    println!("fin");
}
