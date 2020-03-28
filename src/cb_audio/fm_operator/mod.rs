mod amplifier;
mod input;
use input::Input;
mod oscillator;
use oscillator::{Oscillator, WaveForms};
use std::f32::consts::PI;

pub trait fFmOperator {
    fn set_input(&mut self, input: Option<Input>);
    fn toggle_midi_input(&mut self, active: bool);
    fn add_envelope(&mut self);
    fn get_output(&self);
}

/*


"""
  Simple FM_Synth object for experimentation.
  Defaults: f_carrier = 220, f_mod =220, Ind_mod = 1, length = 5, sampleRate = 44100
  if f_carrier/f_mod = N1/N2 and N1, N2 are integers, harmonic spectra will result
  if N1/N2 is irrational, i.e. sqrt(2) or pi, inharmonic spectra will result
  f_0, the fundamental = f_carrier/N1 = f_mod/N2
  # k_th harmonic = N1 + n*N2 for n = 0,1,2,3,4,...
  # so for f_carrier = 100 and f_mod = 300, harmonics are [100, 400, 700, 1000, 1300, 1600, 1900, 2200, 2500, 2800] etc
  """
  def __init__(self, f_carrier = 220, f_mod =220, Ind_mod = 1, length = 5, sampleRate = 44100, waveFile = True):
      self.increment = .01
      self.f_carrier = f_carrier
      self.f_mod = f_mod
      self.Ind_mod = Ind_mod
      self.rate = sampleRate
      self.ident = id(self)
      self.name = '%dHz_carrier-%dHz_mod-%s_Index_%d.wav' % (self.f_carrier, self.f_mod, str(self.Ind_mod),self.ident)
      sampleInc = 1.0/self.rate
      x = np.arange(0,length, sampleInc)
      y = np.sin(2*np.pi*self.f_carrier*x + self.Ind_mod*np.sin(2*np.pi*self.f_mod*x))
      mx = 1.059*(max(abs(y))) # scale to max pk of -.5 dB
      y = y/mx
      wavData = np.asarray(32000*y, dtype = np.int16)
      self.wavData = wavData
      if waveFile:
          write('audio/%s' % self.name, 44100.0, self.wavData)
          */
pub struct FmSynthesizer {
    render_execution_chain: Vec<Vec<usize>>,
    operators: Vec<FmOperator>,
    next_operator_id: usize,
    output_operator_id: Option<usize>,
}

impl FmSynthesizer {
    fn set_output_operator(&mut self, operator: &FmOperator) {
        self.output_operator_id = Some(operator.id);
        self.recalculate_render_execution_chain();
    }

    fn recalculate_render_execution_chain(&mut self) {
        //TODO: calculate the dependent render jobs.
        /*
        Basically need to calculate some sort of heirarchy, e.g. if operator B depends on A, and there exists a nondependanct C, render chain would be [[operator_A_id, operator_C_id], [operator_B_id]]
        Something to get an idea started:
        let mut finished_count = 0;
        loop {
            for (id, operator) in self.operators.iter().enumerate() {
                if operator_values[id].is_some() {
                    // Continue, and possibly stop checking this?
                    continue;
                }

                if operator.carrier_fm_operator_ids.is_empty() {
                    operator_values[id] = Some(operator.get_fm());
                    finished_count += 1;
                } else {
                    // Check to see if all the values are ready. If so, render. Otherwise, continue processing.
                    let mut ready_to_render = true;
                    for carrier_id in operator.carrier_fm_operator_ids.iter() {
                        if operator_values[*carrier_id].is_none() {
                            ready_to_render = false;
                            break;
                        }
                    }

                    if ready_to_render {
                        operator_values[id] = Some(operator.get_fm());
                        finished_count += 1;
                    }
                }
            }

            // All operators have processed, so break.
            if finished_count >= self.operators.len() {
                break;
            }
        }
        */
    }

    /// Monophonic fm synthesis renderer
    fn render(&mut self, midi_note: f32) {
        // For each operator, go through
        let mut operator_values: Vec<f32> = vec![];
        operator_values.reserve(self.operators.len());

        self.render_execution_chain.iter().for_each(|render_batch| {
            for id in render_batch.iter() {
                let id = *id;
                operator_values[id] = self.operators[id].get_fm();
            }
        });

        let output = operator_values[self.output_operator_id.unwrap()];
        //TODO: what to do with renders?
    }
}

pub struct FmOperator {
    id: usize,
    carrier_fm_operator_ids: Vec<usize>,
}

impl FmOperator {
    fn new(id: usize) -> Self {
        return Self {
            id: id,
            carrier_fm_operator_ids: vec![],
        };
    }

    fn link_carriers(&mut self, carrier_fm_operator_ids: Vec<usize>) {
        self.carrier_fm_operator_ids = carrier_fm_operator_ids;
    }

    fn get_amplitude(&self) -> f32 {
        unimplemented!();
        //return self.amplitude;
    }

    fn get_fm(&self) -> f32 {
        let amplitude = 0.0; //TODO: figure out?
        let carrier = 0.0;
        let depth_of_modualation = 0.0;
        let frequency_of_modulation = 0.0;

        let t = 1.0; //TODO: figure out?

        // http://www.cs.cmu.edu/~music/icm-online/readings/fm-synthesis/fm_synthesis.pdf

        const twoPi: f32 = 2.0 * PI;

        let fOfT = amplitude
            * sin((twoPi * carrier * t)
                + depth_of_modualation * sin(twoPi * frequency_of_modulation * t));
        // not sure if more needed?
        return fOfT;
    }
}

fn sin(value: f32) -> f32 {
    return value.sin();
}
