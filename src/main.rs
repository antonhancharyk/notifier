use dotenv::dotenv;

mod broker;
mod email;
mod tg;

fn main() {
    dotenv().ok();

    broker::start();
}
