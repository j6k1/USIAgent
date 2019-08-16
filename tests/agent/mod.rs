use std::thread;
use std::sync::mpsc;
use std::time::Duration;

use usiagent::UsiAgent;
use usiagent::shogi::*;
use usiagent::command::*;
use usiagent::event::*;

use common::*;

#[test]
fn test_sequence() {
	let (pms,pmr) = mpsc::channel();
	let (pns,pnr) = mpsc::channel();

	let logger = MockLogger::new();
	let (input_reader,s) = {
		let (s,r) = mpsc::channel();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (output_writer,r) = {
		let (s,r) = mpsc::channel();

		let output_writer = MockOutputWriter::new(s);
		(output_writer,r)
	};

	let h = thread::spawn(move || {
		let player = MockPlayer::new(pms,pns,
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
										ConsumedIterator::new(vec![Box::new(|player,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),None))
										}),
										Box::new(|player,_,_,_| {
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
		let agent = UsiAgent::new(player);

		let _ = agent.start(input_reader,output_writer,logger,|h,e| {
			if let Some(h) = h {
				let _ = h.lock().map(|h| h.call(e));
			}
		});
	});

	let _ = s.send(String::from("usi"));

	let res = r.recv_timeout(Duration::from_millis(150)).unwrap();

	assert_eq!(&*res,"id name mockplayer");

	let res = r.recv_timeout(Duration::from_millis(150)).unwrap();

	assert_eq!(&*res,"id author j6k1");

	let res = r.recv_timeout(Duration::from_millis(150)).unwrap();

	assert_eq!(&*res,"option name USI_Hash type spin min 1 max 100");

	let res = r.recv_timeout(Duration::from_millis(150)).unwrap();

	assert_eq!(&*res,"option name USI_Ponder type check default false");

	let res = r.recv_timeout(Duration::from_millis(150)).unwrap();

	assert_eq!(&*res,"usiok");

	let _ = s.send(String::from("setoption name USI_Hash value 1"));
	let _ = s.send(String::from("setoption name USI_Ponder value false"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).unwrap();

	assert_eq!(res,Ok(ActionKind::SetOption));

	let _ = s.send(String::from("isready"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).unwrap();

	assert_eq!(res,Ok(ActionKind::TakeReady));

	let res = r.recv_timeout(Duration::from_millis(150)).unwrap();

	assert_eq!(&*res,"readyok");

	let _ = s.send(String::from("usinewgame"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).unwrap();

	assert_eq!(res,Ok(ActionKind::NewGame));

	let _ = s.send(String::from("position startpos"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).unwrap();

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let _ = s.send(String::from("go"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).unwrap();

	assert_eq!(res,Ok(ActionKind::Think));

	let res = r.recv_timeout(Duration::from_millis(150)).unwrap();

	assert_eq!(&*res,"bestmove 1g1f");

	let _ = s.send(String::from("position startpos moves 1g1f"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).unwrap();

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let _ = s.send(String::from("go"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).unwrap();

	assert_eq!(res,Ok(ActionKind::Think));

	let res = r.recv_timeout(Duration::from_millis(150)).unwrap();

	assert_eq!(&*res,"bestmove resign");

	let _ = s.send(String::from("gameover lose"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).unwrap();

	assert_eq!(res,Ok(ActionKind::GameOver));


	let _ = s.send(String::from("quit"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).unwrap();

	assert_eq!(res,Ok(ActionKind::Quit));
}

#[test]
fn test_gameover() {
	let (pms,pmr) = mpsc::channel();
	let (pns,pnr) = mpsc::channel();

	let logger = MockLogger::new();
	let (input_reader,s) = {
		let (s,r) = mpsc::channel();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (output_writer,r) = {
		let (s,r) = mpsc::channel();

		let output_writer = MockOutputWriter::new(s);
		(output_writer,r)
	};

	let lastmoves = vec![
		String::from("bestmove 1f1e"),
		String::from("bestmove resign"),
		String::from("bestmove 1f1e"),
	].into_iter();

	let gamestates = vec![
		String::from("gameover win"),
		String::from("gameover lose"),
		String::from("gameover draw"),
	].into_iter();

	let h = thread::spawn(move || {
		let player = MockPlayer::new(pms,pns,
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
										ConsumedIterator::new(vec![Box::new(|player,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),None))
										}),
										Box::new(|player,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,6),KomaDstToPosition(1,5,false)),None))
										}),
										Box::new(|player,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),None))
										}),
										Box::new(|player,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Resign)
										}),
										Box::new(|player,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),None))
										}),
										Box::new(|player,_,_,_| {
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
												&GameEndState::Draw => {
													let _ = player.sender.send(Ok(ActionKind::GameOver));
												},
												_ => {
													let _ = player.sender.send(Err(String::from("gameend state is invalid.")));
												}
											}

											Ok(())
										})])
		);
		let agent = UsiAgent::new(player);

		let _ = agent.start(input_reader,output_writer,logger,|h,e| {
			if let Some(h) = h {
				let _ = h.lock().map(|h| h.call(e));
			}
		});
	});

	let _ = s.send(String::from("usi"));

	let res = r.recv_timeout(Duration::from_millis(150)).unwrap();

	assert_eq!(&*res,"id name mockplayer");

	let res = r.recv_timeout(Duration::from_millis(150)).unwrap();

	assert_eq!(&*res,"id author j6k1");

	let res = r.recv_timeout(Duration::from_millis(150)).unwrap();

	assert_eq!(&*res,"option name USI_Hash type spin min 1 max 100");

	let res = r.recv_timeout(Duration::from_millis(150)).unwrap();

	assert_eq!(&*res,"option name USI_Ponder type check default false");

	let res = r.recv_timeout(Duration::from_millis(150)).unwrap();

	assert_eq!(&*res,"usiok");

	let _ = s.send(String::from("setoption name USI_Hash value 1"));
	let _ = s.send(String::from("setoption name USI_Ponder value false"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).unwrap();

	assert_eq!(res,Ok(ActionKind::SetOption));

	for (lastmove,gamestate) in lastmoves.zip(gamestates) {
		let _ = s.send(String::from("isready"));

		let res = pmr.recv_timeout(Duration::from_millis(150)).unwrap();

		assert_eq!(res,Ok(ActionKind::TakeReady));

		let res = r.recv_timeout(Duration::from_millis(150)).unwrap();

		assert_eq!(&*res,"readyok");

		let _ = s.send(String::from("usinewgame"));

		let res = pmr.recv_timeout(Duration::from_millis(150)).unwrap();

		assert_eq!(res,Ok(ActionKind::NewGame));

		let _ = s.send(String::from("position startpos"));

		let res = pmr.recv_timeout(Duration::from_millis(150)).unwrap();

		assert_eq!(res,Ok(ActionKind::SetPosition));

		let _ = s.send(String::from("go"));

		let res = pmr.recv_timeout(Duration::from_millis(150)).unwrap();

		assert_eq!(res,Ok(ActionKind::Think));

		let res = r.recv_timeout(Duration::from_millis(150)).unwrap();

		assert_eq!(&*res,"bestmove 1g1f");

		let _ = s.send(String::from("position startpos moves 1g1f"));

		let res = pmr.recv_timeout(Duration::from_millis(150)).unwrap();

		assert_eq!(res,Ok(ActionKind::SetPosition));

		let _ = s.send(String::from("go"));

		let res = pmr.recv_timeout(Duration::from_millis(150)).unwrap();

		assert_eq!(res,Ok(ActionKind::Think));

		let res = r.recv_timeout(Duration::from_millis(150)).unwrap();

		assert_eq!(&*res,&*lastmove);

		let _ = s.send(gamestate);

		let res = pmr.recv_timeout(Duration::from_millis(150)).unwrap();

		assert_eq!(res,Ok(ActionKind::GameOver));
	}

	let _ = s.send(String::from("quit"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).unwrap();

	assert_eq!(res,Ok(ActionKind::Quit));
}