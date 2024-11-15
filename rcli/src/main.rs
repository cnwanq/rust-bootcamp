use clap::Parser;
use rcli::{process_csv, Opts, SubCommand};

// rcli csv -i input.csv -o output.json
fn main() -> anyhow::Result<()> {
    // println!("Hello, world!");
    let opts = Opts::parse();
    println!("{:?}", opts);
    match opts.cmd {
        SubCommand::Csv(opts) => process_csv(&opts.input, &opts.output)?,
    }

    Ok(())
}
