//! 標準出力、標準エラー出力への書き込み機能
use std::io::{self, Write,Result};
use std::ops::Add;

pub trait USIOutputWriter {
	fn write(&self,lines:&Vec<String>) -> Result<usize>;
}
pub struct USIStdOutputWriter {

}
impl USIStdOutputWriter {
	pub fn new() -> USIStdOutputWriter {
		USIStdOutputWriter {

		}
	}
}
impl USIOutputWriter for USIStdOutputWriter {
	fn write(&self,lines:&Vec<String>) -> Result<usize> {
		let stdout = io::stdout();
		let mut writer = stdout.lock();

		writer.write(lines.join("\n").add("\n").as_bytes())
	}
}
pub struct USIStdErrorWriter {
}
impl USIStdErrorWriter {
	pub fn write(s:&str) -> Result<usize> {
		let stderr = io::stderr();
		let mut h = stderr.lock();
		h.write(s.as_bytes())
	}
}