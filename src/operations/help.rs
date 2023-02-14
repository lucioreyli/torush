pub fn help() {
    let actions = vec![
        ("add", "ex: add \"fazer deploy\""),
        ("help", "show this again"),
    ];
    for (action, example) in actions {
        println!("{} | {}", action, example);
    }
}
