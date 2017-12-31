use std::io::{self, Write,Result};
use std::ops::Add;

trait USIOutputWriter {
	fn write(&self,lines:&Vec<String>) -> Result<usize>;
}
struct USIStdOutputWriter {

}
impl USIOutputWriter for USIStdOutputWriter {
	fn write(&self,lines:&Vec<String>) -> Result<usize> {
		let stdout = io::stdout();
		let mut writer = stdout.lock();

		writer.write(lines.join("\n").add("\n").as_bytes())
	}
}