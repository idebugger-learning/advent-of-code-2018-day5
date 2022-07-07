fn main() {
    let example_polymer = include_str!("./data/example.txt").to_string();
    println!(
        "Example reaction result length: {}",
        simulate_reaction(&example_polymer).len()
    );

    println!(
        "Example shortest reaction length: {}",
        find_problem_unit(&example_polymer)
    );

    let input_polymer = include_str!("./data/input.txt").to_string();
    println!(
        "Input reaction result length: {}",
        simulate_reaction(&input_polymer).len()
    );

    println!(
        "Example shortest reaction length: {}",
        find_problem_unit(&input_polymer)
    );
}

fn find_problem_unit(polymer: &String) -> usize {
    let mut shortest_polymer_len = usize::MAX;
    for chr in 'a'..'z' {
        let reacted_length = simulate_reaction(
            &polymer
                .replace(&chr.to_lowercase().to_string(), "")
                .replace(&chr.to_uppercase().to_string(), ""),
        )
        .len();
        if reacted_length < shortest_polymer_len {
            shortest_polymer_len = reacted_length;
        }
    }
    shortest_polymer_len
}

fn simulate_reaction(polymer: &String) -> String {
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
