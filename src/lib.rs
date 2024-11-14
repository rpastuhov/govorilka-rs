
use cpal::Device;
use std::fs::File;
use std::io::{self, BufReader};
use rodio::{
	Decoder, 
	DeviceTrait, 
	OutputStreamHandle, 
	Sink, 
	decoder::DecoderError
};


pub fn run_input() -> String {
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Failed to read line");
	input
}


pub fn get_device(devices: &Vec<Device>) -> &Device {

	for (index, device) in devices.iter().enumerate() {
		println!("{}: {}", index, device.name().unwrap());
	}

	'start: loop {
		let input = run_input();

		let num: usize = match input.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("You need to enter a number!");
				continue 'start;
			}
		};

		match devices.get(num) {
			None => {
				println!("There is no such device, choose another one.");
				continue 'start
			},
			Some(device) => break device,
		};
	}
}


pub fn get_audio_data(mp3: &str) -> Result<Decoder<BufReader<File>>, DecoderError> {
	let file = std::fs::File::open(mp3).unwrap();
	let buffer = BufReader::new(file);
	rodio::Decoder::new(buffer)
}


pub fn play_audio (handle: &OutputStreamHandle, volume: f32, decoder: Decoder<BufReader<File>>) -> Sink {
	let sink = rodio::Sink::try_new(handle).unwrap();
	sink.set_volume(volume);
	sink.append(decoder);
	sink
}


pub fn split_into_sentences(text: &str) -> Vec<String> {
	let mut sentences = Vec::new();
	let mut current_sentence = String::new();
	let mut words = text.split_whitespace();

	while let Some(word) = words.next() {
		let next_sentence = format!("{} {}", current_sentence, word);
		if next_sentence.len() > 300 {
			sentences.push(current_sentence);
			current_sentence = word.to_owned();
		} else {
			current_sentence = next_sentence;
		}
	}
	sentences.push(current_sentence);

	sentences
}




#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn one_result() {
		let text = "This is a long sentence that needs to be split into multiple sentences. This is another sentence that needs to be added.";
		let split = split_into_sentences(text);

		print!("{:#?}", &split);
		assert!(true)
	}
}