mod cli;
mod net;
mod robot;

use robot::Credentials;

fn main() {
    let xd = Credentials::default();

    let idk = robot::connect_to_robot(xd);
    if idk.is_err() {
        println!("{:?}", idk.unwrap_err())
    }

    println!("Hello, world!");
}
