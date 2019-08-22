use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::time::Duration;

use usiagent::selfmatch::*;
use usiagent::shogi::*;
use usiagent::rule::BANMEN_START_POS;
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
		let res = pmr[i].recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetOption timed out.");

		assert_eq!(res,Ok(ActionKind::SetOption));

		let res = pmr[i].recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetOption timed out.");

		assert_eq!(res,Ok(ActionKind::SetOption));

		let res = pmr[i].recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetOption timed out.");

		assert_eq!(res,Ok(ActionKind::SetOption));

		let res = pmr[i].recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetOption timed out.");

		assert_eq!(res,Ok(ActionKind::SetOption));

		let res = pmr[i].recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetOption timed out.");

		assert_eq!(res,Ok(ActionKind::SetOption));

		let res = pmr[i].recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetOption timed out.");

		assert_eq!(res,Ok(ActionKind::SetOption));

		let res = pmr[i].recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetOption timed out.");

		assert_eq!(res,Ok(ActionKind::SetOption));

		let res = pmr[i].recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetOption timed out.");

		assert_eq!(res,Ok(ActionKind::SetOption));

		let res = pmr[i].recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetOption timed out.");

		assert_eq!(res,Ok(ActionKind::SetOption));

		let res = pmr[i].recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetOption timed out.");

		assert_eq!(res,Ok(ActionKind::SetOption));

		let res = pmr[i].recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetOption timed out.");

		assert_eq!(res,Ok(ActionKind::SetOption));
	}
}
fn gamestart_process(pmr:&[Receiver<Result<ActionKind,String>>; 2]) {
	for i in 0..2 {
		let res = pmr[i].recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::TakeReady timed out.");

		assert_eq!(res,Ok(ActionKind::TakeReady));

		let res = pmr[i].recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::NewGame timed out.");

		assert_eq!(res,Ok(ActionKind::NewGame));
	}
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

	let (ks,_) = mpsc::channel();

	let _ =thread::spawn(move || {
		let player1 = MockPlayer::new(pms1,pns1,
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
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
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),None))
										}),
										Box::new(|player,_,_,_,_| {
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
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_| {
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

		let mut engine = SelfMatchEngine::new(
			player1,
			player2,
			info_sender,
			UsiGoTimeLimit::None,
			None,Some(1)
		);

		let system_event_queue = engine.system_event_queue.clone();

		let input_read_handler = move |input| {
			if input == "quit" {
				return match system_event_queue.lock()  {
					Ok(mut system_event_queue) => {
						system_event_queue.push(SystemEvent::Quit);
						Ok(())
					},
					Err(_) => {
						Err(SelfMatchRunningError::InvalidState(String::from(
							"Failed to secure exclusive lock of system_event_queue."
						)))
					}
				};
			}
			Ok(())
		};

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
					match *e {
						SelfMatchEvent::GameEnd(SelfMatchGameEndState::Resign(t)) => {
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
			Some(MockSfenKifuWriter::new(ks)), input_reader, input_read_handler,
			create_options(), create_options(), logger, |h,e| {
				if let Some(h) = h {
					let _ = h.lock().map(|h| h.call(e));
				}
			}
		);

		let _ = ts.send(());
	});

	startup(&pmr);

	gamestart_process(&pmr);

	let res = er.recv_timeout(Duration::from_millis(150)).expect("attempt to receive EventState::GameStart timed out.");

	assert_eq!(res,Ok(EventState::GameStart));

	let res = pmr[0].recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(150)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[1].recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[1].recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(150)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[0].recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = pmr[0].recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = pmr[1].recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = er.recv_timeout(Duration::from_millis(150)).expect("attempt to receive EventState::GameEnd timed out.");

	assert_eq!(res,Ok(EventState::GameEnd));

	let _ = s.send(String::from("quit"));

	let res = pmr[0].recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let res = pmr[1].recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let _ = tr.recv_timeout(Duration::from_millis(300)).expect("attempt to receive on quited timed out.");
}