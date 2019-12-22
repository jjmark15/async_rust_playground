use futures::executor::block_on;
use futures::join;

use async_rust_playground::async_hello_world;

fn main() {
    block_on(async_stuff());
}

async fn async_stuff() {
    join!(async_hello_world());
}
