use petgraph::Graph;

use std::env;

fn help() {
    println!("usage:
        emerge <string>");
}

fn build_dag(_packages: Vec<String>) -> Result<Graph::<&'static str, &'static str>, String> {
    let deps = Graph::<&str, &str>::new();

    Ok(deps)
}

fn print_dag(dag: Graph::<&'static str, &'static str>) {
    let order = petgraph::algo::toposort(&dag, None).unwrap();
    println!("{:?}", order);
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
