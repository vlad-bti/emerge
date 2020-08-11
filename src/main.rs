use petgraph::Graph;

use std::env;

fn help() {
    println!("usage:
        emerge <string>");
}

fn build_dag(_packages: Vec<String>) -> Result<Box<Graph::<&'static str, &'static str>>, String> {
    let deps = Graph::<&str, &str>::new();

    Ok(Box::new(deps))
}

fn print_dag(_dag: Box<Graph::<&'static str, &'static str>>) {
    println!();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help();
        return;
    }

    let result = build_dag(args[1..].to_vec());
    if let Err(e) = result {
        println!("error: {:?}", e);
        return;
    }

    print_dag(result.unwrap());
}
