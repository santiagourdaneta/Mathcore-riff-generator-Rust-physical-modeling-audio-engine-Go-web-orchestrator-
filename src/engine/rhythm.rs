use rand::seq::SliceRandom;

pub fn get_random_note() -> f32 {
    // Escala Frigia Dominante en Drop C (Octavas 1 y 2)
    let drop_c_scale = [
        32.70,  // C1 (Tónica ultra grave)
        34.65,  // C#1 (Segunda menor)
        41.20,  // E1 (Tercera mayor)
        49.00,  // G1 (Quinta)
        51.91,  // G#1 (Sexta menor)
        65.41,  // C2 (Octava)
    ];

    let mut rng = rand::thread_rng();
    *drop_c_scale.choose(&mut rng).unwrap_or(&32.70)
}
