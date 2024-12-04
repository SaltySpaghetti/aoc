#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut left = vec![];
    let mut right = vec![];

    for line in input.lines() {
        let mut items = line.split_whitespace();
        left.push(items.next().unwrap().parse::<usize>().unwrap());
        right.push(items.next().unwrap().parse::<usize>().unwrap());
    }

    let result: usize = left
        .iter()
        .map(|number| number * right.iter().filter(|r| &number == r).count())
        .sum();

    Ok(result.to_string())
}
