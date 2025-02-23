use gumdrop::Options;

use std::path::PathBuf;

#[derive(Debug, Options)]
pub struct ProblemArgs {
    #[options(free, help = "A sequence of json files that define the problem")]
    pub problem_files: Vec<PathBuf>,

    #[options(help = "print help message")]
    pub help: bool,

    #[options(
        help = "controls how many floating point decimals to use in cost calculations",
        default = "2"
    )]
    pub cost_num_decimals: usize,

    #[options(
        help = "controls how many floating point decimals to use in length calculations",
        default = "2"
    )]
    pub length_num_decimals: usize,

    #[options(help = "sets the seed of the internal solver")]
    pub seed: Option<u64>,
}
