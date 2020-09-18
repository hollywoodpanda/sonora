use rodio::Sink;
use std::env;

mod note;
use note::model::{Note, Playable};
use std::{fs};

/// 
/// The main function, called
/// when we execute:
/// 
/// `cargo run [song file]`
/// 
/// ... in the command line.
/// 
fn main() {

    // We get the arguments passed to the cargo run command...
    let args : Vec<String> = env::args().collect();

    /*
     * Here we're trying to match some parameter ending with ".txt"
     * 
     * If we find any, we get the (probably, haven't checked really)
     * first match. Else we inform that no .txt song was given and
     * end the program.
     */
    let filename = match args.iter().find(|&txt_file| txt_file.ends_with(".txt")) {

        Some(name) => name,

        _ => {

            println!("\r\nWell, no file was given. Try to run it like this:\r\n\r\ncargo run [NAME OF THE FILE WITH THE SONG].txt\r\n\r\nAs you've seen, the file must end with the \"txt\" extension.\r\n");

            return;

        }

    };

    // Playing the file identified
    // by filename
    play_file(filename);

    // The song is over. Good bye!
    println!("fin");

}

/// Plays the file identified by the given filename
/// or file path or exits if no file was found or some
/// inconsistency was found while executing the song
/// in the file.
/// 
/// ### Arguments
/// 
/// * `filename : &str` - The name or full path of the song file
/// 
/// ### Example
/// 
/// `play_file("my_super_duper_awesome_song.txt");`
/// 
fn play_file(filename : &str) {

    /*
     * We try to get the system default output
     * sound... We're only proceeding with the
     * execution if we have some device to play!
     */
    let device = match rodio::default_output_device() {

        Some(d) => d,

        None => {

            println!("No default sound device was found. Wonder why... :_-(");

            return;

        }

    };

    // Trying to read the file by the given file name.
    let content_string : String = match fs::read_to_string(filename) {

        Ok(val) => val,

        Err(_err) => {

            println!("\r\n\r\nHum... Couldn't read the given song file. What the hell? Have you typed it correctly?\r\n\r\n");

            return;

        }

    };

    /*
     * Let's split the music file by the new line
     * character (so we get a iterator over the
     * lines in the file)
     */ 
    let mut iterator = content_string.split("\n");

    loop {

        /*
        * Get the next line found in the music file.
        * If it is the first iteration in the loop, gets the
        * first line (and so on).
        */
        let line = match iterator.next() {

            Some(line) => {

                if line.starts_with('#') {
                    
                    continue;

                }

                line

            },

            None => break

        };

        // The index of the comma character on the line (if any)
        let comma_index = match line.find(',') {

            Some(i) => i,

            None => {

                println!("Hum... An invalid line was found! We'll ignore it for now ;-)");

                continue;

            }


        };

        //Parsing the frequency
        let frequency = match line[0 .. comma_index].parse::<u32>() {

            Ok(value) => value,

            Err(_err) => {

                println!("Opsy! The frequency must be a positive integer... What the hell was that?");

                continue;

            }

        };

        //Parsing the duration
        let duration = match line[comma_index + 1 ..].parse::<u64>() {

            Ok(value) => value,

            Err(_err) => {

                println!("Oh my sweet baby... Ok. Got an invalid duration somewhere in the file. It must be an positive integer and it was not! >:-(");

                continue;

            }

        };

        // Creating the musical note...
        let note = &mut Note {

            sink: Sink::new(&device),
            sound: frequency,
            duration: duration

        };

        // ... and playing it! :-D
        note.play();

    }

}



