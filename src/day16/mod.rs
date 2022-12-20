
#[test]
fn day16() {
    let input = include_str!("input.txt");
    // let input = include_str!("input_test.txt");

    let (nodes, edges, start) = parse_input(input);
    let distances = shortest_distances(&nodes, &edges);
    let valves = nonzero_flowrate(&nodes);

    let max_release = max_release(&nodes, &distances, &valves, start);
    println!("{:#?}", max_release);

    let max_release = max_release_2(&nodes, &distances, &valves, start);
    println!("{:#?}", max_release);
}

#[derive(Debug)]
struct Node {
    flowrate: u64,
}

fn parse_input(input: &str) -> (Vec<Node>,Vec<Vec<bool>>,usize) {
    use std::collections::HashMap;

    // parse input and add nodes
    let mut nodes = Vec::new();
    let mut label_to_idx = HashMap::new();

    for (idx,line) in input.lines().enumerate() {
        let mut parts = line.split([' ','=',';',',']);

        let label = parts.nth(1).unwrap().to_string();
        let flowrate = parts.nth(3).unwrap().parse::<u64>().unwrap();

        label_to_idx.insert(label,idx);
        nodes.push(Node{flowrate});
    }

    // parse edges
    let mut edges = vec![vec![false;nodes.len()];nodes.len()];

    for (idx,line) in input.lines().enumerate() {
        let mut parts = line.split([' ','=',';',',']);
        parts.nth(10);

        for t in parts {
            if t.is_empty() {continue;}
            let to_idx = label_to_idx.get(t).unwrap();
            edges[idx][*to_idx] = true;
        }
    }

    return (nodes,edges, *label_to_idx.get("AA").unwrap())
}

fn shortest_distances(nodes: &Vec<Node>, edges: &Vec<Vec<bool>>) -> Vec<Vec<u64>> {
    // floyd-warshall
    let n_nodes = nodes.len();
    let mut dist = vec![vec![u64::MAX/2;n_nodes];n_nodes];

    for from in 0..n_nodes {
        for to in 0..n_nodes {
            if edges[from][to] { dist[from][to] = 1; }
        }
    }

    for node in 0..n_nodes { dist[node][node] = 0; }

    for k in 0..n_nodes {
        for i in 0..n_nodes {
            for j in 0..n_nodes {
                dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j])
            }
        }
    }

    dist
}

fn nonzero_flowrate(nodes: &Vec<Node>) -> Vec<usize> {
    nodes.iter().enumerate()
        .filter(|(_, n)| n.flowrate > 0)
        .map(|(i, _)| i)
        .collect::<Vec<_>>()
}

fn max_release(
        nodes: &Vec<Node>, distances: &Vec<Vec<u64>>, valves: &Vec<usize>,
        start: usize,
        ) -> u64 {

    const MAX_TIME: u64 = 30;

    #[derive(Debug)]
    struct State {
        time: u64,
        release: u64,
        closed_valves: Vec<usize>,
        node: usize,
    }

    let mut max_release = 0;
    let mut front = Vec::new();
    front.push(State {
        time: 0,
        release: 0,
        closed_valves: valves.clone(),
        node: start,
    });

    while let Some(state) = front.pop() {
        for (valve_idx,&valve) in state.closed_valves.iter().enumerate() {
            let new_time = state.time + distances[state.node][valve] + 1;

            if new_time < MAX_TIME {
                let mut new_state = State {
                    time: new_time,
                    release: state.release + (MAX_TIME - new_time) * nodes[valve].flowrate,
                    closed_valves: state.closed_valves.clone(),
                    node: valve,
                };
                new_state.closed_valves.swap_remove(valve_idx);

                max_release = max_release.max(new_state.release);
                front.push(new_state);
            }
        }
    }

    max_release
}

fn max_release_2(
        nodes: &Vec<Node>, distances: &Vec<Vec<u64>>, valves: &Vec<usize>,
        start: usize,
        ) -> u64 {

    const MAX_TIME: u64 = 26;

    #[derive(Debug, Clone)]
    struct Actor {
        time: u64,
        release: u64,
        node: usize,
    }

    let mut max_release = 0;
    let mut front = Vec::new();
    front.push((
        valves.clone(),
        Actor {
            time: 0,
            release: 0,
            node: start,
        },
        Actor {
            time: 0,
            release: 0,
            node: start,
        },
    ));

    while let Some((closed_valves, actor1, actor2)) = front.pop() {
        let (active, passive) = if actor1.time <= actor2.time {
            (actor1, actor2)
        } else {
            (actor2, actor1)
        };

        for (valve_idx, &valve) in closed_valves.iter().enumerate() {
            let new_time = active.time + distances[active.node][valve] + 1;

            if new_time < MAX_TIME {
                let mut new_closed_valves = closed_valves.clone();
                new_closed_valves.swap_remove(valve_idx);

                let new_active = Actor {
                    time: new_time,
                    release: active.release + (MAX_TIME - new_time) * nodes[valve].flowrate,
                    node: valve,
                };

                max_release = max_release.max(new_active.release + passive.release);
                front.push((new_closed_valves, new_active, passive.clone()));
            }
        }
    }

    max_release
}
