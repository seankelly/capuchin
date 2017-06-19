
use std::collections::HashMap;
use std::path::Path;

use errors;

use csv;

pub struct People {
    people: Vec<PeopleRegister>,
    bbref_idx: HashMap<String, usize>,
}

#[derive(Deserialize)]
pub struct PeopleRegister {
    key_person: String,
    key_uuid: String,
    //key_mlbam: Option<String>,
    //key_retro: Option<String>,
    key_bbref: Option<String>,
    /*
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
    */
    birth_year: Option<u16>,
    birth_month: Option<u8>,
    birth_day: Option<u8>,
    /*
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
    */
}

impl People {
    pub fn new() -> Self {
        People {
            people: Vec::new(),
            bbref_idx: HashMap::new(),
        }
    }

    pub fn load_register(&mut self, people_csv: &Path) -> errors::Result<()> {
        let mut rdr = csv::Reader::from_path(people_csv)?;

        for result in rdr.deserialize() {
            let person: PeopleRegister = result?;
            let idx = self.people.len() + 1;
            if let Some(ref bbrefid) = person.key_bbref {
                self.bbref_idx.insert(bbrefid.clone(), idx);
            }
            self.people.push(person);
        }

        Ok(())
    }

    pub fn find_by_bbref(&self, key_bbref: &str) -> Option<&PeopleRegister> {
        self.bbref_idx.get(key_bbref).and_then(|idx| self.people.get(*idx))
    }
}
