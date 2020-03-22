extern crate rmercury;
use rmercury::{RMercuryGameInterface, RMercuryInput};

use super::*;
use input_type::{Press, Range, State};

use contexts::CbContextManager;

const MAX_NUM_ACTIVE_CONTEXTS: usize = 10;

#[derive(Debug, Copy, Clone)]
pub struct CbGameInputWrapper<T> {
    pub context_id: usize, //NOTE: usize may be too large for networking; may need to revisit to something smaller?
    pub action_id: usize, //NOTE: usize may be too large for networking; may need to revisit to something smaller?
    pub networked: bool,
    pub input: T,
}

#[derive(Debug, Copy, Clone)]
pub struct CbGameInput {
    pub player_id: usize,
    pub context_manager: CbContextManager,
}

impl CbGameInput {
    pub fn new(player_id: usize, context_manager: CbContextManager) -> Self {
        return Self {
            player_id: player_id,
            context_manager: context_manager,
        };
    }
}

impl RMercuryInput for CbGameInput {
    fn get_player_id(&self) -> usize {
        return self.player_id;
    }
    fn set_player_id(&mut self, player_id: usize) {
        self.player_id = player_id;
    }
    fn to_bits(&self) -> std::vec::Vec<u8> {
        let mut bits = vec![];

        // Unimplemented!
        let ctx_mgr_bits = self.context_manager.to_bits();

        return bits;
    }
    fn from_bits(bits: std::vec::Vec<u8>) -> Self {
        let player_id = 0;
        let context_id = 0;
        // Unimplemented!

        return Self {
            player_id: player_id,
            context_manager: CbContextManager::from_bits(bits),
        };
    }
}
