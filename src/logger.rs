use std::error::Error;
use std::io::{ self, BufWriter, Write  };
use std::fs;
use std::fs::OpenOptions;
use chrono::prelude::*;

use output::USIStdErrorWriter;
use Logger;
use string::AddIndent;

#[derive(Debug)]
pub struct FileLogger {
	writer:BufWriter<fs::File>,
}
impl FileLogger {
	pub fn new(file:String) -> Result<FileLogger,io::Error> {
		Ok(FileLogger {
			writer:BufWriter::new(OpenOptions::new().append(true).create(true).open(file)?),
		})
	}
}
impl Logger for FileLogger {
	fn logging(&mut self, msg:&String) -> bool {
		let dt = Local::now();

		let msg = format!("{}\n{}\n", dt.format("%Y-%m-%d %H:%M:%S").to_string(), msg.add_indent(2));
		match self.writer.write(msg.as_bytes()) {
			Ok(_) => !self.writer.flush().is_err(),
			Err(_)=> {
				USIStdErrorWriter::write("The log could not be written to the file.").unwrap();
				self.writer.flush().is_err();
				false
			}
		}
	}
	fn logging_error<E: Error>(&mut self, e:&E) -> bool {
		let mut messages:Vec<String> = vec![];
		let mut indent:u32 = 0;

		messages.push(format!("{}", e).add_indent(indent*2));

		let mut e:&Error = e;

		while let Some(cause) = e.cause() {
			indent += 1;
			messages.push(format!("{}", cause).add_indent(indent*2));
			e = cause;
		}

		self.logging(&messages.join("\n"))
	}
}