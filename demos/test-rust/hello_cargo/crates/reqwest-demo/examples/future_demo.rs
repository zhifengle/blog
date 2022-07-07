use futures::executor::block_on;
use std::future::Future;

// ref: chap38

#[tokio::main]
async fn main() {
    let first_name = "Alan";
    let last_name = "Yang";
    say_hello1(&first_name).await;
    say_hello2(&last_name).await;

    // 使用的 executor
    block_on(say_hello1(&first_name));
    block_on(say_hello2(&last_name));
}

async fn say_hello1(name: &str) -> usize {
    println!("Hello {}", name);
    11
}

fn say_hello2<'a>(name: &'a str) -> impl Future<Output = usize> + 'a {
    async move {
        println!("Hello {}", name);
        22
    }
}
