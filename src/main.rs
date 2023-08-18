use flag_algebra::flags::DirectedGraph;
use flag_algebra::*;
use operator::Basis;
use sdp::Problem;

pub fn main() {
    init_default_log();
    let basis: Basis<DirectedGraph> = Basis::new(5);

    let arc: QFlag<f64, DirectedGraph> = flag(&DirectedGraph::new(2, [(0, 1)]));
    let asy: QFlag<f64, DirectedGraph> = flag(&DirectedGraph::new(2, [(0, 1), (1, 0)]));

    // forbidding is induced, not subgraph, so you can't just count the number of arc's
    let edge_density = arc + asy * 2.0;

    // a list of forbidden graphs. forbidding is induced, not subgraph,
    // so we just write in every supergraph of D (incl. D itself)
    let forbidden: Vec<QFlag<f64, DirectedGraph>> = vec![
        flag(&DirectedGraph::new(
            5,
            [
                (0, 1),
                (1, 2),
                (0, 2),
                (0, 3),
                (3, 0),
                (0, 4),
                (4, 0),
                (1, 3),
                (3, 1),
                (1, 4),
                (4, 1),
                (2, 3),
                (3, 2),
                (2, 4),
                (4, 2),
                (3, 4),
                (4, 3),
            ],
        )),
        flag(&DirectedGraph::new(
            5,
            [
                (0, 1),
                (1, 2),
                (0, 2),
                (1, 0),
                (0, 3),
                (3, 0),
                (0, 4),
                (4, 0),
                (1, 3),
                (3, 1),
                (1, 4),
                (4, 1),
                (2, 3),
                (3, 2),
                (2, 4),
                (4, 2),
                (3, 4),
                (4, 3),
            ],
        )),
        flag(&DirectedGraph::new(
            5,
            [
                (0, 1),
                (1, 2),
                (0, 2),
                (2, 0),
                (0, 3),
                (3, 0),
                (0, 4),
                (4, 0),
                (1, 3),
                (3, 1),
                (1, 4),
                (4, 1),
                (2, 3),
                (3, 2),
                (2, 4),
                (4, 2),
                (3, 4),
                (4, 3),
            ],
        )),
        flag(&DirectedGraph::new(
            5,
            [
                (0, 1),
                (1, 2),
                (0, 2),
                (2, 1),
                (0, 3),
                (3, 0),
                (0, 4),
                (4, 0),
                (1, 3),
                (3, 1),
                (1, 4),
                (4, 1),
                (2, 3),
                (3, 2),
                (2, 4),
                (4, 2),
                (3, 4),
                (4, 3),
            ],
        )),
        flag(&DirectedGraph::new(
            5,
            [
                (0, 1),
                (1, 2),
                (2, 1),
                (0, 2),
                (2, 0),
                (0, 3),
                (3, 0),
                (0, 4),
                (4, 0),
                (1, 3),
                (3, 1),
                (1, 4),
                (4, 1),
                (2, 3),
                (3, 2),
                (2, 4),
                (4, 2),
                (3, 4),
                (4, 3),
            ],
        )),
        flag(&DirectedGraph::new(
            5,
            [
                (0, 1),
                (1, 0),
                (1, 2),
                (2, 1),
                (0, 2),
                (2, 0),
                (0, 3),
                (3, 0),
                (0, 4),
                (4, 0),
                (1, 3),
                (3, 1),
                (1, 4),
                (4, 1),
                (2, 3),
                (3, 2),
                (2, 4),
                (4, 2),
                (3, 4),
                (4, 3),
            ],
        )),
    ];

    let mut ineq_list = vec![total_sum_is_one(basis), flags_are_nonnegative(basis)];

    for f in &forbidden {
        ineq_list.push(f.at_most(0.0).multiply_and_unlabel(basis));
    }

    let pb: Problem<f64, DirectedGraph> = Problem::<f64, _> {
        // Constraints
        ineqs: ineq_list,
        // Use all relevant Cauchy-Schwarz inequalities.
        cs: basis.all_cs(),

        obj: -edge_density.expand(basis).no_scale(),
    };

    // Write the corresponding SDP program in "turan.sdpa".
    // This program can then be solved by CSDP.
    pb.write_sdpa("main").unwrap();
}
