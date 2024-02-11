use myip::getmyip::getmyip::{getmyipv4_1, getmyipv4_2};

use futures::executor::block_on;
use futures::join;

async fn async_main() {
    let f_ip4_1 = getmyipv4_1();
    let f_ip4_2 = getmyipv4_2();
    let (ip4_1, ip4_2) = join!(f_ip4_1, f_ip4_2);
    println!("{}", ip4_1);
    println!("{}", ip4_2);
}

#[tokio::main]
async fn main() {
    let handle = tokio::runtime::Handle::current();
    let _ = handle.enter();
    block_on(async_main());
}
