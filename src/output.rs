//! USIコマンド文字列、エラーメッセージ等の出力
use std::io::{self, Write,Result};
use std::ops::Add;

/// USIコマンド文字列を出力
pub trait USIOutputWriter {
	/// 出力を行う
	///
	/// # Arguments
	/// * `lines` - 出力する行のリスト
	fn write(&self,lines:&Vec<String>) -> Result<usize>;
}
/// USIコマンド文字列を標準出力へ出力する`USIOutputWriter`の実装
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
/// 標準エラー出力へ出力を書き込むためのオブジェクト
pub struct USIStdErrorWriter {
}
impl USIStdErrorWriter {
	/// 出力を行う
	///
	/// # Arguments
	/// * `s` - 標準エラー出力へ出力する文字列
	pub fn write(s:&str) -> Result<usize> {
		let stderr = io::stderr();
		let mut h = stderr.lock();
		h.write(s.as_bytes())
	}
}