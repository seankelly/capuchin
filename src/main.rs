extern crate clap;

use clap::{Arg, App};
use std::default::Default;

struct ProjectionOptions {
    peak_age: u16,
    year_weights: Vec<f32>,
}

impl Default for ProjectionOptions {
    fn default() -> Self {
        ProjectionOptions {
            peak_age: 29,
            year_weights: vec![5.0, 4.0, 3.0],
        }
    }
}


fn main() {
    let app = App::new("Capuchin")
        .version("0.1.0")
        .about("Simple baseball projections")
        .arg(Arg::with_name("year")
             .short("y")
             .long("year")
             .value_name("YEAR")
             .help("Year to project")
             .takes_value(true))
        .arg(Arg::with_name("peak_age")
             .short("a")
             .long("peak-age")
             .value_name("AGE")
             .help("Peak age for player")
             .takes_value(true))
        ;
    let matches = app.get_matches();
}
