use rodio::Source;
use A2VConverter::{self, AudioVideoConverter};
use whisper_rs::{WhisperContext, WhisperContextParameters, FullParams, SamplingStrategy};


fn main() {

    // extract an audio track from the video
    use std::path::Path;

    let videopath = Path::new("./video.mov");
    
    if !videopath.exists() {
        println!("video file is not seen.");
    } else {
        println!("{}", videopath.file_name().unwrap().to_str().unwrap());
    }

    match AudioVideoConverter::convert_video_to_audio(
        videopath.to_str().unwrap(), 
        "./extracted_audio.mp3"
    ) {
        Ok(_) => println!("audio extraction finished."),
        Err(error) => println!("{:?}", error)
    };
    
    // getting into whisper
    let mut whisper_parameters = FullParams::new(
        SamplingStrategy::Greedy { best_of: 1 }
    );

    whisper_parameters.set_language(Some("zh" as &str));

    let whisper_context = match WhisperContext::new_with_params(
        "ggml-tiny.bin", 
        WhisperContextParameters::default()
    ) {
        Ok(result) => result,
        Err(error) => panic!("{}", error)
    };

    use std::fs::File;
    use std::io::BufReader;
    use rodio::Decoder;

    let audio_track = match File::open("./extracted_audio.mp3") {
        Ok(result) => BufReader::new(result),
        Err(error) => panic!("{}", error)
    };

    let decoded_audio_track: Vec<f32> = match Decoder::new(audio_track) {
        Ok(result) => result
            .convert_samples::<f32>()
            .map(|sample| sample)
            .collect(),
        Err(error) => panic!("{}", error)
    };

    // now we can run the model
	let mut state = whisper_context.create_state().expect("failed to create state");
	state
		.full(whisper_parameters, &&decoded_audio_track[..])
		.expect("failed to run model");

	// fetch the results
	let num_segments = state
		.full_n_segments()
		.expect("failed to get number of segments");
	for i in 0..num_segments {
		let segment = state
			.full_get_segment_text(i)
			.expect("failed to get segment");
		let start_timestamp = state
			.full_get_segment_t0(i)
			.expect("failed to get segment start timestamp");
		let end_timestamp = state
			.full_get_segment_t1(i)
			.expect("failed to get segment end timestamp");
		println!("[{} - {}]: {}", start_timestamp, end_timestamp, segment);
	}

}