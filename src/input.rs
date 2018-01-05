use std::io;
use std::io::BufRead;

pub trait USIInputReader {
	fn read(&mut self) -> io::Result<String>;
}
pub struct USIStdInputReader {

}
impl USIStdInputReader {
	pub fn new() -> USIStdInputReader {
		USIStdInputReader {

		}
	}
}
impl USIInputReader for USIStdInputReader {
	fn read(&mut self) -> io::Result<String> {
		let stdin = io::stdin();
		let mut lock = stdin.lock();
		let mut buf = String::new();
		lock.read_line(&mut buf)?;
		Ok(buf)
	}
}