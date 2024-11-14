
use cpal::{traits::HostTrait, Device};
use tts_rust::{tts::GTTSClient, languages::Languages};
use govorilka::*;

const MP3: &str = "audio.mp3";

fn main() {

    let host = cpal::default_host();
    let devices_host = host.devices().expect("Failed to get devices");
    let devices: Vec<Device> = devices_host.collect();

    println!("Write the number of the audio device (input virtual cable)");

    let output = get_device(&devices);

    println!("Output selected");
    println!("Now enter the text, and I will voice it <3");

    let narrator = GTTSClient {
        volume: 0.5,
        language: Languages::English,
        tld: "com",
    };

    let (_stream, handle2) = rodio::OutputStream::try_default().unwrap();
    let (_stream, handle1) = rodio::OutputStream::try_from_device(output)
    .unwrap_or_else(|_|{
        println!("Mistake:\nAn unsupported device is selected!\nThe default output will be used");
        rodio::OutputStream::try_default().unwrap()
    });

    


    loop {
        let input = run_input();
        let split_input = govorilka::split_into_sentences(input.as_str());


        for input in split_input {

            narrator.save_to_file(&input, MP3).unwrap();

            let decoder1 = match get_audio_data(MP3) {
                Ok(value) => value,
                Err(e) => {
                    println!("{e}");
                    println!("Api error, something is wrong with the input.");
                    continue;
                }
            };
            let decoder2 = get_audio_data(MP3).unwrap();

            // For the sound to be output to the virtual cable and to the main headphones
            let sink1 = play_audio(&handle1, narrator.volume, decoder1);
            let sink2 = play_audio(&handle2, narrator.volume, decoder2);

            sink1.sleep_until_end();
            sink2.sleep_until_end();
        }
    }
}
