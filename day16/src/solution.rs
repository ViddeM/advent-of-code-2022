use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Valves {
    tunnel_map: HashMap<String, Vec<String>>,
    flow_rate_map: HashMap<String, u32>,
}

pub fn parse<'a>(input: &str) -> Valves {
    let mut tunnel_map = HashMap::new();
    let mut flow_rate_map = HashMap::new();

    input.lines().for_each(|l| {
        let (valve, rest) = l
            .strip_prefix("Valve ")
            .unwrap()
            .split_once(" has flow rate=")
            .unwrap();

        let (flow_rate, rest) = rest.split_once("; ").unwrap();
        let tunnels_to = if let Some(singular) = rest.strip_prefix("tunnel leads to valve") {
            singular
        } else {
            rest.strip_prefix("tunnels lead to valves ").unwrap()
        };

        let tunnels_to = tunnels_to
            .split(", ")
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>();
        tunnel_map.insert(valve.to_string(), tunnels_to);
        flow_rate_map.insert(valve.to_string(), flow_rate.parse().unwrap());
    });

    Valves {
        tunnel_map,
        flow_rate_map,
    }
}

const INITIAL_MINUTES: u32 = 30;

#[inline(always)]
fn calculate_maximum_throughput(
    valves: &Valves,
    minutes_remaining: u32,
    curr_pos: &String,
    open_valves: &Vec<String>,
) -> u32 {
    if minutes_remaining == 0 {
        return 0;
    }

    let mut highest_option = 0;
    if open_valves.contains(&curr_pos) == false {
        let mut open_valves = open_valves.clone();
        open_valves.push(curr_pos.clone());
        let preassure_released = valves.flow_rate_map[curr_pos] * minutes_remaining;
        let future_moves =
            calculate_maximum_throughput(valves, minutes_remaining - 1, curr_pos, &open_valves);

        highest_option = future_moves + preassure_released;
    }

    let tunnels_to = valves.tunnel_map.get(curr_pos).unwrap();
    for tunnel in tunnels_to {
        let val = calculate_maximum_throughput(valves, minutes_remaining - 1, tunnel, open_valves);
        if val > highest_option {
            highest_option = val;
        }
    }

    highest_option
}

pub fn solve_part_one<'a>(input: Valves) -> String {
    todo!("Finish part 1");
    let throughput =
        calculate_maximum_throughput(&input, INITIAL_MINUTES, &"AA".to_string(), &vec![]);

    format!("{throughput}")
}

pub fn solve_part_two<'a>(input: Valves) -> String {
    todo!("Part two is not yet implemented");
}
