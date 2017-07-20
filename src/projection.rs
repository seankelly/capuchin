use std::collections::HashMap;
use std::collections::BTreeMap;
use std::path::Path;

use errors;
use register;
use databank;

pub struct Capuchin {
    batter_regress: u16,
    peak_age: u8,
    year_weights: Vec<f32>,
    people: Option<register::People>,
    players: databank::Players,
    batting_league_totals: BTreeMap<u16, databank::BattingSeasonSummaryRates>,
    //pitching_league_totals: BTreeMap<u16, databank::PitchingSeasonSummary>,
}

impl Capuchin {
    pub fn new(batter_regress: u16, peak_age: u8, year_weights: Vec<f32>) -> Self {
        Capuchin {
            peak_age: peak_age,
            year_weights: year_weights,
            batter_regress: batter_regress,
            people: None,
            players: databank::Players::new(),
            batting_league_totals: BTreeMap::new(),
        }
    }

    pub fn load_register(&mut self, people: register::People) {
        self.people = Some(people);
    }

    pub fn load_batting(&mut self, batting_csv: &Path) -> errors::Result<()> {
        self.players.load_batting(batting_csv)
    }

    pub fn batting_projection(&mut self, year: u16) -> Vec<databank::BattingProjection> {
        // Calculate the totals for each season to get per-PA averages.
        let number_years = self.year_weights.len();
        let start_year = year - number_years as u16;
        let end_year = year - 1;
        let past_seasons = self.players.batting_seasons(start_year, end_year);

        // Build a list of every player that appeared in those seasons. Each will get a projection.
        // Combine each player's split seasons into a single season summary.
        let mut batters = HashMap::new();
        for season in &past_seasons {
            let mut player = batters.entry(season.playerid())
                .or_insert(BTreeMap::new());
            let mut summary = player.entry(season.yearid())
                .or_insert(databank::BattingSeasonSummary::new());
            summary.mut_add_season(season);
        }

        for year in start_year..year {
            if !self.batting_league_totals.contains_key(&year) {
                // TODO: Filter out pitcher seasons.
                let season_summary = past_seasons.iter()
                    .filter(|season| season.is_year(year))
                    .fold(databank::BattingSeasonSummary::new(),
                        |summary, &season| summary.add_season(season))
                    ;
                let summary_rates = season_summary.into();
                self.batting_league_totals.insert(year, summary_rates);
            }
        }

        // Map the years to the weight to use for that year.
        let mut weights_map = Vec::with_capacity(number_years + 1);
        // Make the first element be the year of the projection. This makes the math a bit easier
        // for indexing previous years.
        weights_map.push(0.0);
        for weight in &self.year_weights {
            weights_map.push(*weight);
        }
        let weights_map = weights_map;

        // Weight player and league based on PA.
        let mut player_projections = Vec::with_capacity(batters.len());
        for (batter, batter_seasons) in batters {
            // Weighted batter seasons.
            let mut weighted_batter = databank::BattingProjection::new_player(&batter, year);
            // What the league did with the same PAs, weighted the same.
            let mut batter_league_mean = databank::BattingProjection::league();
            let mut projected_pa = 200.0;
            for (season_year, season) in &batter_seasons {
                let season_year = *season_year;
                let season_pa = *season.pa() as u16;
                projected_pa += match year - season_year {
                    1 => 0.5 * season_pa as f32,
                    2 => 0.1 * season_pa as f32,
                    _ => 0.0,
                };
                let weight_idx = (year - season_year) as usize;
                let weight = weights_map[weight_idx];
                weighted_batter.weighted_add(season, weight);

                let league_rate = self.batting_league_totals.get(&season_year)
                    .expect("Expected to get a rate for this year.");
                batter_league_mean.weighted_rate_add(season_pa, league_rate, weight);
            }

            let projected_pa = projected_pa as u16;
            let prorated_league_mean = batter_league_mean.prorate(self.batter_regress);
            // Merge weighted player and league totals to regress the player.
            weighted_batter.regress(&prorated_league_mean);

            let mut projection = weighted_batter.prorate(projected_pa);
            if let Some(ref people) = self.people {
                people.find_by_bbref(&batter)
                    .and_then(|p| p.get_age(year))
                    .map(|age| {
                        let age_diff = self.peak_age as f32 - age as f32;
                        if self.peak_age < age {
                            projection.age_adjust(1.0 + (age_diff * 0.003));
                        }
                        else if self.peak_age > age {
                            projection.age_adjust(1.0 + (age_diff * 0.006));
                        }
                    });
            }
            projection.round();
            player_projections.push(projection);
        }

        player_projections.sort();
        player_projections
    }

    /*
    pub fn pitching_projection(&mut self, year: u8) -> Vec<databank::PitchingProjection> {
        Vec::new()
    }
    */
}
