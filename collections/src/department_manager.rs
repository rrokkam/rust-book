pub fn manage(lines: &[&str]) {
    let mut department_membership: Vec<(&str, &str)> =
        lines.iter().map(|command| parse_command(command)).collect();

    // List all people in a department
    for name in department_membership
        .iter()
        .filter(|(_, department)| *department == "Sales")
        .map(|(name, _)| name)
    {
        println!("{} is in Sales.", *name);
    }

    println!("");

    // List all people alphabetically.
    department_membership.sort();
    for (name, _) in department_membership.iter() {
        println!("{}", name);
    }
}

fn parse_command(command: &str) -> (&str, &str) {
    let tokens: Vec<_> = command.split_whitespace().collect();
    if tokens.len() != 4 || tokens[0] != "Add" || tokens[2] != "to" {
        panic!("Bad command given to department manager");
    }
    (tokens[1], tokens[3])
}
