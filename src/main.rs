#![recursion_limit = "1024"]

extern crate clap;
extern crate csv;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;

use clap::{Arg, App};
use std::path::Path;
use std::str::FromStr;
use std::process::exit;

mod errors {
    use csv;

    error_chain! {
        foreign_links {
            Io(csv::Error);
        }
    }
}

mod databank;
mod projection;
mod register;


const BATTER_REGRESS: u16 = 1200;
const PEAK_AGE: u8 = 27;
const PITCHER_REGRESS: u16 = 1200;
const BATTER_WEIGHTS: &'static [f32] = &[5.0, 4.0, 3.0];
const PITCHER_WEIGHTS: &'static [f32] = &[3.0, 2.0, 1.0];


fn main() {
    let app = App::new("Capuchin")
        .version("0.1.0")
        .about("Simple baseball projections")
        .arg(Arg::with_name("register")
             .short("r")
             .long("register")
             .value_name("FILE")
             .help("people.csv file")
             .takes_value(true))
        .arg(Arg::with_name("batting")
             .short("b")
             .long("batting")
             .value_name("FILE")
             .help("Batting.csv file")
             .takes_value(true))
        .arg(Arg::with_name("pitching")
             .short("p")
             .long("pitching")
             .value_name("FILE")
             .help("Pitching.csv file")
             .takes_value(true))
        .arg(Arg::with_name("batter_regress")
             .long("batter-regress")
             .value_name("PA")
             .help("Number of league average PA to regress players")
             .takes_value(true))
        .arg(Arg::with_name("pitcher_regress")
             .long("pitcher-regress")
             .value_name("PA")
             .help("Number of league average PA to regress players")
             .takes_value(true))
        .arg(Arg::with_name("peak_age")
             .short("a")
             .long("peak-age")
             .value_name("AGE")
             .help("Peak age for player")
             .takes_value(true))
        .arg(Arg::with_name("batter_weights")
             .short("w")
             .long("batter-weights")
             .value_name("W1,W2,...")
             .help("Weights to use for batters in previous seasons")
             .takes_value(true))
        .arg(Arg::with_name("pitcher_weights")
             .short("W")
             .long("pitcher-weights")
             .value_name("W1,W2,...")
             .help("Weights to use for pitchers in previous seasons")
             .takes_value(true))
        .arg(Arg::with_name("year")
             .value_name("YEAR")
             .required(true)
             .multiple(true)
             .help("Year(s) to project")
             .takes_value(true))
        ;
    let matches = app.get_matches();

    let peak_age = matches.value_of("peak_age")
        .map_or(PEAK_AGE, |age| u8::from_str(age)
                                .expect("Unable to parse peak age."));

    let batter_regress = matches.value_of("batter_regress")
        .map_or(BATTER_REGRESS, |age| u16::from_str(age)
                                .expect("Unable to parse amount to regress batters."));

    let pitcher_regress = matches.value_of("pitcher_regress")
        .map_or(PITCHER_REGRESS, |age| u16::from_str(age)
                                .expect("Unable to parse amount to regress pitchers."));

    let default_weights = Vec::from(BATTER_WEIGHTS);
    let batter_weights = matches.value_of("batter_weights")
        .map_or(default_weights, |weights| split_weights(weights)
                                 .expect("Unable to parse batter weights."));

    let default_weights = Vec::from(PITCHER_WEIGHTS);
    let pitcher_weights = matches.value_of("pitcher_weights")
        .map_or(default_weights, |weights| split_weights(weights)
                                 .expect("Unable to parse pitcher weights."));

    let years: Vec<u16> = matches.values_of("year")
        .expect("Need a year to project.")
        .map(|year| u16::from_str(year).expect("Expected to get integer year"))
        .collect();

    let mut capuchin = projection::Capuchin::new(batter_regress, peak_age, pitcher_regress,
                                                 batter_weights, pitcher_weights);

    // Is the register available? Load it.
    if let Some(register) = matches.value_of("register") {
        let mut people = register::People::new();
        if let Err(e) = people.load_register(Path::new(register)) {
            println!("Unable to load player register, skipping: {}", e);
        }
        else {
            capuchin.load_register(people);
        }
    }

    let mut loaded_batting = false;
    if let Some(batting_csv) = matches.value_of("batting") {
        let batting_csv = Path::new(batting_csv);
        capuchin.load_batting(&batting_csv).expect("Failed load Batting.csv");
        loaded_batting = true;
    }
    else {
        println!("No Batting.csv, skipping batter projections.");
    }

    let mut loaded_pitching = false;
    if let Some(pitching_csv) = matches.value_of("pitching") {
        let pitching_csv = Path::new(pitching_csv);
        capuchin.load_pitching(&pitching_csv).expect("Failed load Pitching.csv");
        loaded_pitching = true;
    }
    else {
        println!("No Pitching.csv, skipping pitcher projections.");
    }

    if loaded_batting && loaded_pitching {
        capuchin.remove_out_of_position_players();
    }
    else if !loaded_batting && !loaded_pitching {
        println!("No Batting.csv nor Pitching.csv provided, exiting.");
        exit(1);
    }

    for year in &years {
        if loaded_batting {
            let b_projections = capuchin.batting_projection(*year);
            if let Err(e) = databank::write_batting_projection(&b_projections, *year) {
                println!("Unable to write batting projection for year {}: {}", year, e);
            }
        }

        if loaded_pitching {
            let p_projections = capuchin.pitching_projection(*year);
            if let Err(e) = databank::write_pitching_projection(&p_projections, *year) {
                println!("Unable to write pitching projection for year {}: {}", year, e);
            }
        }
    }
}

// Free-standing function to make it simpler to see how the weights are converted from the
// commandline arguments to something usable.
fn split_weights(weights: &str) -> Result<Vec<f32>, std::num::ParseFloatError> {
    weights.split(",").map(str::trim).map(f32::from_str).collect()
}
