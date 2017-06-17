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
    pa: u32,
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

#[derive(Debug, RustcEncodable)]
struct BattingProjection {
    playerid: String,
    year: u16,
    pa: f32,
    ab: f32,
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
    let projections = proj.create_projections();
    write_batting_projection(&projections, proj.year);
}

fn write_batting_projection(projections: &HashMap<String, BattingProjection>, year: u16) -> errors::Result<()> {
    let output_file = format!("BattingCapuchin{}.csv", year);
    let output_path = Path::new(&output_file);
    let mut wtr = csv::Writer::from_file(&output_path)?;

    for (_batter, projection) in projections {
        let _result = wtr.encode(projection)?;
    }

    Ok(())
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

    fn create_projections(&mut self) -> HashMap<String, BattingProjection> {
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
        let mut player_projections = HashMap::with_capacity(self.batters.len());
        for (batter, batter_seasons) in &self.batters {
            // Weighted batter seasons.
            let mut weighted_batter = BattingProjection::new_player(&batter, self.year);
            // What the league did with the same PAs, weighted the same.
            let mut batter_league_mean = BattingProjection::default();
            let mut projected_pa = 200.0;
            for season in batter_seasons {
                let year = season.yearid;
                projected_pa += match self.year - year {
                    1 => 0.5 * season.pa as f32,
                    2 => 0.1 * season.pa as f32,
                    _ => 0.0,
                };
                let weight = weights_map.get(&year).unwrap_or(&default_weight);
                weighted_batter.weighted_add(season, *weight);

                let league_rate = league_rates.get(&year)
                    .expect("Expected to get a rate for this year.");
                batter_league_mean.weighted_rate_add(season.pa, league_rate, *weight);
            }

            let projected_pa = projected_pa as u16;
            let prorated_league_mean = batter_league_mean.prorate(self.batter_regress);
            // Merge weighted player and league totals to regress the player.
            weighted_batter.add(&prorated_league_mean);

            let projection = weighted_batter.prorate(projected_pa);
            player_projections.insert(batter.clone(), projection);
        }

        return player_projections;
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
            pa: 0,
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

impl Default for BattingProjection {
    fn default() -> Self {
        BattingProjection {
            playerid: String::from(""),
            year: 0,
            pa: 0.0,
            ab: 0.0,
            r: 0.0,
            h: 0.0,
            double: 0.0,
            triple: 0.0,
            hr: 0.0,
            rbi: 0.0,
            sb: 0.0,
            cs: 0.0,
            bb: 0.0,
            so: 0.0,
            ibb: 0.0,
            hbp: 0.0,
            sh: 0.0,
            sf: 0.0,
            gidp: 0.0,
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
        self.pa += season.pa.into();
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
}

impl BattingSummaryRates {
    fn from_summary(summary: &BattingSummary) -> Self {
        let pa_f = summary.pa as f32;
        BattingSummaryRates {
            pa: summary.pa,
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

impl BattingProjection {
    fn new_player(playerid: &String, year: u16) -> Self {
        BattingProjection {
            playerid: playerid.clone(),
            year: year,
            pa: 0.0,
            ab: 0.0,
            r: 0.0,
            h: 0.0,
            double: 0.0,
            triple: 0.0,
            hr: 0.0,
            rbi: 0.0,
            sb: 0.0,
            cs: 0.0,
            bb: 0.0,
            so: 0.0,
            ibb: 0.0,
            hbp: 0.0,
            sh: 0.0,
            sf: 0.0,
            gidp: 0.0,
        }
    }

    fn add(&mut self, proj: &Self) {
        self.pa += proj.pa;
        self.ab += proj.ab;
        self.r += proj.r;
        self.h += proj.h;
        self.double += proj.double;
        self.triple += proj.triple;
        self.hr += proj.hr;
        self.rbi += proj.rbi;
        self.sb += proj.sb;
        self.cs += proj.cs;
        self.bb += proj.bb;
        self.so += proj.so;
        self.ibb += proj.ibb;
        self.hbp += proj.hbp;
        self.sh += proj.sh;
        self.sf += proj.sf;
        self.gidp += proj.gidp;
    }

    fn weighted_add(&mut self, season: &BattingSeason, weight: f32) {
        self.pa += season.pa as f32 * weight;
        self.ab += season.ab as f32 * weight;
        self.r += season.r as f32 * weight;
        self.h += season.h as f32 * weight;
        self.double += season.double as f32 * weight;
        self.triple += season.triple as f32 * weight;
        self.hr += season.hr as f32 * weight;
        self.rbi += season.rbi as f32 * weight;
        self.sb += season.sb as f32 * weight;
        self.cs += season.cs as f32 * weight;
        self.bb += season.bb as f32 * weight;
        self.so += season.so as f32 * weight;
        self.ibb += season.ibb as f32 * weight;
        self.hbp += season.hbp as f32 * weight;
        self.sh += season.sh as f32 * weight;
        self.sf += season.sf as f32 * weight;
        self.gidp += season.gidp as f32 * weight;
    }

    fn weighted_rate_add(&mut self, pa: u16, rates: &BattingSummaryRates, weight: f32) {
        let pa_f = pa as f32;
        self.pa += pa_f * weight;
        self.ab += 0.0;
        self.r += pa_f * rates.r * weight;
        self.h += pa_f * rates.h * weight;
        self.double += pa_f * rates.double * weight;
        self.triple += pa_f * rates.triple * weight;
        self.hr += pa_f * rates.hr * weight;
        self.rbi += pa_f * rates.rbi * weight;
        self.sb += pa_f * rates.sb * weight;
        self.cs += pa_f * rates.cs * weight;
        self.bb += pa_f * rates.bb * weight;
        self.so += pa_f * rates.so * weight;
        self.ibb += pa_f * rates.ibb * weight;
        self.hbp += pa_f * rates.hbp * weight;
        self.sh += pa_f * rates.sh * weight;
        self.sf += pa_f * rates.sf * weight;
        self.gidp += pa_f * rates.gidp * weight;
    }

    fn prorate(&self, prorated_pa: u16) -> Self {
        let pa_f = prorated_pa as f32;
        let pa_factor = pa_f / self.pa;
        BattingProjection {
            playerid: self.playerid.clone(),
            year: self.year,
            pa: pa_f,
            ab: self.ab * pa_factor,
            r: self.r * pa_factor,
            h: self.h * pa_factor,
            double: self.double * pa_factor,
            triple: self.triple * pa_factor,
            hr: self.hr * pa_factor,
            rbi: self.rbi * pa_factor,
            sb: self.sb * pa_factor,
            cs: self.cs * pa_factor,
            bb: self.bb * pa_factor,
            so: self.so * pa_factor,
            ibb: self.ibb * pa_factor,
            hbp: self.hbp * pa_factor,
            sh: self.sh * pa_factor,
            sf: self.sf * pa_factor,
            gidp: self.gidp * pa_factor,
        }
    }
}
