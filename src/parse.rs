use flag_algebra::flags::DirectedGraph;
use flag_algebra::*;
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    return result;
}

pub fn digraphify(digraph: &str) -> DirectedGraph {
    let parts = digraph.split(" : ").collect::<Vec<&str>>();
    assert_eq!(parts.len(), 2); // n : [arcs]
    let n = parts[0].parse::<usize>().unwrap();

    let edges = parts[1].split(";").collect::<Vec<&str>>();
    let mut arcs: Vec<(usize, usize)> = Vec::new();
    for edge in edges {
        let mut trimmed = edge.trim().to_string();
        assert_eq!(trimmed.remove(0), '(');
        assert_eq!(trimmed.remove(trimmed.len() - 1), ')');
        let vertices = trimmed.split(",").collect::<Vec<&str>>();
        assert_eq!(vertices.len(), 2);
        let head = vertices[0].parse::<usize>().unwrap();
        let tail = vertices[1].parse::<usize>().unwrap();
        arcs.push((head, tail));
    }

    println!("{} vertices \t {} edges", n, arcs.len());

    return DirectedGraph::new(n, arcs);
}

pub fn read_graph_list(filename: &str) -> Vec<QFlag<f64, DirectedGraph>> {
    let mut result: Vec<QFlag<f64, DirectedGraph>> = Vec::new();
    let x: Vec<String> = read_lines(filename);
    for i in 0..x.len() {
        // println!("{} : {}", i, x[i]);
        result.push(flag(&digraphify(&x[i])));
    }
    return result;
}
