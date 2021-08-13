fn iter_option() {
    // some_vec.extend(some_opt);
    let turing = Some("Turing");

    let mut logicians = vec!["Curry", "Kleene", "Markov"];

    logicians.extend(turing);

    // equivalent to
    if let Some(turing_inner) = turing {
        logicians.push(turing_inner);
    }

    // 使用 chain 来添加到末尾
    for logician in logicians.iter().chain(turing.iter()) {
        println!("{} is a logician", logician);
    }
}
