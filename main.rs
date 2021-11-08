use std::process::Command;
use std::io::Write;

fn main() {
	for i in 0..10000 {
		let mut file = std::fs::OpenOptions::new()
			.write(true)
			.append(true)
			.open("test.txt")
			.unwrap();

		write!(file, "idk\n");

		Command::new("sh")
			.args(["-c", "git add ."])
			.output()
			.expect("failed to commit :sadge:");
		let output = Command::new("sh")
			.args(["-c", "git commit -am \"lol\""])
			.output()
			.expect("failed to commit :sadge:");
		
		println!("{} {}", std::str::from_utf8(&output.stdout).unwrap(), i);
	};
}