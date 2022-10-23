use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

const COMMAND_USAGE: &str = "Usage: ./help-me-decide <PATH_TO_FILE> [NUM_RESULTS]";
const DEFAULT_NUM_RESULTS: usize = 3;

struct Parameters {
    pub path: String,
    pub num_results: usize,
}

impl Parameters {
    pub fn from_argv(argv: Vec<String>) -> Result<Self, ()> {
        if argv.len() < 2 {
            return Err(());
        }

        Ok(Self {
            path: argv[1].clone(),
            num_results: if let Some(n) = argv.get(2) {
                n.parse().map_err(|_| ())?
            } else {
                DEFAULT_NUM_RESULTS
            },
        })
    }
}

fn main() {
    let argv = env::args().collect::<Vec<_>>();

    let params = if let Ok(p) = Parameters::from_argv(argv) {
        p
    } else {
        eprintln!("{}", COMMAND_USAGE);
        return;
    };

    let mut lines = match read_lines_from_file(&params.path) {
        Ok(lines) => lines,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    fastrand::shuffle(&mut lines);
    let lines = lines;

    print_results(&lines, params.num_results)
}

fn read_lines_from_file(path: &str) -> Result<Vec<String>, String> {
    BufReader::new(if let Ok(file) = File::open(path) {
        file
    } else {
        return Err(format!("Fatal error: Cannot open path: {:?}", path));
    })
    .lines()
    .enumerate()
    .map(|(i, s)| s.map_err(|_| format!("Fatal error: Failed at line {} in file {}", i + 1, path)))
    .collect()
}

fn print_results(lines: &[String], n: usize) {
    if n > lines.len() {
        println!("Data file only has {} items.\nPrinting all...", lines.len());
    }

    lines
        .iter()
        .take(n)
        .enumerate()
        .for_each(|(i, line)| println!("{}", format_line(i, line)));
}

/// `i` is zero-based.
fn format_line(i: usize, line: &String) -> String {
    const MEDALS: &[char] = &['🥇', '🥈', '🥉'];

    if i < MEDALS.len() {
        return format!("{} {}", MEDALS[i], line);
    }

    format!("{}. {}", i + 1, line)
}
