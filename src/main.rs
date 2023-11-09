use std::fs::File;
use std::io::{stdin, Write, self, BufReader};
use std::collections::{BTreeMap, BTreeSet};
type Graph<T> = BTreeMap<T, BTreeMap<T, usize>>;
fn dijakstra<'a>(graph: &'a Graph<String>, start: &'a str, goal: &'a str) -> Result<Vec<&'a str>, &'a str> {
    let mut open_list: BTreeSet<(usize, &str)> = BTreeSet::new();
    open_list.insert((0, &start));

    // Assign to every node a tentative distance value of infinity
    let mut distances: BTreeMap<&str, usize> = graph.keys().map(|k|{ (k.as_str(), usize::MAX) }).collect();
    // set it to zero for start node
    distances.insert(&start, 0);

    let mut origin: BTreeMap<&str, &str> = BTreeMap::new();

    while !open_list.is_empty() {
        let (_, current) = open_list.pop_first().unwrap();

        if current == goal  {
            // backtrack to reconstruct the path
            let mut path:Vec<&'a str> = vec![&goal];
            let mut node = goal;
            while let Some(next) = origin.get(&node) {
                node = next;
                path.push(&node);
            }
         
            path.reverse();
            return Ok(path);
        }

        // Visit each neighbor of the current node
        for (neighbor, cost) in graph.get(current).unwrap() {
            let new_distance = distances.get(current).unwrap() + cost;
            // Compare the new distance with the cost in distances
            // keep the least distance
            if new_distance < *distances.get(neighbor.as_str()).unwrap() {
                origin.insert(neighbor, current);
                distances.insert(neighbor, new_distance);
                open_list.insert((new_distance, neighbor));
            }
        }
    }

    Err("No path found")
}

// Helper function to get user input
fn input() -> String {
    let mut mode: String = String::new();
    stdin().read_line(&mut mode).unwrap();
    mode.trim().to_string()
}

fn ask_node(graph: &Graph<String>, msg: &str) -> String {
    println!("{}", msg);
    let node = input();
    if graph.contains_key(node.as_str()) {
        return node;
    }
    println!("Invalid node");
    ask_node(graph, msg)
}

fn show_nodes(graph: &Graph<String>) {
    println!("Nodes:");
    for key in graph.keys() {
        print!("{} ", key);
    }
    println!();
}

fn export_json(graph: &Graph<String>, filepath: &str) {
    let json = serde_json::to_string_pretty(graph).unwrap();
    let mut file = File::create(filepath).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    println!("Graph exported to {}", filepath);
}

fn import_json(filepath: &str) -> Graph<String> {
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}

fn exec(graph: &Graph<String>) {
    println!("Dijakstra's Algorithm");
    println!("Graph: {:#?}", graph);
    // Ask user for start and goal node
    show_nodes(&graph);
    let start = ask_node(&graph, "Input START node:");
    println!();
    show_nodes(&graph);
    let goal = ask_node(&graph, "Input GOAL node:");
    if let Ok(path ) = dijakstra(&graph, &start, &goal) {
        let output = path.join(" -> ");
        println!("Least Cost Path: \n{}",output );
    } else {
        println!("Unecpected error");
    }

    // prompt whether to repeat
    println!("continue? (y/n)");
    let repeat = input();
    if repeat == "y" {
        main();
    }
}

fn get_graph_source() -> Graph<String> {
    print!("Choose graph source:\n[0] Use sample graph\n[1] Import json file\n>> ");
    io::stdout().flush().unwrap();
    let mode = input();
    match mode.as_str() {
        "0" => {
            //     A
            //    / \
            //   B - C
            //  / \ / \
            //  |  D   F
            //  \  /
            //   E
            // Define a graph as an adjacency list
            BTreeMap::from(
                [
                    ("A".to_owned(), BTreeMap::from([
                        ("B".to_owned(), 3),
                        ("C".to_owned(), 5),
                    ])),
                    ("B".to_owned(), BTreeMap::from([
                        ("A".to_owned(), 3),
                        ("C".to_owned(), 2),
                        ("D".to_owned(), 6),
                        ("E".to_owned(), 4),
                    ])),
                    ("C".to_owned(), BTreeMap::from([
                        ("B".to_owned(), 2),
                        ("A".to_owned(), 5),
                        ("D".to_owned(), 1),
                        ("F".to_owned(), 8),
                    ])),
                    ("D".to_owned(), BTreeMap::from([
                        ("B".to_owned(), 6),
                        ("C".to_owned(), 1),
                        ("E".to_owned(), 4),
                    ])),
                    ("E".to_owned(), BTreeMap::from([
                        ("B".to_owned(), 4),
                        ("D".to_owned(), 4),
                    ])),
                    ("F".to_owned(), BTreeMap::from([
                        ("C".to_owned(), 8),
                    ])),
                ]
            )
        },
        "1" => {
            println!("Enter filepath:");
            let filepath = input();
            import_json(filepath.as_str())
        },
        _ => {
            println!("Invalid mode");
            get_graph_source()
        }
    }
}
fn main() {
    // prompt whether to import graph, use sample, or input graph
    let graph: Graph<String> = get_graph_source();
    // prompt whether to export graph or show dijakstra
    print!("Choose mode:\n[0] Export graph\n[1] Show dijakstra\n>> ");
    io::stdout().flush().unwrap();
    let mode = input();
    match mode.as_str() {
        "0" => {
            println!("Input filepath:");
            let filepath = input();
            export_json(&graph, filepath.as_str());
        },
        "1" => {
            exec(&graph);
        },
        _ => {
            println!("Invalid mode");
            main();
        }
    };
}
