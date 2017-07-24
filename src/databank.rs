use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::path::Path;

use csv;

use errors;

pub struct Players {
    players: HashMap<String, Player>,
    batting: Vec<BattingSeason>,
    pitching: Vec<PitchingSeason>,
}

pub struct Player {
    ip: BTreeMap<u16, u16>,
    pa: BTreeMap<u16, u16>,
}

#[derive(Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
struct RawBattingSeason {
    #[serde(rename = "playerID")]
    playerid: String,
    #[serde(rename = "yearID")]
    yearid: u16,
    /*
    #[serde(rename = "stint")]
    stint: String,
    #[serde(rename = "teamID")]
    teamid: String,
    #[serde(rename = "lgID")]
    lgid: String,
    */
    g: u8,
    ab: u16,
    r: u8,
    h: u16,
    #[serde(rename = "2B")]
    double: u8,
    #[serde(rename = "3B")]
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
pub struct BattingSeason {
    playerid: String,
    yearid: u16,
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

#[derive(Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
struct RawPitchingSeason {
    #[serde(rename = "playerID")]
    playerid: String,
    #[serde(rename = "yearID")]
    yearid: u16,
    /*
    #[serde(rename = "stint")]
    stint: String,
    #[serde(rename = "teamID")]
    teamid: String,
    #[serde(rename = "lgID")]
    lgid: String,
    */
    w: u8,
    l: u8,
    /*
    g: u8,
    gs: u8,
    cg: u8,
    sho: u8,
    */
    sv: u8,
    #[serde(rename = "IPouts")]
    ipouts: u16,
    h: u16,
    r: u16,
    er: u16,
    hr: u8,
    bb: u16,
    so: u16,
    /*
    #[serde(rename = "BAOpp")]
    baopp: Option<f32>,
    */
    era: Option<f32>,
    ibb: Option<u8>,
    wp: Option<u8>,
    hbp: Option<u8>,
    bk: u8,
    /*
    bfp: Option<u8>,
    gf: Option<u8>,
    sh: Option<u8>,
    sf: Option<u8>,
    gidp: Option<u8>,
    */
}

#[derive(Debug)]
pub struct PitchingSeason {
    playerid: String,
    yearid: u16,
    ip: u16,
    w: u8,
    l: u8,
    sv: u8,
    h: u16,
    r: u16,
    er: u16,
    era: f32,
    hr: u8,
    so: u16,
    bb: u16,
    ibb: u8,
    hbp: u8,
    wp: u8,
    bk: u8,
}

#[derive(Debug)]
pub struct BattingSeasonSummary {
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
pub struct PitchingSeasonSummary {
    ip: u32,
    w: u32,
    l: u32,
    sv: u32,
    h: u32,
    r: u32,
    er: u32,
    era: f32,
    hr: u32,
    so: u32,
    bb: u32,
    ibb: u32,
    hbp: u32,
    wp: u32,
    bk: u32,
}

#[derive(Debug)]
pub struct BattingSeasonSummaryRates {
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

#[derive(Debug)]
pub struct PitchingSeasonSummaryRates {
    ip: u32,
    w: f32,
    l: f32,
    sv: f32,
    h: f32,
    r: f32,
    er: f32,
    era: f32,
    hr: f32,
    so: f32,
    bb: f32,
    ibb: f32,
    hbp: f32,
    wp: f32,
    bk: f32,
}

#[derive(Debug, Serialize)]
pub struct BattingProjection {
    playerid: String,
    year: u16,
    reliability: f32,
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

#[derive(Debug, Serialize)]
pub struct PitchingProjection {
    playerid: String,
    year: u16,
    reliability: f32,
    ip: f32,
    w: f32,
    l: f32,
    sv: f32,
    h: f32,
    r: f32,
    er: f32,
    era: f32,
    hr: f32,
    so: f32,
    bb: f32,
    ibb: f32,
    hbp: f32,
    wp: f32,
    bk: f32,
}

impl Players {
    pub fn new() -> Self {
        Players {
            players: HashMap::new(),
            batting: Vec::new(),
            pitching: Vec::new(),
        }
    }

    pub fn load_batting(&mut self, batting_csv: &Path) -> errors::Result<()> {
        let mut rdr = csv::Reader::from_path(batting_csv)?;
        for record in rdr.deserialize() {
            let record: RawBattingSeason = record?;
            let record = BattingSeason::from(record);
            let mut player = self.players.entry(record.playerid.clone())
                .or_insert(Player::new());
            player.add_pa(&record);
            self.batting.push(record);
        }

        Ok(())
    }

    pub fn batting_seasons(&self, start_year: u16, end_year: u16) -> Vec<&BattingSeason> {
        self.batting.iter().filter(|season| start_year <= season.yearid &&
                                            season.yearid <= end_year)
            .collect()
    }

    pub fn load_pitching(&mut self, pitching_csv: &Path) -> errors::Result<()> {
        let mut rdr = csv::Reader::from_path(pitching_csv)?;
        for record in rdr.deserialize() {
            let record: RawPitchingSeason = record?;
            let record = PitchingSeason::from(record);
            let mut player = self.players.entry(record.playerid.clone())
                .or_insert(Player::new());
            player.add_ip(&record);
            self.pitching.push(record);
        }

        Ok(())
    }

    pub fn pitching_seasons(&self, start_year: u16, end_year: u16) -> Vec<&PitchingSeason> {
        self.pitching.iter().filter(|season| start_year <= season.yearid &&
                                            season.yearid <= end_year)
            .collect()
    }
}

impl Player {
    fn new() -> Self {
        Player {
            ip: BTreeMap::new(),
            pa: BTreeMap::new(),
        }
    }

    fn add_ip(&mut self, record: &PitchingSeason) {
        let year = record.yearid;
        let ip = record.ip;
        let mut season_ip = self.ip.entry(year).or_insert(0);
        *season_ip += ip;
    }

    fn add_pa(&mut self, record: &BattingSeason) {
        let year = record.yearid;
        let pa = record.pa;
        let mut season_pa = self.pa.entry(year).or_insert(0);
        *season_pa += pa;
    }
}

impl From<RawBattingSeason> for BattingSeason {
    fn from(csv: RawBattingSeason) -> BattingSeason {
        BattingSeason {
            playerid: csv.playerid,
            yearid: csv.yearid,
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

impl From<RawPitchingSeason> for PitchingSeason {
    fn from(csv: RawPitchingSeason) -> PitchingSeason {
        // Convert the outs into IP and then into an integer number of IP.
        let ip = csv.ipouts as f32 / 3.0;
        let ip = ip.round() as u16;
        PitchingSeason {
            playerid: csv.playerid,
            yearid: csv.yearid,
            w: csv.w,
            l: csv.l,
            sv: csv.sv,
            ip: ip,
            h: csv.h,
            r: csv.r,
            er: csv.er,
            hr: csv.hr,
            bb: csv.bb,
            so: csv.so,
            era: csv.era.unwrap_or(0.0),
            ibb: csv.ibb.unwrap_or(0),
            wp: csv.wp.unwrap_or(0),
            hbp: csv.hbp.unwrap_or(0),
            bk: csv.bk,
        }
    }
}

impl BattingSeason {
    pub fn playerid(&self) -> &String {
        &self.playerid
    }

    pub fn yearid(&self) -> &u16 {
        &self.yearid
    }

    pub fn is_year(&self, year: u16) -> bool {
        self.yearid == year
    }
}

impl PitchingSeason {
    pub fn playerid(&self) -> &String {
        &self.playerid
    }

    pub fn yearid(&self) -> &u16 {
        &self.yearid
    }

    pub fn is_year(&self, year: u16) -> bool {
        self.yearid == year
    }
}

impl BattingSeasonSummary {
    pub fn new() -> Self {
        BattingSeasonSummary {
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

    pub fn pa(&self) -> &u32 {
        &self.pa
    }

    pub fn add_season(&self, season: &BattingSeason) -> Self {
        BattingSeasonSummary {
            g: self.g + season.g as u32,
            pa: self.pa + season.pa as u32,
            ab: self.ab + season.ab as u32,
            r: self.r + season.r as u32,
            h: self.h + season.h as u32,
            double: self.double + season.double as u32,
            triple: self.triple + season.triple as u32,
            hr: self.hr + season.hr as u32,
            rbi: self.rbi + season.rbi as u32,
            sb: self.sb + season.sb as u32,
            cs: self.cs + season.cs as u32,
            bb: self.bb + season.bb as u32,
            so: self.so + season.so as u32,
            ibb: self.ibb + season.ibb as u32,
            hbp: self.hbp + season.hbp as u32,
            sh: self.sh + season.sh as u32,
            sf: self.sf + season.sf as u32,
            gidp: self.gidp + season.gidp as u32,
        }
    }

    pub fn mut_add_season(&mut self, season: &BattingSeason) {
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

impl PitchingSeasonSummary {
    pub fn new() -> Self {
        PitchingSeasonSummary {
            ip: 0,
            w: 0,
            l: 0,
            sv: 0,
            h: 0,
            r: 0,
            er: 0,
            era: 0.0,
            hr: 0,
            so: 0,
            bb: 0,
            ibb: 0,
            hbp: 0,
            wp: 0,
            bk: 0,
        }
    }

    pub fn ip(&self) -> &u32 {
        &self.ip
    }

    pub fn add_season(&self, season: &PitchingSeason) -> Self {
        PitchingSeasonSummary {
            ip: self.ip + season.ip as u32,
            w: self.w + season.w as u32,
            l: self.l + season.l as u32,
            sv: self.sv + season.sv as u32,
            h: self.h + season.h as u32,
            r: self.r + season.r as u32,
            er: self.er + season.er as u32,
            era: self.er as f32 * 9.0 / self.ip as f32,
            hr: self.hr + season.hr as u32,
            so: self.so + season.so as u32,
            bb: self.bb + season.bb as u32,
            ibb: self.ibb + season.ibb as u32,
            hbp: self.hbp + season.hbp as u32,
            wp: self.wp + season.wp as u32,
            bk: self.bk + season.bk as u32,
        }
    }

    pub fn mut_add_season(&mut self, season: &PitchingSeason) {
        self.ip += season.ip.into();
        self.w += season.w.into();
        self.l += season.l.into();
        self.sv += season.sv.into();
        self.h += season.h.into();
        self.r += season.r.into();
        self.er += season.er.into();
        self.era = self.er as f32 * 9.0 / self.ip as f32;
        self.hr += season.hr.into();
        self.so += season.so.into();
        self.bb += season.bb.into();
        self.ibb += season.ibb.into();
        self.hbp += season.hbp.into();
        self.wp += season.wp.into();
        self.bk += season.bk.into();
    }
}

impl From<BattingSeasonSummary> for BattingSeasonSummaryRates {
    fn from(summary: BattingSeasonSummary) -> BattingSeasonSummaryRates {
        let pa_f = summary.pa as f32;
        BattingSeasonSummaryRates {
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

impl From<PitchingSeasonSummary> for PitchingSeasonSummaryRates {
    fn from(summary: PitchingSeasonSummary) -> PitchingSeasonSummaryRates {
        let ip_f = summary.ip as f32;
        PitchingSeasonSummaryRates {
            ip: summary.ip,
            w: summary.w as f32 / ip_f,
            l: summary.l as f32 / ip_f,
            sv: summary.sv as f32 / ip_f,
            h: summary.h as f32 / ip_f,
            r: summary.r as f32 / ip_f,
            er: summary.er as f32 / ip_f,
            era: summary.era,
            hr: summary.hr as f32 / ip_f,
            so: summary.so as f32 / ip_f,
            bb: summary.bb as f32 / ip_f,
            ibb: summary.ibb as f32 / ip_f,
            hbp: summary.hbp as f32 / ip_f,
            wp: summary.wp as f32 / ip_f,
            bk: summary.bk as f32 / ip_f,
        }
    }
}

impl BattingProjection {
    pub fn new_player(playerid: &String, year: u16) -> Self {
        BattingProjection {
            playerid: playerid.clone(),
            year: year,
            reliability: 0.0,
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

    pub fn league() -> Self {
        BattingProjection {
            playerid: String::from(""),
            year: 0,
            reliability: 0.0,
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

    pub fn regress(&mut self, proj: &Self) {
        self.reliability = self.pa / (self.pa + proj.pa);
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

    pub fn weighted_add(&mut self, season: &BattingSeasonSummary, weight: f32) {
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

    pub fn weighted_rate_add(&mut self, pa: u16, rates: &BattingSeasonSummaryRates, weight: f32) {
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

    pub fn prorate(&self, prorated_pa: u16) -> Self {
        let pa_f = prorated_pa as f32;
        let pa_factor = pa_f / self.pa;
        BattingProjection {
            playerid: self.playerid.clone(),
            year: self.year,
            reliability: self.reliability,
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

    pub fn round(&mut self) {
        self.pa = self.pa.round();
        self.ab = self.ab.round();
        self.r = self.r.round();
        self.h = self.h.round();
        self.double = self.double.round();
        self.triple = self.triple.round();
        self.hr = self.hr.round();
        self.rbi = self.rbi.round();
        self.sb = self.sb.round();
        self.cs = self.cs.round();
        self.bb = self.bb.round();
        self.so = self.so.round();
        self.ibb = self.ibb.round();
        self.hbp = self.hbp.round();
        self.sh = self.sh.round();
        self.sf = self.sf.round();
        self.gidp = self.gidp.round();
    }

    pub fn age_adjust(&mut self, amount: f32) {
        self.r *= amount;
        self.h *= amount;
        self.double *= amount;
        self.triple *= amount;
        self.hr *= amount;
        self.rbi *= amount;
        self.sb *= amount;
        self.cs *= amount;
        self.bb *= amount;
        self.so *= amount;
        self.ibb *= amount;
        self.hbp *= amount;
        self.sh *= amount;
        self.sf *= amount;
        self.gidp *= amount;
    }
}

impl Eq for BattingProjection {}

impl Ord for BattingProjection {
    fn cmp(&self, other: &Self) -> Ordering {
        self.playerid.cmp(&other.playerid)
    }
}

impl PartialOrd for BattingProjection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for BattingProjection {
    fn eq(&self, other: &Self) -> bool {
        self.playerid == other.playerid
    }
}

impl PitchingProjection {
    pub fn new_player(playerid: &String, year: u16) -> Self {
        PitchingProjection {
            playerid: playerid.clone(),
            year: year,
            reliability: 0.0,
            ip: 0.0,
            w: 0.0,
            l: 0.0,
            sv: 0.0,
            h: 0.0,
            r: 0.0,
            er: 0.0,
            era: 0.0,
            hr: 0.0,
            so: 0.0,
            bb: 0.0,
            ibb: 0.0,
            hbp: 0.0,
            wp: 0.0,
            bk: 0.0,
        }
    }

    pub fn league() -> Self {
        PitchingProjection {
            playerid: String::from(""),
            year: 0,
            reliability: 0.0,
            ip: 0.0,
            w: 0.0,
            l: 0.0,
            sv: 0.0,
            h: 0.0,
            r: 0.0,
            er: 0.0,
            era: 0.0,
            hr: 0.0,
            so: 0.0,
            bb: 0.0,
            ibb: 0.0,
            hbp: 0.0,
            wp: 0.0,
            bk: 0.0,
        }
    }

    pub fn regress(&mut self, proj: &Self) {
        self.reliability = self.ip / (self.ip + proj.ip);
        self.ip += proj.ip;
        self.w += proj.w;
        self.l += proj.l;
        self.sv += proj.sv;
        self.h += proj.h;
        self.r += proj.r;
        self.er += proj.er;
        self.era = self.er * 9.0 / self.ip;
        self.hr += proj.hr;
        self.so += proj.so;
        self.bb += proj.bb;
        self.ibb += proj.ibb;
        self.hbp += proj.hbp;
        self.wp += proj.wp;
        self.bk += proj.bk;
    }

    pub fn weighted_add(&mut self, season: &PitchingSeasonSummary, weight: f32) {
        self.ip += season.ip as f32 * weight;
        self.w += season.w as f32 * weight;
        self.l += season.l as f32 * weight;
        self.sv += season.sv as f32 * weight;
        self.h += season.h as f32 * weight;
        self.r += season.r as f32 * weight;
        self.er += season.er as f32 * weight;
        self.era = self.er * 9.0 / self.ip;
        self.hr += season.hr as f32 * weight;
        self.so += season.so as f32 * weight;
        self.bb += season.bb as f32 * weight;
        self.ibb += season.ibb as f32 * weight;
        self.hbp += season.hbp as f32 * weight;
        self.wp += season.wp as f32 * weight;
        self.bk += season.bk as f32 * weight;
    }

    pub fn weighted_rate_add(&mut self, ip: u16, rates: &PitchingSeasonSummaryRates, weight: f32) {
        let ip_f = ip as f32;
        self.ip += ip_f * weight;
        self.w += ip_f * rates.w * weight;
        self.l += ip_f * rates.l * weight;
        self.sv += ip_f * rates.sv * weight;
        self.h += ip_f * rates.h * weight;
        self.r += ip_f * rates.r * weight;
        self.er += ip_f * rates.er * weight;
        self.era = self.er * 9.0 / self.ip;
        self.hr += ip_f * rates.hr * weight;
        self.so += ip_f * rates.so * weight;
        self.bb += ip_f * rates.bb * weight;
        self.ibb += ip_f * rates.ibb * weight;
        self.hbp += ip_f * rates.hbp * weight;
        self.wp += ip_f * rates.wp * weight;
        self.bk += ip_f * rates.bk * weight;
    }

    pub fn prorate(&self, prorated_ip: u16) -> Self {
        let ip_f = prorated_ip as f32;
        let ip_factor = ip_f / self.ip;
        PitchingProjection {
            playerid: self.playerid.clone(),
            year: self.year,
            reliability: self.reliability,
            ip: ip_f,
            w: self.w * ip_factor,
            l: self.l * ip_factor,
            sv: self.sv * ip_factor,
            h: self.h * ip_factor,
            r: self.r * ip_factor,
            er: self.er * ip_factor,
            era: self.er * 9.0 / ip_f,
            hr: self.hr * ip_factor,
            so: self.so * ip_factor,
            bb: self.bb * ip_factor,
            ibb: self.ibb * ip_factor,
            hbp: self.hbp * ip_factor,
            wp: self.wp * ip_factor,
            bk: self.bk * ip_factor,
        }
    }

    pub fn round(&mut self) {
        self.ip = self.ip.round();
        self.w = self.w.round();
        self.l = self.l.round();
        self.sv = self.sv.round();
        self.h = self.h.round();
        self.r = self.r.round();
        self.er = self.er.round();
        // Ideally round the ERA to two digits of precision.
        self.hr = self.hr.round();
        self.so = self.so.round();
        self.bb = self.bb.round();
        self.ibb = self.ibb.round();
        self.hbp = self.hbp.round();
        self.wp = self.wp.round();
        self.bk = self.bk.round();
    }

    pub fn age_adjust(&mut self, amount: f32) {
        self.ip *= amount;
        self.w *= amount;
        self.l *= amount;
        self.sv *= amount;
        self.h *= amount;
        self.r *= amount;
        self.er *= amount;
        self.era = self.er * 9.0 / self.ip;
        self.hr *= amount;
        self.so *= amount;
        self.bb *= amount;
        self.ibb *= amount;
        self.hbp *= amount;
        self.wp *= amount;
        self.bk *= amount;
    }
}

impl Eq for PitchingProjection {}

impl Ord for PitchingProjection {
    fn cmp(&self, other: &Self) -> Ordering {
        self.playerid.cmp(&other.playerid)
    }
}

impl PartialOrd for PitchingProjection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PitchingProjection {
    fn eq(&self, other: &Self) -> bool {
        self.playerid == other.playerid
    }
}

pub fn write_batting_projection(projections: &Vec<BattingProjection>, year: u16) -> errors::Result<()> {
    let output_file = format!("BattingCapuchin{}.csv", year);
    let output_path = Path::new(&output_file);
    let mut wtr = csv::Writer::from_path(&output_path)?;

    for projection in projections {
        let _result = wtr.serialize(projection)?;
    }

    Ok(())
}

pub fn write_pitching_projection(projections: &Vec<PitchingProjection>, year: u16) -> errors::Result<()> {
    let output_file = format!("PitchingCapuchin{}.csv", year);
    let output_path = Path::new(&output_file);
    let mut wtr = csv::Writer::from_path(&output_path)?;

    for projection in projections {
        let _result = wtr.serialize(projection)?;
    }

    Ok(())
}
