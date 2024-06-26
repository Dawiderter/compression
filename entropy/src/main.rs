use clap::Parser;
use entropy::{count, conditional_count, conditional_entropy, entropy};
use std::{
    fs::File,
    io::{stdin, BufRead, BufReader, Read},
    path::PathBuf,
};

#[derive(Debug, Clone, Parser)]
struct Args {
    pathfile: Option<PathBuf>,
    #[arg(long, default_value_t = false)]
    count_table: bool,
    #[arg(long, default_value_t = false)]
    cond_count_table: bool,
}

fn main() {
    let args = Args::parse();

    let mut reader: Box<dyn BufRead> = args
        .pathfile
        .map(|path| {
            Box::new(BufReader::new(
                File::open(path).expect("While opening file"),
            )) as Box<dyn BufRead>
        })
        .unwrap_or_else(|| Box::new(stdin().lock()));

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).unwrap();

    let count_table = count(&buf);
    println!("entropy: {}", entropy(&count_table));

    let cond_count_table = conditional_count(&buf);
    println!("conditional entropy: {}", conditional_entropy(&cond_count_table));

    if args.count_table {
        println!("{}", &count_table);
    }

    if args.cond_count_table {
        println!("{}", &cond_count_table);
    }
}
