use std::collections::HashMap;

use crate::params::{skill_player::Position, PlayerMeta};
use crate::projections::math::get_projection_items;
use crate::projections::odds::{balanced_line, make_yards_line, probability_to_american_odds};
use crate::sim::box_score::{PlayerKey, Projection};

pub struct ProjectionsWriter {
    pub projections: HashMap<PlayerKey, Projection>,
    pub sk_to_player_meta: HashMap<PlayerKey, PlayerMeta>,
    pub slate_dir: String,
    pub opponents: HashMap<String, String>,
}

impl ProjectionsWriter {
    pub fn new(
        slate_dir: &String,
        projections: &HashMap<PlayerKey, Projection>,
        sk_to_player_meta: &HashMap<PlayerKey, PlayerMeta>,
        opponents: &HashMap<String, String>,
    ) -> ProjectionsWriter {
        ProjectionsWriter {
            projections: projections.clone(),
            sk_to_player_meta: sk_to_player_meta.clone(),
            slate_dir: slate_dir.clone(),
            opponents: opponents.clone(),
        }
    }

    pub fn write_projections(&self) {
        let mut items = get_projection_items(
            &self.projections,
            &self.sk_to_player_meta,
            &self.opponents,
            true,
        );

        let proj_path = format!("{}/projections", self.slate_dir);
        std::fs::create_dir_all(&proj_path).unwrap();

        self.write_qb_projections(&mut items);
        self.write_skill_projections(&mut items);

        self.write_team_projections(&mut items);
        // self.write_field_positions(&mut items);
    }

    fn write_qb_projections(&self, projections: &mut Vec<(PlayerKey, Projection)>) {
        let mut wtr =
            csv::Writer::from_path(format!("{}/projections/passing.csv", self.slate_dir)).unwrap();

        wtr.write_record(&[
            "nfl_id",
            "name",
            "team",
            "attempts",
            "completions",
            "comp%",
            "yards",
            "air_yards",
            "yac",
            "pass_td",
            "int",
            "2PT",
            "YdsLine",
            "TDsLine",
            "P(300+)",
            "scrambles",
            "kneels",
        ])
        .unwrap();

        for (sk, proj) in projections {
            if let Projection::Qb(qb_prob) = proj {
                let player_meta = self.sk_to_player_meta.get(sk).unwrap();
                let passing = &mut qb_prob.passing;
                let yards_sampled = &mut passing.yards_sampled;
                yards_sampled.sort();
                let (median_yards, yds_line) = balanced_line(yards_sampled, true);

                let tds_sampled = &mut passing.tds_sampled;
                tds_sampled.sort();
                let (median_tds, td_over_prob) = balanced_line(tds_sampled, true);
                let prob_300_plus: f32 = yards_sampled
                    .iter()
                    .map(|y| if *y >= 300 { 1.0 } else { 0.0 })
                    .sum::<f32>()
                    / yards_sampled.len() as f32;
                wtr.write_record(&[
                    sk.to_string(),
                    player_meta.name.clone(),
                    player_meta.team.clone(),
                    format!("{:.1}", passing.attempts),
                    format!("{:.1}", passing.completions),
                    format!("{:.1}%", 100.0 * passing.completions / passing.attempts),
                    format!("{:.1}", passing.yards),
                    format!("{:.1}", passing.air_yards),
                    format!("{:.1}", passing.yards_after_catch),
                    format!("{:.2}", passing.touchdowns),
                    format!("{:.2}", passing.interceptions),
                    format!("{:.2}", passing.two_point_conversions),
                    make_yards_line(median_yards, yds_line),
                    make_yards_line(median_tds, td_over_prob),
                    format!("{:.2}", prob_300_plus),
                    format!("{:.1}", passing.scrambles),
                    format!("{:.1}", passing.kneels),
                ])
                .unwrap();
            }
        }

        wtr.flush().unwrap();
    }

    fn write_skill_projections(&self, projections: &mut Vec<(PlayerKey, Projection)>) {
        let mut wtr =
            csv::Writer::from_path(format!("{}/projections/skill.csv", self.slate_dir)).unwrap();
        wtr.write_record(&[
            "nfl_id",
            "name",
            "team",
            "pos",
            "carries",
            "rushing_yards",
            "rushing_tds",
            "targets",
            "catches",
            "receiving_tds",
            "receiving_yards",
            "air_yards",
            "yac",
            "fumbles_lost",
            "return_tds",
            "2PT",
            "RuYdsLine",
            "RecYdsLine",
            "TotYdsLine",
            "P(RushTD)",
            "P(RecTD)",
            "P(TD)",
            "P(100+rush)",
            "P(100+rec)",
        ])
        .unwrap();

        for (sk, proj) in projections {
            let player_meta = self.sk_to_player_meta.get(sk).unwrap();
            let skill_proj = match proj {
                Projection::Skill(skill_proj) => &mut skill_proj.skill,
                Projection::Qb(qb_proj) => &mut qb_proj.skill,
                _ => continue,
            };

            let rushing_yards_sampled = &mut skill_proj.rushing_yards_sampled;
            rushing_yards_sampled.sort();
            let (median_rushing_yards, rush_yds_line) = balanced_line(rushing_yards_sampled, true);

            let receiving_yards_sampled = &mut skill_proj.receiving_yards_sampled;
            receiving_yards_sampled.sort();
            let (median_receiving_yards, rec_yds_line) =
                balanced_line(receiving_yards_sampled, true);

            let total_yards_sampled = &mut skill_proj.total_yards_sampled;
            total_yards_sampled.sort();
            let (median_total_yards, tot_yds_line) = balanced_line(total_yards_sampled, true);

            let prob_rush_td = skill_proj
                .rushing_tds_sampled
                .iter()
                .map(|y| if *y >= 1 { 1.0 } else { 0.0 })
                .sum::<f32>()
                / skill_proj.rushing_tds_sampled.len() as f32;
            let prob_rec_td = skill_proj
                .receiving_tds_sampled
                .iter()
                .map(|y| if *y >= 1 { 1.0 } else { 0.0 })
                .sum::<f32>()
                / skill_proj.receiving_tds_sampled.len() as f32;
            let prob_td = skill_proj
                .total_tds_sampled
                .iter()
                .map(|y| if *y >= 1 { 1.0 } else { 0.0 })
                .sum::<f32>()
                / skill_proj.total_tds_sampled.len() as f32;
            let prob_100_rush = rushing_yards_sampled
                .iter()
                .map(|y| if *y >= 100 { 1.0 } else { 0.0 })
                .sum::<f32>()
                / rushing_yards_sampled.len() as f32;
            let prob_100_rec = receiving_yards_sampled
                .iter()
                .map(|y| if *y >= 100 { 1.0 } else { 0.0 })
                .sum::<f32>()
                / receiving_yards_sampled.len() as f32;

            wtr.write_record(&[
                sk.to_string(),
                player_meta.name.clone(),
                player_meta.team.clone(),
                player_meta.pos.to_string(),
                format!("{:.1}", skill_proj.carries),
                format!("{:.1}", skill_proj.rushing_yards),
                format!("{:.2}", skill_proj.rushing_touchdowns),
                format!("{:.1}", skill_proj.targets),
                format!("{:.1}", skill_proj.catches),
                format!("{:.2}", skill_proj.receiving_touchdowns),
                format!("{:.1}", skill_proj.receiving_yards),
                format!("{:.1}", skill_proj.air_yards),
                format!("{:.1}", skill_proj.yards_after_catch),
                format!("{:.1}", skill_proj.fumbles_lost),
                format!("{:.2}", skill_proj.return_touchdowns),
                format!("{:.1}", skill_proj.two_point_conversions),
                make_yards_line(median_rushing_yards, rush_yds_line),
                make_yards_line(median_receiving_yards, rec_yds_line),
                make_yards_line(median_total_yards, tot_yds_line),
                format!("{:.2}", prob_rush_td),
                format!("{:.2}", prob_rec_td),
                format!("{:.2}", prob_td),
                format!("{:.2}", prob_100_rush),
                format!("{:.2}", prob_100_rec),
            ])
            .unwrap()
        }
        wtr.flush().unwrap();
    }

    fn write_team_projections(&self, projections: &Vec<(PlayerKey, Projection)>) {
        let mut wtr =
            csv::Writer::from_path(format!("{}/projections/team.csv", self.slate_dir)).unwrap();
        wtr.write_record(&[
            "team",
            "mean_points",
            "spread",
            "money",
            "plays",
            "rushes",
            "dropbacks",
            "targets",
            "fga",
            "fg",
            "punts",
            "dst_points_allowed",
            "sacks",
            "interceptions",
            "defensive_tds",
            "fumble_recoveries",
            "safeties",
            "blocked_kicks",
            "offensive_penalties",
            "op_yards",
            "defensive_penalties",
            "dp_yards",
        ])
        .unwrap();
        for (sk, proj) in projections {
            if let Projection::Team(team_proj) = proj {
                // let salary = self.dk_slate_id_to_salary.get(sk).unwrap();
                let defense = &team_proj.defense;
                let offense = &team_proj.offense;
                let special_teams = &team_proj.special_teams;
                let opp_team_sk = PlayerKey::TeamPos(
                    self.opponents[&sk.expect_team()].clone(),
                    Position::Defense,
                );
                if let Projection::Team(opponent_team_proj) = &self.projections[&opp_team_sk] {
                    let (mut spreads, mut totals, mut win_prob) = (vec![], vec![], 0.0);
                    for (team_pts, opp_pts) in team_proj
                        .offense
                        .points_sampled
                        .iter()
                        .zip(&opponent_team_proj.offense.points_sampled)
                    {
                        spreads.push(*team_pts as i8 - *opp_pts as i8);
                        totals.push((*team_pts + *opp_pts) as i8);
                        let win_eq = match (team_pts == opp_pts, team_pts > opp_pts) {
                            (true, _) => 0.5,
                            (false, true) => 1.0,
                            (false, false) => 0.0,
                        };
                        win_prob += win_eq / team_proj.n_sims as f32;
                    }
                    spreads.sort();
                    totals.sort();

                    let (best_spread, cover_prob) = balanced_line(&spreads, false);
                    let (best_total, over_prob) = balanced_line(&totals, false);
                    let (line_prefix, median_line, line_prob) = match best_spread > 0.0 {
                        true => ("", -1.0 * best_spread, cover_prob),
                        false => ("O ", best_total, over_prob),
                    };

                    wtr.write_record(&[
                        sk.expect_team(),
                        format!("{:.2}", offense.points),
                        format!(
                            "{}{:.1} {}",
                            line_prefix,
                            median_line,
                            probability_to_american_odds(line_prob)
                        ),
                        format!("{}", probability_to_american_odds(win_prob)),
                        format!("{:.1}", offense.plays_called),
                        format!("{:.1}", offense.rushes),
                        format!("{:.1}", offense.dropbacks),
                        format!("{:.1}", offense.targets),
                        format!("{:.1}", special_teams.fg_attempts),
                        format!("{:.1}", special_teams.fg_made),
                        format!("{:.1}", special_teams.punts),
                        format!("{:.1}", defense.points_allowed),
                        format!("{:.1}", defense.sacks),
                        format!("{:.2}", defense.interceptions),
                        format!("{:.2}", defense.touchdowns),
                        format!("{:.2}", defense.fumble_recoveries),
                        format!("{:.2}", defense.safeties),
                        format!("{:.2}", defense.blocked_kicks),
                        format!("{:.2}", offense.penalties),
                        format!("{:.1}", offense.penalty_yards),
                        format!("{:.2}", defense.penalties),
                        format!("{:.1}", defense.penalty_yards),
                    ])
                    .unwrap();
                }
            }
        }

        wtr.flush().unwrap();
    }

    /// Mainly used to debug whether we have the right distribution of plays called from each yardline
    /// This was particularly useful to learn that modeling pass interference in the endzone
    /// was critical for getting an accurate rushTD/passTD/FG ratio
    fn _write_field_positions(&self, projections: &Vec<(PlayerKey, Projection)>) {
        let mut team_projs = HashMap::new();
        for (sk, proj) in projections {
            if let Projection::Team(team_proj) = proj {
                team_projs.insert(sk.expect_team(), team_proj);
            }
        }

        for (team, team_proj) in &team_projs {
            let mut wtr = csv::Writer::from_path(format!(
                "{}/projections/field_position_{}.csv",
                self.slate_dir, team
            ))
            .unwrap();
            wtr.write_record(&["yardline_100", "play_count"]).unwrap();
            for ii in 1..=99 {
                let count = team_proj.field_position.get(&ii).unwrap_or(&0);
                wtr.write_record(&[&ii.to_string(), &count.to_string()])
                    .unwrap();
            }
            wtr.flush().unwrap();
        }
    }
}
