pub trait AddIndent {
	fn add_indent(&self, u32) -> String;
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