mod command_line_parse;
mod json_utils;

use gumdrop::Options;
use json_utils::Problem;

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

    for problem in problems {
        let sol = problem.solve(opts.seed, cut_width).unwrap();

        problem.pretty_print_result(&sol, opts.cost_num_decimals, opts.length_num_decimals);
    }
}
