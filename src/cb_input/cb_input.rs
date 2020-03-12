extern crate rmercury;
use rmercury::{RMercuryGameInterface, RMercuryInput};

use super::*;
use input_type::{Press, Range, State};

use crate::cb_data_structures;
use cb_data_structures::CbLinkedList;

#[derive(Debug, Copy, Clone)]
pub struct CbGameInput {
    player_id: usize,
    context_id: u8,
    states: Option<CbLinkedList<State>>,
    presses: Option<CbLinkedList<Press>>,
    ranges: Option<CbLinkedList<Range>>,
}

impl RMercuryInput for CbGameInput {
    fn get_player_id(&self) -> usize {
        //unimplemented!()
        return 0;
    }
    fn set_player_id(&mut self, _: usize) {
        //unimplemented!()
    }
    fn to_bits(&self) -> std::vec::Vec<u8> {
        let mut bits = vec![];

        // Unimplemented!

        return bits;
    }
    fn from_bits(bits: std::vec::Vec<u8>) -> Self {
        let player_id = 0;
        let context_id = 0;
        let states = None;
        let presses = None;
        let ranges = None;

        // Unimplemented!

        return Self {
            player_id: player_id,
            context_id: context_id,
            states: states,
            presses: presses,
            ranges: ranges,
        };
    }
}
