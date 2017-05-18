#![recursion_limit = "1024"]

extern crate clap;
extern crate csv;
#[macro_use]
extern crate error_chain;
extern crate rustc_serialize;

use clap::{Arg, App};
use std::collections::HashMap;
use std::default::Default;
use std::path::Path;
use std::ops::Add;

mod errors {
    use csv;

    error_chain! {
        foreign_links {
            Io(csv::Error);
        }
    }
}

struct Projection {
    batters: HashMap<String, Vec<BattingSeason>>,
    peak_age: u8,
    year: u16,
    year_weights: Vec<f32>,
}

#[derive(RustcDecodable)]
struct BattingSeason {
    playerid: String,
    yearid: u16,
    stint: String,
    teamid: String,
    lgid: String,
    g: u8,
    ab: u16,
    r: u8,
    h: u16,
    double: u8,
    triple: u8,
    hr: u8,
    rbi: Option<u8>,
    sb: Option<u8>,
    cs: Option<u8>,
    bb: u16,
    so: Option<u16>,
    ibb: Option<u8>,
    hbp: Option<u8>,
    sh: Option<u8>,
    sf: Option<u8>,
    gidp: Option<u8>,
}


fn main() {
    let app = App::new("Capuchin")
        .version("0.1.0")
        .about("Simple baseball projections")
        .arg(Arg::with_name("batting")
             .short("b")
             .long("batting")
             .value_name("FILE")
             .required(true)
             .help("Batting.csv file")
             .takes_value(true))
        .arg(Arg::with_name("year")
             .short("y")
             .long("year")
             .value_name("YEAR")
             .required(true)
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

    let batting_csv = Path::new(matches.value_of("batting").expect("No Batting.csv file."));
    let projection_year = matches.value_of("year").expect("Need a year to project.")
                            .parse().expect("Expected year to be an integer.");
    let mut proj = Projection::default();
    proj.year = projection_year;
    proj.load_batting_season(batting_csv).expect("Failed loading Batting.csv");
}

impl Projection {
    fn load_batting_season(&mut self, batting_csv: &Path) -> errors::Result<()> {
        let mut rdr = csv::Reader::from_file(batting_csv)?;
        let minimum_year = self.year - self.year_weights.len() as u16;
        let maximum_year = self.year;
        for record in rdr.decode() {
            let record: BattingSeason = record?;
            if record.yearid < minimum_year {
                continue;
            }
            else if maximum_year >= record.yearid {
                continue;
            }
            let mut batter = self.batters.entry(record.playerid.clone()).or_insert(Vec::new());
            batter.push(record);
        }
        Ok(())
    }
}

impl Default for Projection {
    fn default() -> Self {
        Projection {
            batters: HashMap::new(),
            peak_age: 29,
            year: 0,
            year_weights: vec![5.0, 4.0, 3.0],
        }
    }
}

impl Add for BattingSeason {
    type Output = BattingSeason;

    fn add(self, other: BattingSeason) -> BattingSeason {
        assert!(self.playerid == other.playerid);

        fn add_option<T: Add<Output=T>>(a: Option<T>, b: Option<T>) -> Option<T> {
            match (a, b) {
                (Some(x), Some(y)) => Some(x+y),
                (Some(x), None) => Some(x),
                (None, Some(y)) => Some(y),
                (None, None) => None,
            }
        }

        BattingSeason {
            playerid: self.playerid,
            yearid: self.yearid,
            stint: self.stint,
            teamid: self.teamid,
            lgid: self.lgid,
            g: self.g + other.g,
            ab: self.ab + other.ab,
            r: self.r + other.r,
            h: self.h + other.h,
            double: self.double + other.double,
            triple: self.triple + other.triple,
            hr: self.hr + other.hr,
            rbi: add_option(self.rbi, other.rbi),
            sb: add_option(self.sb, other.sb),
            cs: add_option(self.cs, other.cs),
            bb: self.bb + other.bb,
            so: add_option(self.so, other.so),
            ibb: add_option(self.ibb, other.ibb),
            hbp: add_option(self.hbp, other.hbp),
            sh: add_option(self.sh, other.sh),
            sf: add_option(self.sf, other.sf),
            gidp: add_option(self.gidp, other.gidp),
        }
    }
}
