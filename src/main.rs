use flag_algebra::flags::DirectedGraph;
use flag_algebra::*;
use operator::Basis;
use sdp::Problem;
use std::io;

mod parse;

pub fn main() {
    init_default_log();


    let mut n : String = Default::default();
    io::stdin().read_line(&mut n).expect("Error reading input");
    let n = n.trim().parse().expect("Error parsing number");

    let basis: Basis<DirectedGraph> = Basis::new(n);

    let arc: QFlag<f64, DirectedGraph> = flag(&DirectedGraph::new(2, [(0, 1)]));
    let asy: QFlag<f64, DirectedGraph> = flag(&DirectedGraph::new(2, [(0, 1), (1, 0)]));

    // forbidding is induced, not subgraph, so you can't just count the number of arc's
    let edge_density = arc + asy * 2.0;

    // list of forbidden (induced!!) subgraphs, read from file
    let forbidden = parse::read_graph_list("forbidden_digraphs");

    let mut ineq_list = vec![total_sum_is_one(basis), flags_are_nonnegative(basis)];

    for f in &forbidden {
        ineq_list.push(f.at_most(0.0).multiply_and_unlabel(basis));
    }

    let pb: Problem<f64, DirectedGraph> = Problem::<f64, _> {
        // Constraints
        ineqs: ineq_list,
        // Use all relevant Cauchy-Schwarz inequalities.
        cs: basis.all_cs(),

        // no_scale prevents the output from returning some arbitrary constant times the real answer
        obj: -edge_density.expand(basis).no_scale(),
    };

    // Write SDPA file, can be solved by CSDP.
    pb.write_sdpa("turan_output").unwrap();
}
