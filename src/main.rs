fn main() {
    let example_polymer = include_str!("./data/example.txt").to_string();
    println!(
        "Example reaction result length: {}",
        simulate_reaction(example_polymer).len()
    );

    let input_polymer = include_str!("./data/input.txt").to_string();
    println!(
        "Input reaction result length: {}",
        simulate_reaction(input_polymer).len()
    );
}

fn simulate_reaction(polymer: String) -> String {
    let mut stack: Vec<char> = vec![];
    for chr in polymer.chars() {
        if let Some(stack_top) = stack.last() {
            if stack_top.to_lowercase().to_string() == chr.to_lowercase().to_string()
                && stack_top != &chr
            {
                stack.pop();
            } else {
                stack.push(chr);
            }
        } else {
            stack.push(chr);
        }
    }
    stack.into_iter().collect::<String>()
}
