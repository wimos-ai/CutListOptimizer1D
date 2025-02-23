// use super::*;

mod command_line_parse;
mod json_utils;

use std::collections::HashMap;

use cut_optimizer_1d::*;

use gumdrop::Options;
use json_utils::Problem;

fn pretty_print_result(problem: &Problem, res: &Solution) {
    panic!("NOT IMPLEMENTED");
}

fn build_optimizer() -> Optimizer {
    const MENARDS_PIECES_2X4S: [cut_optimizer_1d::StockPiece; 11] = [
        StockPiece {
            length: 36,
            price: 168,
            quantity: None,
        },
        StockPiece {
            length: 48,
            price: 186,
            quantity: None,
        },
        StockPiece {
            length: 72,
            price: 257,
            quantity: None,
        },
        StockPiece {
            length: 84,
            price: 265,
            quantity: None,
        },
        StockPiece {
            length: (8 * 12),
            price: 311,
            quantity: None,
        },
        StockPiece {
            length: (10 * 12),
            price: 411,
            quantity: None,
        },
        StockPiece {
            length: (12 * 12),
            price: 471,
            quantity: None,
        },
        StockPiece {
            length: (12 * 14),
            price: 613,
            quantity: None,
        },
        StockPiece {
            length: (12 * 16),
            price: 749,
            quantity: None,
        },
        StockPiece {
            length: (18 * 12),
            price: 934,
            quantity: None,
        },
        StockPiece {
            length: (20 * 12),
            price: 1112,
            quantity: None,
        },
    ];

    let mut optimizer = Optimizer::new();

    optimizer.add_stock_pieces(MENARDS_PIECES_2X4S);

    let cut_pieces_2x4s = [
        CutPiece {
            quantity: 20,
            external_id: None,
            length: 21,
        },
        CutPiece {
            quantity: 12,
            external_id: None,
            length: 24,
        },
        CutPiece {
            quantity: 6,
            external_id: None,
            length: 42,
        },
        CutPiece {
            quantity: 16,
            external_id: None,
            length: 45,
        },
        CutPiece {
            quantity: 32,
            external_id: None,
            length: 48,
        },
        CutPiece {
            quantity: 4,
            external_id: None,
            length: 58,
        },
        CutPiece {
            quantity: 8,
            external_id: None,
            length: 74,
        },
        CutPiece {
            quantity: 36,
            external_id: None,
            length: 80,
        },
    ];

    optimizer.add_cut_pieces(cut_pieces_2x4s);

    optimizer
}

fn main() {
    let opts = command_line_parse::ProblemArgs::parse_args_default_or_exit();

    let problems = Problem::from_json_files(
        &opts.problem_files,
        opts.cost_num_decimals,
        opts.length_num_decimals,
    );

    for problem in problems {
        println!("Problem: {:#?}", problem);
        let sol = problem.solve(None).unwrap();
        pretty_print_result(&problem, &sol);
    }

    println!("Opts: {:#?}", opts);
}
