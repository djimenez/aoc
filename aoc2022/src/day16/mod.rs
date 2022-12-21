use regex::Regex;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

const DEFAULT_INPUT: &str = include_str!("input");

pub fn run() {
    println!("part1: {}", part1(DEFAULT_INPUT));
    println!("part2: {}", part2(DEFAULT_INPUT));
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct State {
    max_ticks: usize,
    ticks: usize,

    previous: u64, // location as index
    location: u64, // location as index

    ele_turn: bool,
    ele_previous: u64,
    ele_location: u64,

    valves: u64,  // used as bitfield
    visited: u64, // used as bitfield

    flow_rate: u64,
    flow: u64,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        let min_predicted_self = self.flow + self.flow_rate * (self.max_ticks - self.ticks) as u64;
        let min_predicted_other =
            other.flow + other.flow_rate * (other.max_ticks - other.ticks) as u64;

        min_predicted_self.cmp(&min_predicted_other)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part1(input: &str) -> u64 {
    let valves = parse_input(input);
    let graph: HashMap<&str, &Valve> = valves
        .iter()
        .map(|valve| (valve.name.as_str(), valve))
        .collect();

    //valves_as_graphviz(&valves);

    let max_flow_rate: u64 = valves.iter().map(|valve| valve.flow_rate).sum();

    let mut max_flow = 0;
    let mut queue = BinaryHeap::new();

    let start = &graph["AA"];
    let state = State {
        max_ticks: 30,
        ticks: 0,

        previous: start.idx,
        location: start.idx,

        ele_turn: false,
        ele_previous: start.idx,
        ele_location: start.idx,

        valves: 0,
        visited: 0,

        flow_rate: 0,
        flow: 0,
    };

    queue.push(state);

    while queue.len() > 0 {
        let mut state = queue.pop().unwrap();

        state.ticks += 1;
        state.flow += state.flow_rate;

        //dbg!(&state);

        let predicted_flow = state.flow + state.flow_rate * (state.max_ticks - state.ticks) as u64;

        if predicted_flow > max_flow {
            max_flow = predicted_flow;
            //println!("max flow: {max_flow} @ {} of {max_flow_rate} in {}", state.flow_rate, state.ticks);
        }

        if state.ticks < state.max_ticks {
            // trim poor performing branches
            if max_flow > state.flow + max_flow_rate * (state.max_ticks - state.ticks) as u64 {
                continue;
            }

            let current_valve = &valves[state.location as usize];

            // if the current valve isn't on, push a state to turn it on
            // note: ignore valves with 0 flow rate
            if current_valve.flow_rate > 0 && state.valves & (1 << state.location) == 0 {
                let mut valve_state = state.clone();

                valve_state.valves |= 1 << state.location;
                valve_state.flow_rate += current_valve.flow_rate;

                queue.push(valve_state);
            }

            for connection in &current_valve.connections {
                let connection_valve = &graph[connection.as_str()];

                // avoid returning to a previous node if our current one is a 0 flow rate
                // we want to promote continuing through these nodes
                if current_valve.flow_rate > 0 || connection_valve.idx != state.previous {
                    let mut connection_state = state.clone();

                    connection_state.previous = state.location;
                    connection_state.location = connection_valve.idx;
                    connection_state.visited |= 1 << connection_valve.idx;

                    queue.push(connection_state);
                }
            }
        }
    }

    max_flow
}

fn part2(input: &str) -> u64 {
    let valves = parse_input(input);
    let graph: HashMap<&str, &Valve> = valves
        .iter()
        .map(|valve| (valve.name.as_str(), valve))
        .collect();

    //valves_as_graphviz(&valves);

    let max_flow_rate: u64 = valves.iter().map(|valve| valve.flow_rate).sum();

    let mut max_flow = 0;
    let mut queue = BinaryHeap::new();

    let start = &graph["AA"];
    let state = State {
        max_ticks: 26,
        ticks: 0,

        previous: start.idx,
        location: start.idx,

        ele_turn: false,
        ele_previous: start.idx,
        ele_location: start.idx,

        valves: 0,
        visited: 0,

        flow_rate: 0,
        flow: 0,
    };

    queue.push(state);

    while queue.len() > 0 {
        let mut state = queue.pop().unwrap();

        if !state.ele_turn {
            state.ticks += 1;
            state.flow += state.flow_rate;

            let predicted_flow =
                state.flow + state.flow_rate * (state.max_ticks - state.ticks) as u64;

            if predicted_flow > max_flow {
                max_flow = predicted_flow;
                //println!("max flow: {max_flow} @ {} of {max_flow_rate} in {}", state.flow_rate, state.ticks);
            }
        }

        //dbg!(&state);

        if state.ticks < state.max_ticks {
            // trim poor performing branches
            if !state.ele_turn
                && max_flow > state.flow + max_flow_rate * (state.max_ticks - state.ticks) as u64
            {
                continue;
            }

            let previous;
            let location;

            if state.ele_turn {
                previous = state.ele_previous;
                location = state.ele_location;
            } else {
                previous = state.previous;
                location = state.location;
            }

            let current_valve = &valves[location as usize];

            // if the current valve isn't on, push a state to turn it on
            // note: ignore valves with 0 flow rate
            if current_valve.flow_rate > 0 && state.valves & (1 << location) == 0 {
                let mut valve_state = state.clone();

                valve_state.ele_turn = !state.ele_turn;
                valve_state.valves |= 1 << location;
                valve_state.flow_rate += current_valve.flow_rate;

                queue.push(valve_state);
            }

            for connection in &current_valve.connections {
                let connection_valve = &graph[connection.as_str()];

                // avoid the elephant coming to the same node as us
                if state.ele_turn && connection_valve.idx == state.location {
                    continue;
                }

                // avoid returning to a previous node if our current one is a 0 flow rate
                // we want to promote continuing through these nodes
                if current_valve.flow_rate > 0 || connection_valve.idx != previous {
                    let mut connection_state = state.clone();
                    connection_state.ele_turn = !state.ele_turn;

                    if state.ele_turn {
                        connection_state.ele_previous = location;
                        connection_state.ele_location = connection_valve.idx;
                    } else {
                        connection_state.previous = location;
                        connection_state.location = connection_valve.idx;
                    }

                    connection_state.visited |= 1 << connection_valve.idx;

                    queue.push(connection_state);
                }
            }
        }
    }

    max_flow
}

#[derive(Debug)]
struct Valve {
    idx: u64,
    name: String,
    flow_rate: u64,
    connections: Vec<String>,
}

fn parse_input(input: &str) -> Vec<Valve> {
    let re = Regex::new(r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z]{2}(:?, [A-Z]{2})*)").unwrap();
    input
        .lines()
        .enumerate()
        .map(move |(idx, line)| {
            let captures = re.captures(line).unwrap();

            Valve {
                idx: idx as u64,
                name: String::from(&captures[1]),
                flow_rate: captures[2].parse().unwrap(),
                connections: captures[3]
                    .split(", ")
                    .map(|str| String::from(str))
                    .collect(),
            }
        })
        .collect()
}

#[allow(dead_code)]
fn valves_as_graphviz(valves: &Vec<Valve>) {
    let mut connected = HashSet::new();

    println!("graph day16 {{");

    for valve in valves {
        println!(
            "{} [label=\"{}\\n{}\"]",
            valve.name, valve.name, valve.flow_rate
        );
    }

    for valve in valves {
        connected.insert(valve.name.as_str());

        for connection in &valve.connections {
            if !connected.contains(connection.as_str()) {
                println!("{} -- {}", valve.name, connection);
            }
        }
    }

    println!("}}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 1651)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 1707)
    }
}
