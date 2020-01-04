use std::thread;
use std::time::Duration;

use crossbeam_channel::unbounded;
use usiagent::selfmatch::*;
use usiagent::shogi::*;
use usiagent::command::*;
use usiagent::event::*;
use usiagent::rule::BANMEN_START_POS;
use usiagent::error::*;

use super::*;

#[test]
fn test_kifuwrite_7times() {
	let (pms1,pmr1) = unbounded();
	let (pns1,_) = unbounded();
	let (ts,tr) = unbounded();

	let (pms2,pmr2) = unbounded();
	let (pns2,_) = unbounded();

	let pmr = [pmr1,pmr2];

	let logger = StdErrorLogger::new();
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
										}),
										Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										}),
										Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										}),
										Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										}),
										Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										}),
										Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										}),
										Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										}),
										Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										}),
										Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										}),
										Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										}),
										Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										}),
										Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,t,banmen,ms,mg,_,m| {
											if t != Teban::Sente {
												let _ = player.sender.send(Err(String::from("Teban is invalid.")));
											} else if banmen != BANMEN_START_POS {
												let _ = player.sender.send(Err(String::from("Banmen is invalid.")));
											} else if !ms.is_empty() || !mg.is_empty() {
												let _ = player.sender.send(Err(String::from("mochigoma is invalid.")));
											} else if m != vec![
												Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false))
											] {
												let _ = player.sender.send(Err(String::from("moves is invalid.")));
											} else {
												let _ = player.sender.send(Ok(ActionKind::SetPosition));
											}
											Ok(())
										}),
										Box::new(|player,t,banmen,ms,mg,_,m| {
											if t != Teban::Gote {
												let _ = player.sender.send(Err(String::from("Teban is invalid.")));
											} else if banmen != Banmen([
												[GKyou,Blank,Blank,Blank,Blank,Blank,Blank,GKei,GKyou],
												[Blank,Blank,Blank,Blank,Blank,SFuN,Blank,GKin,GOu],
												[Blank,Blank,GKei,GFu,Blank,SGin,Blank,Blank,Blank],
												[GFu,Blank,GFu,Blank,Blank,Blank,Blank,SFu,GFu],
												[Blank,Blank,Blank,SFu,Blank,Blank,SGin,GFu,Blank],
												[Blank,SFu,SFu,GKaku,Blank,Blank,SFu,Blank,SFu],
												[SFu,Blank,Blank,Blank,Blank,Blank,SKin,SGin,Blank],
												[SHisha,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
												[SKyou,SKei,Blank,Blank,Blank,Blank,GKaku,SOu,SKyou]
											]) {
												let _ = player.sender.send(Err(String::from("Banmen is invalid.")));
											} else if ms.get(&MochigomaKind::Fu).map(|&c| c).unwrap_or(0) > 0 ||
														ms.get(&MochigomaKind::Kyou).map(|&c| c).unwrap_or(0) > 0 ||
														ms.get(&MochigomaKind::Kei).map(|&c| c).unwrap_or(0) > 0 ||
														ms.get(&MochigomaKind::Gin).map(|&c| c).unwrap_or(0) != 0 ||
														ms.get(&MochigomaKind::Kin).map(|&c| c).unwrap_or(0) != 1 ||
														ms.get(&MochigomaKind::Kaku).map(|&c| c).unwrap_or(0) > 0 ||
														ms.get(&MochigomaKind::Hisha).map(|&c| c).unwrap_or(0) != 1 ||
														mg.get(&MochigomaKind::Fu).map(|&c| c).unwrap_or(0) != 5 ||
														mg.get(&MochigomaKind::Kyou).map(|&c| c).unwrap_or(0) > 0 ||
														mg.get(&MochigomaKind::Kei).map(|&c| c).unwrap_or(0) != 1 ||
														mg.get(&MochigomaKind::Gin).map(|&c| c).unwrap_or(0) != 1 ||
														mg.get(&MochigomaKind::Kin).map(|&c| c).unwrap_or(0) != 1 ||
														mg.get(&MochigomaKind::Kaku).map(|&c| c).unwrap_or(0) > 0 ||
														mg.get(&MochigomaKind::Hisha).map(|&c| c).unwrap_or(0) > 0 {
												let _ = player.sender.send(Err(String::from("mochigoma is invalid.")));
											} else if m != vec![] {
												let _ = player.sender.send(Err(String::from("moves is invalid.")));
											} else {
												let _ = player.sender.send(Ok(ActionKind::SetPosition));
											}
											Ok(())
										}),
										Box::new(|player,t,banmen,ms,mg,_,m| {
											if t != Teban::Gote {
												let _ = player.sender.send(Err(String::from("Teban is invalid.")));
											} else if banmen != Banmen([
												[GKyou,Blank,Blank,Blank,Blank,Blank,Blank,GKei,GKyou],
												[Blank,Blank,Blank,Blank,Blank,SFuN,Blank,GKin,GOu],
												[Blank,Blank,GKei,GFu,Blank,SGin,Blank,Blank,Blank],
												[GFu,Blank,GFu,Blank,Blank,Blank,Blank,SFu,GFu],
												[Blank,Blank,Blank,SFu,Blank,Blank,SGin,GFu,Blank],
												[Blank,SFu,SFu,GKaku,Blank,Blank,SFu,Blank,SFu],
												[SFu,Blank,Blank,Blank,Blank,Blank,SKin,SGin,Blank],
												[SHisha,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
												[SKyou,SKei,Blank,Blank,Blank,Blank,GKaku,SOu,SKyou]
											]) {
												let _ = player.sender.send(Err(String::from("Banmen is invalid.")));
											} else if ms.get(&MochigomaKind::Fu).map(|&c| c).unwrap_or(0) > 0 ||
														ms.get(&MochigomaKind::Kyou).map(|&c| c).unwrap_or(0) > 0 ||
														ms.get(&MochigomaKind::Kei).map(|&c| c).unwrap_or(0) > 0 ||
														ms.get(&MochigomaKind::Gin).map(|&c| c).unwrap_or(0) != 0 ||
														ms.get(&MochigomaKind::Kin).map(|&c| c).unwrap_or(0) != 1 ||
														ms.get(&MochigomaKind::Kaku).map(|&c| c).unwrap_or(0) > 0 ||
														ms.get(&MochigomaKind::Hisha).map(|&c| c).unwrap_or(0) != 1 ||
														mg.get(&MochigomaKind::Fu).map(|&c| c).unwrap_or(0) != 5 ||
														mg.get(&MochigomaKind::Kyou).map(|&c| c).unwrap_or(0) > 0 ||
														mg.get(&MochigomaKind::Kei).map(|&c| c).unwrap_or(0) != 1 ||
														mg.get(&MochigomaKind::Gin).map(|&c| c).unwrap_or(0) != 1 ||
														mg.get(&MochigomaKind::Kin).map(|&c| c).unwrap_or(0) != 1 ||
														mg.get(&MochigomaKind::Kaku).map(|&c| c).unwrap_or(0) > 0 ||
														mg.get(&MochigomaKind::Hisha).map(|&c| c).unwrap_or(0) > 0 {
												let _ = player.sender.send(Err(String::from("mochigoma is invalid.")));
											} else if m != vec![
												Move::To(KomaSrcPosition(9,4),KomaDstToPosition(9,5,false)),
												Move::To(KomaSrcPosition(1,6),KomaDstToPosition(1,5,false))
											] {
												let _ = player.sender.send(Err(String::from("moves is invalid.")));
											} else {
												let _ = player.sender.send(Ok(ActionKind::SetPosition));
											}
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Resign)
										}),
										Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Resign)
										}),
										Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Resign)
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
										}),
										Box::new(|player,s,_| {
											match s {
												&GameEndState::Lose => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										}),
										Box::new(|player,s,_| {
											match s {
												&GameEndState::Win => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										}),
										Box::new(|player,s,_| {
											match s {
												&GameEndState::Lose => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										}),
										Box::new(|player,s,_| {
											match s {
												&GameEndState::Win => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										}),
										Box::new(|player,s,_| {
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
										}),
										Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										}),
										Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										}),
										Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										}),
										Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										}),
										Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										}),
										Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										}),
										Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										}),
										Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										}),
										Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										}),
										Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										}),
										Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,t,banmen,ms,mg,_,m| {
											if t != Teban::Sente {
												let _ = player.sender.send(Err(String::from("Teban is invalid.")));
											} else if banmen != BANMEN_START_POS {
												let _ = player.sender.send(Err(String::from("Banmen is invalid.")));
											} else if !ms.is_empty() || !mg.is_empty() {
												let _ = player.sender.send(Err(String::from("mochigoma is invalid.")));
											} else if m != vec![] {
												let _ = player.sender.send(Err(String::from("moves is invalid.")));
											} else {
												let _ = player.sender.send(Ok(ActionKind::SetPosition));
											}
											Ok(())
										}),
										Box::new(|player,t,banmen,ms,mg,_,m| {
											if t != Teban::Sente {
												let _ = player.sender.send(Err(String::from("Teban is invalid.")));
											} else if banmen != BANMEN_START_POS {
												let _ = player.sender.send(Err(String::from("Banmen is invalid.")));
											} else if !ms.is_empty() || !mg.is_empty() {
												let _ = player.sender.send(Err(String::from("mochigoma is invalid.")));
											} else if m != vec![
												Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),
												Move::To(KomaSrcPosition(9,3),KomaDstToPosition(9,4,false))
											] {
												let _ = player.sender.send(Err(String::from("moves is invalid.")));
											} else {
												let _ = player.sender.send(Ok(ActionKind::SetPosition));
											}
											Ok(())
										}),
										Box::new(|player,t,banmen,ms,mg,_,m| {
											if t != Teban::Gote {
												let _ = player.sender.send(Err(String::from("Teban is invalid.")));
											} else if banmen != Banmen([
												[GKyou,Blank,Blank,Blank,Blank,Blank,Blank,GKei,GKyou],
												[Blank,Blank,Blank,Blank,Blank,SFuN,Blank,GKin,GOu],
												[Blank,Blank,GKei,GFu,Blank,SGin,Blank,Blank,Blank],
												[GFu,Blank,GFu,Blank,Blank,Blank,Blank,SFu,GFu],
												[Blank,Blank,Blank,SFu,Blank,Blank,SGin,GFu,Blank],
												[Blank,SFu,SFu,GKaku,Blank,Blank,SFu,Blank,SFu],
												[SFu,Blank,Blank,Blank,Blank,Blank,SKin,SGin,Blank],
												[SHisha,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
												[SKyou,SKei,Blank,Blank,Blank,Blank,GKaku,SOu,SKyou]
											]) {
												let _ = player.sender.send(Err(String::from("Banmen is invalid.")));
											} else if ms.get(&MochigomaKind::Fu).map(|&c| c).unwrap_or(0) > 0 ||
														ms.get(&MochigomaKind::Kyou).map(|&c| c).unwrap_or(0) > 0 ||
														ms.get(&MochigomaKind::Kei).map(|&c| c).unwrap_or(0) > 0 ||
														ms.get(&MochigomaKind::Gin).map(|&c| c).unwrap_or(0) != 0 ||
														ms.get(&MochigomaKind::Kin).map(|&c| c).unwrap_or(0) != 1 ||
														ms.get(&MochigomaKind::Kaku).map(|&c| c).unwrap_or(0) > 0 ||
														ms.get(&MochigomaKind::Hisha).map(|&c| c).unwrap_or(0) != 1 ||
														mg.get(&MochigomaKind::Fu).map(|&c| c).unwrap_or(0) != 5 ||
														mg.get(&MochigomaKind::Kyou).map(|&c| c).unwrap_or(0) > 0 ||
														mg.get(&MochigomaKind::Kei).map(|&c| c).unwrap_or(0) != 1 ||
														mg.get(&MochigomaKind::Gin).map(|&c| c).unwrap_or(0) != 1 ||
														mg.get(&MochigomaKind::Kin).map(|&c| c).unwrap_or(0) != 1 ||
														mg.get(&MochigomaKind::Kaku).map(|&c| c).unwrap_or(0) > 0 ||
														mg.get(&MochigomaKind::Hisha).map(|&c| c).unwrap_or(0) > 0 {
												let _ = player.sender.send(Err(String::from("mochigoma is invalid.")));
											} else if m != vec![
												Move::To(KomaSrcPosition(9,4),KomaDstToPosition(9,5,false))
											] {
												let _ = player.sender.send(Err(String::from("moves is invalid.")));
											} else {
												let _ = player.sender.send(Ok(ActionKind::SetPosition));
											}
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Resign)
										}),
										Box::new(|player,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Resign)
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
										}),
										Box::new(|player,s,_| {
											match s {
												&GameEndState::Win => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										}),
										Box::new(|player,s,_| {
											match s {
												&GameEndState::Lose => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										}),
										Box::new(|player,s,_| {
											match s {
												&GameEndState::Win => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										}),
										Box::new(|player,s,_| {
											match s {
												&GameEndState::Lose => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										}),
										Box::new(|player,s,_| {
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
		let mut it = [
					"startpos",
					"startpos moves 1g1f",
					"startpos moves 1g1f 9c9d",
					"sfen l6nl/5+P1gk/2np1S3/p1p4Pp/3P2Sp1/1PPb2P1P/P5GS1/R8/LN4bKL w RGgsn5p 1",
					"sfen l6nl/5+P1gk/2np1S3/p1p4Pp/3P2Sp1/1PPb2P1P/P5GS1/R8/LN4bKL w RGgsn5p 1 moves 9d9e",
					"sfen l6nl/5+P1gk/2np1S3/p1p4Pp/3P2Sp1/1PPb2P1P/P5GS1/R8/LN4bKL w RGgsn5p 1 moves 9d9e 1f1e",
				].into_iter().map(|s| s.to_string());


		let _ = engine.start(|self_match_event_dispatcher| {
			let hes = es.clone();

			self_match_event_dispatcher
				.add_handler(SelfMatchEventKind::GameStart, move |_,e| {
					match e {
						&SelfMatchEvent::GameStart(_,_,_) => {
							let _ = hes.send(Ok(EventState::GameStart));
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
						&SelfMatchEvent::GameEnd(SelfMatchGameEndState::Resign(_)) => {
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
			|| true,
			Some(Box::new(move || it.next().unwrap())),
			None,
			input_reader, input_read_handler,
			player1,player2,
			create_options(), create_options(),
			info_sender,
			UsiGoTimeLimit::None,
			None,Some(6),
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

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::GameEnd timed out.");

	assert_eq!(res,Ok(EventState::GameEnd));

	gamestart_process(&pmr);

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::GameStart timed out.");

	assert_eq!(res,Ok(EventState::GameStart));

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

	gamestart_process(&pmr);

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::GameStart timed out.");

	assert_eq!(res,Ok(EventState::GameStart));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::GameEnd timed out.");

	assert_eq!(res,Ok(EventState::GameEnd));

	gamestart_process(&pmr);

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::GameStart timed out.");

	assert_eq!(res,Ok(EventState::GameStart));

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

	gamestart_process(&pmr);

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::GameStart timed out.");

	assert_eq!(res,Ok(EventState::GameStart));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::GameEnd timed out.");

	assert_eq!(res,Ok(EventState::GameEnd));

	gamestart_process(&pmr);

	let res = er.recv_timeout(Duration::from_millis(60)).expect("attempt to receive EventState::GameStart timed out.");

	assert_eq!(res,Ok(EventState::GameStart));

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

	let res = pmr[0].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let res = pmr[1].recv_timeout(Duration::from_millis(60)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let _ = tr.recv_timeout(Duration::from_millis(180)).expect("attempt to receive on quited timed out.");

	let _ = s.send(String::from(""));
}
