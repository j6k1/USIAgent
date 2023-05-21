//! 入力の読み取り
use std::io;
use std::io::BufRead;

/// 入力を読み取る
pub trait USIInputReader {
	fn read(&mut self) -> io::Result<Option<String>>;
}
/// 標準入力から読み取る`USIInputReader`の実装
pub struct USIStdInputReader {

}
impl USIStdInputReader {
	/// `USIStdInputReader`の生成
	pub fn new() -> USIStdInputReader {
		USIStdInputReader {

		}
	}
}
impl USIInputReader for USIStdInputReader {
	/// 入力を一行読み取る
	fn read(&mut self) -> io::Result<Option<String>> {
		let stdin = io::stdin();
		let mut lock = stdin.lock();
		let mut buf = String::new();

		if lock.read_line(&mut buf)? == 0 {
			Ok(None)
		} else {
			let ptn:&[_] = &['\r','\n'];

			Ok(Some(buf.as_str().trim_end_matches(ptn).to_string()))
		}
	}
}
