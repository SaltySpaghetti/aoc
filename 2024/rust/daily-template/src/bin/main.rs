use {{crate_name}}::part1::process as process_1;
use {{crate_name}}::part2::process as process_2;
use miette::Context;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();

    let input_1 = include_str!("../../input1.txt");
    let result_1 = process_1(input_1).context("process part 1")?;
    
    let input_2 = include_str!("../../input2.txt");
    let result_2 = process_2(input_2).context("process part 2")?;

    println!("Part 1: {result_1}");
    println!("Part 2: {result_2}");
    Ok(())
}