use std::collections::{HashMap, HashSet};

pub fn greedy_set_cover(
    states_needed: &HashSet<&str>,
    stations: &HashMap<&str, HashSet<&str>>,
) -> HashSet<String> {
    let mut states_uncovered = states_needed.clone();
    let mut final_stations: HashSet<String> = HashSet::new();

    while !states_uncovered.is_empty() {
        let mut best_station: Option<&str> = None;
        let mut states_covered: HashSet<&str> = HashSet::new();

        for (station, states_for_station) in stations.iter() {
            let covered: HashSet<&str> = states_uncovered
                .intersection(states_for_station)
                .cloned()
                .collect();

            if covered.len() > states_covered.len() {
                best_station = Some(station);
                states_covered = covered;
            }
        }

        if let Some(station) = best_station {
            final_stations.insert(station.to_string());
            for state in states_covered {
                states_uncovered.remove(state);
            }
        } else {
            break; // No progress possible (incomplete coverage)
        }
    }

    final_stations
}
