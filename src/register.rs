
use std::path::Path;

use errors;

use csv;

struct People {
    people: Vec<PeopleRegister>,
}

#[derive(Deserialize)]
struct PeopleRegister {
    key_person: String,
    key_uuid: String,
    key_mlbam: Option<String>,
    key_retro: Option<String>,
    key_bbref: Option<String>,
    key_bbref_minors: Option<String>,
    key_fangraphs: Option<String>,
    key_npb: Option<String>,
    key_sr_nfl: Option<String>,
    key_sr_nba: Option<String>,
    key_sr_nhl: Option<String>,
    key_findagrave: Option<String>,
    name_last: Option<String>,
    name_first: Option<String>,
    name_given: Option<String>,
    name_suffix: Option<String>,
    name_matrilineal: Option<String>,
    name_nick: Option<String>,
    birth_year: Option<String>,
    birth_month: Option<String>,
    birth_day: Option<String>,
    death_year: Option<String>,
    death_month: Option<String>,
    death_day: Option<String>,
    pro_played_first: Option<String>,
    pro_played_last: Option<String>,
    mlb_played_first: Option<String>,
    mlb_played_last: Option<String>,
    col_played_first: Option<String>,
    col_played_last: Option<String>,
    pro_managed_first: Option<String>,
    pro_managed_last: Option<String>,
    mlb_managed_first: Option<String>,
    mlb_managed_last: Option<String>,
    col_managed_first: Option<String>,
    col_managed_last: Option<String>,
    pro_umpired_first: Option<String>,
    pro_umpired_last: Option<String>,
    mlb_umpired_first: Option<String>,
    mlb_umpired_last: Option<String>,
}

impl People {
    fn from_path(people_csv: &Path) -> errors::Result<Self> {
        let mut rdr = csv::Reader::from_path(people_csv)?;
        let mut people = Vec::new();

        for result in rdr.deserialize() {
            let person = result?;
            people.push(person);
        }

        Ok(People {
            people: people,
        })
    }
}
