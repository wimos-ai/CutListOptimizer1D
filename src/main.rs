mod command_line_parse;
mod json_utils;

use gumdrop::Options;
use json_utils::Problem;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn main() {
    let opts = command_line_parse::ProblemArgs::parse_args_default_or_exit();

    let problems = Problem::from_json_files(
        &opts.problem_files,
        opts.cost_num_decimals,
        opts.length_num_decimals,
    );

    let cut_width =
        Some((opts.cut_width * (usize::pow(10, opts.cost_num_decimals) as f64)) as usize);

    if opts.log_options {
        print!("Options:\n\tproblem_files: [");

        for idx in 0..opts.problem_files.len() {
            let file = &opts.problem_files[idx];
            print!("{:#?}", file);
            if idx != opts.problem_files.len() - 1 {
                print!(", ");
            }
        }
        print!("]\n\t");

        println!(
            "--cost-num-decimals: {}\n\t--length-num-decimals: {}\n\t--cut-width: {}",
            opts.cost_num_decimals, opts.length_num_decimals, opts.cut_width
        );
    }

    let seed = opts.seed.unwrap_or(rand::random());

    for problem in problems {
        let seeds = seed..seed + opts.search_count;
        let sol = seeds
            .into_par_iter()
            .map(|s| problem.solve(Some(s), cut_width).unwrap())
            .min_by(|a, b| {
                let rv = a.get_cumulative_cost().cmp(&b.get_cumulative_cost());
                if rv.is_eq() {
                    problem
                        .get_cut_list_length(a)
                        .cmp(&problem.get_cut_list_length(b))
                } else {
                    rv
                }
            })
            .unwrap();

        problem.pretty_print_result(&sol, opts.cost_num_decimals, opts.length_num_decimals);
    }
}
