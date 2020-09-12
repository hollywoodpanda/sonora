use rodio::Sink;

mod note;
use note::model::{Note, Playable};
use std::{thread, time, fs};


fn main() {
    
    note::say_hi();

    let filename = "song.txt";

    play_file(filename);

    sleep_a_lot();

    println!("fin");

}

fn sleep_a_lot() {

    let period = time::Duration::from_millis(2000);
    thread::sleep(period);

}

fn play_file(filename : &str) {

    let device = rodio::default_output_device().unwrap();

    let content_string : String = fs::read_to_string(filename).expect("Hum... Couldn't read the song file :-(");

    for line in content_string.split("\n") {

        let comma_index = line.find(',').expect("Every note in the song.txt file must be a line in the file: \"frequency,duration\"");

        let frequency = &line[0 .. comma_index];

        let duration = &line[comma_index + 1 ..];

        let note = &mut Note {

            sink: Sink::new(&device),
            sound: frequency.parse::<u32>().unwrap(),
            duration: duration.parse::<u64>().unwrap()

        };

        note.play();

    }

}



