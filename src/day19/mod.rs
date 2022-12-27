#[test]
fn day19() {
    let input = include_str!("input.txt");
    // let input = include_str!("input_test.txt");

    let blueprints = parse_blueprints(input);

    // part 1
    let tot_quality_level = blueprints
        .iter()
        .enumerate()
        .map(|(i, bp)| (i as u32 + 1) * max_geodes(bp, 24))
        .sum::<u32>();
    println!("{}", tot_quality_level);

    // part 2
    let geode_prod = blueprints
        .iter()
        .take(3)
        .map(|bp| max_geodes(bp, 32))
        .product::<u32>();
    println!("{}", geode_prod);
}

#[derive(Debug)]
struct Blueprint {
    ore_robot_ore_cost: u32,
    clay_robot_ore_cost: u32,

    obsidian_robot_ore_cost: u32,
    obsidian_robot_clay_cost: u32,

    geode_robot_ore_cost: u32,
    geode_robot_obsidian_cost: u32,

    max_ore_consumption: u32,
}

#[derive(Debug,Clone)]
struct State {
    time: u32,

    ore: u32,
    clay: u32,
    obsidian: u32,
    geodes: u32,

    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32
}

#[derive(Debug,PartialEq)]
enum BuildDecision {
    OreRobot,
    ClayRobot,
    ObsidianRobot,
    GeodeRobot,
}

use BuildDecision::*;

const BUILD_DECISIONS: [BuildDecision; 4] = [
    GeodeRobot,
    ObsidianRobot,
    ClayRobot,
    OreRobot,
];

fn parse_blueprints(s: &str) -> Vec<Blueprint> {
    let mut blueprints = Vec::new();

    for line in s.lines() {
        let mut parts = line.split(' ');
        let ore_robot_ore_cost = parts.nth(6).unwrap().parse::<u32>().unwrap();
        let clay_robot_ore_cost = parts.nth(5).unwrap().parse::<u32>().unwrap();
        let obsidian_robot_ore_cost = parts.nth(5).unwrap().parse::<u32>().unwrap();
        let obsidian_robot_clay_cost = parts.nth(2).unwrap().parse::<u32>().unwrap();
        let geode_robot_ore_cost = parts.nth(5).unwrap().parse::<u32>().unwrap();
        let geode_robot_obsidian_cost = parts.nth(2).unwrap().parse::<u32>().unwrap();

        let max_ore_consumption =
            ore_robot_ore_cost.max(
                clay_robot_ore_cost.max(
                    obsidian_robot_ore_cost.max(
                        geode_robot_ore_cost
                        )
                    )
                );

        blueprints.push(Blueprint {
            ore_robot_ore_cost,
            clay_robot_ore_cost,
            obsidian_robot_ore_cost,
            obsidian_robot_clay_cost,
            geode_robot_ore_cost,
            geode_robot_obsidian_cost,
            max_ore_consumption,
        });
    }

    blueprints
}

fn can_wait_for_resources(state: &State, decision: &BuildDecision) -> bool {
    match decision {
        OreRobot => state.ore_robots > 0,
        ClayRobot => state.ore_robots > 0,
        ObsidianRobot => state.ore_robots > 0 && state.clay_robots > 0,
        GeodeRobot => state.ore_robots > 0 && state.obsidian_robots > 0,
    }
}

fn time_to_collect_resources(state: &State, decision: &BuildDecision, blueprint: &Blueprint) -> u32 {
    let time_to_collect_resource = |required,current,rate| {
        if current >= required {
            0
        } else {
            // divide rounded up
            if (required - current) % rate == 0 {
                (required - current) / rate
            } else {
                (required - current) / rate + 1
            }
        }
    };

    match decision {
        OreRobot => {
            time_to_collect_resource(blueprint.ore_robot_ore_cost,
                                     state.ore,
                                     state.ore_robots)
        },
        ClayRobot => {
            time_to_collect_resource(blueprint.clay_robot_ore_cost,
                                     state.ore,
                                     state.ore_robots)
        },
        ObsidianRobot => {
            let time_to_collect_ore =
                time_to_collect_resource(blueprint.obsidian_robot_ore_cost,
                                         state.ore,
                                         state.ore_robots);
            let time_to_collect_clay =
                time_to_collect_resource(blueprint.obsidian_robot_clay_cost,
                                         state.clay,
                                         state.clay_robots);

            time_to_collect_ore.max(time_to_collect_clay)
        },
        GeodeRobot => {
            let time_to_collect_ore =
                time_to_collect_resource(blueprint.geode_robot_ore_cost,
                                         state.ore,
                                         state.ore_robots);
            let time_to_collect_obsidian =
                time_to_collect_resource(blueprint.geode_robot_obsidian_cost,
                                         state.obsidian,
                                         state.obsidian_robots);

            time_to_collect_ore.max(time_to_collect_obsidian)
        },
    }
}

fn wait_for_resources(state: &mut State, time: u32) {
    state.time += time;
    state.ore += time * state.ore_robots;
    state.clay += time * state.clay_robots;
    state.obsidian += time * state.obsidian_robots;
    state.geodes += time * state.geode_robots;
}

fn build(state: &mut State, decision: &BuildDecision, blueprint: &Blueprint) {
    match decision {
        OreRobot => {
            state.ore -= blueprint.ore_robot_ore_cost;
            state.ore_robots += 1;
        }
        ClayRobot => {
            state.ore -= blueprint.clay_robot_ore_cost;
            state.clay_robots += 1;
        }
        ObsidianRobot => {
            state.ore -= blueprint.obsidian_robot_ore_cost;
            state.clay -= blueprint.obsidian_robot_clay_cost;
            state.obsidian_robots += 1;
        }
        GeodeRobot => {
            state.ore -= blueprint.geode_robot_ore_cost;
            state.obsidian -= blueprint.geode_robot_obsidian_cost;
            state.geode_robots += 1;
        }
    }
}

fn pointless_to_build(state: &State, decision: &BuildDecision, blueprint: &Blueprint) -> bool {
    if state.obsidian_robots >= blueprint.geode_robot_obsidian_cost {
        if *decision == ObsidianRobot { return true; }
        if *decision == ClayRobot { return true; }
        if *decision == OreRobot
            && state.ore_robots >= blueprint.geode_robot_ore_cost { return true; }
    }

    if state.clay_robots >= blueprint.obsidian_robot_clay_cost {
        let max_ore = blueprint.geode_robot_ore_cost.max(blueprint.obsidian_robot_ore_cost);

        if *decision == ClayRobot { return true; }
        if *decision == OreRobot && state.ore_robots >= max_ore { return true; }
    }

    if state.ore_robots >= blueprint.max_ore_consumption {
        if *decision == OreRobot { return true; }
    }

    return false
}

fn max_geodes(blueprint: &Blueprint, max_time: u32) -> u32 {
    let mut front = Vec::new();
    front.push( State {
        time: 0,

        ore: 0,
        clay: 0,
        obsidian: 0,
        geodes: 0,

        ore_robots: 1,
        clay_robots: 0,
        obsidian_robots: 0,
        geode_robots: 0,
    });
    let mut max_geodes = 0;

    while let Some(state) = front.pop() {

        let mut geodes_upper_bound = state.geodes;
        geodes_upper_bound += (max_time - state.time)*state.geode_robots;
        geodes_upper_bound += (max_time - state.time)*(max_time-state.time)/2;
        if geodes_upper_bound <= max_geodes { continue; }

        let mut had_time_to_build = false;

        for decision in BUILD_DECISIONS.iter() {
            if !can_wait_for_resources(&state, decision) { continue; }

            let time_to_wait = time_to_collect_resources(&state, decision, blueprint);

            if state.time + time_to_wait + 1 >= max_time { continue; }

            if pointless_to_build(&state, decision, blueprint) { continue; }

            had_time_to_build = true;
            let mut new_state = state.clone();
            wait_for_resources(&mut new_state, time_to_wait + 1);
            build(&mut new_state, decision, blueprint);
            front.push(new_state);
        }

        if !had_time_to_build {
            // wait out the time and compare end result
            let mut new_state = state.clone();
            wait_for_resources(&mut new_state, max_time - state.time);
            max_geodes = max_geodes.max(new_state.geodes);
        }
    }

    max_geodes
}
