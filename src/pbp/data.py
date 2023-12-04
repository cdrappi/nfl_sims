import os
import re
from typing import Any, Dict, Iterable, Optional, Set, Tuple, Union

import nfl_data_py
import numpy as np
import pandas as pd

PROB_PASS = 0.61
PROB_COMPLETION = 0.6
PROB_INTERCEPTION = 0.025

POIS_KWARGS = dict(max_iter=5000, alpha=10.0)
LOGR_KWARGS = dict(max_iter=5000, C=0.1)

MODELS_PATH = f'{os.environ["NFL_SIMS_PATH"]}/rust/nfl_pbp_sim/src/models'
DATA_PATH = f'{os.environ["NFL_SIMS_PATH"]}/data'
BASELINES_PATH = f"{DATA_PATH}/baselines"

CURRENT_SEASON = 2023

OUTDOOR = "Outdoor"
RETRACTABLE = "Retractable"
DOME = "Dome"

STADIUM_TYPES = {
    "ARI": RETRACTABLE,
    "ATL": RETRACTABLE,
    "BAL": OUTDOOR,
    "BUF": OUTDOOR,
    "CAR": OUTDOOR,
    "CHI": OUTDOOR,
    "CIN": OUTDOOR,
    "CLE": OUTDOOR,
    "DAL": RETRACTABLE,
    "DEN": OUTDOOR,
    "DET": DOME,
    "GB": OUTDOOR,
    "HOU": RETRACTABLE,
    "IND": RETRACTABLE,
    "JAX": OUTDOOR,
    "KC": OUTDOOR,
    "LA": DOME,
    "LAC": DOME,
    "LV": DOME,
    "MIA": OUTDOOR,
    "MIN": DOME,
    "NE": OUTDOOR,
    "NO": DOME,
    "NYG": OUTDOOR,
    "NYJ": OUTDOOR,
    "PHI": OUTDOOR,
    "PIT": OUTDOOR,
    "SEA": OUTDOOR,
    "SF": OUTDOOR,
    "TB": OUTDOOR,
    "TEN": OUTDOOR,
    "WAS": OUTDOOR,
}

STADIUM_ORIENTATIONS = {
    # North = 0, East = 90, South = 180, West = 270
    "ARI": 148.0,
    "ATL": 90,
    "BAL": 110.4,
    "BUF": 122.2,
    "CAR": 140.2,
    "CHI": 175.8,
    "CIN": 141.2,
    "CLE": 55.7,
    "DAL": 68.8,
    "DEN": 0,
    "DET": 63.9,
    "GB": 0,
    "HOU": 178.9,
    "IND": 25.6,
    "JAX": 15.5,
    "KC": 137.2,
    "LA": 0,
    "LAC": 0,  # Sofi seems to be straight north
    "LV": 0,  # it's a dome so who cares
    "MIA": 121.8,
    "MIN": 90,
    "NE": 162.2,
    "NO": 23.5,
    "NYG": 167.0,
    "NYJ": 167.0,
    "PHI": 171.6,
    "PIT": 155.5,
    "SEA": 0,
    "SF": 151.7,
    "TB": 0,
    "TEN": 155.6,
    "WAS": 120.2,
}

NON_DEFENSIVE_TIMEOUTS = {
    "PUNT",
    "PENALTY_OFFENSE",
    "PENALTY_DEFENSE",
    "FG_ATTEMPT",
    "QB_KNEEL",
    "QB_SPIKE",
    "DESIGNED_RUN",
    "DROPBACK",
}


def is_stadium_dome(row: Dict[str, Any]) -> bool:
    if row["location"] == "Neutral":
        # unclear what the situation is here.
        return False
    home_team = row["home_team"]
    season = row["season"]
    if home_team == "LA" and season < 2020:
        # Los Angeles Memorial Coliseum
        return False
    if home_team == "LAC" and season < 2020:
        # Dignity Health Sports Park
        return False
    if home_team == "LV" and season < 2020:
        # Oakland Coliseum
        return False
    return STADIUM_TYPES[home_team] in {DOME}


def get_stadium_orientation_degrees(row: Dict[str, Any]) -> Optional[float]:
    if row["location"] == "Neutral":
        # unclear what the situation is here.
        return None
    home_team = row["home_team"]
    season = row["season"]
    if home_team == "LA" and season < 2020:
        # Los Angeles Memorial Coliseum
        return 90
    if home_team == "LAC" and season < 2020:
        # Dignity Health Sports Park
        return 0
    if home_team == "LV" and season < 2020:
        # Oakland Coliseum
        return 145.5
    return STADIUM_ORIENTATIONS[home_team]


def possdiff_plus_n(rec, n):
    posteam_score = n + rec["off_score"]
    defteam_score = rec["def_score"]
    if posteam_score == defteam_score:
        return 0
    diff = posteam_score - defteam_score
    return diff // 8 + (1 if diff > 0 else 0)


def possession_diff(rec):
    return possdiff_plus_n(rec, 0)


def fg_possession_diff(rec):
    return possdiff_plus_n(rec, 3)


def fg_make_prob(yardline_100):
    distance = yardline_100 + 17
    """
    0..=17 => panic!("field goals cannot be this short"),
    18..=29 => 0.99,
    30..=34 => 0.95,
    35..=39 => 0.87,
    40..=44 => 0.78,
    45..=49 => 0.70,
    50..=54 => 0.58,
    55..=59 => 0.45,
    60..=64 => 0.32,
    65..=70 => 0.10,
    _ => 0.0,
    """
    if distance < 35:
        return 1.0
    elif distance < 40:
        return 0.9
    elif distance < 45:
        return 0.80
    elif distance < 50:
        return 0.70
    elif distance < 55:
        return 0.6
    elif distance < 60:
        return 0.45
    elif distance < 65:
        return 0.3
    elif distance < 70:
        return 0.1
    return 0


def punt_value(yardline_100):
    if yardline_100 < 40:
        return 0
    elif yardline_100 < 50:
        return 0.2
    elif yardline_100 < 55:
        return 0.4
    elif yardline_100 < 60:
        return 0.6
    elif yardline_100 < 65:
        return 0.8
    else:
        return 1.0


def add_is_home(dataset: pd.DataFrame) -> None:
    dataset["is_offense_home"] = (
        dataset["posteam"] == dataset["home_team"]
    ).astype(int)
    dataset.loc[dataset["location"] == "Neutral", "is_offense_home"] = 0.5


def add_possdiff_features(dataset: pd.DataFrame) -> None:
    add_is_home(dataset)
    dataset["possession_diff"] = dataset.apply(possession_diff, axis=1)
    dataset["fg_possession_diff"] = dataset.apply(fg_possession_diff, axis=1)

    dataset["game_minutes_left"] = (dataset["game_seconds_remaining"] + 1) / 60
    dataset["half_minutes_left"] = (dataset["half_seconds_remaining"] + 1) / 60

    # clip to 10 seconds left
    dataset["inv_half_minutes"] = (1 / dataset["half_minutes_left"]).clip(
        upper=6
    )
    dataset["log_inv_half_minutes"] = np.log(dataset["inv_half_minutes"])

    dataset["inv_game_minutes"] = (1 / dataset["game_minutes_left"]).clip(
        upper=6
    )
    dataset["log_inv_game_minutes"] = np.log(dataset["inv_game_minutes"])

    dataset["possdiff_per_minute"] = (
        dataset["possession_diff"] * dataset["inv_game_minutes"]
    )
    dataset["fgpossdiff_per_minute"] = (
        dataset["fg_possession_diff"] * dataset["inv_game_minutes"]
    )
    dataset["pdpm2"] = dataset["possdiff_per_minute"] * (
        dataset["possdiff_per_minute"].abs()
    )
    dataset["fgpdpm2"] = dataset["fgpossdiff_per_minute"] * (
        dataset["fgpossdiff_per_minute"].abs()
    )

    garbage_time = (dataset["game_minutes_left"] < 5).astype(int) * dataset[
        "inv_game_minutes"
    ].clip(1, 3)
    dataset["garbage_time_win"] = garbage_time * (
        dataset["possession_diff"] >= 3
    ).astype(int)
    dataset["garbage_time_loss"] = garbage_time * (
        dataset["possession_diff"] <= -3
    ).astype(int)


def add_playcall_features(dataset: pd.DataFrame) -> None:
    add_possdiff_features(dataset)

    dataset["yardline_pct"] = dataset["yardline_100"] / 100
    dataset["log_yardline_pct"] = np.log(dataset["yardline_pct"].clip(0.01, 1))

    dataset["z_ydstogo"] = (dataset["ydstogo"] - 10) / 5
    dataset["ydstogo_pct"] = dataset["ydstogo"] / 10
    dataset["log_ydstogo_pct"] = np.log(dataset["ydstogo_pct"].clip(0.1))

    dataset["ydstogo_sigmoid"] = np.exp(-dataset["ydstogo"] / 10)

    # fg_distance = dataset["yardline_100"] + 17
    _EPSILON = 1e-6
    dataset["fg_sigmoid"] = np.log(
        _EPSILON + dataset["yardline_100"].apply(fg_make_prob)
    )
    dataset["punt_sigmoid"] = np.log(
        _EPSILON + dataset["yardline_100"].apply(punt_value)
    )

    dataset["to_go_1st"] = dataset["down_1"] * dataset["ydstogo_sigmoid"]
    dataset["to_go_2nd"] = dataset["down_2"] * dataset["ydstogo_sigmoid"]
    dataset["to_go_3rd"] = dataset["down_3"] * dataset["ydstogo_sigmoid"]
    dataset["to_go_4th"] = dataset["down_4"] * dataset["ydstogo_sigmoid"]

    dataset["log_to_go_1st"] = dataset["down_1"] * dataset["log_ydstogo_pct"]
    dataset["log_to_go_2nd"] = dataset["down_2"] * dataset["log_ydstogo_pct"]
    dataset["log_to_go_3rd"] = dataset["down_3"] * dataset["log_ydstogo_pct"]
    dataset["log_to_go_4th"] = dataset["down_4"] * dataset["log_ydstogo_pct"]

    dataset["yardline_pct_sq"] = dataset["yardline_pct"] ** 2
    dataset["yardline_pct_4th"] = dataset["down_4"] * dataset["yardline_pct"]
    dataset["yardline_fgsig_4th"] = dataset["down_4"] * dataset["fg_sigmoid"]
    dataset["yardline_puntsig_4th"] = (
        dataset["down_4"] * dataset["punt_sigmoid"]
    )

    dataset["fp_1st"] = dataset["to_go_1st"] * dataset["ydstogo_pct"]
    dataset["fp_2nd"] = dataset["to_go_2nd"] * dataset["ydstogo_pct"]
    dataset["fp_3rd"] = dataset["to_go_3rd"] * dataset["ydstogo_pct"]
    dataset["fp_4th"] = dataset["to_go_4th"] * dataset["ydstogo_pct"]

    dataset["fp_fgsig_1st"] = dataset["to_go_1st"] * dataset["fg_sigmoid"]
    dataset["fp_fgsig_2nd"] = dataset["to_go_2nd"] * dataset["fg_sigmoid"]
    dataset["fp_fgsig_3rd"] = dataset["to_go_3rd"] * dataset["fg_sigmoid"]
    dataset["fp_fgsig_4th"] = dataset["to_go_4th"] * dataset["fg_sigmoid"]

    dataset["fp_puntsig_1st"] = dataset["to_go_1st"] * dataset["punt_sigmoid"]
    dataset["fp_puntsig_2nd"] = dataset["to_go_2nd"] * dataset["punt_sigmoid"]
    dataset["fp_puntsig_3rd"] = dataset["to_go_3rd"] * dataset["punt_sigmoid"]
    dataset["fp_puntsig_4th"] = dataset["to_go_4th"] * dataset["punt_sigmoid"]

    dataset["inside_2m_warning"] = (
        dataset["half_seconds_remaining"] <= 120
    ).astype(int)

    dataset["yardline_4th"] = dataset["down_4"] * dataset["yardline_pct"]
    dataset["log_yardline_4th"] = (
        dataset["down_4"] * dataset["log_yardline_pct"]
    )

    dataset["yardline_not_4th"] = (1 - dataset["down_4"]) * dataset[
        "yardline_pct"
    ]
    dataset["log_yardline_not_4th"] = (1 - dataset["down_4"]) * dataset[
        "log_yardline_pct"
    ]

    dataset["goal_to_go_yardline"] = (
        dataset["goal_to_go"] * dataset["yardline_pct"]
    )
    dataset["log_goal_to_go_yardline"] = (
        dataset["goal_to_go"] * dataset["log_yardline_pct"]
    )

    dataset["yards_to_go_yardline"] = (1 - dataset["goal_to_go"]) * dataset[
        "yardline_pct"
    ]
    dataset["log_yards_to_go_yardline"] = (
        1 - dataset["goal_to_go"]
    ) * dataset["log_yardline_pct"]

    dataset["clock_runs_pdpm"] = (
        dataset["clock_running"] * dataset["possdiff_per_minute"]
    )
    dataset["clock_runs_fgpdpm"] = (
        dataset["clock_running"] * dataset["fgpossdiff_per_minute"]
    )
    dataset["clock_runs_pdpm2"] = dataset["clock_running"] * dataset["pdpm2"]
    dataset["clock_runs_fgpdpm2"] = (
        dataset["clock_running"] * dataset["fgpdpm2"]
    )

    dataset["clock_runs_pdpm_off0to"] = (
        dataset["clock_running"]
        * dataset["possdiff_per_minute"]
        * dataset["off_timeouts_remaining_0"]
    )
    dataset["clock_runs_pdpm_off1to"] = (
        dataset["clock_running"]
        * dataset["possdiff_per_minute"]
        * dataset["off_timeouts_remaining_1"]
    )
    dataset["clock_runs_pdpm_off2to"] = (
        dataset["clock_running"]
        * dataset["possdiff_per_minute"]
        * dataset["off_timeouts_remaining_2"]
    )
    dataset["clock_runs_pdpm_off3to"] = (
        dataset["clock_running"]
        * dataset["possdiff_per_minute"]
        * dataset["off_timeouts_remaining_3"]
    )

    dataset["clock_runs_pdpm_def0to"] = (
        dataset["clock_running"]
        * dataset["possdiff_per_minute"]
        * dataset["def_timeouts_remaining_0"]
    )
    dataset["clock_runs_pdpm_def1to"] = (
        dataset["clock_running"]
        * dataset["possdiff_per_minute"]
        * dataset["def_timeouts_remaining_1"]
    )
    dataset["clock_runs_pdpm_def2to"] = (
        dataset["clock_running"]
        * dataset["possdiff_per_minute"]
        * dataset["def_timeouts_remaining_2"]
    )
    dataset["clock_runs_pdpm_def3to"] = (
        dataset["clock_running"]
        * dataset["possdiff_per_minute"]
        * dataset["def_timeouts_remaining_3"]
    )

    dataset["offense_log_pass_prob"] = np.log(
        PROB_PASS + dataset["offense_proe"]
    )
    dataset["defense_log_pass_prob"] = np.log(
        PROB_PASS + dataset["defense_proe"]
    )
    dataset["off_def_lpp"] = (
        dataset["offense_log_pass_prob"] * dataset["defense_log_pass_prob"]
    )

    dataset["off_lpp_rz"] = np.log(PROB_PASS + dataset["offense_rz_proe"])
    dataset["def_lpp_rz"] = np.log(PROB_PASS + dataset["defense_rz_proe"])
    dataset["off_def_lpp_rz"] = dataset["off_lpp_rz"] * dataset["def_lpp_rz"]

    dataset["in_rz"] = (dataset["yardline_100"] <= 20).astype(int)
    dataset["off_lpp_outside_rz"] = dataset["offense_log_pass_prob"] * (
        1 - dataset["in_rz"]
    )
    dataset["off_lpp_inside_rz"] = dataset["off_lpp_rz"] * dataset["in_rz"]
    dataset["def_lpp_outside_rz"] = dataset["defense_log_pass_prob"] * (
        1 - dataset["in_rz"]
    )
    dataset["def_lpp_inside_rz"] = dataset["def_lpp_rz"] * dataset["in_rz"]

    dataset["off_lpp_pdpm"] = (
        dataset["offense_log_pass_prob"] * dataset["possdiff_per_minute"]
    )
    dataset["def_lpp_pdpm"] = (
        dataset["defense_log_pass_prob"] * dataset["possdiff_per_minute"]
    )
    dataset["off_lpp_rz_pdpm"] = (
        dataset["off_lpp_rz"] * dataset["possdiff_per_minute"]
    )
    dataset["def_lpp_rz_pdpm"] = (
        dataset["def_lpp_rz"] * dataset["possdiff_per_minute"]
    )


def add_clock_stops(pbp: pd.DataFrame) -> None:
    ob_stop_after = (pbp["out_of_bounds"] == 1) & (
        (pbp["game_seconds_remaining"] < 5 * 60)
        | (pbp["half_seconds_remaining"] < 2 * 60)
    )
    rush_play = pbp["play_type"] == "run"
    completion = pbp["complete_pass"] == 1
    clock_runs_play = rush_play | completion

    can_clock_run_after = pbp["play_type"].isin({"pass", "run", "qb_kneel"})
    clock_stops_after = (
        (pbp["penalty"] == 1)
        | (pbp["fumble_lost"] == 1)
        | (pbp["qb_spike"] == 1)
        | (pbp["touchdown"] == 1)
        | ((pbp["play_type"] == "pass") & (pbp["complete_pass"] == 0))
        | ~can_clock_run_after
    )
    ob_can_pause_after = (pbp["out_of_bounds"] == 1) & ~ob_stop_after

    pbp["clock_stops_after"] = (
        clock_stops_after | (ob_stop_after & can_clock_run_after)
    ).astype(int)
    pbp["clock_pauses_after"] = (
        ~clock_stops_after & (ob_can_pause_after & clock_runs_play)
    ).astype(int)
    pbp["clock_runs_after"] = (
        1 - pbp["clock_stops_after"] - pbp["clock_pauses_after"]
    )

    pbp["same_drive"] = (pbp["drive"] == pbp["drive"].shift(1)).astype(int)
    pbp["clock_running"] = (
        (pbp["clock_runs_after"] | pbp["clock_pauses_after"])
        .shift(1)
        .fillna(0)
    )
    pbp["clock_paused_before"] = pbp["clock_pauses_after"].shift(1).fillna(0)

    pbp["next_game_seconds_remaining"] = pbp["game_seconds_remaining"].shift(
        -1
    )
    pbp["next_half_seconds_remaining"] = pbp["half_seconds_remaining"].shift(
        -1
    )
    pbp["next_quarter_seconds_remaining"] = pbp[
        "quarter_seconds_remaining"
    ].shift(-1)
    # pbp[['drive', 'same_drive','clock_running', 'clock_runs_after', 'play_type', 'complete_pass', 'out_of_bounds']].head(20)


common_timeout_features = [
    "clock_running",
    "inv_half_minutes",
    "log_inv_half_minutes",
    "inv_game_minutes",
    "log_inv_game_minutes",
    "possession_diff",
    "fg_possession_diff",
    "possdiff_per_minute",
    "fgpossdiff_per_minute",
    "clock_runs_pdpm",
    "clock_runs_fgpdpm",
    "clock_runs_pdpm2",
    "clock_runs_fgpdpm2",
]

only_off_timeout_features = [
    "off_timeouts_remaining_0",
    "off_timeouts_remaining_1",
    "off_timeouts_remaining_2",
    "off_timeouts_remaining_3",
    "clock_runs_pdpm_off0to",
    "clock_runs_pdpm_off1to",
    "clock_runs_pdpm_off2to",
    "clock_runs_pdpm_off3to",
]
off_timeout_features = [*common_timeout_features, *only_off_timeout_features]

only_def_timeout_features = [
    "def_timeouts_remaining_0",
    "def_timeouts_remaining_1",
    "def_timeouts_remaining_2",
    "def_timeouts_remaining_3",
    "clock_runs_pdpm_def0to",
    "clock_runs_pdpm_def1to",
    "clock_runs_pdpm_def2to",
    "clock_runs_pdpm_def3to",
]
def_timeout_features = [*common_timeout_features, *only_def_timeout_features]

all_timeout_features = [
    *common_timeout_features,
    *only_off_timeout_features,
    *only_def_timeout_features,
]

state_features = [
    "is_offense_home",
    "offense_log_pass_prob",
    "defense_log_pass_prob",
    "off_def_lpp",
    "off_lpp_rz",
    "def_lpp_rz",
    "off_def_lpp_rz",
    "off_lpp_outside_rz",
    "off_lpp_inside_rz",
    "def_lpp_outside_rz",
    "def_lpp_inside_rz",
    "off_lpp_pdpm",
    "def_lpp_pdpm",
    "off_lpp_rz_pdpm",
    "def_lpp_rz_pdpm",
    "down_1",
    "down_2",
    "down_3",
    "down_4",
    "goal_to_go",
    "z_ydstogo",
    "ydstogo_pct",
    "log_ydstogo_pct",
    "to_go_1st",
    "to_go_2nd",
    "to_go_3rd",
    "to_go_4th",
    "log_to_go_1st",
    "log_to_go_2nd",
    "log_to_go_3rd",
    "log_to_go_4th",
    "fp_1st",
    "fp_2nd",
    "fp_3rd",
    "fp_4th",
    # '1st_fp_fgsig','2nd_fp_fgsig','3rd_fp_fgsig','4th_fp_fgsig',
    # '1st_fp_puntsig','2nd_fp_puntsig','3rd_fp_puntsig','4th_fp_puntsig',
    "yardline_fgsig_4th",
    "yardline_puntsig_4th",
    "yardline_pct",
    "yardline_pct_sq",
    "log_yardline_pct",
    "fg_sigmoid",
    "punt_sigmoid",
    "goal_to_go_yardline",
    "log_goal_to_go_yardline",
    "yards_to_go_yardline",
    "log_yards_to_go_yardline",
    "yardline_4th",
    "log_yardline_4th",
    "yardline_not_4th",
    "log_yardline_not_4th",
    "inside_2m_warning",
    "garbage_time_win",
    "garbage_time_loss",
]


def make_proe_data(pbp: pd.DataFrame) -> Tuple[pd.DataFrame, ...]:
    proe_data = pbp[
        ["posteam", "defteam", "season", "pass_oe", "yardline_100"]
    ].dropna()
    in_rz = proe_data["yardline_100"] <= 20

    proe_data["pass_oe"] /= 100
    offense_proes = (
        proe_data[~in_rz]
        .groupby(["posteam", "season"], as_index=False)
        .aggregate({"pass_oe": "mean"})
        .sort_values(["pass_oe"])
        .rename(columns={"pass_oe": "offense_proe"})
    )
    defense_proes = (
        proe_data[~in_rz]
        .groupby(["defteam", "season"], as_index=False)
        .aggregate({"pass_oe": "mean"})
        .sort_values(["pass_oe"])
        .rename(columns={"pass_oe": "defense_proe"})
    )

    offense_rz_proes = (
        proe_data[in_rz]
        .groupby(["posteam", "season"], as_index=False)
        .aggregate({"pass_oe": "mean"})
        .sort_values(["pass_oe"])
        .rename(columns={"pass_oe": "offense_rz_proe"})
    )
    defense_rz_proes = (
        proe_data[in_rz]
        .groupby(["defteam", "season"], as_index=False)
        .aggregate({"pass_oe": "mean"})
        .sort_values(["pass_oe"])
        .rename(columns={"pass_oe": "defense_rz_proe"})
    )
    return (offense_proes, defense_proes, offense_rz_proes, defense_rz_proes)


def make_penalty_zs(pbp: pd.DataFrame) -> Tuple[pd.DataFrame, pd.DataFrame]:
    pbp["offense_penalty_z"] = (
        pbp["penalty_team"] == pbp["drive_offense"]
    ).astype(int)
    pbp["defense_penalty_z"] = (
        pbp["penalty_team"] == pbp["drive_defense"]
    ).astype(int)

    off_penalties = (
        pbp[pbp["playcall"].isin(NON_DEFENSIVE_TIMEOUTS)]
        .groupby(["posteam", "season"], as_index=False)
        .aggregate({"offense_penalty_z": "mean"})
    )
    def_penalties = (
        pbp[pbp["playcall"].isin(NON_DEFENSIVE_TIMEOUTS)]
        .groupby(["defteam", "season"], as_index=False)
        .aggregate({"defense_penalty_z": "mean"})
    )

    # NOTE: positive z score is good, meaning they commit less penalties
    off_penalties["offense_penalty_z"] = (
        -1
        * (
            off_penalties["offense_penalty_z"]
            - off_penalties["offense_penalty_z"].mean()
        )
        / off_penalties["offense_penalty_z"].std()
    )
    def_penalties["defense_penalty_z"] = (
        -1
        * (
            def_penalties["defense_penalty_z"]
            - def_penalties["defense_penalty_z"].mean()
        )
        / def_penalties["defense_penalty_z"].std()
    )
    return (off_penalties, def_penalties)


def load_pbp_data(
    seasons: Iterable[int],
    force_reload: bool = True,
) -> pd.DataFrame:
    seasons_list = list(seasons)
    filename = f"{DATA_PATH}/pbp/{min(seasons_list)}-{max(seasons_list)}.csv"
    if force_reload or not os.path.exists(filename):
        try:
            df: pd.DataFrame = nfl_data_py.import_pbp_data(seasons_list)
            df.to_csv(filename, index=False)
            return df
        except:
            if force_reload:
                print(f"Loading backed up PBP data from {filename}")

    return pd.read_csv(filename)


def parse_weather_data(
    weather: str,
) -> Dict[str, Union[str, int, float, None]]:
    weather_dict: Dict[str, Union[str, int, float, None]] = {
        "condition": None,
        "temperature": None,
        "humidity": None,
        "wind_direction": None,
        "wind_speed": None,
    }

    # Extract condition
    condition_match = re.match(r"^(?P<condition>.*?)(?=( Temp:))", weather)
    condition = ""
    if condition_match:
        condition = condition_match.group("condition").strip()
        weather_dict["condition"] = condition

    is_indoor = "indoor" in condition.lower()

    # Extract temperature
    temp_match = re.search(r"Temp: (?P<temp>\d+)", weather)
    if temp_match:
        weather_dict["temperature"] = int(temp_match.group("temp"))
    elif is_indoor:
        weather_dict["temperature"] = 70.0

    # Extract humidity
    if is_indoor:
        weather_dict["humidity"] = 40.0
    else:
        humidity_match = re.search(r"Humidity: (?P<humidity>\d+)%", weather)
        if humidity_match:
            weather_dict["humidity"] = int(humidity_match.group("humidity"))

    if is_indoor:
        weather_dict["wind_speed"] = 0
        weather_dict["wind_direction"] = "indoor"
    else:
        # Extract wind speed and direction
        wind_match = re.search(
            r"Wind: (?:(?P<direction>[a-zA-Z]+(?: [a-zA-Z]+)?) )?(?P<speed>\d+)?",
            weather,
        )
        if wind_match:
            weather_dict["wind_direction"] = wind_match.group("direction")
            wind_speed = wind_match.group("speed")
            weather_dict["wind_speed"] = int(wind_speed) if wind_speed else 0

    return weather_dict


_DIRECTION_REPLACEMENTS = {
    "east": "e",
    "west": "w",
    "south": "s",
    "north": "n",
}

NORTH = 0
EAST = 90
SOUTH = 180
WEST = 270

NORTHEAST = (NORTH + EAST) / 2
NORTHWEST = (NORTH + 360 + WEST) / 2
SOUTHEAST = (SOUTH + EAST) / 2
SOUTHWEST = (SOUTH + WEST) / 2

_DIRECTION_DEGREE_MAP = {
    "calm": None,
    "e": EAST,
    "en": NORTHEAST,
    "ene": (EAST + NORTHEAST) / 2,
    "ese": (EAST + SOUTHEAST) / 2,
    "indoor": None,
    "n": NORTH,
    "nby": None,
    "ne": NORTHEAST,
    "nne": (NORTH + NORTHEAST) / 2,
    "nnecalm": (NORTH + NORTHEAST) / 2,
    "nnw": (NORTH + NORTHWEST) / 2,
    "none": None,
    "nw": NORTHWEST,
    "s": SOUTH,
    "se": SOUTHEAST,
    "serly": None,
    "sse": (SOUTH + SOUTHEAST) / 2,
    "ssw": (SOUTH + SOUTHWEST) / 2,
    "sw": SOUTHWEST,
    "w": WEST,
    "we": SOUTHWEST,
    "wnw": (WEST + NORTHWEST) / 2,
    "wsw": (WEST + SOUTHWEST) / 2,
}


def parse_wind_direction_degrees(
    wind_direction: Optional[str],
) -> Optional[float]:
    if not wind_direction:
        return None
    wd = wind_direction.lower()
    if wind_direction == "Southerly":
        return SOUTH
    rotate_180 = wd.startswith("from ")
    wd = wd.replace("from ", "").replace("calm", "").strip()
    for k, v in _DIRECTION_REPLACEMENTS.items():
        wd = wd.replace(k, v)

    lookup = wd.replace(" ", "")
    if lookup in _DIRECTION_DEGREE_MAP:
        degrees = _DIRECTION_DEGREE_MAP[lookup] + (180 if rotate_180 else 0)
        return degrees % 360
    return None


_PROB_RAIN_CONDITIONS = [
    "rain chance",
    "chance of rain",
    "change of rain",
    "threat of rain",
    "a few light flakes of snow",
]

_SOME_RAIN = [
    "light snow",
    "light rain",
    "periods of rain",
    "shower",
    # 'scattered showers',
    # 'rain shower',
    # 'with showers',
    # 'snow showers',
    "rain expected",
    "snow flurries",
    "rain likely",
]

_HAS_RAIN = [
    "rain",
    # 'steady rain',
    # 'rainy',
    # 'raining',
    "snow",
    # 'with snow',
]


def parse_rain(parsed_condition: Optional[str]) -> float:
    if parsed_condition is None:
        return 0.0

    condition = parsed_condition.lower()
    if any(x in condition for x in _PROB_RAIN_CONDITIONS):
        return 0.25

    if any(x in condition for x in _SOME_RAIN):
        return 0.50

    if any(x in condition for x in _HAS_RAIN):
        return 1.0

    return 0.0


def make_weather_df(full_pbp: pd.DataFrame) -> pd.DataFrame:
    has_weather = full_pbp[~full_pbp["weather"].isna()][
        ["game_id", "weather", "season", "home_team", "roof", "location"]
    ].drop_duplicates(subset=["game_id"])

    parsed_weather_rows = []
    for game in has_weather.to_dict(orient="records"):
        parsed_weather_rows.append(
            {**game, **parse_weather_data(game["weather"])}
        )
    parsed_weather_df = pd.DataFrame(parsed_weather_rows)

    parsed_weather_df["rain"] = parsed_weather_df["condition"].apply(
        parse_rain
    )
    parsed_weather_df["is_closed"] = parsed_weather_df["roof"].isin(
        {"closed", "dome"}
    )
    # parsed_weather_df.apply(is_stadium_dome, axis=1)
    parsed_weather_df["degrees_north"] = parsed_weather_df.apply(
        get_stadium_orientation_degrees, axis=1  # type: ignore
    )
    parsed_weather_df["wind_direction_degrees"] = parsed_weather_df[
        "wind_direction"
    ].apply(parse_wind_direction_degrees)
    parsed_weather_df.loc[
        parsed_weather_df["wind_direction"].isin({"indoor", "none", "Calm"}),
        "wind_speed",
    ] = 0
    parsed_weather_df.loc[parsed_weather_df["is_closed"], "wind_speed"] = 0

    has_orientation = ~parsed_weather_df["degrees_north"].isna()
    has_temp = (~parsed_weather_df["temperature"].isna()) | parsed_weather_df[
        "is_closed"
    ]
    has_humidity = ~parsed_weather_df["humidity"].isna()
    has_wind = (~parsed_weather_df["wind_speed"].isna()) & (
        (parsed_weather_df["wind_speed"] == 0)
        | ~parsed_weather_df["wind_direction_degrees"].isna()
    )
    non_null_weather = has_temp & has_humidity & has_wind & has_orientation

    _PARSED_COLS = [
        "game_id",
        "rain",
        "temp_pct",
        "humidity_pct",
        "wind_speed_z",
    ]  # ,'wind_direction_degrees']
    weather_df = parsed_weather_df[non_null_weather].reset_index(drop=True)
    weather_df["temp_pct"] = weather_df["temperature"] / 100.0
    weather_df["humidity_pct"] = weather_df["humidity"] / 100.0
    weather_df["wind_speed_z"] = (
        weather_df["wind_speed"] - weather_df["wind_speed"].mean()
    ) / weather_df["wind_speed"].std()
    return weather_df
