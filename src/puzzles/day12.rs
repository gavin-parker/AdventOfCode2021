use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;
use std::collections::HashSet;
use lazy_static::lazy_static;

#[derive(Clone)]
struct Cave {
    visits: usize,
    neighbours: HashSet<String>
}

type Graph = HashMap<String, Cave>;

fn read_file(file: &File) -> Graph {
    let edges = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            let mut nodes = l.split("-");
            let a = nodes.next().unwrap();
            let b = nodes.next().unwrap();
            (a.to_string(), b.to_string())
        });
    let mut graph = Graph::new();
    for (a, b) in edges {
        let cave_a = graph.entry(a.clone()).or_insert(Cave{visits: 0, neighbours: HashSet::new()});
        cave_a.neighbours.insert(b.clone());
        let cave_b = graph.entry(b.clone()).or_insert(Cave{visits: 0, neighbours: HashSet::new()});
        cave_b.neighbours.insert(a.clone());
    }
    graph
}

fn find_paths(mut graph: Graph, node: &String, mut visited_small: bool, visitable: fn(&Graph, &String, bool) -> bool) -> Option<usize> {
    {
        let mut cave = graph.get_mut(node).unwrap();
        cave.visits += 1;
    }
    // if this cave is small and visited twice
    if is_small(&node) && graph.get(node).unwrap().visits > 1 && node != &"start".to_string() && node != &"end".to_string(){
        visited_small = true;
    }
    let mut paths = 0;
    for n in &graph.get(node).unwrap().neighbours {
        if n == "end"{
            paths += 1;
            continue;
        }
        if !visitable(&graph, n, visited_small) { continue; }
        let new_graph = graph.clone();
        match find_paths(new_graph, n, visited_small, visitable) {
            Some(p) => paths += p,
            None => {}
        }
    }
    Some(paths)
}

fn is_small(node: &String) -> bool {
    (node == &node.to_lowercase())
}

fn can_visit_p1(graph: &Graph, node: &String, visited_small: bool) -> bool {
    let cave = graph.get(node).unwrap();
    ! (cave.visits > 0 && (node == &node.to_lowercase()))
}

fn can_visit_p2(graph: &Graph, node: &String, visited_small: bool) -> bool {
    let cave = graph.get(node).unwrap();
    if (node == "end" || node == "start") && cave.visits > 0 { return false;}
    if is_small(node) && visited_small && cave.visits > 0 { return false;}
    true
}

// og: 18861ms
// no copy on end: 16393ms
// No string copy: 16073ms
// No copy for unvisitable: 4661ms
// Release build: 858ms

pub fn run(part: usize) -> io::Result<()> {
    let subsystem = File::open("test_data/day12.txt")?;
    let subsystem = read_file(&subsystem);
    match part {
        1 => println!("paths: {}", find_paths(subsystem, &"start".to_string(), false, can_visit_p1).unwrap()),
        2 => println!("paths: {}", find_paths(subsystem, &"start".to_string(), false, can_visit_p2).unwrap()),
        _ => panic!("")
    }
    Ok(())
}