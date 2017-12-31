use std::io::{self, Write,Result};

trait USIOutputWriter {
	fn write(&self,lines:&Vec<String>) -> Result<usize>;
}
struct USIStdOutputWriter {

}
impl USIOutputWriter for USIStdOutputWriter {
	fn write(&self,lines:&Vec<String>) -> Result<usize> {
		let stdout = io::stdout();
		let mut writer = stdout.lock();

		writer.write(lines.join("\n").as_bytes())
	}
}