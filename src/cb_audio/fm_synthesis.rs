pub enum OscillatorInputTypes {
    Midi,
    Hz(f32),
}

pub struct Envelope {}

impl Envelope {
    pub fn new() -> Self {
        unimplemented!();
    }
}

pub enum WaveForms {
    Sine,
    Square,
    Triangle,
    Saw,
}

struct Oscillator {
    wave_form: WaveForms,
}

impl Oscillator {
    pub fn new() -> Self {
        return Self {
            wave_form: WaveForms::Sine,
        };
    }
}

pub trait Operator {
    fn add_input(&mut self, operator: Box<Operator>);
    fn set_oscillator_input_type(&mut self, osc_type: OscillatorInputTypes);
    fn set_amplifier_envelope(&mut self, envelope: Option<Envelope>);
    fn get_output(&self) -> f32;
}

pub struct CbOperator {}

impl CbOperator {
    pub fn new() -> Self {
        unimplemented!();
    }
}

impl Operator for CbOperator {
    fn add_input(&mut self, operator: Box<dyn Operator>) {
        unimplemented!();
    }
    fn set_oscillator_input_type(&mut self, osc_type: OscillatorInputTypes) {
        unimplemented!();
    }
    fn set_amplifier_envelope(&mut self, envelope: Option<Envelope>) {
        unimplemented!();
    }

    fn get_output(&self) -> f32 {
        unimplemented!();
    }
}

pub struct OperatorChain {
    inputs: Vec<Box<Operator>>,
}

impl OperatorChain {
    pub fn init_dumb_chain() -> Self {
        let mut operator_a = CbOperator::new();
        let envelope_a = Envelope::new();
        operator_a.set_oscillator_input_type(OscillatorInputTypes::Hz(200.0));

        let mut operator_b = CbOperator::new();
        operator_b.set_oscillator_input_type(OscillatorInputTypes::Hz(100.0));

        operator_b.add_input(Box::new(operator_a));

        return Self {
            inputs: vec![Box::new(operator_b)],
        };
    }

    pub fn add_input(&mut self, operator: Box<dyn Operator>) {
        self.inputs.push(operator);
    }

    pub fn get_output(&self) {
        let outputs: Vec<f32> = self.inputs.iter().map(|i| i.get_output()).collect();

        unimplemented!();
    }
}
