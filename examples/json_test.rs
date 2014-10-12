extern crate serialize;

use serialize::json::from_str;

fn main() {
	let key = "created_utc".into_string();
	
	let json_str = stringify!({"accounts_active":64,"collapse_deleted_comments":false,"comment_score_hide_mins":0,"created":1291325238,"created_utc":1291325238, "description":"Anything whatsoever related to the Rust programming language"});

	let json = from_str(json_str.as_slice()).unwrap();

    println!("{}", json);
	
	println!("{}", json.find(&key).and_then(|j| j.as_u64()));
	
}
