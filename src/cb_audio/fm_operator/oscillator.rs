pub enum WaveForms {
    Triangle,
    Saw,
    Square,
    Sine,
}

pub struct Oscillator {
    wave_form: WaveForms,
}
