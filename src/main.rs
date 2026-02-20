use clap::Parser;
use hound;
use rand::Rng;
mod engine;

#[derive(Parser)]
struct Args {
    #[arg(short = 'b', long, default_value_t = 160)]
    bpm: u32,
    #[arg(short = 'g', long, default_value_t = 80.0)]
    gain: f32,
    #[arg(short = 'n', long, default_value_t = 4)]
    bars: u32,
}

fn main() {
    let args = Args::parse();
    let sample_rate = 44100.0;
    let mut full_riff: Vec<f32> = Vec::new();
    let mut lfo_phase = 0.0;

    let notes_per_bar = 7;
    let note_duration = (60.0 / args.bpm as f32) / 2.0;
    let mut rng = rand::thread_rng();

    for _ in 1..=args.bars {
        for _ in 0..notes_per_bar {
            if rng.gen_bool(0.2) {
                full_riff.extend(vec![0.0; (sample_rate * note_duration) as usize]);
                lfo_phase += sample_rate * note_duration;
                continue;
            }

            let freq = engine::rhythm::get_random_note();
            let mut guitar = engine::generator::generate_guitar_tone(freq, sample_rate, note_duration);
            engine::generator::apply_guitar_physics(&mut guitar, args.gain, sample_rate, &mut lfo_phase);
            let kick = engine::generator::generate_kick_drum(sample_rate, note_duration);

            for i in 0..guitar.len() {
                let k_val = if i < kick.len() { kick[i] * 0.8 } else { 0.0 };
                full_riff.push((guitar[i] * 0.6 + k_val).tanh());
            }
        }
    }

    let spec = hound::WavSpec { channels: 1, sample_rate: 44100, bits_per_sample: 16, sample_format: hound::SampleFormat::Int };
    let mut writer = hound::WavWriter::create("mathcore_output.wav", spec).unwrap();
    for s in full_riff { writer.write_sample((s * i16::MAX as f32) as i16).unwrap(); }
}
