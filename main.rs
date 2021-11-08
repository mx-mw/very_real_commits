use std::process::Command;

fn main() {
	Command::new("sh")
		.args(["-c", "git add ."])
		.output()
		.expect("failed to commit :sadge:");
	let output = Command::new("sh")
		.args(["-c", "git commit -am \"lol\""])
		.output()
		.expect("failed to commit :sadge:");
	
	println!("{}", std::str::from_utf8(&output.stdout).unwrap());
}