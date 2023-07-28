use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();

    for result in io::stdin().bytes() {
        match result {
            Ok(b) => {
                let c = b as char;
                println!("{}", c);

                if c.is_control() {
                    println!("{:?}  \r ", b);
                } else {
                    println!("{:?} ({})\r", b, c);
                }

                if c == 'q' {
                    break;
                }
            }
            Err(err) => {
                eprintln!("Error reading input: {:?}", err);
                break;
            }
        }
    }
}
