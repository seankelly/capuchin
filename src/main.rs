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
    batter_regress: u16,
}

#[derive(RustcDecodable)]
struct RawBattingSeason {
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

struct BattingSeason {
    playerid: String,
    yearid: u16,
    stint: String,
    teamid: String,
    lgid: String,
    g: u8,
    pa: u16,
    ab: u16,
    r: u8,
    h: u16,
    double: u8,
    triple: u8,
    hr: u8,
    rbi: u8,
    sb: u8,
    cs: u8,
    bb: u16,
    so: u16,
    ibb: u8,
    hbp: u8,
    sh: u8,
    sf: u8,
    gidp: u8,
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
            let record: RawBattingSeason = record?;
            let record = BattingSeason::from(record);
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

        for (_batter, batter_seasons) in self.batters.iter_mut() {
            batter_seasons.sort_by(|a, b| a.yearid.cmp(&b.yearid));
        }

        Ok(())
    }

    fn create_projections(&mut self) {
        // Calculate the totals for each season to get per-PA averages.
        let number_years = self.year_weights.len();
        let mut year_summaries = HashMap::with_capacity(number_years);
        for (_batter, batter_seasons) in &self.batters {
            for season in batter_seasons {
                let year = season.yearid;
                let mut summary = year_summaries.entry(year)
                                                .or_insert(BattingSummary::default());
                summary.add(season);
            }
        }

        let mut league_rates = HashMap::with_capacity(number_years);
        for (year, season) in &year_summaries {
            let rate = BattingSummaryRates::from_summary(&season);
            league_rates.insert(year, rate);
        }
        let league_rates = league_rates;

        for (year, r) in &league_rates {
            println!("year: {:?}, league rates: {:?}", year, r);
        }

        // Map the years to the weight to use for that year.
        let mut weights_map = HashMap::with_capacity(number_years);
        for (i, weight) in self.year_weights.iter().enumerate() {
            weights_map.insert(self.year - (i + 1) as u16, *weight);
        }
        let weights_map = weights_map;

        // Weight player and league based on PA.
        let default_weight = 0.0;
        for (_batter, batter_seasons) in &self.batters {
            // Weighted batter seasons.
            let mut weighted_batter = BattingSummary::default();
            // What the league did with the same PAs, weighted the same.
            let mut batter_league_mean = BattingSummary::default();
            for season in batter_seasons {
                let year = season.yearid;
                let weight = weights_map.get(&year).unwrap_or(&default_weight);
                weighted_batter.weighted_add(season, *weight);

                let league_rate = league_rates.get(&year)
                    .expect("Expected to get a rate for this year.");
                batter_league_mean.weighted_rate_add(season.pa, league_rate, *weight);
            }

            // Merge weighted player and league totals to regress the player.
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
            batter_regress: 1200,
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

impl From<RawBattingSeason> for BattingSeason {
    fn from(csv: RawBattingSeason) -> BattingSeason {
        BattingSeason {
            playerid: csv.playerid,
            yearid: csv.yearid,
            stint: csv.stint,
            teamid: csv.teamid,
            lgid: csv.lgid,
            g: csv.g,
            pa: csv.ab + csv.bb + csv.hbp.unwrap_or(0) as u16 +
                csv.sf.unwrap_or(0) as u16 + csv.sh.unwrap_or(0) as u16,
            ab: csv.ab,
            r: csv.r,
            h: csv.h,
            double: csv.double,
            triple: csv.triple,
            hr: csv.hr,
            rbi: csv.rbi.unwrap_or(0),
            sb: csv.sb.unwrap_or(0),
            cs: csv.cs.unwrap_or(0),
            bb: csv.bb,
            so: csv.so.unwrap_or(0),
            ibb: csv.ibb.unwrap_or(0),
            hbp: csv.hbp.unwrap_or(0),
            sh: csv.sh.unwrap_or(0),
            sf: csv.sf.unwrap_or(0),
            gidp: csv.gidp.unwrap_or(0),
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
        self.rbi += season.rbi.into();
        self.sb += season.sb.into();
        self.cs += season.cs.into();
        self.bb += season.bb.into();
        self.so += season.so.into();
        self.ibb += season.ibb.into();
        self.hbp += season.hbp.into();
        self.sh += season.sh.into();
        self.sf += season.sf.into();
        self.gidp += season.gidp.into();
    }

    fn weighted_add(&mut self, season: &BattingSeason, weight: f32) {
        self.g += (season.g as f32 * weight) as u32;
        self.ab += (season.ab as f32 * weight) as u32;
        self.r += (season.r as f32 * weight) as u32;
        self.h += (season.h as f32 * weight) as u32;
        self.double += (season.double as f32 * weight) as u32;
        self.triple += (season.triple as f32 * weight) as u32;
        self.hr += (season.hr as f32 * weight) as u32;
        self.rbi += (season.rbi as f32 * weight) as u32;
        self.sb += (season.sb as f32 * weight) as u32;
        self.cs += (season.cs as f32 * weight) as u32;
        self.bb += (season.bb as f32 * weight) as u32;
        self.so += (season.so as f32 * weight) as u32;
        self.ibb += (season.ibb as f32 * weight) as u32;
        self.hbp += (season.hbp as f32 * weight) as u32;
        self.sh += (season.sh as f32 * weight) as u32;
        self.sf += (season.sf as f32 * weight) as u32;
        self.gidp += (season.gidp as f32 * weight) as u32;
    }

    fn weighted_rate_add(&mut self, pa: u16, rates: &BattingSummaryRates, weight: f32) {
        self.g += 0;
        self.ab += 0;
        self.r += (pa as f32 * rates.r * weight) as u32;
        self.h += (pa as f32 * rates.h * weight) as u32;
        self.double += (pa as f32 * rates.double * weight) as u32;
        self.triple += (pa as f32 * rates.triple * weight) as u32;
        self.hr += (pa as f32 * rates.hr * weight) as u32;
        self.rbi += (pa as f32 * rates.rbi * weight) as u32;
        self.sb += (pa as f32 * rates.sb * weight) as u32;
        self.cs += (pa as f32 * rates.cs * weight) as u32;
        self.bb += (pa as f32 * rates.bb * weight) as u32;
        self.so += (pa as f32 * rates.so * weight) as u32;
        self.ibb += (pa as f32 * rates.ibb * weight) as u32;
        self.hbp += (pa as f32 * rates.hbp * weight) as u32;
        self.sh += (pa as f32 * rates.sh * weight) as u32;
        self.sf += (pa as f32 * rates.sf * weight) as u32;
        self.gidp += (pa as f32 * rates.gidp * weight) as u32;
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
