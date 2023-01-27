use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};

fn main() -> Result<(), SystemTimeError> {
    let mut input = String::new();
    let start = SystemTime::now().duration_since(UNIX_EPOCH);
    std::io::stdin().read_line(&mut input).unwrap();
    let end = SystemTime::now().duration_since(UNIX_EPOCH);
    // let mut startTime;
    // match start.elapsed() {
    //     Ok(elapsed) => {
    //         // it prints '2'
    //         println!("{}", elapsed.as_secs());
    //     }
    //     Err(e) => {
    //         // an error occurred!
    //         println!("Error: {e:?}");
    //     }
    // }
    let alphabet = String::from("abcdefghijklmnopqrstuvwxyz\n");
    println!("{}", alphabet.eq(&input));
    println!("Time Spent: {:?}", end? - start?);
    // println!("Time Spent: {:?}", start?);
    Ok(())
}
