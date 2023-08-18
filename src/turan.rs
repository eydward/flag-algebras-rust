// forbid the graph 0--1--2
use flag_algebra::flags::Graph;
use flag_algebra::*;
use operator::Basis;
use sdp::Problem;

pub fn main() {
    init_default_log();
    let basis: Basis<Graph> = Basis::new(3);

    let edge: QFlag<f64, Graph> = flag(&Graph::new(2, &[(0, 1)]));

    // you can only forbid induced graphs, so add all supergraphs to the list I guess
    let forbidden: Vec<QFlag<f64, Graph>> = vec![
        flag(&Graph::new(3, &[(0, 1), (1, 2), (0, 2)])),
        flag(&Graph::new(3, &[(0, 1), (1, 2)])),
    ];

    let mut ineq_list = vec![total_sum_is_one(basis), flags_are_nonnegative(basis)];

    for f in &forbidden {
        ineq_list.push(f.at_most(0.0).multiply_and_unlabel(basis));
    }

    let pb: Problem<f64, Graph> = Problem::<f64, _> {
        // Constraints
        ineqs: ineq_list,
        // Use all relevant Cauchy-Schwarz inequalities.
        cs: basis.all_cs(),

        obj: -edge.expand(basis).no_scale(),
    };

    // Write the correspondind SDP program in "turan.sdpa".
    // This program can then be solved by CSDP.
    pb.write_sdpa("main").unwrap();
}
