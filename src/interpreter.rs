use std::thread;
use std::sync::Mutex;
use std::sync::Arc;
use std::marker::Send;
use std::collections::HashMap;

use event::*;
use output::USIStdErrorWriter;
use input::USIInputReader;
use Logger;
use OnErrorHandler;

pub struct USIInterpreter {
}
impl USIInterpreter {
	pub fn new() -> USIInterpreter {
		USIInterpreter {

		}
	}

	pub fn start<L,R>(&self,
		event_queue:Arc<Mutex<EventQueue<SystemEvent,SystemEventKind>>>,
		reader:Arc<Mutex<R>>,optmap:HashMap<String,SysEventOptionKind>, logger:&Arc<Mutex<L>>)
		where R: USIInputReader, L: Logger,
				Arc<Mutex<R>>: Send + 'static, Arc<Mutex<L>>: Send + 'static {
		let event_queue = event_queue.clone();
		let reader = reader.clone();
		let logger = logger.clone();
		let on_error_handler = Arc::new(Mutex::new(OnErrorHandler::new(logger.clone())));
		let on_error_handler = on_error_handler.clone();

		thread::spawn(move || {
			let position_parser = PositionParser::new();
			let go_parser = GoParser::new();

			loop {
				match reader.lock() {
					Err(ref e) => {
						on_error_handler.lock().map(|h| h.call(e)).is_err();
					},
					Ok(ref mut reader) => match reader.read() {
						Err(ref e) => {
							on_error_handler.lock().map(|h| h.call(e)).is_err();
						},
						Ok(ref line) => {
							let line = line.trim_right();
							let f = line.split(" ").collect::<Vec<&str>>();

							match event_queue.lock() {
								Err(ref e) => {
									on_error_handler.lock().map(|h| h.call(e)).is_err();
								},
								Ok(mut event_queue) => {
									match f[0] {
										"usi" => event_queue.push(SystemEvent::Usi),
										"isready" => event_queue.push(SystemEvent::IsReady),
										"setoption" if f.len() >= 5 => {
											match optmap.get(&f[2].to_string()) {
												None => {
													logger.lock().map(|mut logger| logger.logging(&String::from("Could not get option type."))).map_err(|_| {
														USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
														false
													}).is_err();
												},
												Some(kind) => {
													match (f[1],f[2],f[3],f[4], kind) {
														("name", id, "value", v, &SysEventOptionKind::Str) => {
															event_queue.push(SystemEvent::SetOption(id.to_string(),SysEventOption::Str(v.to_string())));
														},
														("name", id, "value", v, &SysEventOptionKind::Num) => {
															v.parse::<u32>().map(|n|{
																event_queue.push(SystemEvent::SetOption(id.to_string(),SysEventOption::Num(n)));
															}).map_err(|ref e| {
																on_error_handler.lock().map(|h| h.call(e))
															}).is_err();
														},
														("name", id, "value", "true", &SysEventOptionKind::Bool) => {
															event_queue.push(SystemEvent::SetOption(id.to_string(),SysEventOption::Bool(true)));
														},
														("name", id, "value", "false", &SysEventOptionKind::Bool) => {
															event_queue.push(SystemEvent::SetOption(id.to_string(),SysEventOption::Bool(false)));
														},
														_ => {
															logger.lock().map(|mut logger| logger.logging(&String::from("The format of the option command is illegal."))).map_err(|_| {
																USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
																false
															}).is_err();
														}
													}
												}
											}
										},
										"usinewgame" => event_queue.push(SystemEvent::UsiNewGame),
										"position" if f.len() > 1 => {
											match position_parser.parse(&f[1..]) {
												Ok(e) => event_queue.push(e),
												Err(ref e) => {
													on_error_handler.lock().map(|h| h.call(e)).is_err();
												}
											}
										}
										"go" if f.len() > 1 => {
											match go_parser.parse(&f[1..]) {
												Ok(e) => event_queue.push(e),
												Err(ref e) => {
													on_error_handler.lock().map(|h| h.call(e)).is_err();
												}
											}
										},
										"stop" => event_queue.push(SystemEvent::Stop),
										"ponderhit" => event_queue.push(SystemEvent::PonderHit),
										"quit" => {
											event_queue.push(SystemEvent::Quit);
											break;
										},
										"gameover" if f.len() == 2 => {
											match f[2] {
												"win" => event_queue.push(SystemEvent::GameOver(GameEndState::Win)),
												"lose" =>event_queue.push(SystemEvent::GameOver(GameEndState::Lose)),
												"draw" => event_queue.push(SystemEvent::GameOver(GameEndState::Draw)),
												_ => {
													logger.lock().map(|mut logger| logger.logging(&String::from("The format of the gameover command is illegal."))).map_err(|_| {
														USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
														false
													}).is_err();
												}
											}
										},
										_ => {
											logger.lock().map(|mut logger| logger.logging(&format!("The format of the command is illegal. (input: {})",line))).map_err(|_| {
												USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
												false
											}).is_err();
										}
									}
								}
							}
						}
					}
				}
			};
		});
	}
}