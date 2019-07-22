#![feature(async_await)]

use tokio;

async fn foo() -> u8 { 5 }

fn bar() -> impl std::future::Future<Output = u8> {
    async {
        let x: u8 = foo().await;
        println!("{}", x);
        x + 5
    }
}

#[tokio::main]
async fn main() {
    let x = bar().await;
    println!("{}", x);
}
