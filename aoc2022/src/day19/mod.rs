use regex::Regex;

const DEFAULT_INPUT: &str = include_str!("input");

pub fn run() {
    println!("part1: {}", part1(DEFAULT_INPUT));
    println!("part2: {}", part2(DEFAULT_INPUT));
}

fn part1(input: &str) -> u64 {
    let blueprints = parse_input(input);

    blueprints
        .iter()
        .map(|blueprint| blueprint.id as u64 * eval_blueprint(24, blueprint))
        .sum()
}

fn part2(input: &str) -> u64 {
    let blueprints = parse_input(input);

    blueprints
        .iter()
        .take(3)
        .map(|blueprint| eval_blueprint(32, blueprint))
        .product()
}

#[derive(Debug)]
struct Blueprint {
    id: u8,

    ore_bot_ore: u8,
    clay_bot_ore: u8,
    obsidian_bot_ore: u8,
    obsidian_bot_clay: u8,
    geode_bot_ore: u8,
    geode_bot_obsidian: u8,
}

fn parse_input(input: &str) -> Vec<Blueprint> {
    // Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
    let re = Regex::new(r"^Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();

    input
        .lines()
        .map(|line| {
            let captures = re.captures(line).unwrap();

            Blueprint {
                id: captures[1].parse().unwrap(),
                ore_bot_ore: captures[2].parse().unwrap(),
                clay_bot_ore: captures[3].parse().unwrap(),
                obsidian_bot_ore: captures[4].parse().unwrap(),
                obsidian_bot_clay: captures[5].parse().unwrap(),
                geode_bot_ore: captures[6].parse().unwrap(),
                geode_bot_obsidian: captures[7].parse().unwrap(),
            }
        })
        .collect()
}

#[derive(Clone, Copy, Debug)]
struct State {
    ticks: u8,
    max_ticks: u8,

    ore: u8,
    clay: u8,
    obsidian: u8,
    geode: u8,

    ore_bots: u8,
    clay_bots: u8,
    obsidian_bots: u8,
    geode_bots: u8,

    max_ore_bots: u8,
}

fn eval_blueprint(max_ticks: u8, blueprint: &Blueprint) -> u64 {
    let state = State {
        ticks: 0,
        max_ticks,

        ore: 0,
        clay: 0,
        obsidian: 0,
        geode: 0,

        ore_bots: 1,
        clay_bots: 0,
        obsidian_bots: 0,
        geode_bots: 0,

        max_ore_bots: *[
            blueprint.ore_bot_ore,
            blueprint.clay_bot_ore,
            blueprint.obsidian_bot_ore,
            blueprint.geode_bot_ore,
        ]
        .iter()
        .max()
        .unwrap(),
    };

    let mut max_geodes = 0;

    eval_blueprint_from(blueprint, &state, &mut max_geodes);

    max_geodes
}

fn eval_blueprint_from(blueprint: &Blueprint, state: &State, max_geodes: &mut u64) {
    // if its not possible to exceed given max_quality from this branch point, prune it
    let ticks_left = state.max_ticks as u64 - state.ticks as u64;
    let max_extra = (ticks_left * ticks_left + ticks_left) / 2;

    if *max_geodes > state.geode as u64 + ticks_left * state.geode_bots as u64 + max_extra {
        return;
    }

    if state.ticks < state.max_ticks {
        let mut next_state = state.clone();

        next_state.ticks += 1;
        next_state.ore += next_state.ore_bots;
        next_state.clay += next_state.clay_bots;
        next_state.obsidian += next_state.obsidian_bots;
        next_state.geode += next_state.geode_bots;

        if state.obsidian >= blueprint.geode_bot_obsidian && state.ore >= blueprint.geode_bot_ore {
            next_state.geode_bots += 1;
            next_state.ore -= blueprint.geode_bot_ore;
            next_state.obsidian -= blueprint.geode_bot_obsidian;

            eval_blueprint_from(blueprint, &next_state, max_geodes);

            next_state.geode_bots -= 1;
            next_state.ore += blueprint.geode_bot_ore;
            next_state.obsidian += blueprint.geode_bot_obsidian;
        }

        // note: dont' make more obsidian bots than it costs to make a geode bot per tick
        if state.obsidian_bots < blueprint.geode_bot_obsidian
            && state.clay >= blueprint.obsidian_bot_clay
            && state.ore >= blueprint.obsidian_bot_ore
        {
            next_state.obsidian_bots += 1;
            next_state.ore -= blueprint.obsidian_bot_ore;
            next_state.clay -= blueprint.obsidian_bot_clay;

            eval_blueprint_from(blueprint, &next_state, max_geodes);

            next_state.obsidian_bots -= 1;
            next_state.ore += blueprint.obsidian_bot_ore;
            next_state.clay += blueprint.obsidian_bot_clay;
        }

        // note: don't make more clay bots than it costs to make an obsidian bot per tick
        if state.clay_bots < blueprint.obsidian_bot_clay && state.ore >= blueprint.clay_bot_ore {
            next_state.clay_bots += 1;
            next_state.ore -= blueprint.clay_bot_ore;

            eval_blueprint_from(blueprint, &next_state, max_geodes);

            next_state.clay_bots -= 1;
            next_state.ore += blueprint.clay_bot_ore;
        }

        // note: don't make more ore bots than max needed for clay, obsidition, or geode bot per tick
        if state.ore_bots < state.max_ore_bots && state.ore >= blueprint.ore_bot_ore {
            next_state.ore_bots += 1;
            next_state.ore -= blueprint.ore_bot_ore;

            eval_blueprint_from(blueprint, &next_state, max_geodes);

            next_state.ore_bots -= 1;
            next_state.ore += blueprint.ore_bot_ore;
        }

        // do nothing case
        eval_blueprint_from(blueprint, &next_state, max_geodes);
    } else {
        *max_geodes = state.geode as u64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 33)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 56 * 62)
    }
}
