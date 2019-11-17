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

	let _ = tr.recv_timeout(Duration::from_millis(180)).expect("attempt to receive on quited timed out.");

	let _ = s.send(String::from(""));
}
