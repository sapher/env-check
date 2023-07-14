use clap::Parser;
use std::env;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[clap(long, value_delimiter = ',')]
    envs: Vec<String>,

    #[clap(long, num_args = 1..)]
    env: Vec<String>,
}

fn main() {
    let args = Args::parse();

    // Combine the two vectors into one
    let envs: Vec<String> = args.envs.into_iter().chain(args.env).collect();

    // Check if each environment variable stored in the vector is set, and exit with an error code only if one of them is not set
    let all_set = envs.iter().all(|env_var| env::var(env_var.clone()).is_ok());
    for env_var in envs {
        if env::var(&env_var).is_ok() {
            println!("{} is set", env_var);
        } else {
            println!("{} is not set", env_var);
        }
    }

    if !all_set {
        std::process::exit(1);
    }
}