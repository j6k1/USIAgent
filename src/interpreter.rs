//! USIプロトコルの入力を解釈するインタプリタ
use std::thread;
use std::sync::Mutex;
use std::sync::Arc;
use std::marker::Send;
use std::collections::BTreeMap;

use event::*;
use protocol::*;
use output::USIStdErrorWriter;
use input::USIInputReader;
use Logger;
use OnErrorHandler;

/// USIプロトコルのコマンド文字列を読み取りイベントを発火する構造体
pub struct USIInterpreter {
}
impl USIInterpreter {
	/// `USIInterpreter`の生成
	pub fn new() -> USIInterpreter {
		USIInterpreter {

		}
	}

	/// 処理の開始
	///
	/// # Arguments
	/// * `event_queue` - システムイベントキュー
	/// * `reader` - 入力を読み取るためのオブジェクト。実装によって標準入力以外から読み取るものを指定することも可能。
	/// * `optmap` - プレイヤーオブジェクトに渡されるオプションの種類のマップ
	/// * `logger` - ログを書き込むためのオブジェクト。実装によってファイル以外に書き込むものを指定することも可能。
	pub fn start<L,R>(&self,
		event_queue:Arc<Mutex<SystemEventQueue>>,
		mut reader:R,optmap:BTreeMap<String,SysEventOptionKind>, logger:&Arc<Mutex<L>>)
		where R: USIInputReader + Send + 'static, L: Logger,
				Arc<Mutex<L>>: Send + 'static {
		let event_queue = event_queue.clone();
		let logger = logger.clone();
		let on_error_handler = Arc::new(Mutex::new(OnErrorHandler::new(logger.clone())));
		let on_error_handler = on_error_handler.clone();

		thread::spawn(move || {
			let position_parser = PositionParser::new();
			let go_parser = GoParser::new();

			loop {
				match reader.read() {
					Err(ref e) => {
						let _ = on_error_handler.lock().map(|h| h.call(e));
					},
					Ok(Some(ref line)) => {
						let f = line.split(" ").collect::<Vec<&str>>();

						match event_queue.lock() {
							Err(ref e) => {
								let _ = on_error_handler.lock().map(|h| h.call(e));
							},
							Ok(mut event_queue) => {
								match f[0] {
									"usi" => event_queue.push(SystemEvent::Usi),
									"isready" => event_queue.push(SystemEvent::IsReady),
									"setoption" if f.len() == 3 => {
										match optmap.get(&f[2].to_string()) {
											None => {
												let _ = logger.lock().map(|mut logger| logger.logging(&String::from("Could not get option type."))).map_err(|_| {
													USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
													false
												});
											},
											Some(kind) => {
												match (f[1],f[2],kind) {
													("name", id, &SysEventOptionKind::Exist) => {
														event_queue.push(SystemEvent::SetOption(id.to_string(),SysEventOption::Exist));
													},
													_ => {
														let _ = logger.lock().map(|mut logger| logger.logging(&String::from("The format of the option command is illegal."))).map_err(|_| {
															USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
															false
														});
													}
												}
											}
										}
									},
									"setoption" if f.len() >= 5 => {
										match optmap.get(&f[2].to_string()) {
											None => {
												let _ = logger.lock().map(|mut logger| logger.logging(&String::from("Could not get option type."))).map_err(|_| {
													USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
													false
												});
											},
											Some(kind) => {
												match (f[1],f[2],f[3],f[4], kind) {
													("name", id, "value", v, &SysEventOptionKind::Str) => {
														event_queue.push(SystemEvent::SetOption(id.to_string(),SysEventOption::Str(v.to_string())));
													},
													("name", id, "value", v, &SysEventOptionKind::Num) => {
														let _ = v.parse::<i64>().map(|n|{
															event_queue.push(SystemEvent::SetOption(id.to_string(),SysEventOption::Num(n)));
														}).map_err(|ref e| {
															on_error_handler.lock().map(|h| h.call(e))
														});
													},
													("name", id, "value", "true", &SysEventOptionKind::Bool) => {
														event_queue.push(SystemEvent::SetOption(id.to_string(),SysEventOption::Bool(true)));
													},
													("name", id, "value", "false", &SysEventOptionKind::Bool) => {
														event_queue.push(SystemEvent::SetOption(id.to_string(),SysEventOption::Bool(false)));
													},
													_ => {
														let _ = logger.lock().map(|mut logger| logger.logging(&String::from("The format of the option command is illegal."))).map_err(|_| {
															USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
															false
														});
													}
												}
											}
										}
									},
									"usinewgame" => event_queue.push(SystemEvent::UsiNewGame),
									"position" if f.len() > 1 => {
										match position_parser.parse(&f[1..]) {
											Ok(p) => {
												match p {
													PositionParseResult(t,p,n,m) => {
														event_queue.push(SystemEvent::Position(t,p,n,m));
													}
												}
											},
											Err(ref e) => {
												let _ = on_error_handler.lock().map(|h| h.call(e));
											}
										}
									}
									"go" => {
										match go_parser.parse(&f[1..]) {
											Ok(g) => {
												event_queue.push(SystemEvent::Go(g));
											},
											Err(ref e) => {
												let _ = on_error_handler.lock().map(|h| h.call(e));
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
										match f[1] {
											"win" => event_queue.push(SystemEvent::GameOver(GameEndState::Win)),
											"lose" =>event_queue.push(SystemEvent::GameOver(GameEndState::Lose)),
											"draw" => event_queue.push(SystemEvent::GameOver(GameEndState::Draw)),
											_ => {
												let _ = logger.lock().map(|mut logger| logger.logging(&String::from("The format of the gameover command is illegal."))).map_err(|_| {
													USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
													false
												});
											}
										}
									},
									_ => {
										let _ = logger.lock().map(|mut logger| logger.logging(&format!("The format of the command is illegal. (input: {})",line))).map_err(|_| {
											USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
											false
										});
									}
								}
							}
						}
					},
					Ok(None) => {
						match event_queue.lock() {
							Err(ref e) => {
								let _ = on_error_handler.lock().map(|h| h.call(e));
							},
							Ok(mut event_queue) => {
								event_queue.push(SystemEvent::Quit);
								break;
							}
						}
					}
				}
			}
		});
	}
}