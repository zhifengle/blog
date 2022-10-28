use std::iter::successors;

// https://www.reddit.com/r/rust/comments/beerg3/im_excited_about_stditerfrom_fn/

fn fib_seq() -> impl Iterator<Item = u128> {
    successors(
        Some((0, 1)), //
        |&(a, b)| Some((b, a + b)),
    )
    .map(|(_, b)| b)
}

#[test]
fn t_fib() {
    println!("{:?}",  fib_seq().take(100).collect::<Vec<u128>>());
}
