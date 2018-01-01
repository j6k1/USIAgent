use std::error::Error;
use std::fs;
use std::io::{ BufWriter, Write };
use chrono::prelude::*;

use usiagent::output::USIStdErrorWriter;
use usiagent::Logger;
use usiagent::string::AddIndent;

pub struct FileLogger {
	file:String,
}
impl FileLogger {
	pub fn new(file:String) -> FileLogger {
		FileLogger {
			file:file,
		}
	}
}
impl Logger for FileLogger {
	fn logging(&self, msg:&String) {
		match fs::File::create(&self.file) {
			Ok(ref f) => {
				let mut writer = BufWriter::new(f);
				let dt = Local::now();

				let msg = format!("{}\n{}", dt.format("%Y-%m-%d %H:%M:%S").to_string(), msg.add_indent(2));
				match writer.write(msg.as_bytes()) {
					Ok(_) => (),
					Err(_)=> {
						USIStdErrorWriter::write("The log could not be written to the file.").unwrap();
					}
				}
			},
			Err(_) => {
				USIStdErrorWriter::write("The log output destination file could not be opened.").unwrap();
			}
		}
	}
	fn logging_error<E: Error>(&self, e:&E) {
		let mut messages:Vec<String> = vec![];
		let mut indent:u32 = 0;

		messages.push(format!("{}", e).add_indent(indent*2));

		let mut e:&Error = e;

		while let Some(cause) = e.cause() {
			indent += 1;
			messages.push(format!("{}", cause).add_indent(indent*2));
			e = cause;
		}

		self.logging(&messages.join("\n"));
	}
}