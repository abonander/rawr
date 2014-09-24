extern crate rawr;

use rawr::sub;

fn main() {
    let mut stdin = std::io::stdio::stdin();
    let mut stdout = std::io::stdio::stdout();

    stdout.write_line("Welcome to the RAWR Test post program.");
    stdout.write_str("Subreddit: ");
    stdout.flush();

    let sub_name = stdin.read_line().unwrap();

    let subreddit = sub(sub_name.as_slice()).unwrap();

            
}
