mod initial_position_creator;
mod kifuwriter;
mod flip_players;
mod quit;
mod error;

use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::{Instant,Duration};

use std::sync::mpsc::Receiver;
use std::sync::mpsc;
use usiagent::selfmatch::*;
use usiagent::shogi::*;
use usiagent::command::*;
use usiagent::event::*;
use usiagent::error::*;

#[allow(unused_imports)]
use usiagent::shogi::KomaKind::{
	SFu,
	SKyou,
	SKei,
	SGin,
	SKin,
	SKaku,
	SHisha,
	SOu,
	SFuN,
	SKyouN,
	SKeiN,
	SGinN,
	SKakuN,
	SHishaN,
	GFu,
	GKyou,
	GKei,
	GGin,
	GKin,
	GKaku,
	GHisha,
	GOu,
	GFuN,
	GKyouN,
	GKeiN,
	GGinN,
	GKakuN,
	GHishaN,
	Blank
};

use common::*;

fn create_options() -> Vec<(String,SysEventOption)> {
	vec![
		(String::from("OptionButton"),SysEventOption::Exist),
		(String::from("OptionCheck"),SysEventOption::Bool(true)),
		(String::from("OptionCombo"),SysEventOption::Str(String::from("cccc"))),
		(String::from("OptionCombo2"),SysEventOption::Str(String::from("eeee"))),
		(String::from("OptionFileName"),SysEventOption::Str(String::from("book.bin"))),
		(String::from("OptionFileName2"),SysEventOption::Str(String::from("book2.bin"))),
		(String::from("OptionSpin"),SysEventOption::Num(25)),
		(String::from("OptionString"),SysEventOption::Str(String::from("string.."))),
		(String::from("OptionString2"),SysEventOption::Str(String::from("string..."))),
		(String::from("USI_Hash"),SysEventOption::Num(1000)),
		(String::from("USI_Ponder"),SysEventOption::Bool(false)),
	]
}
fn startup(pmr:&[Receiver<Result<ActionKind,String>>; 2]) {
	for i in 0..2 {
		let res = pmr[i].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetOption timed out.");

		assert_eq!(res,Ok(ActionKind::SetOption));

		let res = pmr[i].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetOption timed out.");

		assert_eq!(res,Ok(ActionKind::SetOption));

		let res = pmr[i].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetOption timed out.");

		assert_eq!(res,Ok(ActionKind::SetOption));

		let res = pmr[i].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetOption timed out.");

		assert_eq!(res,Ok(ActionKind::SetOption));

		let res = pmr[i].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetOption timed out.");

		assert_eq!(res,Ok(ActionKind::SetOption));

		let res = pmr[i].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetOption timed out.");

		assert_eq!(res,Ok(ActionKind::SetOption));

		let res = pmr[i].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetOption timed out.");

		assert_eq!(res,Ok(ActionKind::SetOption));

		let res = pmr[i].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetOption timed out.");

		assert_eq!(res,Ok(ActionKind::SetOption));

		let res = pmr[i].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetOption timed out.");

		assert_eq!(res,Ok(ActionKind::SetOption));

		let res = pmr[i].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetOption timed out.");

		assert_eq!(res,Ok(ActionKind::SetOption));

		let res = pmr[i].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetOption timed out.");

		assert_eq!(res,Ok(ActionKind::SetOption));
	}
}
fn gamestart_process(pmr:&[Receiver<Result<ActionKind,String>>; 2]) {
	for i in 0..2 {
		let res = pmr[i].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::TakeReady timed out.");

		assert_eq!(res,Ok(ActionKind::TakeReady));

		let res = pmr[i].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::NewGame timed out.");

		assert_eq!(res,Ok(ActionKind::NewGame));
	}
}
fn create_input_read_handler(system_event_queue:&Arc<Mutex<SystemEventQueue>>)
	-> impl FnMut(String) -> Result<bool,SelfMatchRunningError<CommonError>> + Send + 'static {
	let system_event_queue = system_event_queue.clone();

	let input_read_handler = move |input| {
		if input == "quit" {
			return match system_event_queue.lock()  {
				Ok(mut system_event_queue) => {
					system_event_queue.push(SystemEvent::Quit);
					Ok(false)
				},
				Err(_) => {
					Err(SelfMatchRunningError::InvalidState(String::from(
						"Failed to secure exclusive lock of system_event_queue."
					)))
				}
			};
		}
		Ok(true)
	};

	input_read_handler
}
#[test]
fn test_resign_1times() {
	let (pms1,pmr1) = mpsc::channel();
	let (pns1,_) = mpsc::channel();
	let (ts,tr) = mpsc::channel();

	let (pms2,pmr2) = mpsc::channel();
	let (pns2,_) = mpsc::channel();

	let pmr = [pmr1,pmr2];

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = mpsc::channel();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (es,er) = mpsc::channel();
	let (ks,kr) = mpsc::channel();

	let mut kifuwriter = MockSfenKifuWriter::new(ks);

	let _ = thread::spawn(move || {
		let player1 = MockPlayer::new(pms1,pns1,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										}),
										Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),None))
										}),
										Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Resign)
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Lose => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let player2 = MockPlayer::new(pms2,pns2,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(9,3),KomaDstToPosition(9,4,false)),None))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Win => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let (is,_) = mpsc::channel();

		let info_sender = MockInfoSender::new(is);

		let mut engine = SelfMatchEngine::new();

		let input_read_handler = create_input_read_handler(&engine.system_event_queue);

		let _ = engine.start(|self_match_event_dispatcher| {
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameStart, move |_,e| {
					match e {
						&SelfMatchEvent::GameStart(n,t,_) => {
							if t == Teban::Sente && n == 1 {
								let _ = hes.send(Ok(EventState::GameStart));
							} else {
								let _ = hes.send(Err(String::from("GameStart event is invalid.")));
							}
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Moved, move |_,e| {
					match e {
						&SelfMatchEvent::Moved(_,_) => {
							let _ = hes.send(Ok(EventState::Moved));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameEnd, move |_,e| {
					match e {
						&SelfMatchEvent::GameEnd(SelfMatchGameEndState::Resign(t)) => {
							if t == Teban::Sente {
								let _ = hes.send(Ok(EventState::GameEnd));
							} else {
								let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							}
						},
						_ => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
						}
					}
					Ok(())
				});
			let hes = es.clone();
			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Abort, move |_,e| {
					match e {
						&SelfMatchEvent::Abort => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			},
			|| false,
			None,
			Some(Box::new(move |sfen,mvs| kifuwriter.write(sfen,mvs))),
			input_reader, input_read_handler,
			player1,player2,
			create_options(), create_options(),
			info_sender,
			UsiGoTimeLimit::None,
			None,Some(1),
			logger, |h,e| {
				if let Some(h) = h {
					let _ = h.lock().map(|h| h.call(e));
				}
			}
		);

		let _ = ts.send(());
	});

	startup(&pmr);

	gamestart_process(&pmr);

	let res = er.recv_timeout(Duration::from_millis(300)).expect("attempt to receive EventState::GameStart timed out.");

	assert_eq!(res,Ok(EventState::GameStart));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::GameEnd timed out.");

	assert_eq!(res,Ok(EventState::GameEnd));

	let res = kr.recv_timeout(Duration::from_millis(60)).expect("attempt to receive kifu string.");

	assert_eq!(&*res,"startpos moves 1g1f 9c9d");

	let res = pmr[0].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let res = pmr[1].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let _ = tr.recv_timeout(Duration::from_millis(180)).expect("attempt to receive on quited timed out.");

	let _ = s.send(String::from(""));
}
#[test]
fn test_invalidmove_1times() {
	let (pms1,pmr1) = mpsc::channel();
	let (pns1,_) = mpsc::channel();
	let (ts,tr) = mpsc::channel();

	let (pms2,pmr2) = mpsc::channel();
	let (pns2,_) = mpsc::channel();

	let pmr = [pmr1,pmr2];

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = mpsc::channel();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (es,er) = mpsc::channel();
	let (ks,kr) = mpsc::channel();

	let mut kifuwriter = MockSfenKifuWriter::new(ks);

	let _ = thread::spawn(move || {
		let player1 = MockPlayer::new(pms1,pns1,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										}),
										Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),None))
										}),
										Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(8,8),KomaDstToPosition(7,8,false)),None))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Lose => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let player2 = MockPlayer::new(pms2,pns2,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(9,3),KomaDstToPosition(9,4,false)),None))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Win => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let (is,_) = mpsc::channel();

		let info_sender = MockInfoSender::new(is);

		let mut engine = SelfMatchEngine::new();

		let input_read_handler = create_input_read_handler(&engine.system_event_queue);

		let _ = engine.start(|self_match_event_dispatcher| {
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameStart, move |_,e| {
					match e {
						&SelfMatchEvent::GameStart(n,t,_) => {
							if t == Teban::Sente && n == 1 {
								let _ = hes.send(Ok(EventState::GameStart));
							} else {
								let _ = hes.send(Err(String::from("GameStart event is invalid.")));
							}
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Moved, move |_,e| {
					match e {
						&SelfMatchEvent::Moved(_,_) => {
							let _ = hes.send(Ok(EventState::Moved));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameEnd, move |_,e| {
					match e {
						&SelfMatchEvent::GameEnd(SelfMatchGameEndState::Foul(t,FoulKind::InvalidMove)) => {
							if t == Teban::Sente {
								let _ = hes.send(Ok(EventState::GameEnd));
							} else {
								let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							}
						},
						_ => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
						}
					}
					Ok(())
				});
			let hes = es.clone();
			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Abort, move |_,e| {
					match e {
						&SelfMatchEvent::Abort => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			},
			|| false,
			None,
			Some(Box::new(move |sfen,mvs| kifuwriter.write(sfen,mvs))),
			input_reader, input_read_handler,
			player1,player2,
			create_options(), create_options(),
			info_sender,
			UsiGoTimeLimit::None,
			None,Some(1),
			logger, |h,e| {
				if let Some(h) = h {
					let _ = h.lock().map(|h| h.call(e));
				}
			}
		);

		let _ = ts.send(());
	});

	startup(&pmr);

	gamestart_process(&pmr);

	let res = er.recv_timeout(Duration::from_millis(300)).expect("attempt to receive EventState::GameStart timed out.");

	assert_eq!(res,Ok(EventState::GameStart));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::GameEnd timed out.");

	assert_eq!(res,Ok(EventState::GameEnd));

	let res = kr.recv_timeout(Duration::from_millis(60)).expect("attempt to receive kifu string.");

	assert_eq!(&*res,"startpos moves 1g1f 9c9d 8h7h");

	let res = pmr[0].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let res = pmr[1].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let _ = tr.recv_timeout(Duration::from_millis(180)).expect("attempt to receive on quited timed out.");

	let _ = s.send(String::from(""));
}
#[test]
fn test_invalidmove_by_from_blank_1times() {
	let (pms1,pmr1) = mpsc::channel();
	let (pns1,_) = mpsc::channel();
	let (ts,tr) = mpsc::channel();

	let (pms2,pmr2) = mpsc::channel();
	let (pns2,_) = mpsc::channel();

	let pmr = [pmr1,pmr2];

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = mpsc::channel();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (es,er) = mpsc::channel();
	let (ks,kr) = mpsc::channel();

	let mut kifuwriter = MockSfenKifuWriter::new(ks);

	let _ = thread::spawn(move || {
		let player1 = MockPlayer::new(pms1,pns1,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										}),
										Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),None))
										}),
										Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(7,8),KomaDstToPosition(7,7,false)),None))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Lose => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let player2 = MockPlayer::new(pms2,pns2,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(9,3),KomaDstToPosition(9,4,false)),None))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Win => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let (is,_) = mpsc::channel();

		let info_sender = MockInfoSender::new(is);

		let mut engine = SelfMatchEngine::new();

		let input_read_handler = create_input_read_handler(&engine.system_event_queue);

		let _ = engine.start(|self_match_event_dispatcher| {
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameStart, move |_,e| {
					match e {
						&SelfMatchEvent::GameStart(n,t,_) => {
							if t == Teban::Sente && n == 1 {
								let _ = hes.send(Ok(EventState::GameStart));
							} else {
								let _ = hes.send(Err(String::from("GameStart event is invalid.")));
							}
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Moved, move |_,e| {
					match e {
						&SelfMatchEvent::Moved(_,_) => {
							let _ = hes.send(Ok(EventState::Moved));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameEnd, move |_,e| {
					match e {
						&SelfMatchEvent::GameEnd(SelfMatchGameEndState::Foul(t,FoulKind::InvalidMove)) => {
							if t == Teban::Sente {
								let _ = hes.send(Ok(EventState::GameEnd));
							} else {
								let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							}
						},
						_ => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
						}
					}
					Ok(())
				});
			let hes = es.clone();
			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Abort, move |_,e| {
					match e {
						&SelfMatchEvent::Abort => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			},
			|| false,
			None,
			Some(Box::new(move |sfen,mvs| kifuwriter.write(sfen,mvs))),
			input_reader, input_read_handler,
			player1,player2,
			create_options(), create_options(),
			info_sender,
			UsiGoTimeLimit::None,
			None,Some(1),
			logger, |h,e| {
				if let Some(h) = h {
					let _ = h.lock().map(|h| h.call(e));
				}
			}
		);

		let _ = ts.send(());
	});

	startup(&pmr);

	gamestart_process(&pmr);

	let res = er.recv_timeout(Duration::from_millis(300)).expect("attempt to receive EventState::GameStart timed out.");

	assert_eq!(res,Ok(EventState::GameStart));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::GameEnd timed out.");

	assert_eq!(res,Ok(EventState::GameEnd));

	let res = kr.recv_timeout(Duration::from_millis(60)).expect("attempt to receive kifu string.");

	assert_eq!(&*res,"startpos moves 1g1f 9c9d 7h7g");

	let res = pmr[0].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let res = pmr[1].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let _ = tr.recv_timeout(Duration::from_millis(180)).expect("attempt to receive on quited timed out.");

	let _ = s.send(String::from(""));
}
#[test]
fn test_invalidmove_by_no_responded_oute_1times() {
	let (pms1,pmr1) = mpsc::channel();
	let (pns1,_) = mpsc::channel();
	let (ts,tr) = mpsc::channel();

	let (pms2,pmr2) = mpsc::channel();
	let (pns2,_) = mpsc::channel();

	let pmr = [pmr1,pmr2];

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = mpsc::channel();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (es,er) = mpsc::channel();
	let (ks,kr) = mpsc::channel();

	let mut kifuwriter = MockSfenKifuWriter::new(ks);

	let _ = thread::spawn(move || {
		let player1 = MockPlayer::new(pms1,pns1,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(8,8),KomaDstToPosition(3,3,false)),None))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Win => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let player2 = MockPlayer::new(pms2,pns2,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(6,1),KomaDstToPosition(6,2,false)),None))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Lose => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let (is,_) = mpsc::channel();

		let info_sender = MockInfoSender::new(is);

		let mut engine = SelfMatchEngine::new();

		let input_read_handler = create_input_read_handler(&engine.system_event_queue);

		let _ = engine.start(|self_match_event_dispatcher| {
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameStart, move |_,e| {
					match e {
						&SelfMatchEvent::GameStart(n,t,_) => {
							if t == Teban::Sente && n == 1 {
								let _ = hes.send(Ok(EventState::GameStart));
							} else {
								let _ = hes.send(Err(String::from("GameStart event is invalid.")));
							}
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Moved, move |_,e| {
					match e {
						&SelfMatchEvent::Moved(_,_) => {
							let _ = hes.send(Ok(EventState::Moved));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameEnd, move |_,e| {
					match e {
						&SelfMatchEvent::GameEnd(SelfMatchGameEndState::Foul(t,FoulKind::NotRespondedOute)) => {
							if t == Teban::Gote {
								let _ = hes.send(Ok(EventState::GameEnd));
							} else {
								let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							}
						},
						_ => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
						}
					}
					Ok(())
				});
			let hes = es.clone();
			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Abort, move |_,e| {
					match e {
						&SelfMatchEvent::Abort => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			},
			|| false,
			Some(Box::new(|| String::from("startpos moves 7g7f 3c3d"))),
			Some(Box::new(move |sfen,mvs| kifuwriter.write(sfen,mvs))),
			input_reader, input_read_handler,
			player1,player2,
			create_options(), create_options(),
			info_sender,
			UsiGoTimeLimit::None,
			None,Some(1),
			logger, |h,e| {
				if let Some(h) = h {
					let _ = h.lock().map(|h| h.call(e));
				}
			}
		);

		let _ = ts.send(());
	});

	startup(&pmr);

	gamestart_process(&pmr);

	let res = er.recv_timeout(Duration::from_millis(300)).expect("attempt to receive EventState::GameStart timed out.");

	assert_eq!(res,Ok(EventState::GameStart));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::GameEnd timed out.");

	assert_eq!(res,Ok(EventState::GameEnd));

	let res = kr.recv_timeout(Duration::from_millis(60)).expect("attempt to receive kifu string.");

	assert_eq!(&*res,"startpos moves 7g7f 3c3d 8h3c 6a6b");

	let res = pmr[0].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let res = pmr[1].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let _ = tr.recv_timeout(Duration::from_millis(180)).expect("attempt to receive on quited timed out.");

	let _ = s.send(String::from(""));
}
#[test]
fn test_invalidmove_by_suicide_1times() {
	let (pms1,pmr1) = mpsc::channel();
	let (pns1,_) = mpsc::channel();
	let (ts,tr) = mpsc::channel();

	let (pms2,pmr2) = mpsc::channel();
	let (pns2,_) = mpsc::channel();

	let pmr = [pmr1,pmr2];

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = mpsc::channel();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (es,er) = mpsc::channel();
	let (ks,kr) = mpsc::channel();

	let mut kifuwriter = MockSfenKifuWriter::new(ks);

	let _ = thread::spawn(move || {
		let player1 = MockPlayer::new(pms1,pns1,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(5,8),KomaDstToPosition(5,9,false)),None))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Lose => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let player2 = MockPlayer::new(pms2,pns2,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Win => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let (is,_) = mpsc::channel();

		let info_sender = MockInfoSender::new(is);

		let mut engine = SelfMatchEngine::new();

		let input_read_handler = create_input_read_handler(&engine.system_event_queue);

		let _ = engine.start(|self_match_event_dispatcher| {
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameStart, move |_,e| {
					match e {
						&SelfMatchEvent::GameStart(n,t,_) => {
							if t == Teban::Sente && n == 1 {
								let _ = hes.send(Ok(EventState::GameStart));
							} else {
								let _ = hes.send(Err(String::from("GameStart event is invalid.")));
							}
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Moved, move |_,e| {
					match e {
						&SelfMatchEvent::Moved(_,_) => {
							let _ = hes.send(Ok(EventState::Moved));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameEnd, move |_,e| {
					match e {
						&SelfMatchEvent::GameEnd(SelfMatchGameEndState::Foul(t,FoulKind::Suicide)) => {
							if t == Teban::Sente {
								let _ = hes.send(Ok(EventState::GameEnd));
							} else {
								let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							}
						},
						_ => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
						}
					}
					Ok(())
				});
			let hes = es.clone();
			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Abort, move |_,e| {
					match e {
						&SelfMatchEvent::Abort => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			},
			|| false,
			Some(Box::new(|| String::from("startpos moves 5i5h 3c3d 7g7f 2b7g"))),
			Some(Box::new(move |sfen,mvs| kifuwriter.write(sfen,mvs))),
			input_reader, input_read_handler,
			player1,player2,
			create_options(), create_options(),
			info_sender,
			UsiGoTimeLimit::None,
			None,Some(1),
			logger, |h,e| {
				if let Some(h) = h {
					let _ = h.lock().map(|h| h.call(e));
				}
			}
		);

		let _ = ts.send(());
	});

	startup(&pmr);

	gamestart_process(&pmr);

	let res = er.recv_timeout(Duration::from_millis(300)).expect("attempt to receive EventState::GameStart timed out.");

	assert_eq!(res,Ok(EventState::GameStart));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::GameEnd timed out.");

	assert_eq!(res,Ok(EventState::GameEnd));

	let res = kr.recv_timeout(Duration::from_millis(60)).expect("attempt to receive kifu string.");

	assert_eq!(&*res,"startpos moves 5i5h 3c3d 7g7f 2b7g 5h5i");

	let res = pmr[0].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let res = pmr[1].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let _ = tr.recv_timeout(Duration::from_millis(180)).expect("attempt to receive on quited timed out.");

	let _ = s.send(String::from(""));
}
#[test]
fn test_win_move_1times() {
	let (pms1,pmr1) = mpsc::channel();
	let (pns1,_) = mpsc::channel();
	let (ts,tr) = mpsc::channel();

	let (pms2,pmr2) = mpsc::channel();
	let (pns2,_) = mpsc::channel();

	let pmr = [pmr1,pmr2];

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = mpsc::channel();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (es,er) = mpsc::channel();
	let (ks,kr) = mpsc::channel();

	let mut kifuwriter = MockSfenKifuWriter::new(ks);

	let _ = thread::spawn(move || {
		let player1 = MockPlayer::new(pms1,pns1,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(5,2),KomaDstToPosition(5,1,false)),None))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Win => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let player2 = MockPlayer::new(pms2,pns2,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Lose => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let (is,_) = mpsc::channel();

		let info_sender = MockInfoSender::new(is);

		let mut engine = SelfMatchEngine::new();

		let input_read_handler = create_input_read_handler(&engine.system_event_queue);

		let _ = engine.start(|self_match_event_dispatcher| {
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameStart, move |_,e| {
					match e {
						&SelfMatchEvent::GameStart(n,t,_) => {
							if t == Teban::Sente && n == 1 {
								let _ = hes.send(Ok(EventState::GameStart));
							} else {
								let _ = hes.send(Err(String::from("GameStart event is invalid.")));
							}
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Moved, move |_,e| {
					match e {
						&SelfMatchEvent::Moved(_,_) => {
							let _ = hes.send(Ok(EventState::Moved));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameEnd, move |_,e| {
					match e {
						&SelfMatchEvent::GameEnd(SelfMatchGameEndState::Win(t)) => {
							if t == Teban::Sente {
								let _ = hes.send(Ok(EventState::GameEnd));
							} else {
								let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							}
						},
						_ => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
						}
					}
					Ok(())
				});
			let hes = es.clone();
			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Abort, move |_,e| {
					match e {
						&SelfMatchEvent::Abort => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			},
			|| false,
			Some(Box::new(|| String::from("sfen lnsgkgsnl/1r2G2b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGK1SNL b - 1"))),
			Some(Box::new(move |sfen,mvs| kifuwriter.write(sfen,mvs))),
			input_reader, input_read_handler,
			player1,player2,
			create_options(), create_options(),
			info_sender,
			UsiGoTimeLimit::None,
			None,Some(1),
			logger, |h,e| {
				if let Some(h) = h {
					let _ = h.lock().map(|h| h.call(e));
				}
			}
		);

		let _ = ts.send(());
	});

	startup(&pmr);

	gamestart_process(&pmr);

	let res = er.recv_timeout(Duration::from_millis(300)).expect("attempt to receive EventState::GameStart timed out.");

	assert_eq!(res,Ok(EventState::GameStart));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::GameEnd timed out.");

	assert_eq!(res,Ok(EventState::GameEnd));

	let res = kr.recv_timeout(Duration::from_millis(60)).expect("attempt to receive kifu string.");

	assert_eq!(&*res,"sfen lnsgkgsnl/1r2G2b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGK1SNL b - 1 moves 5b5a");

	let res = pmr[0].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let res = pmr[1].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let _ = tr.recv_timeout(Duration::from_millis(180)).expect("attempt to receive on quited timed out.");

	let _ = s.send(String::from(""));
}
#[test]
fn test_win_invalidmove_put_fu_and_mate_1times() {
	let (pms1,pmr1) = mpsc::channel();
	let (pns1,_) = mpsc::channel();
	let (ts,tr) = mpsc::channel();

	let (pms2,pmr2) = mpsc::channel();
	let (pns2,_) = mpsc::channel();

	let pmr = [pmr1,pmr2];

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = mpsc::channel();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (es,er) = mpsc::channel();
	let (ks,kr) = mpsc::channel();

	let mut kifuwriter = MockSfenKifuWriter::new(ks);

	let _ = thread::spawn(move || {
		let player1 = MockPlayer::new(pms1,pns1,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::Put(MochigomaKind::Fu,KomaDstPutPosition(5,2)),None))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Lose => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let player2 = MockPlayer::new(pms2,pns2,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Win => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let (is,_) = mpsc::channel();

		let info_sender = MockInfoSender::new(is);

		let mut engine = SelfMatchEngine::new();

		let input_read_handler = create_input_read_handler(&engine.system_event_queue);

		let _ = engine.start(|self_match_event_dispatcher| {
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameStart, move |_,e| {
					match e {
						&SelfMatchEvent::GameStart(n,t,_) => {
							if t == Teban::Sente && n == 1 {
								let _ = hes.send(Ok(EventState::GameStart));
							} else {
								let _ = hes.send(Err(String::from("GameStart event is invalid.")));
							}
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Moved, move |_,e| {
					match e {
						&SelfMatchEvent::Moved(_,_) => {
							let _ = hes.send(Ok(EventState::Moved));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameEnd, move |_,e| {
					match e {
						&SelfMatchEvent::GameEnd(SelfMatchGameEndState::Foul(t,FoulKind::PutFuAndMate)) => {
							if t == Teban::Sente {
								let _ = hes.send(Ok(EventState::GameEnd));
							} else {
								let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							}
						},
						_ => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
						}
					}
					Ok(())
				});
			let hes = es.clone();
			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Abort, move |_,e| {
					match e {
						&SelfMatchEvent::Abort => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			},
			|| false,
			Some(Box::new(|| String::from("sfen 3nkn3/3s1s3/9/4L4/9/9/1PPPPPPPP/1B5R1/LNSGK1SNL b Prb2g2sl9p 1"))),
			Some(Box::new(move |sfen,mvs| kifuwriter.write(sfen,mvs))),
			input_reader, input_read_handler,
			player1,player2,
			create_options(), create_options(),
			info_sender,
			UsiGoTimeLimit::None,
			None,Some(1),
			logger, |h,e| {
				if let Some(h) = h {
					let _ = h.lock().map(|h| h.call(e));
				}
			}
		);

		let _ = ts.send(());
	});

	startup(&pmr);

	gamestart_process(&pmr);

	let res = er.recv_timeout(Duration::from_millis(300)).expect("attempt to receive EventState::GameStart timed out.");

	assert_eq!(res,Ok(EventState::GameStart));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::GameEnd timed out.");

	assert_eq!(res,Ok(EventState::GameEnd));

	let res = kr.recv_timeout(Duration::from_millis(60)).expect("attempt to receive kifu string.");

	assert_eq!(&*res,"sfen 3nkn3/3s1s3/9/4L4/9/9/1PPPPPPPP/1B5R1/LNSGK1SNL b Prb2g2sl9p 1 moves P*5b");

	let res = pmr[0].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let res = pmr[1].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let _ = tr.recv_timeout(Duration::from_millis(180)).expect("attempt to receive on quited timed out.");

	let _ = s.send(String::from(""));
}
#[test]
fn test_win_invalidmove_sennichite_by_oute_once_move_1times() {
	let (pms1,pmr1) = mpsc::channel();
	let (pns1,_) = mpsc::channel();
	let (ts,tr) = mpsc::channel();

	let (pms2,pmr2) = mpsc::channel();
	let (pns2,_) = mpsc::channel();

	let pmr = [pmr1,pmr2];

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = mpsc::channel();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (es,er) = mpsc::channel();
	let (ks,kr) = mpsc::channel();

	let mut kifuwriter = MockSfenKifuWriter::new(ks);

	let _ = thread::spawn(move || {
		let player1 = MockPlayer::new(pms1,pns1,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(4,3),KomaDstToPosition(5,3,false)),None))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Lose => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let player2 = MockPlayer::new(pms2,pns2,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Win => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let (is,_) = mpsc::channel();

		let info_sender = MockInfoSender::new(is);

		let mut engine = SelfMatchEngine::new();

		let input_read_handler = create_input_read_handler(&engine.system_event_queue);

		let _ = engine.start(|self_match_event_dispatcher| {
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameStart, move |_,e| {
					match e {
						&SelfMatchEvent::GameStart(n,t,_) => {
							if t == Teban::Sente && n == 1 {
								let _ = hes.send(Ok(EventState::GameStart));
							} else {
								let _ = hes.send(Err(String::from("GameStart event is invalid.")));
							}
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Moved, move |_,e| {
					match e {
						&SelfMatchEvent::Moved(_,_) => {
							let _ = hes.send(Ok(EventState::Moved));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameEnd, move |_,e| {
					match e {
						&SelfMatchEvent::GameEnd(SelfMatchGameEndState::Foul(t,FoulKind::SennichiteOu)) => {
							if t == Teban::Sente {
								let _ = hes.send(Ok(EventState::GameEnd));
							} else {
								let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							}
						},
						_ => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
						}
					}
					Ok(())
				});
			let hes = es.clone();
			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Abort, move |_,e| {
					match e {
						&SelfMatchEvent::Abort => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			},
			|| false,
			Some(Box::new(|| String::from("sfen 4k4/9/5R3/9/9/9/PPPPPPPPP/1B52/LNSGKGSNL b rb2g2s2n2l9p 1 moves 4c5c 5a4a 5c4c 4a5a"))),
			Some(Box::new(move |sfen,mvs| kifuwriter.write(sfen,mvs))),
			input_reader, input_read_handler,
			player1,player2,
			create_options(), create_options(),
			info_sender,
			UsiGoTimeLimit::None,
			None,Some(1),
			logger, |h,e| {
				if let Some(h) = h {
					let _ = h.lock().map(|h| h.call(e));
				}
			}
		);

		let _ = ts.send(());
	});

	startup(&pmr);

	gamestart_process(&pmr);

	let res = er.recv_timeout(Duration::from_millis(300)).expect("attempt to receive EventState::GameStart timed out.");

	assert_eq!(res,Ok(EventState::GameStart));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::GameEnd timed out.");

	assert_eq!(res,Ok(EventState::GameEnd));

	let res = kr.recv_timeout(Duration::from_millis(60)).expect("attempt to receive kifu string.");

	assert_eq!(&*res,"sfen 4k4/9/5R3/9/9/9/PPPPPPPPP/1B52/LNSGKGSNL b rb2g2s2n2l9p 1 moves 4c5c 5a4a 5c4c 4a5a 4c5c");

	let res = pmr[0].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let res = pmr[1].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let _ = tr.recv_timeout(Duration::from_millis(180)).expect("attempt to receive on quited timed out.");

	let _ = s.send(String::from(""));
}
#[test]
fn test_win_validmove_not_sennichite_by_oute_1times() {
	let (pms1,pmr1) = mpsc::channel();
	let (pns1,_) = mpsc::channel();
	let (ts,tr) = mpsc::channel();

	let (pms2,pmr2) = mpsc::channel();
	let (pns2,_) = mpsc::channel();

	let pmr = [pmr1,pmr2];

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = mpsc::channel();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (es,er) = mpsc::channel();
	let (ks,kr) = mpsc::channel();

	let mut kifuwriter = MockSfenKifuWriter::new(ks);

	let _ = thread::spawn(move || {
		let player1 = MockPlayer::new(pms1,pns1,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(6,3),KomaDstToPosition(5,3,false)),None))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Win => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let player2 = MockPlayer::new(pms2,pns2,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Resign)
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Lose => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let (is,_) = mpsc::channel();

		let info_sender = MockInfoSender::new(is);

		let mut engine = SelfMatchEngine::new();

		let input_read_handler = create_input_read_handler(&engine.system_event_queue);

		let _ = engine.start(|self_match_event_dispatcher| {
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameStart, move |_,e| {
					match e {
						&SelfMatchEvent::GameStart(n,t,_) => {
							if t == Teban::Sente && n == 1 {
								let _ = hes.send(Ok(EventState::GameStart));
							} else {
								let _ = hes.send(Err(String::from("GameStart event is invalid.")));
							}
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Moved, move |_,e| {
					match e {
						&SelfMatchEvent::Moved(_,_) => {
							let _ = hes.send(Ok(EventState::Moved));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameEnd, move |_,e| {
					match e {
						&SelfMatchEvent::GameEnd(SelfMatchGameEndState::Resign(t)) => {
							if t == Teban::Gote {
								let _ = hes.send(Ok(EventState::GameEnd));
							} else {
								let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							}
						},
						_ => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
						}
					}
					Ok(())
				});
			let hes = es.clone();
			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Abort, move |_,e| {
					match e {
						&SelfMatchEvent::Abort => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			},
			|| false,
			Some(Box::new(|| String::from("sfen 4k4/9/5R3/9/9/9/PPPPPPPPP/1B52/LNSGKGSNL b rb2g2s2n2l9p 1 moves 4c5c 5a4a 5c6c 4a5a"))),
			Some(Box::new(move |sfen,mvs| kifuwriter.write(sfen,mvs))),
			input_reader, input_read_handler,
			player1,player2,
			create_options(), create_options(),
			info_sender,
			UsiGoTimeLimit::None,
			None,Some(1),
			logger, |h,e| {
				if let Some(h) = h {
					let _ = h.lock().map(|h| h.call(e));
				}
			}
		);

		let _ = ts.send(());
	});

	startup(&pmr);

	gamestart_process(&pmr);

	let res = er.recv_timeout(Duration::from_millis(300)).expect("attempt to receive EventState::GameStart timed out.");

	assert_eq!(res,Ok(EventState::GameStart));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::GameEnd timed out.");

	assert_eq!(res,Ok(EventState::GameEnd));

	let res = kr.recv_timeout(Duration::from_millis(60)).expect("attempt to receive kifu string.");

	assert_eq!(&*res,"sfen 4k4/9/5R3/9/9/9/PPPPPPPPP/1B52/LNSGKGSNL b rb2g2s2n2l9p 1 moves 4c5c 5a4a 5c6c 4a5a 6c5c");

	let res = pmr[0].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let res = pmr[1].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let _ = tr.recv_timeout(Duration::from_millis(180)).expect("attempt to receive on quited timed out.");

	let _ = s.send(String::from(""));
}
#[test]
fn test_win_invalid_move_sennichite_once_move_1times() {
	let (pms1,pmr1) = mpsc::channel();
	let (pns1,_) = mpsc::channel();
	let (ts,tr) = mpsc::channel();

	let (pms2,pmr2) = mpsc::channel();
	let (pns2,_) = mpsc::channel();

	let pmr = [pmr1,pmr2];

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = mpsc::channel();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (es,er) = mpsc::channel();
	let (ks,kr) = mpsc::channel();

	let mut kifuwriter = MockSfenKifuWriter::new(ks);

	let _ = thread::spawn(move || {
		let player1 = MockPlayer::new(pms1,pns1,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(4,9),KomaDstToPosition(4,8,false)),None))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Lose => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let player2 = MockPlayer::new(pms2,pns2,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Win => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let (is,_) = mpsc::channel();

		let info_sender = MockInfoSender::new(is);

		let mut engine = SelfMatchEngine::new();

		let input_read_handler = create_input_read_handler(&engine.system_event_queue);

		let _ = engine.start(|self_match_event_dispatcher| {
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameStart, move |_,e| {
					match e {
						&SelfMatchEvent::GameStart(n,t,_) => {
							if t == Teban::Sente && n == 1 {
								let _ = hes.send(Ok(EventState::GameStart));
							} else {
								let _ = hes.send(Err(String::from("GameStart event is invalid.")));
							}
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Moved, move |_,e| {
					match e {
						&SelfMatchEvent::Moved(_,_) => {
							let _ = hes.send(Ok(EventState::Moved));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameEnd, move |_,e| {
					match e {
						&SelfMatchEvent::GameEnd(SelfMatchGameEndState::Foul(t,FoulKind::Sennichite)) => {
							if t == Teban::Sente {
								let _ = hes.send(Ok(EventState::GameEnd));
							} else {
								let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							}
						},
						_ => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
						}
					}
					Ok(())
				});
			let hes = es.clone();
			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Abort, move |_,e| {
					match e {
						&SelfMatchEvent::Abort => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			},
			|| false,
			Some(Box::new(|| String::from("startpos moves 4i4h 6a6b 4h4i 6b6a 4i4h 6a6b 4h4i 6b6a 4i4h 6a6b 4h4i 6b6a"))),
			Some(Box::new(move |sfen,mvs| kifuwriter.write(sfen,mvs))),
			input_reader, input_read_handler,
			player1,player2,
			create_options(), create_options(),
			info_sender,
			UsiGoTimeLimit::None,
			None,Some(1),
			logger, |h,e| {
				if let Some(h) = h {
					let _ = h.lock().map(|h| h.call(e));
				}
			}
		);

		let _ = ts.send(());
	});

	startup(&pmr);

	gamestart_process(&pmr);

	let res = er.recv_timeout(Duration::from_millis(300)).expect("attempt to receive EventState::GameStart timed out.");

	assert_eq!(res,Ok(EventState::GameStart));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::GameEnd timed out.");

	assert_eq!(res,Ok(EventState::GameEnd));

	let res = kr.recv_timeout(Duration::from_millis(60)).expect("attempt to receive kifu string.");

	assert_eq!(&*res,"startpos moves 4i4h 6a6b 4h4i 6b6a 4i4h 6a6b 4h4i 6b6a 4i4h 6a6b 4h4i 6b6a 4i4h");

	let res = pmr[0].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let res = pmr[1].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let _ = tr.recv_timeout(Duration::from_millis(180)).expect("attempt to receive on quited timed out.");

	let _ = s.send(String::from(""));
}
#[test]
fn test_win_invalid_move_sennichite_by_oute_once_move_1times_with_empty_kyokumen_map() {
	let (pms1,pmr1) = mpsc::channel();
	let (pns1,_) = mpsc::channel();
	let (ts,tr) = mpsc::channel();

	let (pms2,pmr2) = mpsc::channel();
	let (pns2,_) = mpsc::channel();

	let pmr = [pmr1,pmr2];

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = mpsc::channel();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (es,er) = mpsc::channel();
	let (ks,kr) = mpsc::channel();

	let mut kifuwriter = MockSfenKifuWriter::new(ks);

	let _ = thread::spawn(move || {
		let player1 = MockPlayer::new(pms1,pns1,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										}),
										Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										}),
										Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(4,3),KomaDstToPosition(5,3,false)),None))
										}),
										Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(5,3),KomaDstToPosition(4,3,false)),None))
										}),
										Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(4,3),KomaDstToPosition(5,3,false)),None))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Lose => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let player2 = MockPlayer::new(pms2,pns2,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										}),
										Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(5,1),KomaDstToPosition(4,1,false)),None))
										}),
										Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(4,1),KomaDstToPosition(5,1,false)),None))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Win => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let (is,_) = mpsc::channel();

		let info_sender = MockInfoSender::new(is);

		let mut engine = SelfMatchEngine::new();

		let input_read_handler = create_input_read_handler(&engine.system_event_queue);

		let _ = engine.start(|self_match_event_dispatcher| {
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameStart, move |_,e| {
					match e {
						&SelfMatchEvent::GameStart(n,t,_) => {
							if t == Teban::Sente && n == 1 {
								let _ = hes.send(Ok(EventState::GameStart));
							} else {
								let _ = hes.send(Err(String::from("GameStart event is invalid.")));
							}
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Moved, move |_,e| {
					match e {
						&SelfMatchEvent::Moved(_,_) => {
							let _ = hes.send(Ok(EventState::Moved));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameEnd, move |_,e| {
					match e {
						&SelfMatchEvent::GameEnd(SelfMatchGameEndState::Foul(t,FoulKind::SennichiteOu)) => {
							if t == Teban::Sente {
								let _ = hes.send(Ok(EventState::GameEnd));
							} else {
								let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							}
						},
						_ => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
						}
					}
					Ok(())
				});
			let hes = es.clone();
			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Abort, move |_,e| {
					match e {
						&SelfMatchEvent::Abort => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			},
			|| false,
			Some(Box::new(|| String::from("sfen 4k4/9/5R3/9/9/9/PPPPPPPPP/1B52/LNSGKGSNL b rb2g2s2n2l9p 1"))),
			Some(Box::new(move |sfen,mvs| kifuwriter.write(sfen,mvs))),
			input_reader, input_read_handler,
			player1,player2,
			create_options(), create_options(),
			info_sender,
			UsiGoTimeLimit::None,
			None,Some(1),
			logger, |h,e| {
				if let Some(h) = h {
					let _ = h.lock().map(|h| h.call(e));
				}
			}
		);

		let _ = ts.send(());
	});

	startup(&pmr);

	gamestart_process(&pmr);

	let res = er.recv_timeout(Duration::from_millis(300)).expect("attempt to receive EventState::GameStart timed out.");

	assert_eq!(res,Ok(EventState::GameStart));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::GameEnd timed out.");

	assert_eq!(res,Ok(EventState::GameEnd));

	let res = kr.recv_timeout(Duration::from_millis(60)).expect("attempt to receive kifu string.");

	assert_eq!(&*res,"sfen 4k4/9/5R3/9/9/9/PPPPPPPPP/1B52/LNSGKGSNL b rb2g2s2n2l9p 1 moves 4c5c 5a4a 5c4c 4a5a 4c5c");

	let res = pmr[0].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let res = pmr[1].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let _ = tr.recv_timeout(Duration::from_millis(180)).expect("attempt to receive on quited timed out.");

	let _ = s.send(String::from(""));
}
#[test]
fn test_win_invalid_move_sennichite_1times_with_empty_kyokumen_map() {
	let (pms1,pmr1) = mpsc::channel();
	let (pns1,_) = mpsc::channel();
	let (ts,tr) = mpsc::channel();

	let (pms2,pmr2) = mpsc::channel();
	let (pns2,_) = mpsc::channel();

	let pmr = [pmr1,pmr2];

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = mpsc::channel();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (es,er) = mpsc::channel();
	let (ks,kr) = mpsc::channel();

	let mut kifuwriter = MockSfenKifuWriter::new(ks);

	let _ = thread::spawn(move || {
		let player1 = MockPlayer::new(pms1,pns1,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										}),
										Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										}),
										Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										}),
										Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										}),
										Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										}),
										Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										}),
										Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(4,9),KomaDstToPosition(4,8,false)),None))
										}),
										Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(4,8),KomaDstToPosition(4,9,false)),None))
										}),
										Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(4,9),KomaDstToPosition(4,8,false)),None))
										}),
										Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(4,8),KomaDstToPosition(4,9,false)),None))
										}),
										Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(4,9),KomaDstToPosition(4,8,false)),None))
										}),
										Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(4,8),KomaDstToPosition(4,9,false)),None))
										}),
										Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(4,9),KomaDstToPosition(4,8,false)),None))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Lose => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let player2 = MockPlayer::new(pms2,pns2,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										}),
										Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										}),
										Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										}),
										Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										}),
										Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										}),
										Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(6,1),KomaDstToPosition(6,2,false)),None))
										}),
										Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(6,2),KomaDstToPosition(6,1,false)),None))
										}),
										Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(6,1),KomaDstToPosition(6,2,false)),None))
										}),
										Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(6,2),KomaDstToPosition(6,1,false)),None))
										}),
										Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(6,1),KomaDstToPosition(6,2,false)),None))
										}),
										Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(6,2),KomaDstToPosition(6,1,false)),None))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Win => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let (is,_) = mpsc::channel();

		let info_sender = MockInfoSender::new(is);

		let mut engine = SelfMatchEngine::new();

		let input_read_handler = create_input_read_handler(&engine.system_event_queue);

		let _ = engine.start(|self_match_event_dispatcher| {
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameStart, move |_,e| {
					match e {
						&SelfMatchEvent::GameStart(n,t,_) => {
							if t == Teban::Sente && n == 1 {
								let _ = hes.send(Ok(EventState::GameStart));
							} else {
								let _ = hes.send(Err(String::from("GameStart event is invalid.")));
							}
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Moved, move |_,e| {
					match e {
						&SelfMatchEvent::Moved(_,_) => {
							let _ = hes.send(Ok(EventState::Moved));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameEnd, move |_,e| {
					match e {
						&SelfMatchEvent::GameEnd(SelfMatchGameEndState::Foul(t,FoulKind::Sennichite)) => {
							if t == Teban::Sente {
								let _ = hes.send(Ok(EventState::GameEnd));
							} else {
								let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							}
						},
						_ => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
						}
					}
					Ok(())
				});
			let hes = es.clone();
			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Abort, move |_,e| {
					match e {
						&SelfMatchEvent::Abort => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			},
			|| false,
			Some(Box::new(|| String::from("startpos"))),
			Some(Box::new(move |sfen,mvs| kifuwriter.write(sfen,mvs))),
			input_reader, input_read_handler,
			player1,player2,
			create_options(), create_options(),
			info_sender,
			UsiGoTimeLimit::None,
			None,Some(1),
			logger, |h,e| {
				if let Some(h) = h {
					let _ = h.lock().map(|h| h.call(e));
				}
			}
		);

		let _ = ts.send(());
	});

	startup(&pmr);

	gamestart_process(&pmr);

	let res = er.recv_timeout(Duration::from_millis(300)).expect("attempt to receive EventState::GameStart timed out.");

	assert_eq!(res,Ok(EventState::GameStart));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::GameEnd timed out.");

	assert_eq!(res,Ok(EventState::GameEnd));

	let res = kr.recv_timeout(Duration::from_millis(60)).expect("attempt to receive kifu string.");

	assert_eq!(&*res,"startpos moves 4i4h 6a6b 4h4i 6b6a 4i4h 6a6b 4h4i 6b6a 4i4h 6a6b 4h4i 6b6a 4i4h");

	let res = pmr[0].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let res = pmr[1].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let _ = tr.recv_timeout(Duration::from_millis(180)).expect("attempt to receive on quited timed out.");

	let _ = s.send(String::from(""));
}
#[test]
fn test_game_time_limit_1times() {
	let (pms1,pmr1) = mpsc::channel();
	let (pns1,_) = mpsc::channel();
	let (ts,tr) = mpsc::channel();

	let (pms2,pmr2) = mpsc::channel();
	let (pns2,_) = mpsc::channel();

	let pmr = [pmr1,pmr2];

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = mpsc::channel();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (es,er) = mpsc::channel();
	let (ks,kr) = mpsc::channel();

	let mut kifuwriter = MockSfenKifuWriter::new(ks);

	let _ = thread::spawn(move || {
		let player1 = MockPlayer::new(pms1,pns1,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										}),
										Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),None))
										}),
										Box::new(|player,_,_,_,_,mut handle_events| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											while !player.stop {
												thread::sleep(Duration::from_millis(10));
												handle_events(player)?;
											}
											Ok(BestMove::Resign)
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Lose => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let player2 = MockPlayer::new(pms2,pns2,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(9,3),KomaDstToPosition(9,4,false)),None))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Win => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let (is,_) = mpsc::channel();

		let info_sender = MockInfoSender::new(is);

		let mut engine = SelfMatchEngine::new();

		let input_read_handler = create_input_read_handler(&engine.system_event_queue);

		let _ = engine.start(|self_match_event_dispatcher| {
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameStart, move |_,e| {
					match e {
						&SelfMatchEvent::GameStart(n,t,_) => {
							if t == Teban::Sente && n == 1 {
								let _ = hes.send(Ok(EventState::GameStart));
							} else {
								let _ = hes.send(Err(String::from("GameStart event is invalid.")));
							}
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Moved, move |_,e| {
					match e {
						&SelfMatchEvent::Moved(_,_) => {
							let _ = hes.send(Ok(EventState::Moved));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameEnd, move |_,e| {
					match e {
						&SelfMatchEvent::GameEnd(SelfMatchGameEndState::Timeover(t)) => {
							if t == Teban::Sente {
								let _ = hes.send(Ok(EventState::GameEnd));
							} else {
								let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							}
						},
						_ => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
						}
					}
					Ok(())
				});
			let hes = es.clone();
			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Abort, move |_,e| {
					match e {
						&SelfMatchEvent::Abort => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			},
			|| false,
			None,
			Some(Box::new(move |sfen,mvs| kifuwriter.write(sfen,mvs))),
			input_reader, input_read_handler,
			player1,player2,
			create_options(), create_options(),
			info_sender,
			UsiGoTimeLimit::Limit(Some((1000,1000)),None),
			None,Some(1),
			logger, |h,e| {
				if let Some(h) = h {
					let _ = h.lock().map(|h| h.call(e));
				}
			}
		);

		let _ = ts.send(());
	});

	startup(&pmr);

	gamestart_process(&pmr);

	let res = er.recv_timeout(Duration::from_millis(300)).expect("attempt to receive EventState::GameStart timed out.");

	assert_eq!(res,Ok(EventState::GameStart));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = pmr[0].recv_timeout(Duration::from_millis(1200)).expect("attempt to receive ActionKind::OnStop timed out.");

	assert_eq!(res,Ok(ActionKind::OnStop));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::GameEnd timed out.");

	assert_eq!(res,Ok(EventState::GameEnd));

	let res = kr.recv_timeout(Duration::from_millis(60)).expect("attempt to receive kifu string.");

	assert_eq!(&*res,"startpos moves 1g1f 9c9d");

	let res = pmr[0].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let res = pmr[1].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let _ = tr.recv_timeout(Duration::from_millis(180)).expect("attempt to receive on quited timed out.");

	let _ = s.send(String::from(""));
}
#[test]
fn test_game_time_limit_less_uptime_1times() {
	let (pms1,pmr1) = mpsc::channel();
	let (pns1,_) = mpsc::channel();
	let (ts,tr) = mpsc::channel();

	let (pms2,pmr2) = mpsc::channel();
	let (pns2,_) = mpsc::channel();

	let pmr = [pmr1,pmr2];

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = mpsc::channel();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (es,er) = mpsc::channel();

	let _ = thread::spawn(move || {
		let player1 = MockPlayer::new(pms1,pns1,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										}),
										Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),None))
										}),
										Box::new(|player,_,_,_,_,mut handle_events| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											while !player.stop {
												thread::sleep(Duration::from_millis(10));
												handle_events(player)?;
											}
											Ok(BestMove::Resign)
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Lose => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let player2 = MockPlayer::new(pms2,pns2,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(9,3),KomaDstToPosition(9,4,false)),None))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Win => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let (is,_) = mpsc::channel();

		let info_sender = MockInfoSender::new(is);

		let mut engine = SelfMatchEngine::new();

		let input_read_handler = create_input_read_handler(&engine.system_event_queue);

		let _ = engine.start(|self_match_event_dispatcher| {
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameStart, move |_,e| {
					match e {
						&SelfMatchEvent::GameStart(n,t,_) => {
							if t == Teban::Sente && n == 1 {
								let _ = hes.send(Ok(EventState::GameStart));
							} else {
								let _ = hes.send(Err(String::from("GameStart event is invalid.")));
							}
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Moved, move |_,e| {
					match e {
						&SelfMatchEvent::Moved(_,_) => {
							let _ = hes.send(Ok(EventState::Moved));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameEnd, move |_,e| {
					match e {
						&SelfMatchEvent::GameEnd(SelfMatchGameEndState::Timeover(t)) => {
							if t == Teban::Sente {
								let _ = hes.send(Ok(EventState::GameEnd));
							} else {
								let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							}
						},
						_ => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
						}
					}
					Ok(())
				});
			let hes = es.clone();
			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Abort, move |_,e| {
					match e {
						&SelfMatchEvent::Abort => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			},
			|| false,
			None,
			None, input_reader, input_read_handler,
			player1,player2,
			create_options(), create_options(),
			info_sender,
			UsiGoTimeLimit::Limit(Some((10000,10000)),None),
			Some(Duration::from_millis(1000)),None,
			logger, |h,e| {
				if let Some(h) = h {
					let _ = h.lock().map(|h| h.call(e));
				}
			}
		);

		let _ = ts.send(());
	});

	startup(&pmr);

	gamestart_process(&pmr);

	let res = er.recv_timeout(Duration::from_millis(300)).expect("attempt to receive EventState::GameStart timed out.");

	assert_eq!(res,Ok(EventState::GameStart));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = pmr[0].recv_timeout(Duration::from_millis(1200)).expect("attempt to receive ActionKind::OnStop timed out.");

	assert_eq!(res,Ok(ActionKind::OnStop));

	let res = pmr[0].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let res = pmr[1].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let _ = tr.recv_timeout(Duration::from_millis(180)).expect("attempt to receive on quited timed out.");

	let _ = s.send(String::from(""));
}
#[test]
fn test_game_uptime_less_game_time_limit_1times() {
	let (pms1,pmr1) = mpsc::channel();
	let (pns1,_) = mpsc::channel();
	let (ts,tr) = mpsc::channel();

	let (pms2,pmr2) = mpsc::channel();
	let (pns2,_) = mpsc::channel();

	let pmr = [pmr1,pmr2];

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = mpsc::channel();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (es,er) = mpsc::channel();

	let _ = thread::spawn(move || {
		let player1 = MockPlayer::new(pms1,pns1,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										}),
										Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),None))
										}),
										Box::new(|player,_,_,_,_,mut handle_events| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											while !player.stop {
												thread::sleep(Duration::from_millis(10));
												handle_events(player)?;
											}
											Ok(BestMove::Resign)
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Lose => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let player2 = MockPlayer::new(pms2,pns2,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(9,3),KomaDstToPosition(9,4,false)),None))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Win => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let (is,_) = mpsc::channel();

		let info_sender = MockInfoSender::new(is);

		let mut engine = SelfMatchEngine::new();

		let input_read_handler = create_input_read_handler(&engine.system_event_queue);

		let _ = engine.start(|self_match_event_dispatcher| {
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameStart, move |_,e| {
					match e {
						&SelfMatchEvent::GameStart(n,t,_) => {
							if t == Teban::Sente && n == 1 {
								let _ = hes.send(Ok(EventState::GameStart));
							} else {
								let _ = hes.send(Err(String::from("GameStart event is invalid.")));
							}
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Moved, move |_,e| {
					match e {
						&SelfMatchEvent::Moved(_,_) => {
							let _ = hes.send(Ok(EventState::Moved));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameEnd, move |_,e| {
					match e {
						&SelfMatchEvent::GameEnd(SelfMatchGameEndState::Timeover(t)) => {
							if t == Teban::Sente {
								let _ = hes.send(Ok(EventState::GameEnd));
							} else {
								let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							}
						},
						_ => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
						}
					}
					Ok(())
				});
			let hes = es.clone();
			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Abort, move |_,e| {
					match e {
						&SelfMatchEvent::Abort => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			},
			|| false,
			None,
			None, input_reader, input_read_handler,
			player1,player2,
			create_options(), create_options(),
			info_sender,
			UsiGoTimeLimit::Limit(Some((3000,3000)),None),
			Some(Duration::from_millis(1200)),None,
			logger, |h,e| {
				if let Some(h) = h {
					let _ = h.lock().map(|h| h.call(e));
				}
			}
		);

		let _ = ts.send(());
	});

	startup(&pmr);

	gamestart_process(&pmr);

	let res = er.recv_timeout(Duration::from_millis(300)).expect("attempt to receive EventState::GameStart timed out.");

	assert_eq!(res,Ok(EventState::GameStart));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = pmr[0].recv_timeout(Duration::from_millis(1200)).expect("attempt to receive ActionKind::OnStop timed out.");

	assert_eq!(res,Ok(ActionKind::OnStop));

	let res = pmr[0].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let res = pmr[1].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let _ = tr.recv_timeout(Duration::from_millis(180)).expect("attempt to receive on quited timed out.");

	let _ = s.send(String::from(""));
}
#[test]
fn test_ponderhit_1times() {
	let (pms1,pmr1) = mpsc::channel();
	let (pns1,_) = mpsc::channel();
	let (ts,tr) = mpsc::channel();

	let (pms2,pmr2) = mpsc::channel();
	let (pns2,_) = mpsc::channel();

	let pmr = [pmr1,pmr2];

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = mpsc::channel();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (es,er) = mpsc::channel();

	let _ = thread::spawn(move || {
		let player1 = MockPlayer::new(pms1,pns1,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										}),
										Box::new(|player,_,_,_,_,_,mvs| {
											if mvs == vec![Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),
															Move::To(KomaSrcPosition(9,3),KomaDstToPosition(9,4,false))] {
												let _ = player.sender.send(Ok(ActionKind::SetPosition));
											} else {
												let _ = player.sender.send(Err(String::from("mvs is invalid.")));
											}
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),
																Some(Move::To(KomaSrcPosition(9,3),KomaDstToPosition(9,4,false)))))
										}),
										Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Resign)
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Lose => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let player2 = MockPlayer::new(pms2,pns2,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(9,3),KomaDstToPosition(9,4,false)),None))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Win => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let (is,_) = mpsc::channel();

		let info_sender = MockInfoSender::new(is);

		let mut engine = SelfMatchEngine::new();

		let input_read_handler = create_input_read_handler(&engine.system_event_queue);

		let _ = engine.start(|self_match_event_dispatcher| {
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameStart, move |_,e| {
					match e {
						&SelfMatchEvent::GameStart(n,t,_) => {
							if t == Teban::Sente && n == 1 {
								let _ = hes.send(Ok(EventState::GameStart));
							} else {
								let _ = hes.send(Err(String::from("GameStart event is invalid.")));
							}
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Moved, move |_,e| {
					match e {
						&SelfMatchEvent::Moved(_,_) => {
							let _ = hes.send(Ok(EventState::Moved));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameEnd, move |_,e| {
					match e {
						&SelfMatchEvent::GameEnd(SelfMatchGameEndState::Resign(t)) => {
							if t == Teban::Sente {
								let _ = hes.send(Ok(EventState::GameEnd));
							} else {
								let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							}
						},
						_ => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
						}
					}
					Ok(())
				});
			let hes = es.clone();
			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Abort, move |_,e| {
					match e {
						&SelfMatchEvent::Abort => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			},
			|| false,
			None,
			None, input_reader, input_read_handler,
			player1,player2,
			create_options(), create_options(),
			info_sender,
			UsiGoTimeLimit::None,
			None,Some(1),
			logger, |h,e| {
				if let Some(h) = h {
					let _ = h.lock().map(|h| h.call(e));
				}
			}
		);

		let _ = ts.send(());
	});

	startup(&pmr);

	gamestart_process(&pmr);

	let res = er.recv_timeout(Duration::from_millis(300)).expect("attempt to receive EventState::GameStart timed out.");

	assert_eq!(res,Ok(EventState::GameStart));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::GameEnd timed out.");

	assert_eq!(res,Ok(EventState::GameEnd));

	let res = pmr[0].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let res = pmr[1].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let _ = tr.recv_timeout(Duration::from_millis(180)).expect("attempt to receive on quited timed out.");

	let _ = s.send(String::from(""));
}
#[test]
fn test_ponderhit_thinking() {
	let (pms1,pmr1) = mpsc::channel();
	let (pns1,_) = mpsc::channel();
	let (ts,tr) = mpsc::channel();

	let (pms2,pmr2) = mpsc::channel();
	let (pns2,_) = mpsc::channel();

	let pmr = [pmr1,pmr2];

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = mpsc::channel();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (es,er) = mpsc::channel();

	let _ = thread::spawn(move || {
		let player1 = MockPlayer::new(pms1,pns1,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										}),
										Box::new(|player,_,_,_,_,_,mvs| {
											if mvs == vec![Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),
															Move::To(KomaSrcPosition(9,3),KomaDstToPosition(9,4,false))] {
												let _ = player.sender.send(Ok(ActionKind::SetPosition));
											} else {
												let _ = player.sender.send(Err(String::from("mvs is invalid.")));
											}
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),
																Some(Move::To(KomaSrcPosition(9,3),KomaDstToPosition(9,4,false)))))
										}),
										Box::new(|player,_,_,_,_,mut handle_events| {
											let _ = player.sender.send(Ok(ActionKind::Think));

											thread::sleep(Duration::from_millis(400));
											handle_events(player)?;

											let now = Instant::now();

											if player.ponderhit_time.map(|t| now - t < Duration::from_millis(100)).unwrap_or(false) {
												let _ = player.sender.send(Err(format!(
															"ponderhit state is invalid. ({:?})",
															player.ponderhit_time.map(|t| (now - t).subsec_millis()))));
											}

											Ok(BestMove::Resign)
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Lose => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let player2 = MockPlayer::new(pms2,pns2,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(9,3),KomaDstToPosition(9,4,false)),None))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Win => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let (is,_) = mpsc::channel();

		let info_sender = MockInfoSender::new(is);

		let mut engine = SelfMatchEngine::new();

		let input_read_handler = create_input_read_handler(&engine.system_event_queue);

		let _ = engine.start(|self_match_event_dispatcher| {
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameStart, move |_,e| {
					match e {
						&SelfMatchEvent::GameStart(n,t,_) => {
							if t == Teban::Sente && n == 1 {
								let _ = hes.send(Ok(EventState::GameStart));
							} else {
								let _ = hes.send(Err(String::from("GameStart event is invalid.")));
							}
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Moved, move |_,e| {
					match e {
						&SelfMatchEvent::Moved(_,_) => {
							let _ = hes.send(Ok(EventState::Moved));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameEnd, move |_,e| {
					match e {
						&SelfMatchEvent::GameEnd(SelfMatchGameEndState::Resign(t)) => {
							if t == Teban::Sente {
								let _ = hes.send(Ok(EventState::GameEnd));
							} else {
								let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							}
						},
						_ => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
						}
					}
					Ok(())
				});
			let hes = es.clone();
			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Abort, move |_,e| {
					match e {
						&SelfMatchEvent::Abort => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			},
			|| false,
			None,
			None, input_reader, input_read_handler,
			player1,player2,
			create_options(), create_options(),
			info_sender,
			UsiGoTimeLimit::None,
			None,Some(1),
			logger, |h,e| {
				if let Some(h) = h {
					let _ = h.lock().map(|h| h.call(e));
				}
			}
		);

		let _ = ts.send(());
	});

	startup(&pmr);

	gamestart_process(&pmr);

	let res = er.recv_timeout(Duration::from_millis(300)).expect("attempt to receive EventState::GameStart timed out.");

	assert_eq!(res,Ok(EventState::GameStart));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[0].recv_timeout(Duration::from_millis(460)).expect("attempt to receive ActionKind::PonderHit timed out.");

	assert_eq!(res,Ok(ActionKind::PonderHit));

	let res = pmr[0].recv_timeout(Duration::from_millis(460)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::GameEnd timed out.");

	assert_eq!(res,Ok(EventState::GameEnd));

	let res = pmr[0].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let res = pmr[1].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let _ = tr.recv_timeout(Duration::from_millis(180)).expect("attempt to receive on quited timed out.");

	let _ = s.send(String::from(""));
}
#[test]
fn test_ponderhit_thinking_check_next_turn_eventqueue() {
	let (pms1,pmr1) = mpsc::channel();
	let (pns1,_) = mpsc::channel();
	let (ts,tr) = mpsc::channel();

	let (pms2,pmr2) = mpsc::channel();
	let (pns2,_) = mpsc::channel();

	let pmr = [pmr1,pmr2];

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = mpsc::channel();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (es,er) = mpsc::channel();

	let _ = thread::spawn(move || {
		let player1 = MockPlayer::new(pms1,pns1,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										}),
										Box::new(|player,_,_,_,_,_,mvs| {
											if mvs == vec![Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),
															Move::To(KomaSrcPosition(9,3),KomaDstToPosition(9,4,false))] {
												let _ = player.sender.send(Ok(ActionKind::SetPosition));
											} else {
												let _ = player.sender.send(Err(String::from("mvs is invalid.")));
											}
											Ok(())
										}),
										Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),
																Some(Move::To(KomaSrcPosition(9,3),KomaDstToPosition(9,4,false)))))
										}),
										Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));

											thread::sleep(Duration::from_millis(400));

											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,6),KomaDstToPosition(1,5,false)),None))
										}),
										Box::new(|player,_,_,_,_,mut handle_events| {
											handle_events(player)?;

											if player.ponderhit_time.is_none() {
												let _ = player.sender.send(Ok(ActionKind::Think));
											} else {
												let _ = player.sender.send(Err(String::from(
													"eventqueue state invalid!"
												)));
											}

											Ok(BestMove::Resign)
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Lose => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let player2 = MockPlayer::new(pms2,pns2,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										}),
										Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(9,3),KomaDstToPosition(9,4,false)),None))
										}),
										Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(9,4),KomaDstToPosition(9,5,false)),None))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Win => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let (is,_) = mpsc::channel();

		let info_sender = MockInfoSender::new(is);

		let mut engine = SelfMatchEngine::new();

		let input_read_handler = create_input_read_handler(&engine.system_event_queue);

		let _ = engine.start(|self_match_event_dispatcher| {
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameStart, move |_,e| {
					match e {
						&SelfMatchEvent::GameStart(n,t,_) => {
							if t == Teban::Sente && n == 1 {
								let _ = hes.send(Ok(EventState::GameStart));
							} else {
								let _ = hes.send(Err(String::from("GameStart event is invalid.")));
							}
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Moved, move |_,e| {
					match e {
						&SelfMatchEvent::Moved(_,_) => {
							let _ = hes.send(Ok(EventState::Moved));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameEnd, move |_,e| {
					match e {
						&SelfMatchEvent::GameEnd(SelfMatchGameEndState::Resign(t)) => {
							if t == Teban::Sente {
								let _ = hes.send(Ok(EventState::GameEnd));
							} else {
								let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							}
						},
						_ => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
						}
					}
					Ok(())
				});
			let hes = es.clone();
			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Abort, move |_,e| {
					match e {
						&SelfMatchEvent::Abort => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			},
			|| false,
			None,
			None, input_reader, input_read_handler,
			player1,player2,
			create_options(), create_options(),
			info_sender,
			UsiGoTimeLimit::None,
			None,Some(1),
			logger, |h,e| {
				if let Some(h) = h {
					let _ = h.lock().map(|h| h.call(e));
				}
			}
		);

		let _ = ts.send(());
	});

	startup(&pmr);

	gamestart_process(&pmr);

	let res = er.recv_timeout(Duration::from_millis(300)).expect("attempt to receive EventState::GameStart timed out.");

	assert_eq!(res,Ok(EventState::GameStart));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = er.recv_timeout(Duration::from_millis(460)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = pmr[0].recv_timeout(Duration::from_millis(460)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::GameEnd timed out.");

	assert_eq!(res,Ok(EventState::GameEnd));

	let res = pmr[0].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let res = pmr[1].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let _ = tr.recv_timeout(Duration::from_millis(180)).expect("attempt to receive on quited timed out.");

	let _ = s.send(String::from(""));
}
#[test]
fn test_ponderng_1times() {
	let (pms1,pmr1) = mpsc::channel();
	let (pns1,_) = mpsc::channel();
	let (ts,tr) = mpsc::channel();

	let (pms2,pmr2) = mpsc::channel();
	let (pns2,_) = mpsc::channel();

	let pmr = [pmr1,pmr2];

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = mpsc::channel();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (es,er) = mpsc::channel();

	let _ = thread::spawn(move || {
		let player1 = MockPlayer::new(pms1,pns1,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										}),
										Box::new(|player,_,_,_,_,_,mvs| {
											if mvs == vec![Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),
															Move::To(KomaSrcPosition(9,3),KomaDstToPosition(9,4,false))] {
												let _ = player.sender.send(Ok(ActionKind::SetPosition));
											} else {
												let _ = player.sender.send(Err(String::from("mvs is invalid.")));
											}
											Ok(())
										}),
										Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),
																Some(Move::To(KomaSrcPosition(9,3),KomaDstToPosition(9,4,false)))))
										}),
										Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,6),KomaDstToPosition(1,5,false)),None))
										}),
										Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Resign)
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Lose => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let player2 = MockPlayer::new(pms2,pns2,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(3,3),KomaDstToPosition(3,4,false)),None))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Win => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let (is,_) = mpsc::channel();

		let info_sender = MockInfoSender::new(is);

		let mut engine = SelfMatchEngine::new();

		let input_read_handler = create_input_read_handler(&engine.system_event_queue);

		let _ = engine.start(|self_match_event_dispatcher| {
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameStart, move |_,e| {
					match e {
						&SelfMatchEvent::GameStart(n,t,_) => {
							if t == Teban::Sente && n == 1 {
								let _ = hes.send(Ok(EventState::GameStart));
							} else {
								let _ = hes.send(Err(String::from("GameStart event is invalid.")));
							}
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Moved, move |_,e| {
					match e {
						&SelfMatchEvent::Moved(_,_) => {
							let _ = hes.send(Ok(EventState::Moved));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameEnd, move |_,e| {
					match e {
						&SelfMatchEvent::GameEnd(SelfMatchGameEndState::Resign(t)) => {
							if t == Teban::Sente {
								let _ = hes.send(Ok(EventState::GameEnd));
							} else {
								let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							}
						},
						_ => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
						}
					}
					Ok(())
				});
			let hes = es.clone();
			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Abort, move |_,e| {
					match e {
						&SelfMatchEvent::Abort => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			},
			|| false,
			None,
			None, input_reader, input_read_handler,
			player1,player2,
			create_options(), create_options(),
			info_sender,
			UsiGoTimeLimit::None,
			None,Some(1),
			logger, |h,e| {
				if let Some(h) = h {
					let _ = h.lock().map(|h| h.call(e));
				}
			}
		);

		let _ = ts.send(());
	});

	startup(&pmr);

	gamestart_process(&pmr);

	let res = er.recv_timeout(Duration::from_millis(300)).expect("attempt to receive EventState::GameStart timed out.");

	assert_eq!(res,Ok(EventState::GameStart));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::GameEnd timed out.");

	assert_eq!(res,Ok(EventState::GameEnd));

	let res = pmr[0].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let res = pmr[1].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let _ = tr.recv_timeout(Duration::from_millis(180)).expect("attempt to receive on quited timed out.");

	let _ = s.send(String::from(""));
}
#[test]
fn test_ponderng_thinking() {
	let (pms1,pmr1) = mpsc::channel();
	let (pns1,_) = mpsc::channel();
	let (ts,tr) = mpsc::channel();

	let (pms2,pmr2) = mpsc::channel();
	let (pns2,_) = mpsc::channel();

	let pmr = [pmr1,pmr2];

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = mpsc::channel();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (es,er) = mpsc::channel();

	let _ = thread::spawn(move || {
		let player1 = MockPlayer::new(pms1,pns1,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										}),
										Box::new(|player,_,_,_,_,_,mvs| {
											if mvs == vec![Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),
															Move::To(KomaSrcPosition(9,3),KomaDstToPosition(9,4,false))] {
												let _ = player.sender.send(Ok(ActionKind::SetPosition));
											} else {
												let _ = player.sender.send(Err(String::from("mvs is invalid.")));
											}
											Ok(())
										}),
										Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),
																Some(Move::To(KomaSrcPosition(9,3),KomaDstToPosition(9,4,false)))))
										}),
										Box::new(|player,_,_,_,_,mut handle_events| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											while !player.stop {
												handle_events(player)?;
												thread::sleep(Duration::from_millis(10));
											}
											thread::sleep(Duration::from_millis(100));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,6),KomaDstToPosition(1,5,false)),None))
										}),
										Box::new(|player,think_start_time,_,_,_,_| {
											let now = Instant::now();

											if think_start_time.map(|t| now - t >= Duration::from_millis(100)).unwrap_or(false) {
												let _ = player.sender.send(Ok(ActionKind::Think));
											} else {
												let _ = player.sender.send(Err(format!(
															"think_start_time is invalid. ({:?})",
															think_start_time.map(|t| (now - t).subsec_millis()))));
											}

											Ok(BestMove::Resign)
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Lose => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let player2 = MockPlayer::new(pms2,pns2,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(3,3),KomaDstToPosition(3,4,false)),None))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Win => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let (is,_) = mpsc::channel();

		let info_sender = MockInfoSender::new(is);

		let mut engine = SelfMatchEngine::new();

		let input_read_handler = create_input_read_handler(&engine.system_event_queue);

		let _ = engine.start(|self_match_event_dispatcher| {
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameStart, move |_,e| {
					match e {
						&SelfMatchEvent::GameStart(n,t,_) => {
							if t == Teban::Sente && n == 1 {
								let _ = hes.send(Ok(EventState::GameStart));
							} else {
								let _ = hes.send(Err(String::from("GameStart event is invalid.")));
							}
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Moved, move |_,e| {
					match e {
						&SelfMatchEvent::Moved(_,_) => {
							let _ = hes.send(Ok(EventState::Moved));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameEnd, move |_,e| {
					match e {
						&SelfMatchEvent::GameEnd(SelfMatchGameEndState::Resign(t)) => {
							if t == Teban::Sente {
								let _ = hes.send(Ok(EventState::GameEnd));
							} else {
								let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							}
						},
						_ => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
						}
					}
					Ok(())
				});
			let hes = es.clone();
			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Abort, move |_,e| {
					match e {
						&SelfMatchEvent::Abort => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			},
			|| false,
			None,
			None, input_reader, input_read_handler,
			player1,player2,
			create_options(), create_options(),
			info_sender,
			UsiGoTimeLimit::None,
			None,Some(1),
			logger, |h,e| {
				if let Some(h) = h {
					let _ = h.lock().map(|h| h.call(e));
				}
			}
		);

		let _ = ts.send(());
	});

	startup(&pmr);

	gamestart_process(&pmr);

	let res = er.recv_timeout(Duration::from_millis(300)).expect("attempt to receive EventState::GameStart timed out.");

	assert_eq!(res,Ok(EventState::GameStart));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::OnStop timed out.");

	assert_eq!(res,Ok(ActionKind::OnStop));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[0].recv_timeout(Duration::from_millis(160)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::GameEnd timed out.");

	assert_eq!(res,Ok(EventState::GameEnd));

	let res = pmr[0].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let res = pmr[1].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let _ = tr.recv_timeout(Duration::from_millis(180)).expect("attempt to receive on quited timed out.");

	let _ = s.send(String::from(""));
}
#[test]
fn test_ponderng_thinking_check_next_turn_eventqueue() {
	let (pms1,pmr1) = mpsc::channel();
	let (pns1,_) = mpsc::channel();
	let (ts,tr) = mpsc::channel();

	let (pms2,pmr2) = mpsc::channel();
	let (pns2,_) = mpsc::channel();

	let pmr = [pmr1,pmr2];

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = mpsc::channel();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (es,er) = mpsc::channel();

	let _ = thread::spawn(move || {
		let player1 = MockPlayer::new(pms1,pns1,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										}),
										Box::new(|player,_,_,_,_,_,mvs| {
											if mvs == vec![Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),
															Move::To(KomaSrcPosition(9,3),KomaDstToPosition(9,4,false))] {
												let _ = player.sender.send(Ok(ActionKind::SetPosition));
											} else {
												let _ = player.sender.send(Err(String::from("mvs is invalid.")));
											}
											Ok(())
										}),
										Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										}),
										Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),
																Some(Move::To(KomaSrcPosition(9,3),KomaDstToPosition(9,4,false)))))
										}),
										Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											thread::sleep(Duration::from_millis(400));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,6),KomaDstToPosition(1,5,false)),None))
										}),
										Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,6),KomaDstToPosition(1,5,false)),None))
										}),
										Box::new(|player,_,_,_,_,mut handle_events| {
											handle_events(player)?;

											if !player.stop {
												let _ = player.sender.send(Ok(ActionKind::Think));
											} else {
												let _ = player.sender.send(Err(String::from(
													"eventqueue state invalid!"
												)));
											}

											thread::sleep(Duration::from_millis(100));

											Ok(BestMove::Resign)
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Lose => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let player2 = MockPlayer::new(pms2,pns2,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										}),
										Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(3,3),KomaDstToPosition(3,4,false)),None))
										}),
										Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(3,4),KomaDstToPosition(3,5,false)),None))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Win => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let (is,_) = mpsc::channel();

		let info_sender = MockInfoSender::new(is);

		let mut engine = SelfMatchEngine::new();

		let input_read_handler = create_input_read_handler(&engine.system_event_queue);

		let _ = engine.start(|self_match_event_dispatcher| {
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameStart, move |_,e| {
					match e {
						&SelfMatchEvent::GameStart(n,t,_) => {
							if t == Teban::Sente && n == 1 {
								let _ = hes.send(Ok(EventState::GameStart));
							} else {
								let _ = hes.send(Err(String::from("GameStart event is invalid.")));
							}
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Moved, move |_,e| {
					match e {
						&SelfMatchEvent::Moved(_,_) => {
							let _ = hes.send(Ok(EventState::Moved));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameEnd, move |_,e| {
					match e {
						&SelfMatchEvent::GameEnd(SelfMatchGameEndState::Resign(t)) => {
							if t == Teban::Sente {
								let _ = hes.send(Ok(EventState::GameEnd));
							} else {
								let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							}
						},
						_ => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
						}
					}
					Ok(())
				});
			let hes = es.clone();
			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Abort, move |_,e| {
					match e {
						&SelfMatchEvent::Abort => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			},
			|| false,
			None,
			None, input_reader, input_read_handler,
			player1,player2,
			create_options(), create_options(),
			info_sender,
			UsiGoTimeLimit::None,
			None,Some(1),
			logger, |h,e| {
				if let Some(h) = h {
					let _ = h.lock().map(|h| h.call(e));
				}
			}
		);

		let _ = ts.send(());
	});

	startup(&pmr);

	gamestart_process(&pmr);

	let res = er.recv_timeout(Duration::from_millis(300)).expect("attempt to receive EventState::GameStart timed out.");

	assert_eq!(res,Ok(EventState::GameStart));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[0].recv_timeout(Duration::from_millis(460)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[0].recv_timeout(Duration::from_millis(460)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = pmr[0].recv_timeout(Duration::from_millis(160)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::GameEnd timed out.");

	assert_eq!(res,Ok(EventState::GameEnd));

	let res = pmr[0].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let res = pmr[1].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let _ = tr.recv_timeout(Duration::from_millis(180)).expect("attempt to receive on quited timed out.");

	let _ = s.send(String::from(""));
}
#[test]
fn test_abort_1times() {
	let (pms1,pmr1) = mpsc::channel();
	let (pns1,_) = mpsc::channel();
	let (ts,tr) = mpsc::channel();

	let (pms2,pmr2) = mpsc::channel();
	let (pns2,_) = mpsc::channel();

	let pmr = [pmr1,pmr2];

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = mpsc::channel();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (es,er) = mpsc::channel();

	let _ = thread::spawn(move || {
		let player1 = MockPlayer::new(pms1,pns1,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										}),
										Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),None))
										}),
										Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Abort)
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![])
		);

		let player2 = MockPlayer::new(pms2,pns2,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(9,3),KomaDstToPosition(9,4,false)),None))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![])
		);

		let (is,_) = mpsc::channel();

		let info_sender = MockInfoSender::new(is);

		let mut engine = SelfMatchEngine::new();

		let input_read_handler = create_input_read_handler(&engine.system_event_queue);

		let _ = engine.start(|self_match_event_dispatcher| {
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameStart, move |_,e| {
					match e {
						&SelfMatchEvent::GameStart(n,t,_) => {
							if t == Teban::Sente && n == 1 {
								let _ = hes.send(Ok(EventState::GameStart));
							} else {
								let _ = hes.send(Err(String::from("GameStart event is invalid.")));
							}
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Moved, move |_,e| {
					match e {
						&SelfMatchEvent::Moved(_,_) => {
							let _ = hes.send(Ok(EventState::Moved));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameEnd, move |_,e| {
					match e {
						&SelfMatchEvent::GameEnd(_) => {
							let _ = hes.send(Ok(EventState::GameEnd));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();
			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Abort, move |_,e| {
					match e {
						&SelfMatchEvent::Abort => {
							let _ = hes.send(Ok(EventState::Abort));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			},
			|| false,
			None,
			None, input_reader, input_read_handler,
			player1,player2,
			create_options(), create_options(),
			info_sender,
			UsiGoTimeLimit::None,
			None,Some(1),
			logger, |h,e| {
				if let Some(h) = h {
					let _ = h.lock().map(|h| h.call(e));
				}
			}
		);

		let _ = ts.send(());
	});

	startup(&pmr);

	gamestart_process(&pmr);

	let res = er.recv_timeout(Duration::from_millis(300)).expect("attempt to receive EventState::GameStart timed out.");

	assert_eq!(res,Ok(EventState::GameStart));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Abort timed out.");

	assert_eq!(res,Ok(EventState::Abort));

	let res = pmr[0].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let res = pmr[1].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let _ = tr.recv_timeout(Duration::from_millis(180)).expect("attempt to receive on quited timed out.");

	let _ = s.send(String::from(""));
}
#[test]
fn test_nyugyoku_win_win_sente_1times() {
	let (pms1,pmr1) = mpsc::channel();
	let (pns1,_) = mpsc::channel();
	let (ts,tr) = mpsc::channel();

	let (pms2,pmr2) = mpsc::channel();
	let (pns2,_) = mpsc::channel();

	let pmr = [pmr1,pmr2];

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = mpsc::channel();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (es,er) = mpsc::channel();
	let (ks,kr) = mpsc::channel();

	let mut kifuwriter = MockSfenKifuWriter::new(ks);

	let _ = thread::spawn(move || {
		let player1 = MockPlayer::new(pms1,pns1,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										}),
										Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,4),KomaDstToPosition(1,3,true)),None))
										}),
										Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Win)
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Win => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let player2 = MockPlayer::new(pms2,pns2,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(9,6),KomaDstToPosition(9,7,true)),None))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Lose => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let (is,_) = mpsc::channel();

		let info_sender = MockInfoSender::new(is);

		let mut engine = SelfMatchEngine::new();

		let input_read_handler = create_input_read_handler(&engine.system_event_queue);

		let _ = engine.start(|self_match_event_dispatcher| {
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameStart, move |_,e| {
					match e {
						&SelfMatchEvent::GameStart(n,t,_) => {
							if t == Teban::Sente && n == 1 {
								let _ = hes.send(Ok(EventState::GameStart));
							} else {
								let _ = hes.send(Err(String::from("GameStart event is invalid.")));
							}
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Moved, move |_,e| {
					match e {
						&SelfMatchEvent::Moved(_,_) => {
							let _ = hes.send(Ok(EventState::Moved));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameEnd, move |_,e| {
					match e {
						&SelfMatchEvent::GameEnd(SelfMatchGameEndState::NyuGyokuWin(t)) => {
							if t == Teban::Sente {
								let _ = hes.send(Ok(EventState::GameEnd));
							} else {
								let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							}
						},
						_ => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
						}
					}
					Ok(())
				});
			let hes = es.clone();
			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Abort, move |_,e| {
					match e {
						&SelfMatchEvent::Abort => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			},
			|| false,
			Some(Box::new(|| String::from("sfen ln5n+P/1+R+B1K3+P/+P+P+P+P+P2+P1/ln6P/9/pp5NL/2+p1+p+p+p+p+p/2+b1k3+p/+pN5NL b R2G2S2g2s 1"))),
			Some(Box::new(move |sfen,mvs| kifuwriter.write(sfen,mvs))),
			input_reader, input_read_handler,
			player1,player2,
			create_options(), create_options(),
			info_sender,
			UsiGoTimeLimit::None,
			None,Some(1),
			logger, |h,e| {
				if let Some(h) = h {
					let _ = h.lock().map(|h| h.call(e));
				}
			}
		);

		let _ = ts.send(());
	});

	startup(&pmr);

	gamestart_process(&pmr);

	let res = er.recv_timeout(Duration::from_millis(300)).expect("attempt to receive EventState::GameStart timed out.");

	assert_eq!(res,Ok(EventState::GameStart));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::GameEnd timed out.");

	assert_eq!(res,Ok(EventState::GameEnd));

	let res = kr.recv_timeout(Duration::from_millis(60)).expect("attempt to receive kifu string.");

	assert_eq!(&*res,"sfen ln5n+P/1+R+B1K3+P/+P+P+P+P+P2+P1/ln6P/9/pp5NL/2+p1+p+p+p+p+p/2+b1k3+p/+pN5NL b R2G2S2g2s 1 moves 1d1c+ 9f9g+");

	let res = pmr[0].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let res = pmr[1].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let _ = tr.recv_timeout(Duration::from_millis(180)).expect("attempt to receive on quited timed out.");

	let _ = s.send(String::from(""));
}
#[test]
fn test_nyugyoku_win_win_gote_1times() {
	let (pms1,pmr1) = mpsc::channel();
	let (pns1,_) = mpsc::channel();
	let (ts,tr) = mpsc::channel();

	let (pms2,pmr2) = mpsc::channel();
	let (pns2,_) = mpsc::channel();

	let pmr = [pmr1,pmr2];

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = mpsc::channel();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (es,er) = mpsc::channel();
	let (ks,kr) = mpsc::channel();

	let mut kifuwriter = MockSfenKifuWriter::new(ks);

	let _ = thread::spawn(move || {
		let player1 = MockPlayer::new(pms1,pns1,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										}),
										Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,4),KomaDstToPosition(1,3,true)),None))
										}),
										Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,6),KomaDstToPosition(1,5,false)),None))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Lose => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let player2 = MockPlayer::new(pms2,pns2,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										}),
										Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(9,6),KomaDstToPosition(9,7,true)),None))
										}),
										Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Win)
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Win => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let (is,_) = mpsc::channel();

		let info_sender = MockInfoSender::new(is);

		let mut engine = SelfMatchEngine::new();

		let input_read_handler = create_input_read_handler(&engine.system_event_queue);

		let _ = engine.start(|self_match_event_dispatcher| {
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameStart, move |_,e| {
					match e {
						&SelfMatchEvent::GameStart(n,t,_) => {
							if t == Teban::Sente && n == 1 {
								let _ = hes.send(Ok(EventState::GameStart));
							} else {
								let _ = hes.send(Err(String::from("GameStart event is invalid.")));
							}
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Moved, move |_,e| {
					match e {
						&SelfMatchEvent::Moved(_,_) => {
							let _ = hes.send(Ok(EventState::Moved));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameEnd, move |_,e| {
					match e {
						&SelfMatchEvent::GameEnd(SelfMatchGameEndState::NyuGyokuWin(t)) => {
							if t == Teban::Gote {
								let _ = hes.send(Ok(EventState::GameEnd));
							} else {
								let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							}
						},
						_ => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
						}
					}
					Ok(())
				});
			let hes = es.clone();
			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Abort, move |_,e| {
					match e {
						&SelfMatchEvent::Abort => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			},
			|| false,
			Some(Box::new(|| String::from("sfen ln5n+P/2+B1K3+P/+P+P+P+P+P2+P1/ln6P/9/pp5NL/4+p+p+p+p+p/+p3k1+b+r1/+pN5NL b 2G2Sr2g2s 1"))),
			Some(Box::new(move |sfen,mvs| kifuwriter.write(sfen,mvs))),
			input_reader, input_read_handler,
			player1,player2,
			create_options(), create_options(),
			info_sender,
			UsiGoTimeLimit::None,
			None,Some(1),
			logger, |h,e| {
				if let Some(h) = h {
					let _ = h.lock().map(|h| h.call(e));
				}
			}
		);

		let _ = ts.send(());
	});

	startup(&pmr);

	gamestart_process(&pmr);

	let res = er.recv_timeout(Duration::from_millis(300)).expect("attempt to receive EventState::GameStart timed out.");

	assert_eq!(res,Ok(EventState::GameStart));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::GameEnd timed out.");

	assert_eq!(res,Ok(EventState::GameEnd));

	let res = kr.recv_timeout(Duration::from_millis(60)).expect("attempt to receive kifu string.");

	assert_eq!(&*res,"sfen ln5n+P/2+B1K3+P/+P+P+P+P+P2+P1/ln6P/9/pp5NL/4+p+p+p+p+p/+p3k1+b+r1/+pN5NL b 2G2Sr2g2s 1 moves 1d1c+ 9f9g+ 1f1e");

	let res = pmr[0].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let res = pmr[1].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let _ = tr.recv_timeout(Duration::from_millis(180)).expect("attempt to receive on quited timed out.");

	let _ = s.send(String::from(""));
}
#[test]
fn test_nyugyoku_win_lose_sente_1times() {
	let (pms1,pmr1) = mpsc::channel();
	let (pns1,_) = mpsc::channel();
	let (ts,tr) = mpsc::channel();

	let (pms2,pmr2) = mpsc::channel();
	let (pns2,_) = mpsc::channel();

	let pmr = [pmr1,pmr2];

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = mpsc::channel();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (es,er) = mpsc::channel();
	let (ks,kr) = mpsc::channel();

	let mut kifuwriter = MockSfenKifuWriter::new(ks);

	let _ = thread::spawn(move || {
		let player1 = MockPlayer::new(pms1,pns1,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										}),
										Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,6),KomaDstToPosition(1,5,false)),None))
										}),
										Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Win)
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Lose => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let player2 = MockPlayer::new(pms2,pns2,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(9,6),KomaDstToPosition(9,7,true)),None))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Win => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let (is,_) = mpsc::channel();

		let info_sender = MockInfoSender::new(is);

		let mut engine = SelfMatchEngine::new();

		let input_read_handler = create_input_read_handler(&engine.system_event_queue);

		let _ = engine.start(|self_match_event_dispatcher| {
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameStart, move |_,e| {
					match e {
						&SelfMatchEvent::GameStart(n,t,_) => {
							if t == Teban::Sente && n == 1 {
								let _ = hes.send(Ok(EventState::GameStart));
							} else {
								let _ = hes.send(Err(String::from("GameStart event is invalid.")));
							}
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Moved, move |_,e| {
					match e {
						&SelfMatchEvent::Moved(_,_) => {
							let _ = hes.send(Ok(EventState::Moved));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameEnd, move |_,e| {
					match e {
						&SelfMatchEvent::GameEnd(SelfMatchGameEndState::NyuGyokuLose(t)) => {
							if t == Teban::Sente {
								let _ = hes.send(Ok(EventState::GameEnd));
							} else {
								let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							}
						},
						_ => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
						}
					}
					Ok(())
				});
			let hes = es.clone();
			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Abort, move |_,e| {
					match e {
						&SelfMatchEvent::Abort => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			},
			|| false,
			Some(Box::new(|| String::from("sfen ln5n+P/1+R+B1K3+P/+P+P+P+P+P2+P1/ln6P/9/pp5NL/2+p1+p+p+p+p+p/2+b1k3+p/+pN5NL b R2G2S2g2s 1"))),
			Some(Box::new(move |sfen,mvs| kifuwriter.write(sfen,mvs))),
			input_reader, input_read_handler,
			player1,player2,
			create_options(), create_options(),
			info_sender,
			UsiGoTimeLimit::None,
			None,Some(1),
			logger, |h,e| {
				if let Some(h) = h {
					let _ = h.lock().map(|h| h.call(e));
				}
			}
		);

		let _ = ts.send(());
	});

	startup(&pmr);

	gamestart_process(&pmr);

	let res = er.recv_timeout(Duration::from_millis(300)).expect("attempt to receive EventState::GameStart timed out.");

	assert_eq!(res,Ok(EventState::GameStart));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::GameEnd timed out.");

	assert_eq!(res,Ok(EventState::GameEnd));

	let res = kr.recv_timeout(Duration::from_millis(60)).expect("attempt to receive kifu string.");

	assert_eq!(&*res,"sfen ln5n+P/1+R+B1K3+P/+P+P+P+P+P2+P1/ln6P/9/pp5NL/2+p1+p+p+p+p+p/2+b1k3+p/+pN5NL b R2G2S2g2s 1 moves 1f1e 9f9g+");

	let res = pmr[0].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let res = pmr[1].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let _ = tr.recv_timeout(Duration::from_millis(180)).expect("attempt to receive on quited timed out.");

	let _ = s.send(String::from(""));
}
#[test]
fn test_nyugyoku_win_lose_gote_1times() {
	let (pms1,pmr1) = mpsc::channel();
	let (pns1,_) = mpsc::channel();
	let (ts,tr) = mpsc::channel();

	let (pms2,pmr2) = mpsc::channel();
	let (pns2,_) = mpsc::channel();

	let pmr = [pmr1,pmr2];

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = mpsc::channel();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (es,er) = mpsc::channel();
	let (ks,kr) = mpsc::channel();

	let mut kifuwriter = MockSfenKifuWriter::new(ks);

	let _ = thread::spawn(move || {
		let player1 = MockPlayer::new(pms1,pns1,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										}),
										Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,4),KomaDstToPosition(1,3,true)),None))
										}),
										Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,6),KomaDstToPosition(1,5,false)),None))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Win => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let player2 = MockPlayer::new(pms2,pns2,
										ConsumedIterator::new(vec![Box::new(|player,_| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										}),
										Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(9,4),KomaDstToPosition(9,5,false)),None))
										}),
										Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Win)
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,s,_| {
											match s {
												&GameEndState::Lose => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);

		let (is,_) = mpsc::channel();

		let info_sender = MockInfoSender::new(is);

		let mut engine = SelfMatchEngine::new();

		let input_read_handler = create_input_read_handler(&engine.system_event_queue);

		let _ = engine.start(|self_match_event_dispatcher| {
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameStart, move |_,e| {
					match e {
						&SelfMatchEvent::GameStart(n,t,_) => {
							if t == Teban::Sente && n == 1 {
								let _ = hes.send(Ok(EventState::GameStart));
							} else {
								let _ = hes.send(Err(String::from("GameStart event is invalid.")));
							}
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Moved, move |_,e| {
					match e {
						&SelfMatchEvent::Moved(_,_) => {
							let _ = hes.send(Ok(EventState::Moved));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameEnd, move |_,e| {
					match e {
						&SelfMatchEvent::GameEnd(SelfMatchGameEndState::NyuGyokuLose(t)) => {
							if t == Teban::Gote {
								let _ = hes.send(Ok(EventState::GameEnd));
							} else {
								let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							}
						},
						_ => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
						}
					}
					Ok(())
				});
			let hes = es.clone();
			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::Abort, move |_,e| {
					match e {
						&SelfMatchEvent::Abort => {
							let _ = hes.send(Err(String::from("GameEnd event is invalid.")));
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				});
			},
			|| false,
			Some(Box::new(|| String::from("sfen ln5n+P/2+B1K3+P/+P+P+P+P+P2+P1/ln6P/9/pp5NL/4+p+p+p+p+p/+p3k1+b+r1/+pN5NL b 2G2Sr2g2s 1"))),
			Some(Box::new(move |sfen,mvs| kifuwriter.write(sfen,mvs))),
			input_reader, input_read_handler,
			player1,player2,
			create_options(), create_options(),
			info_sender,
			UsiGoTimeLimit::None,
			None,Some(1),
			logger, |h,e| {
				if let Some(h) = h {
					let _ = h.lock().map(|h| h.call(e));
				}
			}
		);

		let _ = ts.send(());
	});

	startup(&pmr);

	gamestart_process(&pmr);

	let res = er.recv_timeout(Duration::from_millis(300)).expect("attempt to receive EventState::GameStart timed out.");

	assert_eq!(res,Ok(EventState::GameStart));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::GameEnd timed out.");

	assert_eq!(res,Ok(EventState::GameEnd));

	let res = kr.recv_timeout(Duration::from_millis(60)).expect("attempt to receive kifu string.");

	assert_eq!(&*res,"sfen ln5n+P/2+B1K3+P/+P+P+P+P+P2+P1/ln6P/9/pp5NL/4+p+p+p+p+p/+p3k1+b+r1/+pN5NL b 2G2Sr2g2s 1 moves 1d1c+ 9d9e 1f1e");

	let res = pmr[0].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let res = pmr[1].recv_timeout(Duration::from_millis(500)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let _ = tr.recv_timeout(Duration::from_millis(180)).expect("attempt to receive on quited timed out.");

	let _ = s.send(String::from(""));
}
