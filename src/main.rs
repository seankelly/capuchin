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

#[derive(Debug)]
struct BattingSummary {
    g: u32,
    ab: u32,
    r: u32,
    h: u32,
    double: u32,
    triple: u32,
    hr: u32,
    rbi: u32,
    sb: u32,
    cs: u32,
    bb: u32,
    so: u32,
    ibb: u32,
    hbp: u32,
    sh: u32,
    sf: u32,
    gidp: u32,
}

#[derive(Debug)]
struct BattingSummaryRates {
    pa: u32,
    r: f32,
    h: f32,
    double: f32,
    triple: f32,
    hr: f32,
    rbi: f32,
    sb: f32,
    cs: f32,
    bb: f32,
    so: f32,
    ibb: f32,
    hbp: f32,
    sh: f32,
    sf: f32,
    gidp: f32,
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
    proj.create_projections();
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
            else if record.yearid >= maximum_year {
                continue;
            }
            let mut batter = self.batters.entry(record.playerid.clone())
                                .or_insert(Vec::with_capacity(self.year_weights.len()));
            batter.push(record);
        }
        Ok(())
    }

    fn create_projections(&mut self) {
        // Calculate the totals for each season to get per-PA averages.
        let mut year_summaries: HashMap<u16, BattingSummary> = HashMap::with_capacity(
            self.year_weights.len());
        for (_batter, batter_seasons) in &self.batters {
            for season in batter_seasons {
                let year = season.yearid;
                let mut summary = year_summaries.entry(year)
                                                .or_insert(BattingSummary::default());
                summary.add(season);
            }
        }

        let mut league_rates = HashMap::with_capacity(self.year_weights.len());
        for (year, season) in &year_summaries {
            let rate = BattingSummaryRates::from_summary(&season);
            league_rates.insert(year, rate);
        }
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

impl Default for BattingSummary {
    fn default() -> Self {
        BattingSummary {
            g: 0,
            ab: 0,
            r: 0,
            h: 0,
            double: 0,
            triple: 0,
            hr: 0,
            rbi: 0,
            sb: 0,
            cs: 0,
            bb: 0,
            so: 0,
            ibb: 0,
            hbp: 0,
            sh: 0,
            sf: 0,
            gidp: 0,
        }
    }
}

impl BattingSummary {
    fn add(&mut self, season: &BattingSeason) {
        self.g += season.g.into();
        self.ab += season.ab.into();
        self.r += season.r.into();
        self.h += season.h.into();
        self.double += season.double.into();
        self.triple += season.triple.into();
        self.hr += season.hr.into();
        self.rbi += season.rbi.unwrap_or(0).into();
        self.sb += season.sb.unwrap_or(0).into();
        self.cs += season.cs.unwrap_or(0).into();
        self.bb += season.bb.into();
        self.so += season.so.unwrap_or(0).into();
        self.ibb += season.ibb.unwrap_or(0).into();
        self.hbp += season.hbp.unwrap_or(0).into();
        self.sh += season.sh.unwrap_or(0).into();
        self.sf += season.sf.unwrap_or(0).into();
        self.gidp += season.gidp.unwrap_or(0).into();
    }

    fn add_seasons(&mut self, seasons: &Vec<BattingSeason>) {
        for season in seasons {
            self.add(season);
        }
    }
}

impl BattingSummaryRates {
    fn from_summary(summary: &BattingSummary) -> Self {
        let pa = summary.ab + summary.bb + summary.hbp + summary.sf + summary.sh;
        let pa_f = pa as f32;
        BattingSummaryRates {
            pa: pa,
            r: summary.r as f32 / pa_f,
            h: summary.h as f32 / pa_f,
            double: summary.double as f32 / pa_f,
            triple: summary.triple as f32 / pa_f,
            hr: summary.hr as f32 / pa_f,
            rbi: summary.rbi as f32 / pa_f,
            sb: summary.sb as f32 / pa_f,
            cs: summary.cs as f32 / pa_f,
            bb: summary.bb as f32 / pa_f,
            so: summary.so as f32 / pa_f,
            ibb: summary.ibb as f32 / pa_f,
            hbp: summary.hbp as f32 / pa_f,
            sh: summary.sh as f32 / pa_f,
            sf: summary.sf as f32 / pa_f,
            gidp: summary.gidp as f32 / pa_f,
        }
    }
}