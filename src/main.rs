use std::env;

fn help() {
    println!();
}

fn build_dag(packages: Vec<String>) -> Result<(), String> {}

fn print_dag() {
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

    print_dag();
}
