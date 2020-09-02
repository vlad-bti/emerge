use std::env;

use emerge::depgraph::GraphData;

fn main() {
    // TODO: https://docs.rs/structopt/0.3.17/structopt/
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help();
        return;
    }

    match GraphData::build_dag_from(&args[1..]) {
        Ok(graph_data) => graph_data.print_dag(),
        Err(e) => println!("error: {:?}", e),
    }
}

fn help() {
    println!(
        "usage:
        emerge <string>"
    );
}
