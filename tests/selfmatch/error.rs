use std::thread;
use std::time::Duration;

use crossbeam_channel::unbounded;
use usiagent::selfmatch::*;
use usiagent::shogi::*;
use usiagent::command::*;
use usiagent::event::*;
use usiagent::error::*;

use super::*;

#[test]
fn test_error_take_ready_player1() {
	let (pms1,pmr1) = unbounded();
	let (pns1,_) = unbounded();
	let (ts,tr) = unbounded();

	let (pms2,pmr2) = unbounded();
	let (pns2,_) = unbounded();

	let pmr = [pmr1,pmr2];

	let (ls,lr) = unbounded();

	let logger = MockLogger::new(ls);
	let (input_reader,s) = {
		let (s,r) = unbounded();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (es,_) = unbounded();

	let _ = thread::spawn(move || {
		let player1 = MockPlayer::new(pms1,pns1,
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Err(CommonError::Fail(String::from(
												"ready process fail."
											)))
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![])
		);

		let player2 = MockPlayer::new(pms2,pns2,
										ConsumedIterator::new(vec![Box::new(|player| {
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
										ConsumedIterator::new(vec![])
		);

		let (is,_) = unbounded();

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
			None,None,
			logger, |h,e| {
				if let Some(h) = h {
					let _ = h.lock().map(|h| h.call(e));
				}
			}
		);

		let _ = ts.send(());
	});

	startup(&pmr);

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::TakeReady timed out.");

	assert_eq!(res,Ok(ActionKind::TakeReady));

	for m in vec![
		"An error occurred in player thread.\n  ready process fail.",
		"An error occurred in player 0's thread."
	].into_iter().map(|m| m.to_string()) {
		let res = lr.recv_timeout(Duration::from_millis(60)).expect("attempt to receive log message timed out.");

		assert_eq!(res,m);
	}

	let _ = tr.recv_timeout(Duration::from_millis(350)).expect("attempt to receive on quited timed out.");

	let _ = s.send(String::from(""));
}
#[test]
fn test_error_take_ready_player2() {
	let (pms1,pmr1) = unbounded();
	let (pns1,_) = unbounded();
	let (ts,tr) = unbounded();

	let (pms2,pmr2) = unbounded();
	let (pns2,_) = unbounded();

	let pmr = [pmr1,pmr2];

	let (ls,lr) = unbounded();

	let logger = MockLogger::new(ls);
	let (input_reader,s) = {
		let (s,r) = unbounded();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (es,_) = unbounded();

	let _ = thread::spawn(move || {
		let player1 = MockPlayer::new(pms1,pns1,
										ConsumedIterator::new(vec![Box::new(|player| {
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
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(9,3),KomaDstToPosition(9,4,false)),None))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![])
		);

		let player2 = MockPlayer::new(pms2,pns2,
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Err(CommonError::Fail(String::from(
												"ready process fail."
											)))
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![])
		);

		let (is,_) = unbounded();

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
			None,None,
			logger, |h,e| {
				if let Some(h) = h {
					let _ = h.lock().map(|h| h.call(e));
				}
			}
		);

		let _ = ts.send(());
	});

	startup(&pmr);

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::TakeReady timed out.");

	assert_eq!(res,Ok(ActionKind::TakeReady));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::TakeReady timed out.");

	assert_eq!(res,Ok(ActionKind::TakeReady));

	for m in vec![
		"An error occurred in player thread.\n  ready process fail.",
		"An error occurred in player 1's thread."
	].into_iter().map(|m| m.to_string()) {
		let res = lr.recv_timeout(Duration::from_millis(60)).expect("attempt to receive log message timed out.");

		assert_eq!(res,m);
	}

	let _ = tr.recv_timeout(Duration::from_millis(350)).expect("attempt to receive on quited timed out.");

	let _ = s.send(String::from(""));
}
#[test]
fn test_error_set_position_player1() {
	let (pms1,pmr1) = unbounded();
	let (pns1,_) = unbounded();
	let (ts,tr) = unbounded();

	let (pms2,pmr2) = unbounded();
	let (pns2,_) = unbounded();

	let pmr = [pmr1,pmr2];

	let (ls,lr) = unbounded();

	let logger = MockLogger::new(ls);
	let (input_reader,s) = {
		let (s,r) = unbounded();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (es,er) = unbounded();

	let _ = thread::spawn(move || {
		let player1 = MockPlayer::new(pms1,pns1,
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Err(CommonError::Fail(String::from(
												"set position process fail."
											)))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![])
		);

		let player2 = MockPlayer::new(pms2,pns2,
										ConsumedIterator::new(vec![Box::new(|player| {
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
										ConsumedIterator::new(vec![])
		);

		let (is,_) = unbounded();

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
			None,None,
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

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::GameStart timed out.");

	assert_eq!(res,Ok(EventState::GameStart));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	for m in vec![
		"An error occurred in player thread.\n  set position process fail.",
		"An error occurred in player 0's thread."
	].into_iter().map(|m| m.to_string()) {
		let res = lr.recv_timeout(Duration::from_millis(60)).expect("attempt to receive log message timed out.");

		assert_eq!(res,m);
	}

	let _ = tr.recv_timeout(Duration::from_millis(350)).expect("attempt to receive on quited timed out.");

	let _ = s.send(String::from(""));
}
#[test]
fn test_error_set_position_player2() {
	let (pms1,pmr1) = unbounded();
	let (pns1,_) = unbounded();
	let (ts,tr) = unbounded();

	let (pms2,pmr2) = unbounded();
	let (pns2,_) = unbounded();

	let pmr = [pmr1,pmr2];

	let (ls,lr) = unbounded();

	let logger = MockLogger::new(ls);
	let (input_reader,s) = {
		let (s,r) = unbounded();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (es,er) = unbounded();

	let _ = thread::spawn(move || {
		let player1 = MockPlayer::new(pms1,pns1,
										ConsumedIterator::new(vec![Box::new(|player| {
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
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),None))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![])
		);

		let player2 = MockPlayer::new(pms2,pns2,
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Err(CommonError::Fail(String::from(
												"set position process fail."
											)))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![])
		);

		let (is,_) = unbounded();

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
			None,None,
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

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::GameStart timed out.");

	assert_eq!(res,Ok(EventState::GameStart));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::Moved timed out.");

	assert_eq!(res,Ok(EventState::Moved));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	for m in vec![
		"An error occurred in player thread.\n  set position process fail.",
		"An error occurred in player 1's thread."
	].into_iter().map(|m| m.to_string()) {
		let res = lr.recv_timeout(Duration::from_millis(60)).expect("attempt to receive log message timed out.");

		assert_eq!(res,m);
	}

	let _ = tr.recv_timeout(Duration::from_millis(350)).expect("attempt to receive on quited timed out.");

	let _ = s.send(String::from(""));
}
#[test]
fn test_error_think_player1() {
	let (pms1,pmr1) = unbounded();
	let (pns1,_) = unbounded();
	let (ts,tr) = unbounded();

	let (pms2,pmr2) = unbounded();
	let (pns2,_) = unbounded();

	let pmr = [pmr1,pmr2];

	let (ls,lr) = unbounded();

	let logger = MockLogger::new(ls);
	let (input_reader,s) = {
		let (s,r) = unbounded();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (es,er) = unbounded();

	let _ = thread::spawn(move || {
		let player1 = MockPlayer::new(pms1,pns1,
										ConsumedIterator::new(vec![Box::new(|player| {
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
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Err(CommonError::Fail(String::from(
												"think process fail."
											)))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![])
		);

		let player2 = MockPlayer::new(pms2,pns2,
										ConsumedIterator::new(vec![Box::new(|player| {
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
										ConsumedIterator::new(vec![])
		);

		let (is,_) = unbounded();

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
			None,None,
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

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::GameStart timed out.");

	assert_eq!(res,Ok(EventState::GameStart));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	for m in vec![
		"An error occurred in player thread.\n  think process fail.",
		"An error occurred in player 0's thread."
	].into_iter().map(|m| m.to_string()) {
		let res = lr.recv_timeout(Duration::from_millis(60)).expect("attempt to receive log message timed out.");

		assert_eq!(res,m);
	}

	let _ = tr.recv_timeout(Duration::from_millis(350)).expect("attempt to receive on quited timed out.");

	let _ = s.send(String::from(""));
}
#[test]
fn test_error_set_think_player2() {
	let (pms1,pmr1) = unbounded();
	let (pns1,_) = unbounded();
	let (ts,tr) = unbounded();

	let (pms2,pmr2) = unbounded();
	let (pns2,_) = unbounded();

	let pmr = [pmr1,pmr2];

	let (ls,lr) = unbounded();

	let logger = MockLogger::new(ls);
	let (input_reader,s) = {
		let (s,r) = unbounded();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (es,er) = unbounded();

	let _ = thread::spawn(move || {
		let player1 = MockPlayer::new(pms1,pns1,
										ConsumedIterator::new(vec![Box::new(|player| {
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
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),None))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![])
		);

		let player2 = MockPlayer::new(pms2,pns2,
										ConsumedIterator::new(vec![Box::new(|player| {
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
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Err(CommonError::Fail(String::from(
												"think process fail."
											)))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![])
		);

		let (is,_) = unbounded();

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
			None,None,
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

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::GameStart timed out.");

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

	for m in vec![
		"An error occurred in player thread.\n  think process fail.",
		"An error occurred in player 1's thread."
	].into_iter().map(|m| m.to_string()) {
		let res = lr.recv_timeout(Duration::from_millis(60)).expect("attempt to receive log message timed out.");

		assert_eq!(res,m);
	}

	let _ = tr.recv_timeout(Duration::from_millis(350)).expect("attempt to receive on quited timed out.");

	let _ = s.send(String::from(""));
}
#[test]
fn test_error_ponder_player1_set_position_recv_opponents_turn() {
	let (pms1,pmr1) = unbounded();
	let (pns1,_) = unbounded();
	let (ts,tr) = unbounded();

	let (pms2,pmr2) = unbounded();
	let (pns2,_) = unbounded();

	let pmr = [pmr1,pmr2];

	let (ls,lr) = unbounded();

	let logger = MockLogger::new(ls);
	let (input_reader,s) = {
		let (s,r) = unbounded();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (es,er) = unbounded();

	let _ = thread::spawn(move || {
		let player1 = MockPlayer::new(pms1,pns1,
										ConsumedIterator::new(vec![Box::new(|player| {
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
											Err(CommonError::Fail(String::from(
												"set position process fail."
											)))
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),
												Some(Move::To(KomaSrcPosition(1,3),KomaDstToPosition(1,4,false)))))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![])
		);

		let player2 = MockPlayer::new(pms2,pns2,
										ConsumedIterator::new(vec![Box::new(|player| {
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
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											thread::sleep(Duration::from_millis(350));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,3),KomaDstToPosition(1,4,false)),None))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![])
		);

		let (is,_) = unbounded();

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
			None,None,
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

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::GameStart timed out.");

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

	for m in vec![
		"An error occurred in player thread.\n  set position process fail.",
		"An error occurred in player 0's thread."
	].into_iter().map(|m| m.to_string()) {
		let res = lr.recv_timeout(Duration::from_millis(60)).expect("attempt to receive log message timed out.");

		assert_eq!(res,m);
	}

	let _ = tr.recv_timeout(Duration::from_millis(350)).expect("attempt to receive on quited timed out.");

	let _ = s.send(String::from(""));
}
#[test]
fn test_error_ponder_player1_think_recv_opponents_turn() {
	let (pms1,pmr1) = unbounded();
	let (pns1,_) = unbounded();
	let (ts,tr) = unbounded();

	let (pms2,pmr2) = unbounded();
	let (pns2,_) = unbounded();

	let pmr = [pmr1,pmr2];

	let (ls,lr) = unbounded();

	let logger = MockLogger::new(ls);
	let (input_reader,s) = {
		let (s,r) = unbounded();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (es,er) = unbounded();

	let _ = thread::spawn(move || {
		let player1 = MockPlayer::new(pms1,pns1,
										ConsumedIterator::new(vec![Box::new(|player| {
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
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),
												Some(Move::To(KomaSrcPosition(1,3),KomaDstToPosition(1,4,false)))))
										}),
										Box::new(|player,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Err(CommonError::Fail(String::from(
												"think process fail."
											)))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![])
		);

		let player2 = MockPlayer::new(pms2,pns2,
										ConsumedIterator::new(vec![Box::new(|player| {
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
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											thread::sleep(Duration::from_millis(350));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,3),KomaDstToPosition(1,4,false)),None))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![])
		);

		let (is,_) = unbounded();

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
			None,None,
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

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::GameStart timed out.");

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

	for m in vec![
		"An error occurred in player thread.\n  think process fail.",
		"An error occurred in player 0's thread."
	].into_iter().map(|m| m.to_string()) {
		let res = lr.recv_timeout(Duration::from_millis(60)).expect("attempt to receive log message timed out.");

		assert_eq!(res,m);
	}

	let _ = tr.recv_timeout(Duration::from_millis(350)).expect("attempt to receive on quited timed out.");

	let _ = s.send(String::from(""));
}
#[test]
fn test_error_ponder_player1_set_position_recv_next_turn() {
	let (pms1,pmr1) = unbounded();
	let (pns1,_) = unbounded();
	let (ts,tr) = unbounded();

	let (pms2,pmr2) = unbounded();
	let (pns2,_) = unbounded();

	let pmr = [pmr1,pmr2];

	let (ls,lr) = unbounded();

	let logger = MockLogger::new(ls);
	let (input_reader,s) = {
		let (s,r) = unbounded();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (es,er) = unbounded();

	let _ = thread::spawn(move || {
		let player1 = MockPlayer::new(pms1,pns1,
										ConsumedIterator::new(vec![Box::new(|player| {
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
											thread::sleep(Duration::from_millis(350));
											Err(CommonError::Fail(String::from(
												"set position process fail."
											)))
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),
												Some(Move::To(KomaSrcPosition(1,3),KomaDstToPosition(1,4,false)))))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![])
		);

		let player2 = MockPlayer::new(pms2,pns2,
										ConsumedIterator::new(vec![Box::new(|player| {
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
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,3),KomaDstToPosition(1,4,false)),None))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![])
		);

		let (is,_) = unbounded();

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
			None,None,
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

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::GameStart timed out.");

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

	for m in vec![
		"An error occurred in player thread.\n  set position process fail.",
		"An error occurred in player 0's thread."
	].into_iter().map(|m| m.to_string()) {
		let res = lr.recv_timeout(Duration::from_millis(350)).expect("attempt to receive log message timed out.");

		assert_eq!(res,m);
	}

	let _ = tr.recv_timeout(Duration::from_millis(350)).expect("attempt to receive on quited timed out.");

	let _ = s.send(String::from(""));
}
#[test]
fn test_error_ponder_player1_think_recv_next_turn() {
	let (pms1,pmr1) = unbounded();
	let (pns1,_) = unbounded();
	let (ts,tr) = unbounded();

	let (pms2,pmr2) = unbounded();
	let (pns2,_) = unbounded();

	let pmr = [pmr1,pmr2];

	let (ls,lr) = unbounded();

	let logger = MockLogger::new(ls);
	let (input_reader,s) = {
		let (s,r) = unbounded();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (es,er) = unbounded();

	let _ = thread::spawn(move || {
		let player1 = MockPlayer::new(pms1,pns1,
										ConsumedIterator::new(vec![Box::new(|player| {
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
											thread::sleep(Duration::from_millis(350));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),
												Some(Move::To(KomaSrcPosition(1,3),KomaDstToPosition(1,4,false)))))
										}),
										Box::new(|player,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Err(CommonError::Fail(String::from(
												"think process fail."
											)))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![])
		);

		let player2 = MockPlayer::new(pms2,pns2,
										ConsumedIterator::new(vec![Box::new(|player| {
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
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,3),KomaDstToPosition(1,4,false)),None))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![])
		);

		let (is,_) = unbounded();

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
			None,None,
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

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::GameStart timed out.");

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

	for m in vec![
		"An error occurred in player thread.\n  think process fail.",
		"An error occurred in player 0's thread."
	].into_iter().map(|m| m.to_string()) {
		let res = lr.recv_timeout(Duration::from_millis(350)).expect("attempt to receive log message timed out.");

		assert_eq!(res,m);
	}

	let _ = tr.recv_timeout(Duration::from_millis(350)).expect("attempt to receive on quited timed out.");

	let _ = s.send(String::from(""));
}
#[test]
fn test_error_ponder_player2_set_position_recv_opponents_turn() {
	let (pms1,pmr1) = unbounded();
	let (pns1,_) = unbounded();
	let (ts,tr) = unbounded();

	let (pms2,pmr2) = unbounded();
	let (pns2,_) = unbounded();

	let pmr = [pmr1,pmr2];

	let (ls,lr) = unbounded();

	let logger = MockLogger::new(ls);
	let (input_reader,s) = {
		let (s,r) = unbounded();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (es,er) = unbounded();

	let _ = thread::spawn(move || {
		let player1 = MockPlayer::new(pms1,pns1,
										ConsumedIterator::new(vec![Box::new(|player| {
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
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),None))
										}),
										Box::new(|player,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											thread::sleep(Duration::from_millis(350));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,6),KomaDstToPosition(1,5,false)),None))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![])
		);

		let player2 = MockPlayer::new(pms2,pns2,
										ConsumedIterator::new(vec![Box::new(|player| {
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
											Err(CommonError::Fail(String::from(
												"set position process fail."
											)))
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,3),KomaDstToPosition(1,4,false)),
												Some(Move::To(KomaSrcPosition(1,6),KomaDstToPosition(1,5,false)))))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![])
		);

		let (is,_) = unbounded();

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
			None,None,
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

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::GameStart timed out.");

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

	for m in vec![
		"An error occurred in player thread.\n  set position process fail.",
		"An error occurred in player 1's thread."
	].into_iter().map(|m| m.to_string()) {
		let res = lr.recv_timeout(Duration::from_millis(60)).expect("attempt to receive log message timed out.");

		assert_eq!(res,m);
	}

	let _ = tr.recv_timeout(Duration::from_millis(350)).expect("attempt to receive on quited timed out.");

	let _ = s.send(String::from(""));
}
#[test]
fn test_error_ponder_player2_think_recv_opponents_turn() {
	let (pms1,pmr1) = unbounded();
	let (pns1,_) = unbounded();
	let (ts,tr) = unbounded();

	let (pms2,pmr2) = unbounded();
	let (pns2,_) = unbounded();

	let pmr = [pmr1,pmr2];

	let (ls,lr) = unbounded();

	let logger = MockLogger::new(ls);
	let (input_reader,s) = {
		let (s,r) = unbounded();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (es,er) = unbounded();

	let _ = thread::spawn(move || {
		let player1 = MockPlayer::new(pms1,pns1,
										ConsumedIterator::new(vec![Box::new(|player| {
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
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),None))
										}),
										Box::new(|player,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											thread::sleep(Duration::from_millis(350));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,6),KomaDstToPosition(1,5,false)),None))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![])
		);

		let player2 = MockPlayer::new(pms2,pns2,
										ConsumedIterator::new(vec![Box::new(|player| {
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
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,3),KomaDstToPosition(1,4,false)),
												Some(Move::To(KomaSrcPosition(1,6),KomaDstToPosition(1,5,false)))))
										}),
										Box::new(|player,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Err(CommonError::Fail(String::from(
												"think process fail."
											)))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![])
		);

		let (is,_) = unbounded();

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
			None,None,
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

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::GameStart timed out.");

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

	for m in vec![
		"An error occurred in player thread.\n  think process fail.",
		"An error occurred in player 1's thread."
	].into_iter().map(|m| m.to_string()) {
		let res = lr.recv_timeout(Duration::from_millis(60)).expect("attempt to receive log message timed out.");

		assert_eq!(res,m);
	}

	let _ = tr.recv_timeout(Duration::from_millis(350)).expect("attempt to receive on quited timed out.");

	let _ = s.send(String::from(""));
}
#[test]
fn test_error_ponder_player2_set_position_recv_next_turn() {
	let (pms1,pmr1) = unbounded();
	let (pns1,_) = unbounded();
	let (ts,tr) = unbounded();

	let (pms2,pmr2) = unbounded();
	let (pns2,_) = unbounded();

	let pmr = [pmr1,pmr2];

	let (ls,lr) = unbounded();

	let logger = MockLogger::new(ls);
	let (input_reader,s) = {
		let (s,r) = unbounded();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (es,er) = unbounded();

	let _ = thread::spawn(move || {
		let player1 = MockPlayer::new(pms1,pns1,
										ConsumedIterator::new(vec![Box::new(|player| {
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
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),None))
										}),
										Box::new(|player,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,6),KomaDstToPosition(1,5,false)),None))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![])
		);

		let player2 = MockPlayer::new(pms2,pns2,
										ConsumedIterator::new(vec![Box::new(|player| {
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
											thread::sleep(Duration::from_millis(350));
											Err(CommonError::Fail(String::from(
												"set position process fail."
											)))
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,3),KomaDstToPosition(1,4,false)),
												Some(Move::To(KomaSrcPosition(1,6),KomaDstToPosition(1,5,false)))))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![])
		);

		let (is,_) = unbounded();

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
			None,None,
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

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::GameStart timed out.");

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

	for m in vec![
		"An error occurred in player thread.\n  set position process fail.",
		"An error occurred in player 1's thread."
	].into_iter().map(|m| m.to_string()) {
		let res = lr.recv_timeout(Duration::from_millis(350)).expect("attempt to receive log message timed out.");

		assert_eq!(res,m);
	}

	let _ = tr.recv_timeout(Duration::from_millis(350)).expect("attempt to receive on quited timed out.");

	let _ = s.send(String::from(""));
}
#[test]
fn test_error_ponder_player2_set_think_recv_next_turn() {
	let (pms1,pmr1) = unbounded();
	let (pns1,_) = unbounded();
	let (ts,tr) = unbounded();

	let (pms2,pmr2) = unbounded();
	let (pns2,_) = unbounded();

	let pmr = [pmr1,pmr2];

	let (ls,lr) = unbounded();

	let logger = MockLogger::new(ls);
	let (input_reader,s) = {
		let (s,r) = unbounded();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (es,er) = unbounded();

	let _ = thread::spawn(move || {
		let player1 = MockPlayer::new(pms1,pns1,
										ConsumedIterator::new(vec![Box::new(|player| {
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
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),None))
										}),
										Box::new(|player,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,6),KomaDstToPosition(1,5,false)),None))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![])
		);

		let player2 = MockPlayer::new(pms2,pns2,
										ConsumedIterator::new(vec![Box::new(|player| {
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
											thread::sleep(Duration::from_millis(350));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,3),KomaDstToPosition(1,4,false)),
												Some(Move::To(KomaSrcPosition(1,6),KomaDstToPosition(1,5,false)))))
										}),
										Box::new(|player,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Err(CommonError::Fail(String::from(
												"think process fail."
											)))
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![])
		);

		let (is,_) = unbounded();

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
			None,None,
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

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::GameStart timed out.");

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

	for m in vec![
		"An error occurred in player thread.\n  think process fail.",
		"An error occurred in player 1's thread."
	].into_iter().map(|m| m.to_string()) {
		let res = lr.recv_timeout(Duration::from_millis(350)).expect("attempt to receive log message timed out.");

		assert_eq!(res,m);
	}

	let _ = tr.recv_timeout(Duration::from_millis(350)).expect("attempt to receive on quited timed out.");

	let _ = s.send(String::from(""));
}
#[test]
fn test_error_gameover_player1() {
	let (pms1,pmr1) = unbounded();
	let (pns1,_) = unbounded();
	let (ts,tr) = unbounded();

	let (pms2,pmr2) = unbounded();
	let (pns2,_) = unbounded();

	let pmr = [pmr1,pmr2];

	let (ls,lr) = unbounded();

	let logger = MockLogger::new(ls);
	let (input_reader,s) = {
		let (s,r) = unbounded();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (es,er) = unbounded();

	let _ = thread::spawn(move || {
		let player1 = MockPlayer::new(pms1,pns1,
										ConsumedIterator::new(vec![Box::new(|player| {
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
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_| {
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

											Err(CommonError::Fail(String::from(
												"gameover process fail."
											)))
										})])
		);

		let player2 = MockPlayer::new(pms2,pns2,
										ConsumedIterator::new(vec![Box::new(|player| {
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

		let (is,_) = unbounded();

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
			None,None,
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

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::GameStart timed out.");

	assert_eq!(res,Ok(EventState::GameStart));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	for m in vec![
		"An error occurred in player thread.\n  gameover process fail.",
		"An error occurred in player 0's thread."
	].into_iter().map(|m| m.to_string()) {
		let res = lr.recv_timeout(Duration::from_millis(60)).expect("attempt to receive log message timed out.");

		assert_eq!(res,m);
	}

	let _ = tr.recv_timeout(Duration::from_millis(350)).expect("attempt to receive on quited timed out.");

	let _ = s.send(String::from(""));
}
#[test]
fn test_error_gameover_player2() {
	let (pms1,pmr1) = unbounded();
	let (pns1,_) = unbounded();
	let (ts,tr) = unbounded();

	let (pms2,pmr2) = unbounded();
	let (pns2,_) = unbounded();

	let pmr = [pmr1,pmr2];

	let (ls,lr) = unbounded();

	let logger = MockLogger::new(ls);
	let (input_reader,s) = {
		let (s,r) = unbounded();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (es,er) = unbounded();

	let _ = thread::spawn(move || {
		let player1 = MockPlayer::new(pms1,pns1,
										ConsumedIterator::new(vec![Box::new(|player| {
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
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),None))
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
										ConsumedIterator::new(vec![Box::new(|player| {
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
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_| {
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

											Err(CommonError::Fail(String::from(
												"gameover process fail."
											)))
										})])
		);

		let (is,_) = unbounded();

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
			None,None,
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

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::GameStart timed out.");

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

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	for m in vec![
		"An error occurred in player thread.\n  gameover process fail.",
		"An error occurred in player 1's thread."
	].into_iter().map(|m| m.to_string()) {
		let res = lr.recv_timeout(Duration::from_millis(60)).expect("attempt to receive log message timed out.");

		assert_eq!(res,m);
	}

	let _ = tr.recv_timeout(Duration::from_millis(350)).expect("attempt to receive on quited timed out.");

	let _ = s.send(String::from(""));
}
