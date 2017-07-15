use std::collections::HashMap;
use std::path::Path;

use errors;
use register;
use databank;

pub struct Capuchin {
    batter_regress: u16,
    peak_age: u8,
    year_weights: Vec<f32>,
    people: Option<register::People>,
    players: databank::Players,
}

impl Capuchin {
    pub fn new(batter_regress: u16, peak_age: u8) -> Self {
        Capuchin {
            peak_age: peak_age,
            year_weights: Vec::new(),
            batter_regress: batter_regress,
            people: None,
            players: databank::Players::new(),
        }
    }

    pub fn load_register(&mut self, people: register::People) {
        self.people = Some(people);
    }

    pub fn load_batting(&mut self, batting_csv: &Path) -> errors::Result<()> {
        self.players.load_batting(batting_csv)
    }

    pub fn batting_projection(&mut self, year: u8) -> Vec<databank::BattingProjection> {
        Vec::new()
    }

    /*
    pub fn pitching_projection(&mut self, year: u8) -> Vec<databank::PitchingProjection> {
        Vec::new()
    }
    */
}
