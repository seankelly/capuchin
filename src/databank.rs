use std::collections::BTreeMap;
use std::collections::HashMap;
use std::path::Path;

use csv;

use errors;

pub struct Players {
    players: HashMap<String, Player>,
    batting: Vec<BattingSeason>,
    //pitching: Vec<PitchingSeason>,
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

struct BattingSeason {
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

impl Players {
    pub fn new() -> Self {
        Players {
            players: HashMap::new(),
            batting: Vec::new(),
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

    pub fn batting_season(&self, start_year: u16, end_year: u16) -> Vec<&BattingSeason> {
        self.batting.iter().filter(|season| start_year <= season.yearid &&
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

    fn add_ip(&mut self, year: u16, ip: u16) {
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
