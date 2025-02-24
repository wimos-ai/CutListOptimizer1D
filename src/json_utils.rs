use cut_optimizer_1d::*;
use std::{collections::HashMap, fs::File, io::Read, path::PathBuf, usize};

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
struct SourceNameKey {
    pub length: usize,
    pub price: usize,
}

#[derive(Debug, Clone)]
pub struct Problem {
    tag: String,
    source_names: HashMap<SourceNameKey, String>,
    stock_pieces: Vec<StockPiece>,
    cuts: Vec<CutPiece>,
}

#[derive(PartialEq, Eq)]
struct SourceS {
    tag: String,
    name: String,
    cost: usize,
    length: usize,
    quantity: Option<usize>,
}

struct CutS {
    tag: String,
    length: usize,
    quantity: usize,
}

struct Purchase<'a> {
    name: &'a str,
    quantity: usize,
}

struct CutOrder<'a> {
    quantity: usize,
    name: &'a str,
    cuts: Vec<usize>,
}

impl Into<StockPiece> for &SourceS {
    fn into(self) -> StockPiece {
        StockPiece {
            quantity: self.quantity,
            length: self.length,
            price: self.cost,
        }
    }
}

impl Into<CutPiece> for &CutS {
    fn into(self) -> CutPiece {
        CutPiece {
            external_id: None,
            quantity: self.quantity,
            length: self.length,
        }
    }
}

fn parse_json_file(
    file: &PathBuf,
    num_cost_decimals: u32,
    num_length_decimals: u32,
) -> (Vec<SourceS>, Vec<CutS>) {
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

        sources_vec.push(SourceS {
            tag: tag.unwrap().to_string(),
            name: name.unwrap().to_string(),
            cost: (cost.unwrap() * usize::pow(10, num_cost_decimals) as f64) as usize,
            length: (length.unwrap() * usize::pow(10, num_length_decimals) as f64) as usize,
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
        cuts_vec.push(CutS {
            tag: tag.unwrap().to_string(),
            length: (length.unwrap() * usize::pow(10, num_length_decimals) as f64) as usize,
            quantity: quantity.unwrap(),
        });
    }

    (sources_vec, cuts_vec)
}

impl Problem {
    pub fn solve(
        &self,
        seed: Option<u64>,
        cut_width: Option<usize>,
    ) -> Result<cut_optimizer_1d::Solution, cut_optimizer_1d::Error> {
        let mut optimizer = Optimizer::new();

        optimizer.add_cut_pieces(self.cuts.clone());
        optimizer.add_stock_pieces(self.stock_pieces.clone());

        if seed.is_some() {
            optimizer.set_random_seed(seed.unwrap());
        }

        if cut_width.is_some() {
            optimizer.set_cut_width(cut_width.unwrap());
        }

        return optimizer.optimize(|_| {});
    }

    fn new(tag: &String) -> Self {
        Problem {
            tag: tag.clone(),
            source_names: HashMap::new(),
            stock_pieces: Vec::new(),
            cuts: Vec::new(),
        }
    }

    fn add_cut(&mut self, piece: &CutS) {
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

    fn add_source(&mut self, source: &SourceS) {
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
                self.source_names.insert(
                    SourceNameKey {
                        length: source.length,
                        price: source.cost,
                    },
                    source.name.clone(),
                );
            }

            Some(value) => {
                *value = source.into();
                *self
                    .source_names
                    .get_mut(&SourceNameKey {
                        length: value.length,
                        price: value.price,
                    })
                    .unwrap() = source.name.clone();
            }
        }
    }

    fn is_valid_problem(&self) -> bool {
        return self.cuts.len() > 0 && self.stock_pieces.len() > 0;
    }

    fn get_purchase_order(&self, res: &Solution) -> Vec<Purchase> {
        let mut map: HashMap<&str, usize> = HashMap::new();

        for stock_piece in &res.stock_pieces {
            let tag = self
                .source_names
                .get(&SourceNameKey {
                    length: stock_piece.length,
                    price: stock_piece.price,
                })
                .unwrap()
                .as_str();

            match map.get_mut(tag) {
                None => {
                    map.insert(tag, 1);
                }
                Some(s) => {
                    *s += 1;
                }
            }
        }

        let mut purchases: Vec<Purchase> = map
            .into_iter()
            .map(|(tag, quantity)| Purchase { name: tag, quantity })
            .collect();

        purchases.sort_by(|a, b| a.name.cmp(b.name));
        purchases
    }

    fn get_cut_list(&self, res: &Solution) -> Vec<CutOrder> {
        let mut map: HashMap<(&str, Vec<usize>), usize> = HashMap::new();

        for piece in &res.stock_pieces {
            let name: &str = self
                .source_names
                .get(&SourceNameKey {
                    length: piece.length,
                    price: piece.price,
                })
                .unwrap();

            let mut vec: Vec<usize> = Vec::new();
            for cut in &piece.cut_pieces {
                let len: usize = cut.end - cut.start;
                vec.push(len);
            }
            vec.sort();

            *map.entry((name, vec)).or_insert(0) += 1;
        }

        let mut cuts: Vec<CutOrder> = map
            .into_iter()
            .map(|(tup, quantity)| CutOrder {
                quantity: quantity,
                name: tup.0,
                cuts: tup.1,
            })
            .collect();

        cuts.sort_by(|a, b| a.name.cmp(b.name));

        cuts
    }

    /*
    Something like this:
    For: 2x4, Cost: 21, Effiency: 98.23%
        Purchase List (name, quantity):
            menards 2x4x1, 1
            menards 2x4x2, 2
            menards 2x4x3, 2
        Cut List ((cut quantity) name -> [cutLength1, cutLength2, ...]):
            (1x) name -> [1]
            (2x) name -> [2]
            (2x) name -> [3]
    */
    pub fn pretty_print_result(
        &self,
        res: &Solution,
        num_price_decimals: u32,
        num_length_decimals: u32,
    ) {
        let solution_cost = res.get_cumulative_cost();
        let solution_cost_upper = solution_cost / usize::pow(10, num_price_decimals);
        let solution_cost_lower = solution_cost % usize::pow(10, num_price_decimals);
        println!(
            "For: {}, Cost: {}.{}, Effiency: {:.2}%",
            self.tag,
            solution_cost_upper,
            solution_cost_lower,
            res.fitness * 100.0
        );

        println!("\tPurchase List (name, quantity):");
        let purchase_order = self.get_purchase_order(res);
        for purchase in purchase_order {
            println!("\t\t{}, {}", purchase.name, purchase.quantity);
        }

        println!("\tCut List ((cut quantity) name -> [cutLength1, cutLength2, ...]):");
        let cuts = self.get_cut_list(res);
        for cut in cuts {
            print!("\t\t({}) {} -> [", cut.quantity, cut.name);
            for idx in 0..cut.cuts.len() {
                let length = cut.cuts[idx];

                let length_upper = length / usize::pow(10, num_length_decimals);
                let length_lower = length % usize::pow(10, num_length_decimals);
                print!("{}.{}", length_upper, length_lower);
                if idx !=  cut.cuts.len() -1{
                    print!(", ");
                }
            }
            print!("]\n");
        }

        // panic!("NOT IMPLEMENTED");
    }

    pub fn from_json_files(
        files: &Vec<PathBuf>,
        num_cost_decimals: u32,
        num_length_decimals: u32,
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
