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

    location: u64, // location as index
    traveling: usize,

    ele_turn: bool,
    ele_location: u64,
    ele_traveling: usize,

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

        //min_predicted_self.cmp(&min_predicted_other)

        // ticks left, valves open, min predicted flow
        //(self.max_ticks - self.ticks, self.valves.count_ones(), min_predicted_self).cmp(&(other.max_ticks - other.ticks, other.valves.count_ones(), min_predicted_other))
        (min_predicted_self, self.max_ticks - self.ticks)
            .cmp(&(min_predicted_other, self.max_ticks - self.ticks))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part1(input: &str) -> u64 {
    let mut valves = parse_input(input);
    //valves_as_graphviz(&valves);

    colaesce_tunnels(&mut valves);
    //valves_as_graphviz(&valves);
    //dbg!(&valves);

    let graph: HashMap<&str, &Valve> = valves
        .iter()
        .map(|valve| (valve.name.as_str(), valve))
        .collect();

    let max_flow_rate: u64 = valves.iter().map(|valve| valve.flow_rate).sum();

    let mut max_flow = 0;
    let mut queue = BinaryHeap::new();

    let start = &graph["AA"];
    let state = State {
        max_ticks: 30,
        ticks: 0,

        location: start.idx,
        traveling: 0,

        ele_turn: false,
        ele_location: start.idx,
        ele_traveling: 0,

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

        if state.traveling > 0 {
            state.traveling -= 1;
        }

        //dbg!(&state);

        let predicted_flow = state.flow + state.flow_rate * (state.max_ticks - state.ticks) as u64;

        if predicted_flow > max_flow {
            max_flow = predicted_flow;
            //println!("max flow: {max_flow} @ {} of {max_flow_rate} in {}", state.flow_rate, state.ticks);
        }

        if state.ticks < state.max_ticks {
            // trim poor performing branches
            if max_flow >= state.flow + max_flow_rate * (state.max_ticks - state.ticks) as u64 {
                continue;
            }

            // if we're still traveling, just requeue ourselves
            if state.traveling > 0 {
                queue.push(state);
                continue;
            }

            let current_valve = &valves[state.location as usize];

            // if the current valve has flow rate and isn't on, push a state to turn it on
            if current_valve.flow_rate > 0 && state.valves & (1 << state.location) == 0 {
                let mut valve_state = state.clone();

                valve_state.valves |= 1 << state.location;
                valve_state.flow_rate += current_valve.flow_rate;

                queue.push(valve_state);
            }

            for connection in &current_valve.connections {
                let connection_valve = &graph[connection.destination.as_str()];
                let mut connection_state = state.clone();

                connection_state.location = connection_valve.idx;
                connection_state.visited |= 1 << connection_valve.idx;

                connection_state.traveling = connection.cost;

                queue.push(connection_state);
            }
        }
    }

    max_flow
}

fn part2(input: &str) -> u64 {
    let mut valves = parse_input(input);
    //valves_as_graphviz(&valves);

    colaesce_tunnels(&mut valves);
    //valves_as_graphviz(&valves);
    //dbg!(&valves);

    let graph: HashMap<&str, &Valve> = valves
        .iter()
        .map(|valve| (valve.name.as_str(), valve))
        .collect();

    let max_flow_rate: u64 = valves.iter().map(|valve| valve.flow_rate).sum();

    let mut max_flow = 0;
    let mut queue = BinaryHeap::new();

    let start = &graph["AA"];
    let state = State {
        max_ticks: 26,
        ticks: 0,

        location: start.idx,
        traveling: 0,

        ele_turn: false,
        ele_location: start.idx,
        ele_traveling: 0,

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
        } else {
            if state.traveling > 0 {
                state.traveling -= 1;
            }

            if state.ele_traveling > 0 {
                state.ele_traveling -= 1;
            }
        }

        //dbg!(&state);

        if state.ticks < state.max_ticks {
            // trim poor performing branches
            if max_flow >= state.flow + max_flow_rate * (state.max_ticks - state.ticks) as u64 {
                continue;
            }

            let traveling;
            let location;

            if state.ele_turn {
                traveling = state.ele_traveling;
                location = state.ele_location;
            } else {
                traveling = state.traveling;
                location = state.location;
            }

            // if we're still traveling push ourself on queue in next turn
            if traveling > 0 {
                state.ele_turn = !state.ele_turn;

                queue.push(state);
                continue;
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
                let connection_valve = &graph[connection.destination.as_str()];

                // avoid the elephant coming to the same node as us
                if state.ele_turn && connection_valve.idx == state.location {
                    continue;
                }

                let mut connection_state = state.clone();
                connection_state.ele_turn = !state.ele_turn;

                if state.ele_turn {
                    connection_state.ele_location = connection_valve.idx;
                    connection_state.ele_traveling = connection.cost;
                } else {
                    connection_state.location = connection_valve.idx;
                    connection_state.traveling = connection.cost;
                }

                connection_state.visited |= 1 << connection_valve.idx;

                queue.push(connection_state);
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
    connections: Vec<Tunnel>,
}

#[derive(Debug)]
struct Tunnel {
    destination: String,
    cost: usize,
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
                    .map(|str| Tunnel {
                        destination: String::from(str),
                        cost: 1,
                    })
                    .collect(),
            }
        })
        .collect()
}

fn colaesce_tunnels(valves: &mut Vec<Valve>) {
    // look for junctions that have 0 flow rate and 2 connections - do not remove AA (start point)
    while let Some(candidate_idx) = valves.iter().position(|valve| {
        valve.name != "AA" && valve.flow_rate == 0 && valve.connections.len() == 2
    }) {
        let candidate = valves.remove(candidate_idx);

        let Tunnel {
            destination: left_id,
            cost: left_cost,
        } = &candidate.connections[0];
        let Tunnel {
            destination: right_id,
            cost: right_cost,
        } = &candidate.connections[1];

        let total_cost = left_cost + right_cost;

        let left_idx = valves
            .iter()
            .position(|valve| &valve.name == left_id)
            .unwrap();
        let right_idx = valves
            .iter()
            .position(|valve| &valve.name == right_id)
            .unwrap();

        let left = &mut valves[left_idx];

        let left_connection_idx = left
            .connections
            .iter()
            .position(|tunnel| tunnel.destination == candidate.name)
            .unwrap();
        let mut left_connection = &mut left.connections[left_connection_idx];

        left_connection.destination = String::from(right_id);
        left_connection.cost = total_cost;

        let right = &mut valves[right_idx];

        let right_connection_idx = right
            .connections
            .iter()
            .position(|tunnel| tunnel.destination == candidate.name)
            .unwrap();
        let mut right_connection = &mut right.connections[right_connection_idx];

        right_connection.destination = String::from(left_id);
        right_connection.cost = total_cost;
    }

    // fixup idx values
    valves
        .iter_mut()
        .enumerate()
        .for_each(|(idx, valve)| valve.idx = idx as u64);
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
            if !connected.contains(connection.destination.as_str()) {
                println!("{} -- {}", valve.name, connection.destination);
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
