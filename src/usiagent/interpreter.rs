use std::thread;
use std::sync::Mutex;
use std::sync::Arc;
use std::marker::Send;
use std::collections::HashMap;

use usiagent::event::*;
use usiagent::output::USIStdErrorWriter;
use usiagent::input::USIInputReader;
use usiagent::Logger;

struct USIInterpreter {
}
impl USIInterpreter {
	pub fn new() -> USIInterpreter {
		USIInterpreter {

		}
	}

	pub fn start<L,R>(
		mut event_queue:EventQueue<SystemEvent,SystemEventKind>,
		reader:Arc<Mutex<R>>,optmap:HashMap<String,SysEventOptionKind>, logger:Arc<Mutex<L>>)
		where R: USIInputReader, L: Logger,
				Arc<Mutex<R>>: Send + 'static, Mutex<L>: Send, Arc<Mutex<L>>: Send + 'static {

		let reader = reader.clone();
		let logger = logger.clone();

		thread::spawn(move || {
			let position_parser = PositionParser::new();
			let go_parser = GoParser::new();

			loop {
				match reader.lock() {
					Err(ref e) => {
						match logger.lock() {
							Ok(mut logger) => logger.logging_error(e),
							Err(_) => {
								USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
							}
						}
					},
					Ok(ref mut reader) => match reader.read() {
						Err(ref e) => {
							match logger.lock() {
								Ok(mut logger) => logger.logging_error(e),
								Err(_) => {
									USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
								}
							}
						},
						Ok(ref line) => {
							let f = line.split(" ").collect::<Vec<&str>>();

							match f[0] {
								"usi" => event_queue.push(SystemEvent::Usi),
								"isready" => event_queue.push(SystemEvent::IsReady),
								"setoption" if f.len() == 5 => {
									match optmap.get(&f[2].to_string()) {
										None => {
											match logger.lock() {
												Ok(mut logger) => logger.logging(&String::from("Could not get option type.")),
												Err(_) => {
													USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
												}
											}
										},
										Some(kind) => {
											match (f[1],f[2],f[3],f[4], kind) {
												("name", id, "value", v, &SysEventOptionKind::Str) => {
													event_queue.push(SystemEvent::SetOption(id.to_string(),SysEventOption::Str(v.to_string())));
												},
												("name", id, "value", v, &SysEventOptionKind::Num) => {
													match v.parse::<u32>() {
														Ok(n) => event_queue.push(SystemEvent::SetOption(id.to_string(),SysEventOption::Num(n))),
														Err(ref e) => match logger.lock() {
															Ok(mut logger) => logger.logging_error(e),
															Err(_) => {
																USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
															}
														}
													}
												},
												("name", id, "value", "true", &SysEventOptionKind::Bool) => {
													event_queue.push(SystemEvent::SetOption(id.to_string(),SysEventOption::Bool(true)));
												},
												("name", id, "value", "false", &SysEventOptionKind::Bool) => {
													event_queue.push(SystemEvent::SetOption(id.to_string(),SysEventOption::Bool(false)));
												},
												_ => {
													match logger.lock() {
														Ok(mut logger) => logger.logging(&String::from("The format of the option command is illegal.")),
														Err(_) => {
															USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
														}
													}
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
											match logger.lock() {
												Ok(mut logger) => logger.logging_error(e),
												Err(_) => {
													USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
												}
											}
										}
									}
								}
								"go" if f.len() > 1 => {
									match go_parser.parse(&f[1..]) {
										Ok(e) => event_queue.push(e),
										Err(ref e) => {
											match logger.lock() {
												Ok(mut logger) => logger.logging_error(e),
												Err(_) => {
													USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
												}
											}
										}
									}
								},
								"stop" => event_queue.push(SystemEvent::Stop),
								"ponderhit" => event_queue.push(SystemEvent::PonderHit),
								"quit" => event_queue.push(SystemEvent::Quit),
								"gameover" if f.len() == 2 => {
									match f[2] {
										"win" => event_queue.push(SystemEvent::GameOver(GameEndState::Win)),
										"lose" =>event_queue.push(SystemEvent::GameOver(GameEndState::Lose)),
										"draw" => event_queue.push(SystemEvent::GameOver(GameEndState::Draw)),
										_ => {
											match logger.lock() {
												Ok(mut logger) => logger.logging(&String::from("The format of the gameover command is illegal.")),
												Err(_) => {
													USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
												}
											}
										}
									}
								},
								_ => {
									match logger.lock() {
										Ok(mut logger) => logger.logging(&String::from("The format of the command is illegal.")),
										Err(_) => {
											USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
										}
									}
								}
							}
						}
					}
				}
			}
		});
	}
}