use chrono::{DateTime, Local, Utc};

fn main() {
    let utc: DateTime<Utc> = Utc::now();
    let local_time: DateTime<Local> = Local::now();
    println!("current UTC time is: {}", utc);
    println!("local time is: {}", local_time);
}
