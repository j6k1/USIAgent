use std::io;
use std::io::BufRead;

trait USIInputReader {
	fn read(&self) -> io::Result<String>;
}
struct USIStdInputReader {

}
impl USIInputReader for USIStdInputReader {
	fn read(&self) -> io::Result<String> {
		let stdin = io::stdin();
		let mut lock = stdin.lock();
		let mut buf = String::new();
		lock.read_line(&mut buf)?;
		Ok(buf)
	}
}