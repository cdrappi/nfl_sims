use std::collections::HashMap;

use crate::params::{skill_player::Position, PlayerMeta};
use crate::sim::box_score::{
    BoxScore, KickingProjection, PlayerFantasyPoints, PlayerKey, Projection, QbProjection,
    SkillProjection, TeamProjection,
};

pub fn get_projection_items(
    projections: &HashMap<PlayerKey, Projection>,
    sk_to_meta: &HashMap<PlayerKey, PlayerMeta>,
    opponents: &HashMap<String, String>,
    teams_ordered: bool,
) -> Vec<(PlayerKey, Projection)> {
    let mut items: Vec<(PlayerKey, Projection)> = projections
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    items.sort_by(|a, b| {
        let a_pts = a.1.dk_points();
        let b_pts = b.1.dk_points();
        let a_pos = &sk_to_meta
            .get(&a.0)
            .expect(&format!("no entry for {:?}", &a.0))
            .pos;
        let b_pos = &sk_to_meta
            .get(&b.0)
            .expect(&format!("no entry for {:?}", b.0))
            .pos;
        let (a_tm, b_tm) = match teams_ordered {
            true => {
                let a_tm = &sk_to_meta[&a.0].team;
                let a_opp = &opponents[a_tm];
                let b_tm = &sk_to_meta[&b.0].team;
                let b_opp = &opponents[b_tm];
                (join_teamopp(a_tm, a_opp), join_teamopp(b_tm, b_opp))
            }
            false => (String::from(""), String::from("")),
        };
        (a_tm, a_pos, b_pts)
            .partial_cmp(&(b_tm, b_pos, a_pts))
            .unwrap()
    });
    items
}

pub fn accumulate_projections(
    sims: &Vec<Vec<BoxScore>>,
    ignore_kickers: bool,
) -> HashMap<PlayerKey, Projection> {
    let mut projections = HashMap::new();

    let n_sims = sims.len() as u32;
    for box_scores in sims {
        update_projections(&mut projections, box_scores, n_sims, ignore_kickers);
    }

    projections
}

fn update_projections(
    projections: &mut HashMap<PlayerKey, Projection>,
    box_scores: &Vec<BoxScore>,
    n_sims: u32,
    ignore_kickers: bool,
) {
    for box_score in box_scores {
        // passing projections
        for (nfl_player_id, pbs) in &box_score.passers {
            let sk = PlayerKey::NflId(nfl_player_id.clone());
            if !projections.contains_key(&sk) {
                projections.insert(sk.clone(), Projection::Qb(QbProjection::new(n_sims)));
            }

            let proj = projections.get_mut(&sk).unwrap();
            if let Projection::Qb(passing_proj) = proj {
                passing_proj.add(pbs);
                passing_proj
                    .points
                    .add_points(n_sims, box_score.passing_fantasy_points(nfl_player_id));
            }
        }

        // skill players
        for (nfl_player_id, sbs) in &box_score.skill_players {
            let sk = PlayerKey::NflId(nfl_player_id.clone());
            if !projections.contains_key(&sk) {
                projections.insert(sk.clone(), Projection::Skill(SkillProjection::new(n_sims)));
            }

            let proj = projections.get_mut(&sk).unwrap();
            if let Projection::Qb(passing_proj) = proj {
                passing_proj.skill.add(sbs, passing_proj.n_sims);
                passing_proj
                    .points
                    .add_points(n_sims, box_score.skill_fantasy_points(nfl_player_id));
            } else if let Projection::Skill(skill_player_proj) = proj {
                skill_player_proj.add(sbs);
                skill_player_proj
                    .points
                    .add_points(n_sims, box_score.skill_fantasy_points(nfl_player_id));
            }
        }

        // defense projections
        for (team, points) in &box_score.defenses {
            let sk = PlayerKey::TeamPos(team.clone(), Position::Defense);
            // let slate_id = sk_to_slate_id
            //     .get(&sk)
            //     .expect(&format!("Missing slate id for {:?}", sk));
            if !projections.contains_key(&sk) {
                projections.insert(sk.clone(), Projection::Team(TeamProjection::new(n_sims)));
            }
            let proj = projections.get_mut(&sk).unwrap();
            if let Projection::Team(team_proj) = proj {
                team_proj.defense.add(points, team_proj.n_sims);
                team_proj.offense.add(team, box_score, team_proj.n_sims);
                team_proj.update_field_position(box_score.field_position.get(team).unwrap());
                team_proj
                    .special_teams
                    .add(team, box_score, team_proj.n_sims);
                team_proj.points.add_points(
                    n_sims,
                    PlayerFantasyPoints::copy_standard(
                        box_score
                            .defenses
                            .get(team)
                            .unwrap()
                            .standard_fantasy_points(),
                    ),
                );
            }
        }

        if !ignore_kickers {
            for (team, points) in &box_score.kickers {
                let sk = PlayerKey::TeamPos(team.clone(), Position::Kicker);
                if !projections.contains_key(&sk) {
                    projections.insert(
                        sk.clone(),
                        Projection::Kicker(KickingProjection::new(n_sims)),
                    );
                }
                let proj = projections.get_mut(&sk).unwrap();
                if let Projection::Kicker(kicking_proj) = proj {
                    kicking_proj.add(points);
                    kicking_proj.points.add_points(
                        n_sims,
                        PlayerFantasyPoints::copy_standard(
                            box_score
                                .kickers
                                .get(team)
                                .unwrap()
                                .standard_fantasy_points(),
                        ),
                    );
                }
            }
        }
    }
}

fn join_teamopp(tm: &String, opp: &String) -> String {
    match tm > opp {
        true => format!("{}v{}-{}", opp, tm, tm),
        false => format!("{}v{}-{}", tm, opp, tm),
    }
}
