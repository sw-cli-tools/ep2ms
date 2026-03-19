//! ep2ms - Convert current time to milliseconds since Unix epoch.

mod args;
mod version;

use chrono::Utc;

fn main() {
    let args = args::parse();

    if args.version {
        version::print();
        return;
    }

    let now = Utc::now();
    let ms = now.timestamp_millis();

    if args.verbose {
        eprintln!("{}", now.to_rfc3339());
    }

    println!("{ms}");
}
