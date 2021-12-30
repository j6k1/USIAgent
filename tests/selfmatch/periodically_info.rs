use std::thread;
use std::time::Duration;

use std::sync::mpsc;
use usiagent::selfmatch::*;
use usiagent::shogi::*;
use usiagent::command::*;
use usiagent::event::*;
use usiagent::error::*;

use super::*;

#[test]
fn test_periodically_info_send() {
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

	let (output_writer,_) = {
		let (s,r) = mpsc::channel();

		let output_writer = MockOutputWriter::new(s);
		(output_writer,r)
	};

	let output_writer =  Arc::new(Mutex::new(output_writer));

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
									  ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
										  let _ = player.sender.send(Ok(ActionKind::Think));
										  Ok(BestMove::Move(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),None))
									  }),
																 Box::new(|player,_,_,_,_,start_periodically_info,_| {
																	 let _ = player.sender.send(Ok(ActionKind::Think));

																	 let _pinfo_sender = start_periodically_info(200,Box::new(|| {
																		 let mut commands = vec![];
																		 commands.push(UsiInfoSubCommand::Nps(10));
																		 commands.push(UsiInfoSubCommand::Nodes(100));

																		 commands
																	 }));

																	 thread::sleep(Duration::from_millis(500));
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
									  ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
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
							 USIPeriodicallyInfo::new(output_writer,false),
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

	let res = r.recv_timeout(Duration::from_millis(220)).expect("attempt to receive 'info depth 1 seldepth 3 time 10000 nodes 1000000 score cp -100 currmove 1g1f hashfull 10000 nps 100 multipv 1 pv 1g1f 9c9d 1f1e' timed out.");

	assert_eq!(&*res,"info nps 10 nodes 100");

	let res = r.recv_timeout(Duration::from_millis(220)).expect("attempt to receive 'info depth 1 seldepth 3 time 10000 nodes 1000000 score cp -100 currmove 1g1f hashfull 10000 nps 100 multipv 1 pv 1g1f 9c9d 1f1e' timed out.");

	assert_eq!(&*res,"info nps 10 nodes 100");

	let res = pmr[0].recv_timeout(Duration::from_millis(300)).expect("attempt to receive ActionKind::GameOver timed out.");

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
fn test_periodically_info_send_drop_sender() {
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

	let (output_writer,_) = {
		let (s,r) = mpsc::channel();

		let output_writer = MockOutputWriter::new(s);
		(output_writer,r)
	};

	let output_writer =  Arc::new(Mutex::new(output_writer));

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
									  ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
										  let _ = player.sender.send(Ok(ActionKind::Think));
										  Ok(BestMove::Move(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),None))
									  }),
																 Box::new(|player,_,_,_,_,start_periodically_info,_| {
																	 let _ = player.sender.send(Ok(ActionKind::Think));

																	 let _ = start_periodically_info(200,Box::new(|| {
																		 let mut commands = vec![];
																		 commands.push(UsiInfoSubCommand::Nps(10));
																		 commands.push(UsiInfoSubCommand::Nodes(100));

																		 commands
																	 }));

																	 thread::sleep(Duration::from_millis(500));
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
									  ConsumedIterator::new(vec![Box::new(|player,_,_,_,_,_,_| {
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
							 USIPeriodicallyInfo::new(output_writer,false),
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

	if let Ok(_) = r.recv_timeout(Duration::from_millis(500)) {
		assert!(false,"It didn't time out where it was expected to time out.");
	}

	let res = pmr[0].recv_timeout(Duration::from_millis(300)).expect("attempt to receive ActionKind::GameOver timed out.");

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
