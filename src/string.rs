//! Stringに関連した機能
/// 文字列の各行をインデントする
pub trait AddIndent {
	/// 文字列の各行をインデントした結果を返す
	///
	/// # Arguments
	/// * `sz` - インデントの回数
	fn add_indent(&self, sz:u32) -> String;
}
impl AddIndent for String {
	fn add_indent(&self, sz:u32) -> String {
		let lines = self.replace("\r\n","\n");
		let lines = lines.split("\n");
		lines.map(|s| format!("{}{}", (0..sz)
										.map(|_| String::from(" "))
										.collect::<Vec<String>>().join(""), s))
															.collect::<Vec<String>>().join("\n")
	}
}