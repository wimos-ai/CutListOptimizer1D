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
    pub cost_num_decimals: u32,

    #[options(
        help = "controls how many floating point decimals to use in length calculations",
        default = "2"
    )]
    pub length_num_decimals: u32,

    #[options(help = "sets the seed of the internal solver")]
    pub seed: Option<u64>,

    #[options(help = "sets the blade with when making cuts. Precision is affected by length_num_decimals", default="0")]
    pub cut_width : f64,

    #[options(help = "toggles on logging of command line options", default="true")]
    pub log_options : bool,

    #[options(help = "configures the size of the search space. Higher number is more compute and higher chance of finding an optimal solution", default="100")]
    pub search_count : u64
}
