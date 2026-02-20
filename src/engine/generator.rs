use rand::Rng;

pub fn generate_guitar_tone(frequency: f32, sample_rate: f32, duration_secs: f32) -> Vec<f32> {
    let num_samples = (sample_rate * duration_secs) as usize;
    let mut rng = rand::thread_rng();

    // Algoritmo Karplus-Strong (Modelado de cuerda)
    let buffer_size = (sample_rate / frequency) as usize;
    let mut delay_line: Vec<f32> = (0..buffer_size)
        .map(|_| rng.gen_range(-1.0..1.0))
        .collect();

    let mut samples = Vec::with_capacity(num_samples);
    let mut p0 = 0;

    for _ in 0..num_samples {
        let p1 = (p0 + 1) % buffer_size;
        let new_sample = (delay_line[p0] + delay_line[p1]) * 0.496;
        delay_line[p0] = new_sample;
        samples.push(new_sample);
        p0 = p1;
    }
    samples
}

pub fn generate_kick_drum(sample_rate: f32, duration_secs: f32) -> Vec<f32> {
    let num_samples = (sample_rate * duration_secs) as usize;
    (0..num_samples).map(|t| {
        let time = t as f32 / sample_rate;
        let freq_env = 150.0 * (-25.0 * time).exp();
        let body = (time * freq_env * 2.0 * std::f32::consts::PI).sin();
        let envelope = (-18.0 * time).exp();
        (body * envelope * 1.5).tanh()
    }).collect()
}

pub fn apply_guitar_physics(samples: &mut Vec<f32>, gain: f32, sample_rate: f32, phase: &mut f32) {
    let mut last_sample = 0.0;
    for sample in samples.iter_mut() {
        let mut s = *sample * gain;
        s = s.tanh(); // Distorsión de válvula

        // Cab Sim (Filtro Low Pass a 5kHz)
        let lp_factor = 0.18;
        s = lp_factor * s + (1.0 - lp_factor) * last_sample;
        last_sample = s;

        // Glitch LFO
        let lfo_val = (*phase * 12.0 * 2.0 * std::f32::consts::PI / sample_rate).sin();
        *sample = if lfo_val < -0.85 { s * 0.05 } else { s };
        *phase += 1.0;
    }
}
