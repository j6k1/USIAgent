use std::thread;
use std::sync::mpsc;
use std::time::Duration;

use usiagent::UsiAgent;
use usiagent::shogi::*;
use usiagent::rule::BANMEN_START_POS;
use usiagent::command::*;
use usiagent::event::*;

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

#[test]
fn test_sequence() {
	let (pms,pmr) = mpsc::channel();
	let (pns,_) = mpsc::channel();
	let (ts,tr) = mpsc::channel();

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

	let _ =thread::spawn(move || {
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
		let agent = UsiAgent::new(player);

		let _ = agent.start(input_reader,output_writer,logger,|h,e| {
			if let Some(h) = h {
				let _ = h.lock().map(|h| h.call(e));
			}
		});

		let _ = ts.send(());
	});

	let _ = s.send(String::from("usi"));

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'id name mockplayer' timed out.");

	assert_eq!(&*res,"id name mockplayer");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'id author j6k1' timed out.");

	assert_eq!(&*res,"id author j6k1");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'option name USI_Hash type spin min 1 max 100' timed out.");

	assert_eq!(&*res,"option name USI_Hash type spin min 1 max 100");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'option name USI_Ponder type check default false' timed out.");

	assert_eq!(&*res,"option name USI_Ponder type check default false");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'usiok' timed out.");

	assert_eq!(&*res,"usiok");

	let _ = s.send(String::from("setoption name USI_Hash value 1"));
	let _ = s.send(String::from("setoption name USI_Ponder value false"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetOption timed out.");

	assert_eq!(res,Ok(ActionKind::SetOption));

	let _ = s.send(String::from("isready"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::TakeReady timed out.");

	assert_eq!(res,Ok(ActionKind::TakeReady));

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'readyok' timed out.");

	assert_eq!(&*res,"readyok");

	let _ = s.send(String::from("usinewgame"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::NewGame timed out.");

	assert_eq!(res,Ok(ActionKind::NewGame));

	let _ = s.send(String::from("position startpos"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let _ = s.send(String::from("go"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'bestmove 1g1f' timed out.");

	assert_eq!(&*res,"bestmove 1g1f");

	let _ = s.send(String::from("position startpos moves 1g1f"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let _ = s.send(String::from("go"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'bestmove resign' timed out.");

	assert_eq!(&*res,"bestmove resign");

	let _ = s.send(String::from("gameover lose"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));


	let _ = s.send(String::from("quit"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let _ = tr.recv_timeout(Duration::from_millis(300)).expect("attempt to receive on quited timed out.");
}
#[test]
fn test_gameover() {
	let (pms,pmr) = mpsc::channel();
	let (pns,_) = mpsc::channel();
	let (ts,tr) = mpsc::channel();

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

	let _ =thread::spawn(move || {
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
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),None))
										}),
										Box::new(|player,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,6),KomaDstToPosition(1,5,false)),None))
										}),
										Box::new(|player,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),None))
										}),
										Box::new(|player,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Resign)
										}),
										Box::new(|player,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),None))
										}),
										Box::new(|player,_,_,_,_| {
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

		let _ = ts.send(());
	});

	let _ = s.send(String::from("usi"));

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'id name mockplayer' timed out.");

	assert_eq!(&*res,"id name mockplayer");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'id author j6k1' timed out.");

	assert_eq!(&*res,"id author j6k1");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'option name USI_Hash type spin min 1 max 100' timed out.");

	assert_eq!(&*res,"option name USI_Hash type spin min 1 max 100");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'option name USI_Ponder type check default false' timed out.");

	assert_eq!(&*res,"option name USI_Ponder type check default false");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'usiok' timed out.");

	assert_eq!(&*res,"usiok");

	let _ = s.send(String::from("setoption name USI_Hash value 1"));
	let _ = s.send(String::from("setoption name USI_Ponder value false"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetOption timed out.");

	assert_eq!(res,Ok(ActionKind::SetOption));

	for (lastmove,gamestate) in lastmoves.zip(gamestates) {
		let _ = s.send(String::from("isready"));

		let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::TakeReady timed out.");

		assert_eq!(res,Ok(ActionKind::TakeReady));

		let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'readyok' timed out.");

		assert_eq!(&*res,"readyok");

		let _ = s.send(String::from("usinewgame"));

		let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::NewGame timed out.");

		assert_eq!(res,Ok(ActionKind::NewGame));

		let _ = s.send(String::from("position startpos"));

		let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetPosition timed out.");

		assert_eq!(res,Ok(ActionKind::SetPosition));

		let _ = s.send(String::from("go"));

		let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Think timed out.");

		assert_eq!(res,Ok(ActionKind::Think));

		let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'bestmove 1g1f' timed out.");

		assert_eq!(&*res,"bestmove 1g1f");

		let _ = s.send(String::from("position startpos moves 1g1f"));

		let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetPosition timed out.");

		assert_eq!(res,Ok(ActionKind::SetPosition));

		let _ = s.send(String::from("go"));

		let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Think timed out.");

		assert_eq!(res,Ok(ActionKind::Think));

		let res = r.recv_timeout(Duration::from_millis(150)).expect(format!("attempt to receive '{}' timed out.",lastmove).as_str());

		assert_eq!(&*res,&*lastmove);

		let _ = s.send(gamestate);

		let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::GameOver timed out.");

		assert_eq!(res,Ok(ActionKind::GameOver));
	}

	let _ = s.send(String::from("quit"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let _ = tr.recv_timeout(Duration::from_millis(300)).expect("attempt to receive on quited timed out.");
}
#[test]
fn test_check_kyokumen_with_startpos() {
	let (pms,pmr) = mpsc::channel();
	let (pns,_) = mpsc::channel();
	let (ts,tr) = mpsc::channel();

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

	let _ = thread::spawn(move || {
		let player = MockPlayer::new(pms,pns,
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
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
		let agent = UsiAgent::new(player);

		let _ = agent.start(input_reader,output_writer,logger,|h,e| {
			if let Some(h) = h {
				let _ = h.lock().map(|h| h.call(e));
			}
		});

		let _ = ts.send(());
	});

	let _ = s.send(String::from("usi"));

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'id name mockplayer' timed out.");

	assert_eq!(&*res,"id name mockplayer");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'id author j6k1' timed out.");

	assert_eq!(&*res,"id author j6k1");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'option name USI_Hash type spin min 1 max 100' timed out.");

	assert_eq!(&*res,"option name USI_Hash type spin min 1 max 100");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'option name USI_Ponder type check default false' timed out.");

	assert_eq!(&*res,"option name USI_Ponder type check default false");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'usiok' timed out.");

	assert_eq!(&*res,"usiok");

	let _ = s.send(String::from("setoption name USI_Hash value 1"));
	let _ = s.send(String::from("setoption name USI_Ponder value false"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetOption timed out.");

	assert_eq!(res,Ok(ActionKind::SetOption));

	let _ = s.send(String::from("isready"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::TakeReady timed out.");

	assert_eq!(res,Ok(ActionKind::TakeReady));

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'readyok' timed out.");

	assert_eq!(&*res,"readyok");

	let _ = s.send(String::from("usinewgame"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::NewGame timed out.");

	assert_eq!(res,Ok(ActionKind::NewGame));

	let _ = s.send(String::from("position startpos"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let _ = s.send(String::from("go"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'bestmove 1g1f' timed out.");

	assert_eq!(&*res,"bestmove 1g1f");

	let _ = s.send(String::from("position startpos moves 1g1f 9c9d"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let _ = s.send(String::from("go"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'bestmove resign' timed out.");

	assert_eq!(&*res,"bestmove resign");

	let _ = s.send(String::from("gameover lose"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));


	let _ = s.send(String::from("quit"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let _ = tr.recv_timeout(Duration::from_millis(300)).expect("attempt to receive on quited timed out.");
}
#[test]
fn test_check_kyokumen_with_sfen_sente() {
	let (pms,pmr) = mpsc::channel();
	let (pns,_) = mpsc::channel();
	let (ts,tr) = mpsc::channel();

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

	let _ = thread::spawn(move || {
		let player = MockPlayer::new(pms,pns,
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,t,banmen,ms,mg,_,m| {
											if t != Teban::Sente {
												let _ = player.sender.send(Err(String::from("Teban is invalid.")));
											} else if banmen != Banmen([
												[GKaku,GKei,Blank,Blank,Blank,Blank,Blank, SKakuN,Blank],
												[GKyou,GHisha,Blank,GGin,Blank,GOu,Blank,Blank,Blank],
												[GFu, SKei,Blank,GFu,GFu,GFu,Blank,Blank,GFu],
												[Blank,GFu,GFu,Blank,Blank,Blank,Blank,Blank,Blank],
												[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
												[Blank,Blank, SFu,Blank,Blank,Blank,Blank, SFu,Blank],
												[SFu, SFu,Blank, SFu, SFu, SFu, SFu,Blank, SFu],
												[Blank, SGin,Blank,Blank,Blank, SGin,Blank, SHisha,Blank],
												[SKyou, SKei,Blank, SKin, SOu, SKin,Blank, SKei, SKyou]
											]) {
												let _ = player.sender.send(Err(String::from("Banmen is invalid.")));
											} else if ms.get(&MochigomaKind::Fu).unwrap_or(&0) != &2 ||
												ms.get(&MochigomaKind::Kyou).unwrap_or(&0) != &0 ||
												ms.get(&MochigomaKind::Kei).unwrap_or(&0) != &0 ||
												ms.get(&MochigomaKind::Gin).unwrap_or(&0) != &1 ||
												ms.get(&MochigomaKind::Kin).unwrap_or(&0) != &2 ||
												ms.get(&MochigomaKind::Kaku).unwrap_or(&0) != &0 ||
												ms.get(&MochigomaKind::Hisha).unwrap_or(&0) != &0 ||
												mg.get(&MochigomaKind::Fu).unwrap_or(&0) != &0 ||
												mg.get(&MochigomaKind::Kyou).unwrap_or(&0) != &1 ||
												mg.get(&MochigomaKind::Kei).unwrap_or(&0) != &0 ||
												mg.get(&MochigomaKind::Gin).unwrap_or(&0) != &0 ||
												mg.get(&MochigomaKind::Kin).unwrap_or(&0) != &0 ||
												mg.get(&MochigomaKind::Kaku).unwrap_or(&0) != &0 ||
												mg.get(&MochigomaKind::Hisha).unwrap_or(&0) != &0 {

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
											} else if banmen != Banmen([
												[GKaku,GKei,Blank,Blank,Blank,Blank,Blank, SKakuN,Blank],
												[GKyou,GHisha,Blank,GGin,Blank,GOu,Blank,Blank,Blank],
												[GFu, SKei,Blank,GFu,GFu,GFu,Blank,Blank,GFu],
												[Blank,GFu,GFu,Blank,Blank,Blank,Blank,Blank,Blank],
												[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
												[Blank,Blank, SFu,Blank,Blank,Blank,Blank, SFu,Blank],
												[SFu, SFu,Blank, SFu, SFu, SFu, SFu,Blank, SFu],
												[Blank, SGin,Blank,Blank,Blank, SGin,Blank, SHisha,Blank],
												[SKyou, SKei,Blank, SKin, SOu, SKin,Blank, SKei, SKyou]
											]) {
												let _ = player.sender.send(Err(String::from("Banmen is invalid.")));
											} else if ms.get(&MochigomaKind::Fu).unwrap_or(&0) != &2 ||
												ms.get(&MochigomaKind::Kyou).unwrap_or(&0) != &0 ||
												ms.get(&MochigomaKind::Kei).unwrap_or(&0) != &0 ||
												ms.get(&MochigomaKind::Gin).unwrap_or(&0) != &1 ||
												ms.get(&MochigomaKind::Kin).unwrap_or(&0) != &2 ||
												ms.get(&MochigomaKind::Kaku).unwrap_or(&0) != &0 ||
												ms.get(&MochigomaKind::Hisha).unwrap_or(&0) != &0 ||
												mg.get(&MochigomaKind::Fu).unwrap_or(&0) != &0 ||
												mg.get(&MochigomaKind::Kyou).unwrap_or(&0) != &1 ||
												mg.get(&MochigomaKind::Kei).unwrap_or(&0) != &0 ||
												mg.get(&MochigomaKind::Gin).unwrap_or(&0) != &0 ||
												mg.get(&MochigomaKind::Kin).unwrap_or(&0) != &0 ||
												mg.get(&MochigomaKind::Kaku).unwrap_or(&0) != &0 ||
												mg.get(&MochigomaKind::Hisha).unwrap_or(&0) != &0 {

												let _ = player.sender.send(Err(String::from("mochigoma is invalid.")));
											} else if m != vec![
												Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),
												Move::Put(MochigomaKind::Kyou,KomaDstPutPosition(5,5))
											] {
												let _ = player.sender.send(Err(String::from("moves is invalid.")));
											} else {
												let _ = player.sender.send(Ok(ActionKind::SetPosition));
											}
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
		let agent = UsiAgent::new(player);

		let _ = agent.start(input_reader,output_writer,logger,|h,e| {
			if let Some(h) = h {
				let _ = h.lock().map(|h| h.call(e));
			}
		});

		let _ = ts.send(());
	});

	let _ = s.send(String::from("usi"));

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'id name mockplayer' timed out.");

	assert_eq!(&*res,"id name mockplayer");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'id author j6k1' timed out.");

	assert_eq!(&*res,"id author j6k1");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'option name USI_Hash type spin min 1 max 100' timed out.");

	assert_eq!(&*res,"option name USI_Hash type spin min 1 max 100");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'option name USI_Ponder type check default false' timed out.");

	assert_eq!(&*res,"option name USI_Ponder type check default false");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'usiok' timed out.");

	assert_eq!(&*res,"usiok");

	let _ = s.send(String::from("setoption name USI_Hash value 1"));
	let _ = s.send(String::from("setoption name USI_Ponder value false"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetOption timed out.");

	assert_eq!(res,Ok(ActionKind::SetOption));

	let _ = s.send(String::from("isready"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::TakeReady timed out.");

	assert_eq!(res,Ok(ActionKind::TakeReady));

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'readyok' timed out.");

	assert_eq!(&*res,"readyok");

	let _ = s.send(String::from("usinewgame"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::NewGame timed out.");

	assert_eq!(res,Ok(ActionKind::NewGame));

	let _ = s.send(String::from("position sfen bn5+B1/lr1s1k3/pN1ppp2p/1pp6/9/2P4P1/PP1PPPP1P/1S3S1R1/LN1GKG1NL b 2GS2Pl 1"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let _ = s.send(String::from("go"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'bestmove 1g1f' timed out.");

	assert_eq!(&*res,"bestmove 1g1f");

	let _ = s.send(String::from("position sfen bn5+B1/lr1s1k3/pN1ppp2p/1pp6/9/2P4P1/PP1PPPP1P/1S3S1R1/LN1GKG1NL b 2GS2Pl 1 moves 1g1f L*5e"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let _ = s.send(String::from("go"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'bestmove resign' timed out.");

	assert_eq!(&*res,"bestmove resign");

	let _ = s.send(String::from("gameover lose"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));


	let _ = s.send(String::from("quit"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let _ = tr.recv_timeout(Duration::from_millis(300)).expect("attempt to receive on quited timed out.");
}
#[test]
fn test_check_kyokumen_with_sfen_gote() {
	let (pms,pmr) = mpsc::channel();
	let (pns,_) = mpsc::channel();
	let (ts,tr) = mpsc::channel();

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

	let _ = thread::spawn(move || {
		let player = MockPlayer::new(pms,pns,
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,t,banmen,ms,mg,_,m| {
											if t != Teban::Gote {
												let _ = player.sender.send(Err(String::from("Teban is invalid.")));
											} else if banmen != Banmen([
												[GKaku,GKei,Blank,Blank,Blank,Blank,Blank, SKakuN,Blank],
												[GKyou,GHisha,Blank,GGin,Blank,GOu,Blank,Blank,Blank],
												[GFu, SKei,Blank,GFu,GFu,GFu,Blank,Blank,GFu],
												[Blank,GFu,GFu,Blank,Blank,Blank,Blank,Blank,Blank],
												[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
												[Blank,Blank, SFu,Blank,Blank,Blank,Blank, SFu,Blank],
												[SFu, SFu,Blank, SFu, SFu, SFu, SFu,Blank, SFu],
												[Blank, SGin,Blank,Blank,Blank, SGin,Blank, SHisha,Blank],
												[SKyou, SKei,Blank, SKin, SOu, SKin,Blank, SKei, SKyou]
											]) {
												let _ = player.sender.send(Err(String::from("Banmen is invalid.")));
											} else if ms.get(&MochigomaKind::Fu).unwrap_or(&0) != &2 ||
												ms.get(&MochigomaKind::Kyou).unwrap_or(&0) != &0 ||
												ms.get(&MochigomaKind::Kei).unwrap_or(&0) != &0 ||
												ms.get(&MochigomaKind::Gin).unwrap_or(&0) != &1 ||
												ms.get(&MochigomaKind::Kin).unwrap_or(&0) != &2 ||
												ms.get(&MochigomaKind::Kaku).unwrap_or(&0) != &0 ||
												ms.get(&MochigomaKind::Hisha).unwrap_or(&0) != &0 ||
												mg.get(&MochigomaKind::Fu).unwrap_or(&0) != &0 ||
												mg.get(&MochigomaKind::Kyou).unwrap_or(&0) != &1 ||
												mg.get(&MochigomaKind::Kei).unwrap_or(&0) != &0 ||
												mg.get(&MochigomaKind::Gin).unwrap_or(&0) != &0 ||
												mg.get(&MochigomaKind::Kin).unwrap_or(&0) != &0 ||
												mg.get(&MochigomaKind::Kaku).unwrap_or(&0) != &0 ||
												mg.get(&MochigomaKind::Hisha).unwrap_or(&0) != &0 {

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
												[GKaku,GKei,Blank,Blank,Blank,Blank,Blank, SKakuN,Blank],
												[GKyou,GHisha,Blank,GGin,Blank,GOu,Blank,Blank,Blank],
												[GFu, SKei,Blank,GFu,GFu,GFu,Blank,Blank,GFu],
												[Blank,GFu,GFu,Blank,Blank,Blank,Blank,Blank,Blank],
												[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
												[Blank,Blank, SFu,Blank,Blank,Blank,Blank, SFu,Blank],
												[SFu, SFu,Blank, SFu, SFu, SFu, SFu,Blank, SFu],
												[Blank, SGin,Blank,Blank,Blank, SGin,Blank, SHisha,Blank],
												[SKyou, SKei,Blank, SKin, SOu, SKin,Blank, SKei, SKyou]
											]) {
												let _ = player.sender.send(Err(String::from("Banmen is invalid.")));
											} else if ms.get(&MochigomaKind::Fu).unwrap_or(&0) != &2 ||
												ms.get(&MochigomaKind::Kyou).unwrap_or(&0) != &0 ||
												ms.get(&MochigomaKind::Kei).unwrap_or(&0) != &0 ||
												ms.get(&MochigomaKind::Gin).unwrap_or(&0) != &1 ||
												ms.get(&MochigomaKind::Kin).unwrap_or(&0) != &2 ||
												ms.get(&MochigomaKind::Kaku).unwrap_or(&0) != &0 ||
												ms.get(&MochigomaKind::Hisha).unwrap_or(&0) != &0 ||
												mg.get(&MochigomaKind::Fu).unwrap_or(&0) != &0 ||
												mg.get(&MochigomaKind::Kyou).unwrap_or(&0) != &1 ||
												mg.get(&MochigomaKind::Kei).unwrap_or(&0) != &0 ||
												mg.get(&MochigomaKind::Gin).unwrap_or(&0) != &0 ||
												mg.get(&MochigomaKind::Kin).unwrap_or(&0) != &0 ||
												mg.get(&MochigomaKind::Kaku).unwrap_or(&0) != &0 ||
												mg.get(&MochigomaKind::Hisha).unwrap_or(&0) != &0 {

												let _ = player.sender.send(Err(String::from("mochigoma is invalid.")));
											} else if m != vec![
												Move::Put(MochigomaKind::Kyou,KomaDstPutPosition(5,5)),
												Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false))
											] {
												let _ = player.sender.send(Err(String::from("moves is invalid.")));
											} else {
												let _ = player.sender.send(Ok(ActionKind::SetPosition));
											}
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::Put(MochigomaKind::Kyou,KomaDstPutPosition(5,5)),None))
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
		let agent = UsiAgent::new(player);

		let _ = agent.start(input_reader,output_writer,logger,|h,e| {
			if let Some(h) = h {
				let _ = h.lock().map(|h| h.call(e));
			}
		});

		let _ = ts.send(());
	});

	let _ = s.send(String::from("usi"));

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'id name mockplayer' timed out.");

	assert_eq!(&*res,"id name mockplayer");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'id author j6k1' timed out.");

	assert_eq!(&*res,"id author j6k1");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'option name USI_Hash type spin min 1 max 100' timed out.");

	assert_eq!(&*res,"option name USI_Hash type spin min 1 max 100");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'option name USI_Ponder type check default false' timed out.");

	assert_eq!(&*res,"option name USI_Ponder type check default false");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'usiok' timed out.");

	assert_eq!(&*res,"usiok");

	let _ = s.send(String::from("setoption name USI_Hash value 1"));
	let _ = s.send(String::from("setoption name USI_Ponder value false"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetOption timed out.");

	assert_eq!(res,Ok(ActionKind::SetOption));

	let _ = s.send(String::from("isready"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::TakeReady timed out.");

	assert_eq!(res,Ok(ActionKind::TakeReady));

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'readyok' timed out.");

	assert_eq!(&*res,"readyok");

	let _ = s.send(String::from("usinewgame"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::NewGame timed out.");

	assert_eq!(res,Ok(ActionKind::NewGame));

	let _ = s.send(String::from("position sfen bn5+B1/lr1s1k3/pN1ppp2p/1pp6/9/2P4P1/PP1PPPP1P/1S3S1R1/LN1GKG1NL w 2GS2Pl 1"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let _ = s.send(String::from("go"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'bestmovef L*5e' timed out.");

	assert_eq!(&*res,"bestmove L*5e");

	let _ = s.send(String::from("position sfen bn5+B1/lr1s1k3/pN1ppp2p/1pp6/9/2P4P1/PP1PPPP1P/1S3S1R1/LN1GKG1NL w 2GS2Pl 1 moves L*5e 1g1f"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let _ = s.send(String::from("go"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'bestmove resign' timed out.");

	assert_eq!(&*res,"bestmove resign");

	let _ = s.send(String::from("gameover lose"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));


	let _ = s.send(String::from("quit"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let _ = tr.recv_timeout(Duration::from_millis(300)).expect("attempt to receive on quited timed out.");
}
#[test]
fn test_ponderhit_move_already_been_decided() {
	let (pms,pmr) = mpsc::channel();
	let (pns,_) = mpsc::channel();
	let (ts,tr) = mpsc::channel();

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

	let _ =thread::spawn(move || {
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
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),
																Some(Move::To(KomaSrcPosition(9,3),KomaDstToPosition(9,4,false)))))
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
		let agent = UsiAgent::new(player);

		let _ = agent.start(input_reader,output_writer,logger,|h,e| {
			if let Some(h) = h {
				let _ = h.lock().map(|h| h.call(e));
			}
		});

		let _ = ts.send(());
	});

	let _ = s.send(String::from("usi"));

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'id name mockplayer' timed out.");

	assert_eq!(&*res,"id name mockplayer");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'id author j6k1' timed out.");

	assert_eq!(&*res,"id author j6k1");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'option name USI_Hash type spin min 1 max 100' timed out.");

	assert_eq!(&*res,"option name USI_Hash type spin min 1 max 100");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'option name USI_Ponder type check default false' timed out.");

	assert_eq!(&*res,"option name USI_Ponder type check default false");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'usiok' timed out.");

	assert_eq!(&*res,"usiok");

	let _ = s.send(String::from("setoption name USI_Hash value 1"));
	let _ = s.send(String::from("setoption name USI_Ponder value false"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetOption timed out.");

	assert_eq!(res,Ok(ActionKind::SetOption));

	let _ = s.send(String::from("isready"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::TakeReady timed out.");

	assert_eq!(res,Ok(ActionKind::TakeReady));

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'readyok' timed out.");

	assert_eq!(&*res,"readyok");

	let _ = s.send(String::from("usinewgame"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::NewGame timed out.");

	assert_eq!(res,Ok(ActionKind::NewGame));

	let _ = s.send(String::from("position startpos"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let _ = s.send(String::from("go"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'bestmove 1g1f' timed out.");

	assert_eq!(&*res,"bestmove 1g1f ponder 9c9d");

	let _ = s.send(String::from("position startpos moves 1g1f 9c9d"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let _ = s.send(String::from("go ponder"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	thread::sleep(Duration::from_millis(300));

	if let Ok(s) = r.recv_timeout(Duration::from_millis(150)) {
		if s.starts_with("bestmove ") {
			assert!(false,"Move returned before accept ponderhit.");
		} else {
			assert!(false,format!("An unexpected command '{}' was returned.",s));
		}
	}

	let _  = s.send(String::from("ponderhit"));

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'bestmove resign' timed out.");

	assert_eq!(&*res,"bestmove resign");

	let _ = s.send(String::from("gameover lose"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));


	let _ = s.send(String::from("quit"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let _ = tr.recv_timeout(Duration::from_millis(300)).expect("attempt to receive on quited timed out.");
}
#[test]
fn test_ponderhit_thinking() {
	let (pms,pmr) = mpsc::channel();
	let (pns,_) = mpsc::channel();
	let (ts,tr) = mpsc::channel();

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

	let _ =thread::spawn(move || {
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
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),
																Some(Move::To(KomaSrcPosition(9,3),KomaDstToPosition(9,4,false)))))
										}),
										Box::new(|player,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));

											thread::sleep(Duration::from_millis(300));
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

		let _ = ts.send(());
	});

	let _ = s.send(String::from("usi"));

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'id name mockplayer' timed out.");

	assert_eq!(&*res,"id name mockplayer");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'id author j6k1' timed out.");

	assert_eq!(&*res,"id author j6k1");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'option name USI_Hash type spin min 1 max 100' timed out.");

	assert_eq!(&*res,"option name USI_Hash type spin min 1 max 100");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'option name USI_Ponder type check default false' timed out.");

	assert_eq!(&*res,"option name USI_Ponder type check default false");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'usiok' timed out.");

	assert_eq!(&*res,"usiok");

	let _ = s.send(String::from("setoption name USI_Hash value 1"));
	let _ = s.send(String::from("setoption name USI_Ponder value false"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetOption timed out.");

	assert_eq!(res,Ok(ActionKind::SetOption));

	let _ = s.send(String::from("isready"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::TakeReady timed out.");

	assert_eq!(res,Ok(ActionKind::TakeReady));

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'readyok' timed out.");

	assert_eq!(&*res,"readyok");

	let _ = s.send(String::from("usinewgame"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::NewGame timed out.");

	assert_eq!(res,Ok(ActionKind::NewGame));

	let _ = s.send(String::from("position startpos"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let _ = s.send(String::from("go"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'bestmove 1g1f' timed out.");

	assert_eq!(&*res,"bestmove 1g1f ponder 9c9d");

	let _ = s.send(String::from("position startpos moves 1g1f 9c9d"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let _ = s.send(String::from("go ponder"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	if let Ok(s) = r.recv_timeout(Duration::from_millis(150)) {
		if s.starts_with("bestmove ") {
			assert!(false,"Move returned before accept ponderhit.");
		} else {
			assert!(false,format!("An unexpected command '{}' was returned.",s));
		}
	}

	let _  = s.send(String::from("ponderhit"));

	thread::sleep(Duration::from_millis(300));

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'bestmove resign' timed out.");

	assert_eq!(&*res,"bestmove resign");

	let _ = s.send(String::from("gameover lose"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));


	let _ = s.send(String::from("quit"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let _ = tr.recv_timeout(Duration::from_millis(300)).expect("attempt to receive on quited timed out.");
}
#[test]
fn test_ponderng_move_already_been_decided() {
	let (pms,pmr) = mpsc::channel();
	let (pns,_) = mpsc::channel();
	let (ts,tr) = mpsc::channel();

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

	let _ =thread::spawn(move || {
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
										}),
										Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),
																Some(Move::To(KomaSrcPosition(9,3),KomaDstToPosition(9,4,false)))))
										}),
										Box::new(|player,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));

											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,6),KomaDstToPosition(1,5,false)),None))
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
		let agent = UsiAgent::new(player);

		let _ = agent.start(input_reader,output_writer,logger,|h,e| {
			if let Some(h) = h {
				let _ = h.lock().map(|h| h.call(e));
			}
		});

		let _ = ts.send(());
	});

	let _ = s.send(String::from("usi"));

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'id name mockplayer' timed out.");

	assert_eq!(&*res,"id name mockplayer");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'id author j6k1' timed out.");

	assert_eq!(&*res,"id author j6k1");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'option name USI_Hash type spin min 1 max 100' timed out.");

	assert_eq!(&*res,"option name USI_Hash type spin min 1 max 100");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'option name USI_Ponder type check default false' timed out.");

	assert_eq!(&*res,"option name USI_Ponder type check default false");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'usiok' timed out.");

	assert_eq!(&*res,"usiok");

	let _ = s.send(String::from("setoption name USI_Hash value 1"));
	let _ = s.send(String::from("setoption name USI_Ponder value false"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetOption timed out.");

	assert_eq!(res,Ok(ActionKind::SetOption));

	let _ = s.send(String::from("isready"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::TakeReady timed out.");

	assert_eq!(res,Ok(ActionKind::TakeReady));

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'readyok' timed out.");

	assert_eq!(&*res,"readyok");

	let _ = s.send(String::from("usinewgame"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::NewGame timed out.");

	assert_eq!(res,Ok(ActionKind::NewGame));

	let _ = s.send(String::from("position startpos"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let _ = s.send(String::from("go"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'bestmove 1g1f' timed out.");

	assert_eq!(&*res,"bestmove 1g1f ponder 9c9d");

	let _ = s.send(String::from("position startpos moves 1g1f 9c9d"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let _ = s.send(String::from("go ponder"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	thread::sleep(Duration::from_millis(300));

	if let Ok(s) = r.recv_timeout(Duration::from_millis(150)) {
		if s.starts_with("bestmove ") {
			assert!(false,"Move returned before accept ponderhit.");
		} else {
			assert!(false,format!("An unexpected command '{}' was returned.",s));
		}
	}

	let _  = s.send(String::from("stop"));

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'bestmove resign' timed out.");

	assert_eq!(&*res,"bestmove 1f1e");

	let _ = s.send(String::from("position startpos moves 1g1f 8c8d"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let _ = s.send(String::from("go"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'bestmove resign' timed out.");

	assert_eq!(&*res,"bestmove resign");

	let _ = s.send(String::from("gameover lose"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));


	let _ = s.send(String::from("quit"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let _ = tr.recv_timeout(Duration::from_millis(300)).expect("attempt to receive on quited timed out.");
}
#[test]
fn test_ponderng_thinking() {
	let (pms,pmr) = mpsc::channel();
	let (pns,_) = mpsc::channel();
	let (ts,tr) = mpsc::channel();

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

	let _ =thread::spawn(move || {
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
										}),
										Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),
																Some(Move::To(KomaSrcPosition(9,3),KomaDstToPosition(9,4,false)))))
										}),
										Box::new(|player,_,_,_,mut handle_events| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											while !player.stop {
												handle_events(player)?;
												thread::sleep(Duration::from_millis(10));
											}
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,6),KomaDstToPosition(1,5,false)),None))
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
		let agent = UsiAgent::new(player);

		let _ = agent.start(input_reader,output_writer,logger,|h,e| {
			if let Some(h) = h {
				let _ = h.lock().map(|h| h.call(e));
			}
		});

		let _ = ts.send(());
	});

	let _ = s.send(String::from("usi"));

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'id name mockplayer' timed out.");

	assert_eq!(&*res,"id name mockplayer");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'id author j6k1' timed out.");

	assert_eq!(&*res,"id author j6k1");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'option name USI_Hash type spin min 1 max 100' timed out.");

	assert_eq!(&*res,"option name USI_Hash type spin min 1 max 100");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'option name USI_Ponder type check default false' timed out.");

	assert_eq!(&*res,"option name USI_Ponder type check default false");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'usiok' timed out.");

	assert_eq!(&*res,"usiok");

	let _ = s.send(String::from("setoption name USI_Hash value 1"));
	let _ = s.send(String::from("setoption name USI_Ponder value false"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetOption timed out.");

	assert_eq!(res,Ok(ActionKind::SetOption));

	let _ = s.send(String::from("isready"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::TakeReady timed out.");

	assert_eq!(res,Ok(ActionKind::TakeReady));

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'readyok' timed out.");

	assert_eq!(&*res,"readyok");

	let _ = s.send(String::from("usinewgame"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::NewGame timed out.");

	assert_eq!(res,Ok(ActionKind::NewGame));

	let _ = s.send(String::from("position startpos"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let _ = s.send(String::from("go"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'bestmove 1g1f' timed out.");

	assert_eq!(&*res,"bestmove 1g1f ponder 9c9d");

	let _ = s.send(String::from("position startpos moves 1g1f 9c9d"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let _ = s.send(String::from("go ponder"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	thread::sleep(Duration::from_millis(300));

	if let Ok(s) = r.recv_timeout(Duration::from_millis(150)) {
		if s.starts_with("bestmove ") {
			assert!(false,"Move returned before accept ponderhit.");
		} else {
			assert!(false,format!("An unexpected command '{}' was returned.",s));
		}
	}

	let _  = s.send(String::from("stop"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::OnStop timed out.");

	assert_eq!(res,Ok(ActionKind::OnStop));

	let res = r.recv_timeout(Duration::from_millis(180)).expect("attempt to receive 'bestmove 1f1e' timed out.");

	assert_eq!(&*res,"bestmove 1f1e");

	let _ = s.send(String::from("position startpos moves 1g1f 8c8d"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let _ = s.send(String::from("go"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'bestmove resign' timed out.");

	assert_eq!(&*res,"bestmove resign");

	let _ = s.send(String::from("gameover lose"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let _ = s.send(String::from("quit"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let _ = tr.recv_timeout(Duration::from_millis(300)).expect("attempt to receive on quited timed out.");
}
