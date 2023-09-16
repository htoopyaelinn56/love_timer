use dotenv::dotenv;
use love_timer::run;

fn main() {
    dotenv().ok();
    run();
}
