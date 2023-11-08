use std::io::stdin;
use std::collections::{BTreeMap, BTreeSet};

type Graph<'a > = BTreeMap<&'a str, BTreeMap<&'a str, usize>>;
fn dijakstra<'a>(graph: Graph<'a>, start: &'a str, goal: &'a str) -> Result<Vec<&'a str>, &'a str> {
    let mut open_list: BTreeSet<(usize, &str)> = BTreeSet::new();
    open_list.insert((0, start));

    // Assign to every node a tentative distance value
    // infinity for all other nodes
    let mut distances: BTreeMap<&str, usize> = graph.keys().map(|k|{ (*k, usize::MAX) }).collect();
    // set it to zero for start node
    distances.insert(start, 0);

    let mut origin: BTreeMap<&str, &str> = BTreeMap::new();

    while !open_list.is_empty() {
        let (_, current) = open_list.pop_first().unwrap();

        if current == goal  {
            // backtrack to reconstruct the path
            let mut path:Vec<&'a str> = vec![goal];
            let mut node = goal;
            while let Some(next) = origin.get(node) {
                node = next;
                path.push(node);
            }
         
            path.reverse();
            return Ok(path);
        }

        // Visit each neighbor of the current node
        for (neighbor, cost) in graph.get(&current).unwrap() {
            let new_distance = distances.get(current).unwrap() + cost;
            // Compare the new distance with the previous ones
            // keep the least distance
            if new_distance < *distances.get(neighbor).unwrap() {
                origin.insert(neighbor, current);
                distances.insert(neighbor, new_distance);
                open_list.insert((new_distance, neighbor));
            }
        }
    }

    Err("No path found")
}

fn ask_node(graph: &Graph, msg: &str) -> String {
    println!("{}", msg);
    let mut node: String = String::new();
    stdin().read_line(&mut node).unwrap();
    node = node.trim().to_string();
    if graph.contains_key(node.as_str()) {
        return node;
    }
    println!("Invalid node");
    ask_node(graph, msg)
}

fn show_nodes(graph: &Graph) {
    println!("Nodes:");
    for key in graph.keys() {
        print!("{} ", key);
    }
    println!();
}

fn main() {
//     A
//    / \
//   B - C
//  / \ / \
//  |  D   F
//  \  /
//   E
// Define a graph as an adjacency list
    let graph: Graph = BTreeMap::from(
        [
            ("A", BTreeMap::from([
                ("B", 3),
                ("C", 5),
            ])),
            ("B", BTreeMap::from([
                ("A", 3),
                ("C", 2),
                ("D", 6),
                ("E", 4),
            ])),
            ("C", BTreeMap::from([
                ("B", 2),
                ("A", 5),
                ("D", 1),
                ("F", 8),
            ])),
            ("D", BTreeMap::from([
                ("B", 6),
                ("C", 1),
                ("E", 4),
            ])),
            ("E", BTreeMap::from([
                ("B", 4),
                ("D", 4),
            ])),
            ("F", BTreeMap::from([
                ("C", 8),
            ])),
        ]
    );
    println!("Dijakstra's Algorithm");
    println!("Graph: {:#?}", graph);
    // Ask user for start and goal node
    show_nodes(&graph);
    let start = ask_node(&graph, "Input START node:");
    println!();
    show_nodes(&graph);
    let goal = ask_node(&graph, "Input GOAL node:");
    if let Ok(path ) = dijakstra(graph, &start, &goal) {
        let output = path.join(" -> ");
        println!("Least Cost Path: \n{}",output );
    } else {
        println!("Unecpected error");
    }

    // prompt whether to repeat
    println!("continue? (y/n)");
    let mut repeat: String = String::new();
    stdin().read_line(&mut repeat).unwrap();
    repeat = repeat.trim().to_string();
    if repeat == "y" {
        main();
    }
}
