use std::{collections::HashMap, fmt::Debug, hash::Hash};

pub fn compute_conditional_shares<T: Eq + Hash + Debug>(
    marginal_shares: HashMap<String, f32>,
    player_loc_probs: HashMap<String, HashMap<T, f32>>,
    team_loc_probs: HashMap<T, f32>,
    remaining_t: T,
) -> HashMap<T, Vec<(String, f32)>> {
    let mut cond_shares = HashMap::new();
    let mut non_remaining_contrib = HashMap::new();
    for (pid, _) in marginal_shares.iter() {
        non_remaining_contrib.insert(pid.clone(), 0.0);
    }
    let mut team_prob_sum = 0.0;
    for (loc, team_prob) in team_loc_probs {
        team_prob_sum += team_prob;
        let mut cumsum = 0.0;
        let mut id_shares = Vec::new();
        for (pid, marginal_share) in marginal_shares.iter() {
            let share_given_loc = marginal_share * player_loc_probs[pid][&loc];
            cumsum += share_given_loc;
            id_shares.push((pid.clone(), share_given_loc));
        }
        for (_, share) in id_shares.iter_mut() {
            *share /= cumsum;
        }
        for (pid, cond_share) in id_shares.iter() {
            let rc = non_remaining_contrib.get_mut(pid).unwrap();
            *rc += cond_share * team_prob;
        }
        cond_shares.insert(loc, id_shares);
    }

    let remaining_prob = 1.0 - team_prob_sum;
    let mut cumsum = 0.0;
    let mut remaining_shares = Vec::new();
    for (pid, marginal_share) in marginal_shares.iter() {
        let non_remaining = non_remaining_contrib[pid];
        // marginal share = remaining_prob * X + non_remaining
        // X = (marginal_share - non_remaining) / remaining_prob
        let remaining_share = ((marginal_share - non_remaining) / remaining_prob).max(0.0);
        cumsum += remaining_share;
        remaining_shares.push((pid.clone(), remaining_share));
    }
    // log::info!("remaining cumsum: {} (should be exactly 1)", cumsum);
    for (_, share) in remaining_shares.iter_mut() {
        *share /= cumsum;
    }
    cond_shares.insert(remaining_t, remaining_shares);

    // log::info!("cond_shares: {:?}", cond_shares);

    cond_shares
}

/*
let mut cumsum = 0.0;
for (pid, player) in offense.skill_players.iter() {
    let prob_target = player.ms_targets_live;
    let (prob_loc_given_tgt, prob_loc) = match yards_to_goal.is_redzone() {
        true => (
            player.prob_rz_given_target,
            offense.team.prob_rz_given_target,
        ),
        false => (
            1.0 - player.prob_rz_given_target,
            1.0 - offense.team.prob_rz_given_target,
        ),
    };
    let prob_target_given_loc = prob_target * prob_loc_given_tgt / prob_loc;
    cumsum += prob_target_given_loc;
    id_shares.push((pid.clone(), prob_target_given_loc));
}

// if cumsum == 0.0 {
//     log::info!("loc air yards cumsum = 0.0. {}", yards_to_goal);
// }
for (_, share) in id_shares.iter_mut() {
    *share /= cumsum;
}
id_shares
*/
