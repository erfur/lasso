use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// pid of the target process
    #[arg(short, long)]
    pid: u16,

    /// path of the library to inject
    #[arg(short, long)]
    lib_path: String,
}

fn main() {
    let args = Args::parse();

    linjector_rs::inject_code_to_pid(args.pid as i32, args.lib_path);
}