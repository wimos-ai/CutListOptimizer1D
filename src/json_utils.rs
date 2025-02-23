use cut_optimizer_1d::*;
use std::{collections::HashMap, fs::File, io::Read, path::PathBuf, usize};

#[derive(Debug, Clone)]
pub struct Problem {
    tag: String,
    source_names: HashMap<StockPiece, String>,
    stock_pieces: Vec<StockPiece>,
    cuts: Vec<CutPiece>,
}

#[derive(PartialEq, Eq)]
struct Source_s {
    tag: String,
    name: String,
    cost: usize,
    length: usize,
    quantity: Option<usize>,
}

struct Cut_s {
    tag: String,
    length: usize,
    quantity: usize,
}

impl Into<StockPiece> for &Source_s {
    fn into(self) -> StockPiece {
        StockPiece {
            quantity: self.quantity,
            length: self.length,
            price: self.cost,
        }
    }
}

impl Into<CutPiece> for &Cut_s {
    fn into(self) -> CutPiece {
        CutPiece {
            external_id: None,
            quantity: self.quantity,
            length: self.length,
        }
    }
}

fn i_pow(mut base: usize, exp: usize) -> usize {
    let mut rv = base;
    for _ in 1..exp {
        rv *= base;
    }
    return rv;
}

fn parse_json_file(
    file: &PathBuf,
    num_cost_decimals: usize,
    num_length_decimals: usize,
) -> (Vec<Source_s>, Vec<Cut_s>) {
    if !file.is_file() {
        println!("Path: {:?} is not a file!", &file);
        return (Vec::new(), Vec::new());
    }

    let f = File::open(file.as_path());
    if f.is_err() {
        println!("File: {:?} could not be opened!", &file);
        return (Vec::new(), Vec::new());
    }

    let mut file_ = f.unwrap();

    let mut contents = String::new();

    if file_.read_to_string(&mut contents).is_err() {
        println!("File: {:?} could not be read!", &file);
        return (Vec::new(), Vec::new());
    }

    let json_data_r = json::parse(&contents);
    if json_data_r.is_err() {
        println!("File: {:?} could not parsed as json!", &file);
        return (Vec::new(), Vec::new());
    }

    let json_data = json_data_r.unwrap();

    let mut sources_vec = Vec::new();
    let mut cuts_vec = Vec::new();
    let sources = &json_data["sources"];
    for source in sources.members() {
        let tag = &source["tag"].as_str();
        let name = &source["name"].as_str();
        let cost = &source["cost"].as_f64();
        let length = &source["length"].as_f64();
        let quantity = &source["quantity"].as_usize();

        if tag.is_none() || name.is_none() || cost.is_none() || length.is_none() {
            println!("Json string: {} could not be parsed into a source", source);
            continue;
        }

        sources_vec.push(Source_s {
            tag: tag.unwrap().to_string(),
            name: name.unwrap().to_string(),
            cost: (cost.unwrap() * i_pow(10, num_cost_decimals) as f64) as usize,
            length: (length.unwrap() * i_pow(10, num_length_decimals) as f64) as usize,
            quantity: *quantity,
        });
    }

    let cuts = &json_data["cuts"];
    for cut in cuts.members() {
        let tag = &cut["tag"].as_str();
        let length = &cut["length"].as_f64();
        let quantity = &cut["quantity"].as_usize();

        if tag.is_none() || length.is_none() || quantity.is_none() {
            println!("Json string: {} could not be parsed into a cut", cut);
            continue;
        }
        cuts_vec.push(Cut_s {
            tag: tag.unwrap().to_string(),
            length: (length.unwrap() * i_pow(10, num_length_decimals) as f64) as usize,
            quantity: quantity.unwrap(),
        });
    }

    (sources_vec, cuts_vec)
}

impl Problem {
    fn new(tag: &String) -> Self {
        Problem {
            tag: tag.clone(),
            source_names: HashMap::new(),
            stock_pieces: Vec::new(),
            cuts: Vec::new(),
        }
    }

    fn add_cut(&mut self, piece: &Cut_s) {
        if self.tag != piece.tag {
            return;
        }
        match self.cuts.iter_mut().find(|s| s.length == piece.length) {
            None => {
                self.cuts.push(piece.into());
            }
            Some(value) => {
                value.quantity += piece.quantity;
            }
        }
    }

    fn add_source(&mut self, source: &Source_s) {
        if self.tag != source.tag {
            return;
        }

        match self
            .stock_pieces
            .iter_mut()
            .find(|s| s.length == source.length && s.quantity == source.quantity)
        {
            None => {
                let s: StockPiece = source.into();
                self.stock_pieces.push(s);
                self.source_names.insert(s, source.name.clone());
            }

            Some(value) => {
                *value = source.into();
                *self.source_names.get_mut(*&value).unwrap() = source.name.clone();
            }
        }
    }

    fn is_valid_problem(&self) -> bool {
        return self.cuts.len() > 0 && self.stock_pieces.len() > 0;
    }

    pub fn from_json_files(
        files: &Vec<PathBuf>,
        num_cost_decimals: usize,
        num_length_decimals: usize,
    ) -> Vec<Problem> {
        let mut tag_to_problem: HashMap<String, Problem> = HashMap::new();

        for file in files {
            let (sources, cuts) = parse_json_file(&file, num_cost_decimals, num_length_decimals);

            for source in sources {
                if !tag_to_problem.contains_key(&source.tag) {
                    tag_to_problem.insert(source.tag.clone(), Problem::new(&source.tag));
                }

                tag_to_problem
                    .get_mut(&source.tag)
                    .unwrap()
                    .add_source(&source);
            }

            for cut in cuts {
                if !tag_to_problem.contains_key(&cut.tag) {
                    tag_to_problem.insert(cut.tag.clone(), Problem::new(&cut.tag));
                }

                tag_to_problem.get_mut(&cut.tag).unwrap().add_cut(&cut);
            }
        }

        tag_to_problem
            .values()
            .filter(|s| s.is_valid_problem())
            .cloned()
            .collect()
    }
}
