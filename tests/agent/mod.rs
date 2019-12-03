use std::thread;
use std::time::Duration;

use crossbeam_channel::Sender;
use crossbeam_channel::Receiver;
use crossbeam_channel::unbounded;

use usiagent::UsiAgent;
use usiagent::player::USIPlayer;
use usiagent::rule::*;
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

fn startup(s:&Sender<String>,r:&Receiver<String>,pmr:&Receiver<Result<ActionKind,String>>) {

	let _ = s.send(String::from("usi"));

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'id name mockplayer' timed out.");

	assert_eq!(&*res,"id name mockplayer");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'id author j6k1' timed out.");

	assert_eq!(&*res,"id author j6k1");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'option name OptionButton type button' timed out.");

	assert_eq!(&*res,"option name OptionButton type button");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'option name OptionCheck type check' timed out.");

	assert_eq!(&*res,"option name OptionCheck type check");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'option name OptionCombo type combo default bbbb var bbbb var cccc' timed out.");

	assert_eq!(&*res,"option name OptionCombo type combo default bbbb var bbbb var cccc");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'option name OptionCombo2 type combo var dddd var eeee' timed out.");

	assert_eq!(&*res,"option name OptionCombo2 type combo var dddd var eeee");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'option name OptionFileName type filename default filename.");

	assert_eq!(&*res,"option name OptionFileName type filename default filename.");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'option name OptionFileName2 type filename");

	assert_eq!(&*res,"option name OptionFileName2 type filename");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'option name OptionSpin type spin default 50 min 5 max 50");

	assert_eq!(&*res,"option name OptionSpin type spin default 10 min 5 max 50");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'option name OptionString type string default string.");

	assert_eq!(&*res,"option name OptionString type string default string.");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'option name OptionString2 type string");

	assert_eq!(&*res,"option name OptionString2 type string");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'option name USI_Hash type spin min 1 max 100' timed out.");

	assert_eq!(&*res,"option name USI_Hash type spin min 1 max 100");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'option name USI_Ponder type check default false' timed out.");

	assert_eq!(&*res,"option name USI_Ponder type check default false");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'usiok' timed out.");

	assert_eq!(&*res,"usiok");

	let _ = s.send(String::from("setoption name OptionButton"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetOption timed out.");

	assert_eq!(res,Ok(ActionKind::SetOption));

	let _ = s.send(String::from("setoption name OptionCheck value true"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetOption timed out.");

	assert_eq!(res,Ok(ActionKind::SetOption));

	let _ = s.send(String::from("setoption name OptionCombo value cccc"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetOption timed out.");

	assert_eq!(res,Ok(ActionKind::SetOption));

	let _ = s.send(String::from("setoption name OptionCombo2 value eeee"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetOption timed out.");

	assert_eq!(res,Ok(ActionKind::SetOption));

	let _ = s.send(String::from("setoption name OptionFileName value book.bin"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetOption timed out.");

	assert_eq!(res,Ok(ActionKind::SetOption));

	let _ = s.send(String::from("setoption name OptionFileName2 value book2.bin"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetOption timed out.");

	assert_eq!(res,Ok(ActionKind::SetOption));

	let _ = s.send(String::from("setoption name OptionSpin value 25"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetOption timed out.");

	assert_eq!(res,Ok(ActionKind::SetOption));

	let _ = s.send(String::from("setoption name OptionString value string.."));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetOption timed out.");

	assert_eq!(res,Ok(ActionKind::SetOption));

	let _ = s.send(String::from("setoption name OptionString2 value string..."));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetOption timed out.");

	assert_eq!(res,Ok(ActionKind::SetOption));

	let _ = s.send(String::from("setoption name USI_Hash value 1000"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetOption timed out.");

	assert_eq!(res,Ok(ActionKind::SetOption));

	let _ = s.send(String::from("setoption name USI_Ponder value false"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetOption timed out.");

	assert_eq!(res,Ok(ActionKind::SetOption));
}
#[test]
fn test_sequence() {
	let (pms,pmr) = unbounded();
	let (pns,_) = unbounded();
	let (ts,tr) = unbounded();

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = unbounded();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (output_writer,r) = {
		let (s,r) = unbounded();

		let output_writer = MockOutputWriter::new(s);
		(output_writer,r)
	};

	let _ = thread::spawn(move || {
		let player = MockPlayer::new(pms,pns,
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

	startup(&s,&r,&pmr);

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
	let (pms,pmr) = unbounded();
	let (pns,_) = unbounded();
	let (ts,tr) = unbounded();

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = unbounded();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (output_writer,r) = {
		let (s,r) = unbounded();

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

	let _ = thread::spawn(move || {
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

	startup(&s,&r,&pmr);

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
	let (pms,pmr) = unbounded();
	let (pns,_) = unbounded();
	let (ts,tr) = unbounded();

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = unbounded();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (output_writer,r) = {
		let (s,r) = unbounded();

		let output_writer = MockOutputWriter::new(s);
		(output_writer,r)
	};

	let _ = thread::spawn(move || {
		let player = MockPlayer::new(pms,pns,
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
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

	startup(&s,&r,&pmr);

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
	let (pms,pmr) = unbounded();
	let (pns,_) = unbounded();
	let (ts,tr) = unbounded();

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = unbounded();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (output_writer,r) = {
		let (s,r) = unbounded();

		let output_writer = MockOutputWriter::new(s);
		(output_writer,r)
	};

	let _ = thread::spawn(move || {
		let player = MockPlayer::new(pms,pns,
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
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

	startup(&s,&r,&pmr);

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
	let (pms,pmr) = unbounded();
	let (pns,_) = unbounded();
	let (ts,tr) = unbounded();

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = unbounded();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (output_writer,r) = {
		let (s,r) = unbounded();

		let output_writer = MockOutputWriter::new(s);
		(output_writer,r)
	};

	let _ = thread::spawn(move || {
		let player = MockPlayer::new(pms,pns,
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
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

	startup(&s,&r,&pmr);

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
fn test_check_kyokumen_nowait() {
	let (pms,pmr) = unbounded();
	let (pns,_) = unbounded();
	let (ts,tr) = unbounded();

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = unbounded();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (output_writer,r) = {
		let (s,r) = unbounded();

		let output_writer = MockOutputWriter::new(s);
		(output_writer,r)
	};

	let _ = thread::spawn(move || {
		let player = MockPlayer::new(pms,pns,
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::TakeReady));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player| {
											let _ = player.sender.send(Ok(ActionKind::NewGame));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,t,banmen,ms,mg,_,m| {
											if !player.started {
												let _ = player.sender.send(Err(String::from("player not started.")));
											} else if t != Teban::Sente {
												let _ = player.sender.send(Err(String::from("Teban is invalid.")));
											} else if banmen != BANMEN_START_POS {
												let _ = player.sender.send(Err(String::from("Banmen is invalid.")));
											} else if !ms.is_empty() || !mg.is_empty() {
												let _ = player.sender.send(Err(String::from("mochigoma is invalid.")));
											} else if m != vec![] {
												let _ = player.sender.send(Err(String::from("moves is invalid.")));
											} else {
												let _ = player.sender.send(Ok(ActionKind::SetPosition));
												let mc = MochigomaCollections::Pair(ms,mg);

												let (t,state,mc,_) = player.apply_moves(t,State::new(banmen),
														mc,m.into_iter()
														.map(|m| m.to_applied_move())
														.collect::<Vec<AppliedMove>>(),
														(),
														|_,_,_,_,_,_,_| {

														});
												player.kyokumen = Some(Kyokumen {
													state:state,
													mc:mc,
													teban:t
												});
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
												let mc = MochigomaCollections::Pair(ms,mg);

												let (t,state,mc,_) = player.apply_moves(t,State::new(banmen),
														mc,m.into_iter()
														.map(|m| m.to_applied_move())
														.collect::<Vec<AppliedMove>>(),
														(),
														|_,_,_,_,_,_,_| {

														});
												player.kyokumen = Some(Kyokumen {
													state:state,
													mc:mc,
													teban:t
												});
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
												Move::To(KomaSrcPosition(9,3),KomaDstToPosition(9,4,false)),
												Move::To(KomaSrcPosition(1,6),KomaDstToPosition(1,5,false)),
												Move::To(KomaSrcPosition(9,4),KomaDstToPosition(9,5,false))
											] {
												let _ = player.sender.send(Err(String::from("moves is invalid.")));
											} else {
												let _ = player.sender.send(Ok(ActionKind::SetPosition));
												let mc = MochigomaCollections::Pair(ms,mg);

												let (t,state,mc,_) = player.apply_moves(t,State::new(banmen),
														mc,m.into_iter()
														.map(|m| m.to_applied_move())
														.collect::<Vec<AppliedMove>>(),
														(),
														|_,_,_,_,_,_,_| {

														});
												player.kyokumen = Some(Kyokumen {
													state:state,
													mc:mc,
													teban:t
												});
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
												Move::To(KomaSrcPosition(9,3),KomaDstToPosition(9,4,false)),
												Move::To(KomaSrcPosition(1,6),KomaDstToPosition(1,5,false)),
												Move::To(KomaSrcPosition(9,4),KomaDstToPosition(9,5,false)),
												Move::To(KomaSrcPosition(1,5),KomaDstToPosition(1,4,false)),
												Move::To(KomaSrcPosition(9,5),KomaDstToPosition(9,6,false))
											] {
												let _ = player.sender.send(Err(String::from("moves is invalid.")));
											} else {
												let _ = player.sender.send(Ok(ActionKind::SetPosition));
												let mc = MochigomaCollections::Pair(ms,mg);

												let (t,state,mc,_) = player.apply_moves(t,State::new(banmen),
														mc,m.into_iter()
														.map(|m| m.to_applied_move())
														.collect::<Vec<AppliedMove>>(),
														(),
														|_,_,_,_,_,_,_| {

														});
												player.kyokumen = Some(Kyokumen {
													state:state,
													mc:mc,
													teban:t
												});
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
												Move::To(KomaSrcPosition(9,3),KomaDstToPosition(9,4,false)),
												Move::To(KomaSrcPosition(1,6),KomaDstToPosition(1,5,false)),
												Move::To(KomaSrcPosition(9,4),KomaDstToPosition(9,5,false)),
												Move::To(KomaSrcPosition(1,5),KomaDstToPosition(1,4,false)),
												Move::To(KomaSrcPosition(9,5),KomaDstToPosition(9,6,false)),
												Move::To(KomaSrcPosition(1,4),KomaDstToPosition(1,3,true)),
												Move::To(KomaSrcPosition(9,6),KomaDstToPosition(9,7,true))
											] {
												let _ = player.sender.send(Err(String::from("moves is invalid.")));
											} else {
												let _ = player.sender.send(Ok(ActionKind::SetPosition));
												let mc = MochigomaCollections::Pair(ms,mg);

												let (t,state,mc,_) = player.apply_moves(t,State::new(banmen),
														mc,m.into_iter()
														.map(|m| m.to_applied_move())
														.collect::<Vec<AppliedMove>>(),
														(),
														|_,_,_,_,_,_,_| {

														});
												player.kyokumen = Some(Kyokumen {
													state:state,
													mc:mc,
													teban:t
												});
											}
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_| {
											if let Some(kyokumen) = player.kyokumen.as_ref() {
												if kyokumen.state == State::new(Banmen([
													[GKyou,GKei,GGin,GKin,GOu,GKin,GGin,GKei,GKyou],
													[Blank,GHisha,Blank,Blank,Blank,Blank,Blank,GKaku,Blank],
													[GFu,GFu,GFu,GFu,GFu,GFu,GFu,GFu,GFu],
													[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
													[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
													[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
													[SFu,SFu,SFu,SFu,SFu,SFu,SFu,SFu,SFu],
													[Blank,SKaku,Blank,Blank,Blank,Blank,Blank,SHisha,Blank],
													[SKyou,SKei,SGin,SKin,SOu,SKin,SGin,SKei,SKyou]
												])) && kyokumen.teban == Teban::Sente && kyokumen.mc.is_empty() {
													let _ = player.sender.send(Ok(ActionKind::Think));
												} else {
													let _ = player.sender.send(Err(String::from("kyokumen is invalid.")));
												}
											} else {
												let _ = player.sender.send(Err(String::from("kyokumen is not set.")));
											}
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),None))
										}),Box::new(|player,_,_,_,_| {
											if let Some(kyokumen) = player.kyokumen.as_ref() {
												if kyokumen.state == State::new(Banmen([
													[GKyou,GKei,GGin,GKin,GOu,GKin,GGin,GKei,GKyou],
													[Blank,GHisha,Blank,Blank,Blank,Blank,Blank,GKaku,Blank],
													[Blank,GFu,GFu,GFu,GFu,GFu,GFu,GFu,GFu],
													[GFu,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
													[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
													[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,SFu],
													[SFu,SFu,SFu,SFu,SFu,SFu,SFu,SFu,Blank],
													[Blank,SKaku,Blank,Blank,Blank,Blank,Blank,SHisha,Blank],
													[SKyou,SKei,SGin,SKin,SOu,SKin,SGin,SKei,SKyou]
												])) && kyokumen.teban == Teban::Sente && kyokumen.mc.is_empty() {
													let _ = player.sender.send(Ok(ActionKind::Think));
												} else {
													let _ = player.sender.send(Err(String::from("kyokumen is invalid.")));
												}
											} else {
												let _ = player.sender.send(Err(String::from("kyokumen is not set.")));
											}
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,6),KomaDstToPosition(1,5,false)),None))
										}),
										Box::new(|player,_,_,_,_| {
											if let Some(kyokumen) = player.kyokumen.as_ref() {
												if kyokumen.state == State::new(Banmen([
													[GKyou,GKei,GGin,GKin,GOu,GKin,GGin,GKei,GKyou],
													[Blank,GHisha,Blank,Blank,Blank,Blank,Blank,GKaku,Blank],
													[Blank,GFu,GFu,GFu,GFu,GFu,GFu,GFu,GFu],
													[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
													[GFu,Blank,Blank,Blank,Blank,Blank,Blank,Blank,SFu],
													[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
													[SFu,SFu,SFu,SFu,SFu,SFu,SFu,SFu,Blank],
													[Blank,SKaku,Blank,Blank,Blank,Blank,Blank,SHisha,Blank],
													[SKyou,SKei,SGin,SKin,SOu,SKin,SGin,SKei,SKyou]
												])) && kyokumen.teban == Teban::Sente && kyokumen.mc.is_empty() {
													let _ = player.sender.send(Ok(ActionKind::Think));
												} else {
													let _ = player.sender.send(Err(String::from("kyokumen is invalid.")));
												}
											} else {
												let _ = player.sender.send(Err(String::from("kyokumen is not set.")));
											}
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,5),KomaDstToPosition(1,4,false)),None))
										}),
										Box::new(|player,_,_,_,_| {
											if let Some(kyokumen) = player.kyokumen.as_ref() {
												if kyokumen.state == State::new(Banmen([
													[GKyou,GKei,GGin,GKin,GOu,GKin,GGin,GKei,GKyou],
													[Blank,GHisha,Blank,Blank,Blank,Blank,Blank,GKaku,Blank],
													[Blank,GFu,GFu,GFu,GFu,GFu,GFu,GFu,GFu],
													[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,SFu],
													[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
													[GFu,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
													[SFu,SFu,SFu,SFu,SFu,SFu,SFu,SFu,Blank],
													[Blank,SKaku,Blank,Blank,Blank,Blank,Blank,SHisha,Blank],
													[SKyou,SKei,SGin,SKin,SOu,SKin,SGin,SKei,SKyou]
												])) && kyokumen.teban == Teban::Sente && kyokumen.mc.is_empty() {
													let _ = player.sender.send(Ok(ActionKind::Think));
												} else {
													let _ = player.sender.send(Err(String::from("kyokumen is invalid.")));
												}
											} else {
												let _ = player.sender.send(Err(String::from("kyokumen is not set.")));
											}
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,4),KomaDstToPosition(1,3,true)),None))
										}),
										Box::new(|player,_,_,_,_| {
											if let Some(kyokumen) = player.kyokumen.as_ref() {
												if let &MochigomaCollections::Pair(ref ms,ref mg) = &kyokumen.mc {
													if kyokumen.state == State::new(Banmen([
														[GKyou,GKei,GGin,GKin,GOu,GKin,GGin,GKei,GKyou],
														[Blank,GHisha,Blank,Blank,Blank,Blank,Blank,GKaku,Blank],
														[Blank,GFu,GFu,GFu,GFu,GFu,GFu,GFu,SFuN],
														[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
														[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
														[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
														[GFuN,SFu,SFu,SFu,SFu,SFu,SFu,SFu,Blank],
														[Blank,SKaku,Blank,Blank,Blank,Blank,Blank,SHisha,Blank],
														[SKyou,SKei,SGin,SKin,SOu,SKin,SGin,SKei,SKyou]
													])) && kyokumen.teban == Teban::Sente &&
														ms.get(&MochigomaKind::Fu).unwrap_or(&0) ==&1 &&
														ms.get(&MochigomaKind::Kyou).unwrap_or(&0) ==&0 &&
														ms.get(&MochigomaKind::Kei).unwrap_or(&0) ==&0 &&
														ms.get(&MochigomaKind::Gin).unwrap_or(&0) ==&0 &&
														ms.get(&MochigomaKind::Kin).unwrap_or(&0) ==&0 &&
														ms.get(&MochigomaKind::Kaku).unwrap_or(&0) ==&0 &&
														ms.get(&MochigomaKind::Hisha).unwrap_or(&0) ==&0 &&
														mg.get(&MochigomaKind::Fu).unwrap_or(&0) ==&1 &&
														mg.get(&MochigomaKind::Kyou).unwrap_or(&0) ==&0 &&
														mg.get(&MochigomaKind::Kei).unwrap_or(&0) ==&0 &&
														mg.get(&MochigomaKind::Gin).unwrap_or(&0) ==&0 &&
														mg.get(&MochigomaKind::Kin).unwrap_or(&0) ==&0 &&
														mg.get(&MochigomaKind::Kaku).unwrap_or(&0) ==&0 &&
														mg.get(&MochigomaKind::Hisha).unwrap_or(&0) ==&0 {

														let _ = player.sender.send(Ok(ActionKind::Think));
													} else {
														let _ = player.sender.send(Err(String::from("kyokumen is invalid.")));
													}
												} else {
													let _ = player.sender.send(Err(String::from("kyokumen is invalid.")));
												}
											} else {
												let _ = player.sender.send(Err(String::from("kyokumen is not set.")));
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
		let agent = UsiAgent::new(player);

		let _ = agent.start(input_reader,output_writer,logger,|h,e| {
			if let Some(h) = h {
				let _ = h.lock().map(|h| h.call(e));
			}
		});

		let _ = ts.send(());
	});

	startup(&s,&r,&pmr);

	let _ = s.send(String::from("isready"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::TakeReady timed out.");

	assert_eq!(res,Ok(ActionKind::TakeReady));

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'readyok' timed out.");

	assert_eq!(&*res,"readyok");

	let _ = s.send(String::from("usinewgame"));

	let _ = s.send(String::from("position startpos"));

	let _ = s.send(String::from("go"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::NewGame timed out.");

	assert_eq!(res,Ok(ActionKind::NewGame));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'bestmove 1g1f' timed out.");

	assert_eq!(&*res,"bestmove 1g1f");

	let _ = s.send(String::from("position startpos moves 1g1f 9c9d"));

	let _ = s.send(String::from("go"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'bestmove 1f1e' timed out.");

	assert_eq!(&*res,"bestmove 1f1e");

	let _ = s.send(String::from("position startpos moves 1g1f 9c9d 1f1e 9d9e"));

	let _ = s.send(String::from("go"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'bestmove 1e1d' timed out.");

	assert_eq!(&*res,"bestmove 1e1d");

	let _ = s.send(String::from("position startpos moves 1g1f 9c9d 1f1e 9d9e 1e1d 9e9f"));

	let _ = s.send(String::from("go"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'bestmove 1d1c+' timed out.");

	assert_eq!(&*res,"bestmove 1d1c+");

	let _ = s.send(String::from("position startpos moves 1g1f 9c9d 1f1e 9d9e 1e1d 9e9f 1d1c+ 9f9g+"));

	let _ = s.send(String::from("go"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'bestmove 1d1c+' timed out.");

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
	let (pms,pmr) = unbounded();
	let (pns,_) = unbounded();
	let (ts,tr) = unbounded();

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = unbounded();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (output_writer,r) = {
		let (s,r) = unbounded();

		let output_writer = MockOutputWriter::new(s);
		(output_writer,r)
	};

	let _ = thread::spawn(move || {
		let player = MockPlayer::new(pms,pns,
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

	startup(&s,&r,&pmr);

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
	let (pms,pmr) = unbounded();
	let (pns,_) = unbounded();
	let (ts,tr) = unbounded();

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = unbounded();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (output_writer,r) = {
		let (s,r) = unbounded();

		let output_writer = MockOutputWriter::new(s);
		(output_writer,r)
	};

	let _ = thread::spawn(move || {
		let player = MockPlayer::new(pms,pns,
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

	startup(&s,&r,&pmr);

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
	let (pms,pmr) = unbounded();
	let (pns,_) = unbounded();
	let (ts,tr) = unbounded();

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = unbounded();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (output_writer,r) = {
		let (s,r) = unbounded();

		let output_writer = MockOutputWriter::new(s);
		(output_writer,r)
	};

	let _ = thread::spawn(move || {
		let player = MockPlayer::new(pms,pns,
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

	startup(&s,&r,&pmr);

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
			assert!(false,"Move returned before accept stop.");
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
	let (pms,pmr) = unbounded();
	let (pns,_) = unbounded();
	let (ts,tr) = unbounded();

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = unbounded();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (output_writer,r) = {
		let (s,r) = unbounded();

		let output_writer = MockOutputWriter::new(s);
		(output_writer,r)
	};

	let _ = thread::spawn(move || {
		let player = MockPlayer::new(pms,pns,
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

	startup(&s,&r,&pmr);

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
			assert!(false,"Move returned before accept stop.");
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
#[test]
fn test_stop_thinking() {
	let (pms,pmr) = unbounded();
	let (pns,_) = unbounded();
	let (ts,tr) = unbounded();

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = unbounded();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (output_writer,r) = {
		let (s,r) = unbounded();

		let output_writer = MockOutputWriter::new(s);
		(output_writer,r)
	};

	let _ = thread::spawn(move || {
		let player = MockPlayer::new(pms,pns,
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
										}),
										Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),None))
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

	startup(&s,&r,&pmr);

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

	thread::sleep(Duration::from_millis(300));

	if let Ok(s) = r.recv_timeout(Duration::from_millis(150)) {
		if s.starts_with("bestmove ") {
			assert!(false,"Move returned before accept stop.");
		} else {
			assert!(false,format!("An unexpected command '{}' was returned.",s));
		}
	}

	let _  = s.send(String::from("stop"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::OnStop timed out.");

	assert_eq!(res,Ok(ActionKind::OnStop));

	let res = r.recv_timeout(Duration::from_millis(180)).expect("attempt to receive 'bestmove 1f1e' timed out.");

	assert_eq!(&*res,"bestmove 1f1e");

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
fn test_quit_thinking() {
	let (pms,pmr) = unbounded();
	let (pns,_) = unbounded();
	let (ts,tr) = unbounded();

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = unbounded();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (output_writer,r) = {
		let (s,r) = unbounded();

		let output_writer = MockOutputWriter::new(s);
		(output_writer,r)
	};

	let _ = thread::spawn(move || {
		let player = MockPlayer::new(pms,pns,
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
										}),
										Box::new(|player,_,_,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::SetPosition));
											Ok(())
										})]),
										ConsumedIterator::new(vec![Box::new(|player,_,_,_,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),None))
										}),
										Box::new(|player,_,_,_,mut handle_events| {
											let _ = player.sender.send(Ok(ActionKind::Think));
											while !player.stop {
												handle_events(player)?;
												thread::sleep(Duration::from_millis(10));
											}

											if player.quited {
												let _ = player.sender.send(Ok(ActionKind::Think));
												Ok(BestMove::Abort)
											} else {
												let _ = player.sender.send(Err(String::from("not quited.")));
												Ok(BestMove::Move(Move::To(KomaSrcPosition(1,6),KomaDstToPosition(1,5,false)),None))
											}
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![])
		);
		let agent = UsiAgent::new(player);

		let _ = agent.start(input_reader,output_writer,logger,|h,e| {
			if let Some(h) = h {
				let _ = h.lock().map(|h| h.call(e));
			}
		});

		let _ = ts.send(());
	});

	startup(&s,&r,&pmr);

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

	thread::sleep(Duration::from_millis(300));

	if let Ok(s) = r.recv_timeout(Duration::from_millis(150)) {
		if s.starts_with("bestmove ") {
			assert!(false,"Move returned before accept quit.");
		} else {
			assert!(false,format!("An unexpected command '{}' was returned.",s));
		}
	}

	let _  = s.send(String::from("quit"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::OnQuit timed out.");

	assert_eq!(res,Ok(ActionKind::OnQuit));

	thread::sleep(Duration::from_millis(300));

	if let Ok(s) = r.recv_timeout(Duration::from_millis(150)) {
		if s.starts_with("bestmove ") {
			assert!(false,"Move returned.");
		} else {
			assert!(false,format!("An unexpected command '{}' was returned.",s));
		}
	}

	let _ = tr.recv_timeout(Duration::from_millis(300)).expect("attempt to receive on quited timed out.");
}
#[test]
fn test_go_infinite() {
	let (pms,pmr) = unbounded();
	let (pns,_) = unbounded();
	let (ts,tr) = unbounded();

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = unbounded();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (output_writer,r) = {
		let (s,r) = unbounded();

		let output_writer = MockOutputWriter::new(s);
		(output_writer,r)
	};

	let _ = thread::spawn(move || {
		let player = MockPlayer::new(pms,pns,
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

	startup(&s,&r,&pmr);

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

	let _ = s.send(String::from("go infinite"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	thread::sleep(Duration::from_millis(300));

	if let Ok(s) = r.recv_timeout(Duration::from_millis(150)) {
		if s.starts_with("bestmove ") {
			assert!(false,"Move returned before accept stop.");
		} else {
			assert!(false,format!("An unexpected command '{}' was returned.",s));
		}
	}

	let _  = s.send(String::from("stop"));

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
fn test_go_none_limit() {
	let (pms,pmr) = unbounded();
	let (pns,_) = unbounded();
	let (ts,tr) = unbounded();

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = unbounded();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (output_writer,r) = {
		let (s,r) = unbounded();

		let output_writer = MockOutputWriter::new(s);
		(output_writer,r)
	};

	let _ = thread::spawn(move || {
		let player = MockPlayer::new(pms,pns,
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
										ConsumedIterator::new(vec![Box::new(|player,limit,_,_,_| {
											if let UsiGoTimeLimit::None = limit {
												let _ = player.sender.send(Ok(ActionKind::Think));
											} else {
												let _ = player.sender.send(Err(
															format!("The time limit value is incorrect. expected = {:?} actual = {:?}",
																										UsiGoTimeLimit::None,limit
												)));
											}

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

	startup(&s,&r,&pmr);

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
fn test_go_with_limit() {
	let (pms,pmr) = unbounded();
	let (pns,_) = unbounded();
	let (ts,tr) = unbounded();

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = unbounded();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (output_writer,r) = {
		let (s,r) = unbounded();

		let output_writer = MockOutputWriter::new(s);
		(output_writer,r)
	};

	let _ = thread::spawn(move || {
		let player = MockPlayer::new(pms,pns,
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
										ConsumedIterator::new(vec![Box::new(|player,limit,_,_,_| {
											const EXPECTED:UsiGoTimeLimit = UsiGoTimeLimit::Limit(Some((100,200)),None);

											if *limit == EXPECTED {
												let _ = player.sender.send(Ok(ActionKind::Think));
											} else {
												let _ = player.sender.send(Err(
															format!("The time limit value is incorrect. expected = {:?} actual = {:?}",
																															EXPECTED,limit
												)));
											}

											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),None))
										}),
										Box::new(|player,limit,_,_,_| {
											const EXPECTED:UsiGoTimeLimit = UsiGoTimeLimit::Limit(Some((100,200)),None);

											if *limit == EXPECTED {
												let _ = player.sender.send(Ok(ActionKind::Think));
											} else {
												let _ = player.sender.send(Err(
															format!("The time limit value is incorrect. expected = {:?} actual = {:?}",
																															EXPECTED,limit
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
		let agent = UsiAgent::new(player);

		let _ = agent.start(input_reader,output_writer,logger,|h,e| {
			if let Some(h) = h {
				let _ = h.lock().map(|h| h.call(e));
			}
		});

		let _ = ts.send(());
	});

	startup(&s,&r,&pmr);

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

	let _ = s.send(String::from("go btime 100 wtime 200"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'bestmove 1g1f' timed out.");

	assert_eq!(&*res,"bestmove 1g1f");

	let _ = s.send(String::from("position startpos moves 1g1f"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let _ = s.send(String::from("go wtime 200 btime 100"));

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
fn test_go_with_limit_and_byoyomi() {
	let (pms,pmr) = unbounded();
	let (pns,_) = unbounded();
	let (ts,tr) = unbounded();

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = unbounded();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (output_writer,r) = {
		let (s,r) = unbounded();

		let output_writer = MockOutputWriter::new(s);
		(output_writer,r)
	};

	let _ = thread::spawn(move || {
		let player = MockPlayer::new(pms,pns,
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
										ConsumedIterator::new(vec![Box::new(|player,limit,_,_,_| {
											const EXPECTED:UsiGoTimeLimit = UsiGoTimeLimit::Limit(Some((100,200)),
																									Some(UsiGoByoyomiOrInc::Byoyomi(10000)));

											if *limit == EXPECTED {
												let _ = player.sender.send(Ok(ActionKind::Think));
											} else {
												let _ = player.sender.send(Err(
															format!("The time limit value is incorrect. expected = {:?} actual = {:?}",
																															EXPECTED,limit
												)));
											}

											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),None))
										}),
										Box::new(|player,limit,_,_,_| {
											const EXPECTED:UsiGoTimeLimit = UsiGoTimeLimit::Limit(Some((100,200)),
																									Some(UsiGoByoyomiOrInc::Byoyomi(10000)));

											if *limit == EXPECTED {
												let _ = player.sender.send(Ok(ActionKind::Think));
											} else {
												let _ = player.sender.send(Err(
															format!("The time limit value is incorrect. expected = {:?} actual = {:?}",
																															EXPECTED,limit
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
		let agent = UsiAgent::new(player);

		let _ = agent.start(input_reader,output_writer,logger,|h,e| {
			if let Some(h) = h {
				let _ = h.lock().map(|h| h.call(e));
			}
		});

		let _ = ts.send(());
	});

	startup(&s,&r,&pmr);

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

	let _ = s.send(String::from("go btime 100 wtime 200 byoyomi 10000"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'bestmove 1g1f' timed out.");

	assert_eq!(&*res,"bestmove 1g1f");

	let _ = s.send(String::from("position startpos moves 1g1f"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let _ = s.send(String::from("go byoyomi 10000 wtime 200 btime 100"));

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
fn test_go_with_limit_and_inc() {
	let (pms,pmr) = unbounded();
	let (pns,_) = unbounded();
	let (ts,tr) = unbounded();

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = unbounded();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (output_writer,r) = {
		let (s,r) = unbounded();

		let output_writer = MockOutputWriter::new(s);
		(output_writer,r)
	};

	let _ = thread::spawn(move || {
		let player = MockPlayer::new(pms,pns,
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
										ConsumedIterator::new(vec![Box::new(|player,limit,_,_,_| {
											const EXPECTED:UsiGoTimeLimit = UsiGoTimeLimit::Limit(Some((100,200)),
																									Some(UsiGoByoyomiOrInc::Inc(1000,2000)));

											if *limit == EXPECTED {
												let _ = player.sender.send(Ok(ActionKind::Think));
											} else {
												let _ = player.sender.send(Err(
															format!("The time limit value is incorrect. expected = {:?} actual = {:?}",
																															EXPECTED,limit
												)));
											}

											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),None))
										}),
										Box::new(|player,limit,_,_,_| {
											const EXPECTED:UsiGoTimeLimit = UsiGoTimeLimit::Limit(Some((100,200)),
																									Some(UsiGoByoyomiOrInc::Inc(1000,2000)));

											if *limit == EXPECTED {
												let _ = player.sender.send(Ok(ActionKind::Think));
											} else {
												let _ = player.sender.send(Err(
															format!("The time limit value is incorrect. expected = {:?} actual = {:?}",
																															EXPECTED,limit
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
		let agent = UsiAgent::new(player);

		let _ = agent.start(input_reader,output_writer,logger,|h,e| {
			if let Some(h) = h {
				let _ = h.lock().map(|h| h.call(e));
			}
		});

		let _ = ts.send(());
	});

	startup(&s,&r,&pmr);

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

	let _ = s.send(String::from("go btime 100 wtime 200 binc 1000 winc 2000"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'bestmove 1g1f' timed out.");

	assert_eq!(&*res,"bestmove 1g1f");

	let _ = s.send(String::from("position startpos moves 1g1f"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let _ = s.send(String::from("go winc 2000 binc 1000 wtime 200 btime 100"));

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
fn test_go_with_byoyomi() {
	let (pms,pmr) = unbounded();
	let (pns,_) = unbounded();
	let (ts,tr) = unbounded();

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = unbounded();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (output_writer,r) = {
		let (s,r) = unbounded();

		let output_writer = MockOutputWriter::new(s);
		(output_writer,r)
	};

	let _ = thread::spawn(move || {
		let player = MockPlayer::new(pms,pns,
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
										ConsumedIterator::new(vec![Box::new(|player,limit,_,_,_| {
											const EXPECTED:UsiGoTimeLimit = UsiGoTimeLimit::Limit(None,
																									Some(UsiGoByoyomiOrInc::Byoyomi(10000)));

											if *limit == EXPECTED {
												let _ = player.sender.send(Ok(ActionKind::Think));
											} else {
												let _ = player.sender.send(Err(
															format!("The time limit value is incorrect. expected = {:?} actual = {:?}",
																															EXPECTED,limit
												)));
											}

											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),None))
										}),
										Box::new(|player,limit,_,_,_| {
											const EXPECTED:UsiGoTimeLimit = UsiGoTimeLimit::Limit(None,
																									Some(UsiGoByoyomiOrInc::Byoyomi(20000)));

											if *limit == EXPECTED {
												let _ = player.sender.send(Ok(ActionKind::Think));
											} else {
												let _ = player.sender.send(Err(
															format!("The time limit value is incorrect. expected = {:?} actual = {:?}",
																															EXPECTED,limit
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
		let agent = UsiAgent::new(player);

		let _ = agent.start(input_reader,output_writer,logger,|h,e| {
			if let Some(h) = h {
				let _ = h.lock().map(|h| h.call(e));
			}
		});

		let _ = ts.send(());
	});

	startup(&s,&r,&pmr);

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

	let _ = s.send(String::from("go byoyomi 10000"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'bestmove 1g1f' timed out.");

	assert_eq!(&*res,"bestmove 1g1f");

	let _ = s.send(String::from("position startpos moves 1g1f"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let _ = s.send(String::from("go byoyomi 20000"));

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
fn test_go_with_inc() {
	let (pms,pmr) = unbounded();
	let (pns,_) = unbounded();
	let (ts,tr) = unbounded();

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = unbounded();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (output_writer,r) = {
		let (s,r) = unbounded();

		let output_writer = MockOutputWriter::new(s);
		(output_writer,r)
	};

	let _ = thread::spawn(move || {
		let player = MockPlayer::new(pms,pns,
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
										ConsumedIterator::new(vec![Box::new(|player,limit,_,_,_| {
											const EXPECTED:UsiGoTimeLimit = UsiGoTimeLimit::Limit(None,
																									Some(UsiGoByoyomiOrInc::Inc(1000,2000)));

											if *limit == EXPECTED {
												let _ = player.sender.send(Ok(ActionKind::Think));
											} else {
												let _ = player.sender.send(Err(
															format!("The time limit value is incorrect. expected = {:?} actual = {:?}",
																															EXPECTED,limit
												)));
											}

											Ok(BestMove::Move(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),None))
										}),
										Box::new(|player,limit,_,_,_| {
											const EXPECTED:UsiGoTimeLimit = UsiGoTimeLimit::Limit(None,
																									Some(UsiGoByoyomiOrInc::Inc(1000,2000)));

											if *limit == EXPECTED {
												let _ = player.sender.send(Ok(ActionKind::Think));
											} else {
												let _ = player.sender.send(Err(
															format!("The time limit value is incorrect. expected = {:?} actual = {:?}",
																															EXPECTED,limit
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
		let agent = UsiAgent::new(player);

		let _ = agent.start(input_reader,output_writer,logger,|h,e| {
			if let Some(h) = h {
				let _ = h.lock().map(|h| h.call(e));
			}
		});

		let _ = ts.send(());
	});

	startup(&s,&r,&pmr);

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

	let _ = s.send(String::from("go binc 1000 winc 2000"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::Think));

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'bestmove 1g1f' timed out.");

	assert_eq!(&*res,"bestmove 1g1f");

	let _ = s.send(String::from("position startpos moves 1g1f"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::SetPosition timed out.");

	assert_eq!(res,Ok(ActionKind::SetPosition));

	let _ = s.send(String::from("go winc 2000 binc 1000"));

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
fn test_go_mate() {
	let (pms,pmr) = unbounded();
	let (pns,_) = unbounded();
	let (ts,tr) = unbounded();

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = unbounded();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (output_writer,r) = {
		let (s,r) = unbounded();

		let output_writer = MockOutputWriter::new(s);
		(output_writer,r)
	};

	let lastmoves = vec![
		String::from("checkmate 1g1f 1f1e 1e1d 1d1c"),
		String::from("checkmate notimplemented"),
		String::from("checkmate timeout"),
		String::from("checkmate nomate"),
	].into_iter();

	let _ = thread::spawn(move || {
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
										})]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,limit,_,_,_| {
											const EXPECTED:UsiGoMateTimeLimit = UsiGoMateTimeLimit::None;

											if *limit == EXPECTED {
												let _ = player.sender.send(Ok(ActionKind::ThinkMate));
											} else {
												let _ = player.sender.send(Err(
														format!("The time limit value is incorrect. expected = {:?} actual = {:?}",
																														EXPECTED,limit
												)));
											}

											Ok(CheckMate::Moves(vec![
												Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),
												Move::To(KomaSrcPosition(1,6),KomaDstToPosition(1,5,false)),
												Move::To(KomaSrcPosition(1,5),KomaDstToPosition(1,4,false)),
												Move::To(KomaSrcPosition(1,4),KomaDstToPosition(1,3,false)),
											]))
										}),
										Box::new(|player,limit,_,_,_| {
											const EXPECTED:UsiGoMateTimeLimit = UsiGoMateTimeLimit::None;

											if *limit == EXPECTED {
												let _ = player.sender.send(Ok(ActionKind::ThinkMate));
											} else {
												let _ = player.sender.send(Err(
														format!("The time limit value is incorrect. expected = {:?} actual = {:?}",
																														EXPECTED,limit
												)));
											}

											Ok(CheckMate::NotiImplemented)
										}),
										Box::new(|player,limit,_,_,_| {
											const EXPECTED:UsiGoMateTimeLimit = UsiGoMateTimeLimit::None;

											if *limit == EXPECTED {
												let _ = player.sender.send(Ok(ActionKind::ThinkMate));
											} else {
												let _ = player.sender.send(Err(
														format!("The time limit value is incorrect. expected = {:?} actual = {:?}",
																														EXPECTED,limit
												)));
											}

											Ok(CheckMate::Timeout)
										}),
										Box::new(|player,limit,_,_,_| {
											const EXPECTED:UsiGoMateTimeLimit = UsiGoMateTimeLimit::None;

											if *limit == EXPECTED {
												let _ = player.sender.send(Ok(ActionKind::ThinkMate));
											} else {
												let _ = player.sender.send(Err(
														format!("The time limit value is incorrect. expected = {:?} actual = {:?}",
																														EXPECTED,limit
												)));
											}

											Ok(CheckMate::Nomate)
										})]),
										ConsumedIterator::new(vec![]),
		);
		let agent = UsiAgent::new(player);

		let _ = agent.start(input_reader,output_writer,logger,|h,e| {
			if let Some(h) = h {
				let _ = h.lock().map(|h| h.call(e));
			}
		});

		let _ = ts.send(());
	});

	startup(&s,&r,&pmr);

	for lastmove in lastmoves {
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

		let _ = s.send(String::from("go mate"));

		let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Think timed out.");

		assert_eq!(res,Ok(ActionKind::ThinkMate));

		let res = r.recv_timeout(Duration::from_millis(150)).expect(format!("attempt to receive '{}' timed out.",lastmove).as_str());

		assert_eq!(&*res,&*lastmove);
	}

	let _ = s.send(String::from("quit"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let _ = tr.recv_timeout(Duration::from_millis(300)).expect("attempt to receive on quited timed out.");
}
#[test]
fn test_mate_with_limit() {
	let (pms,pmr) = unbounded();
	let (pns,_) = unbounded();
	let (ts,tr) = unbounded();

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = unbounded();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (output_writer,r) = {
		let (s,r) = unbounded();

		let output_writer = MockOutputWriter::new(s);
		(output_writer,r)
	};

	let _ = thread::spawn(move || {
		let player = MockPlayer::new(pms,pns,
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
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,limit,_,_,_| {
											const EXPECTED:UsiGoMateTimeLimit = UsiGoMateTimeLimit::Limit(100000);

											if *limit == EXPECTED {
												let _ = player.sender.send(Ok(ActionKind::ThinkMate));
											} else {
												let _ = player.sender.send(Err(
															format!("The time limit value is incorrect. expected = {:?} actual = {:?}",
																															EXPECTED,limit
												)));
											}

											Ok(CheckMate::Timeout)
										})]),
										ConsumedIterator::new(vec![])
		);
		let agent = UsiAgent::new(player);

		let _ = agent.start(input_reader,output_writer,logger,|h,e| {
			if let Some(h) = h {
				let _ = h.lock().map(|h| h.call(e));
			}
		});

		let _ = ts.send(());
	});

	startup(&s,&r,&pmr);

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

	let _ = s.send(String::from("go mate 100000"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::ThinkMate));

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'bestmove resign' timed out.");

	assert_eq!(&*res,"checkmate timeout");

	let _ = s.send(String::from("quit"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let _ = tr.recv_timeout(Duration::from_millis(300)).expect("attempt to receive on quited timed out.");
}
#[test]
fn test_mate_with_limit_of_infinite() {
	let (pms,pmr) = unbounded();
	let (pns,_) = unbounded();
	let (ts,tr) = unbounded();

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = unbounded();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (output_writer,r) = {
		let (s,r) = unbounded();

		let output_writer = MockOutputWriter::new(s);
		(output_writer,r)
	};

	let _ = thread::spawn(move || {
		let player = MockPlayer::new(pms,pns,
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
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![Box::new(|player,limit,_,_,_| {
											const EXPECTED:UsiGoMateTimeLimit = UsiGoMateTimeLimit::Infinite;

											if *limit == EXPECTED {
												let _ = player.sender.send(Ok(ActionKind::ThinkMate));
											} else {
												let _ = player.sender.send(Err(
															format!("The time limit value is incorrect. expected = {:?} actual = {:?}",
																															EXPECTED,limit
												)));
											}

											Ok(CheckMate::Timeout)
										})]),
										ConsumedIterator::new(vec![])
		);
		let agent = UsiAgent::new(player);

		let _ = agent.start(input_reader,output_writer,logger,|h,e| {
			if let Some(h) = h {
				let _ = h.lock().map(|h| h.call(e));
			}
		});

		let _ = ts.send(());
	});

	startup(&s,&r,&pmr);

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

	let _ = s.send(String::from("go mate infinite"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Think timed out.");

	assert_eq!(res,Ok(ActionKind::ThinkMate));

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'bestmove resign' timed out.");

	assert_eq!(&*res,"checkmate timeout");

	let _ = s.send(String::from("quit"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let _ = tr.recv_timeout(Duration::from_millis(300)).expect("attempt to receive on quited timed out.");
}
#[test]
fn test_info_send_commands_without_str() {
	let (pms,pmr) = unbounded();
	let (pns,pnr) = unbounded();
	let (ts,tr) = unbounded();

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = unbounded();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (output_writer,r) = {
		let (s,r) = unbounded();

		let output_writer = MockOutputWriter::new(s);
		(output_writer,r)
	};

	let _ = thread::spawn(move || {
		let player = MockPlayer::new(pms,pns,
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
										Box::new(|player,_,_,mut send_info_commands,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));

											if let Err(_) = send_info_commands(vec![
												UsiInfoSubCommand::Depth(1),
												UsiInfoSubCommand::SelDepth(3),
												UsiInfoSubCommand::Time(10000),
												UsiInfoSubCommand::Nodes(1000000),
												UsiInfoSubCommand::Pv(vec![
													Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),
													Move::To(KomaSrcPosition(9,3),KomaDstToPosition(9,4,false)),
													Move::To(KomaSrcPosition(1,6),KomaDstToPosition(1,5,false))
												]),
												UsiInfoSubCommand::MultiPv(1),
												UsiInfoSubCommand::Score(UsiScore::Cp(-100)),
												UsiInfoSubCommand::CurMove(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false))),
												UsiInfoSubCommand::Hashfull(10000),
												UsiInfoSubCommand::Nps(100)
											]) {
												Err(CommonError::Fail(String::from("An error occurred when sending the info command.")))
											} else {
												Ok(BestMove::Resign)
											}
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

	startup(&s,&r,&pmr);

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

	let _ = pnr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive info send notify timed out.");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'info depth 1 seldepth 3 time 10000 nodes 1000000 score cp -100 curmove 1g1f hashfull 10000 nps 100 multipv 1 pv 1g1f 9c9d 1f1e' timed out.");

	assert_eq!(&*res,"info depth 1 seldepth 3 time 10000 nodes 1000000 score cp -100 curmove 1g1f hashfull 10000 nps 100 multipv 1 pv 1g1f 9c9d 1f1e");

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
fn test_info_send_commands_without_str_and_multipv() {
	let (pms,pmr) = unbounded();
	let (pns,pnr) = unbounded();
	let (ts,tr) = unbounded();

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = unbounded();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (output_writer,r) = {
		let (s,r) = unbounded();

		let output_writer = MockOutputWriter::new(s);
		(output_writer,r)
	};

	let _ = thread::spawn(move || {
		let player = MockPlayer::new(pms,pns,
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
										Box::new(|player,_,_,mut send_info_commands,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));

											if let Err(_) = send_info_commands(vec![
												UsiInfoSubCommand::Depth(1),
												UsiInfoSubCommand::SelDepth(3),
												UsiInfoSubCommand::Time(10000),
												UsiInfoSubCommand::Nodes(1000000),
												UsiInfoSubCommand::Pv(vec![
													Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),
													Move::To(KomaSrcPosition(9,3),KomaDstToPosition(9,4,false)),
													Move::To(KomaSrcPosition(1,6),KomaDstToPosition(1,5,false))
												]),
												UsiInfoSubCommand::Score(UsiScore::Cp(-100)),
												UsiInfoSubCommand::CurMove(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false))),
												UsiInfoSubCommand::Hashfull(10000),
												UsiInfoSubCommand::Nps(100)
											]) {
												Err(CommonError::Fail(String::from("An error occurred when sending the info command.")))
											} else {
												Ok(BestMove::Resign)
											}
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

	startup(&s,&r,&pmr);

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

	let _ = pnr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive info send notify timed out.");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'info depth 1 seldepth 3 time 10000 nodes 1000000 score cp -100 curmove 1g1f hashfull 10000 nps 100 pv 1g1f 9c9d 1f1e' timed out.");

	assert_eq!(&*res,"info depth 1 seldepth 3 time 10000 nodes 1000000 score cp -100 curmove 1g1f hashfull 10000 nps 100 pv 1g1f 9c9d 1f1e");

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
fn test_info_send_commands_without_pv_and_multipv() {
	let (pms,pmr) = unbounded();
	let (pns,pnr) = unbounded();
	let (ts,tr) = unbounded();

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = unbounded();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (output_writer,r) = {
		let (s,r) = unbounded();

		let output_writer = MockOutputWriter::new(s);
		(output_writer,r)
	};

	let _ = thread::spawn(move || {
		let player = MockPlayer::new(pms,pns,
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
										Box::new(|player,_,_,mut send_info_commands,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));

											if let Err(_) = send_info_commands(vec![
												UsiInfoSubCommand::Depth(1),
												UsiInfoSubCommand::SelDepth(3),
												UsiInfoSubCommand::Time(10000),
												UsiInfoSubCommand::Nodes(1000000),
												UsiInfoSubCommand::Str(String::from("hellow!")),
												UsiInfoSubCommand::Score(UsiScore::Cp(-100)),
												UsiInfoSubCommand::CurMove(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false))),
												UsiInfoSubCommand::Hashfull(10000),
												UsiInfoSubCommand::Nps(100)
											]) {
												Err(CommonError::Fail(String::from("An error occurred when sending the info command.")))
											} else {
												Ok(BestMove::Resign)
											}
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

	startup(&s,&r,&pmr);

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

	let _ = pnr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive info send notify timed out.");

	let res = r.recv_timeout(Duration::from_millis(150)).expect("attempt to receive 'info depth 1 seldepth 3 time 10000 nodes 1000000 string hellow! score cp -100 curmove 1g1f hashfull 10000 nps 100' timed out.");

	assert_eq!(&*res,"info depth 1 seldepth 3 time 10000 nodes 1000000 string hellow! score cp -100 curmove 1g1f hashfull 10000 nps 100");

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
fn test_info_send_commands_with_str_5times() {
	let (pms,pmr) = unbounded();
	let (pns,pnr) = unbounded();
	let (ts,tr) = unbounded();

	let logger = StdErrorLogger::new();
	let (input_reader,s) = {
		let (s,r) = unbounded();

		let input_reader = MockInputReader::new(r);
		(input_reader,s)
	};

	let (output_writer,r) = {
		let (s,r) = unbounded();

		let output_writer = MockOutputWriter::new(s);
		(output_writer,r)
	};

	let _ = thread::spawn(move || {
		let player = MockPlayer::new(pms,pns,
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
										Box::new(|player,_,_,mut send_info_commands,_| {
											let _ = player.sender.send(Ok(ActionKind::Think));

											for i in 0..5 {
												if let Err(_) = send_info_commands(vec![
													UsiInfoSubCommand::Str(format!("hellow! {}",i+1))
												]) {
													return Err(CommonError::Fail(String::from("An error occurred when sending the info command.")))
												}
											}

											thread::sleep(Duration::from_millis(250));

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

	startup(&s,&r,&pmr);

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

	for i in 0..5 {
		let _ = pnr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive info send notify timed out.");

		let res = r.recv_timeout(Duration::from_millis(200)).expect(format!("attempt to receive 'info string hellow! {}' timed out.",i+1).as_str());;

		assert_eq!(&*res,format!("info string hellow! {}",i+1).as_str());
	}

	let res = r.recv_timeout(Duration::from_millis(300)).expect("attempt to receive 'bestmove resign' timed out.");

	assert_eq!(&*res,"bestmove resign");

	let _ = s.send(String::from("gameover lose"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::GameOver timed out.");

	assert_eq!(res,Ok(ActionKind::GameOver));

	let _ = s.send(String::from("quit"));

	let res = pmr.recv_timeout(Duration::from_millis(150)).expect("attempt to receive ActionKind::Quit timed out.");

	assert_eq!(res,Ok(ActionKind::Quit));

	let _ = tr.recv_timeout(Duration::from_millis(300)).expect("attempt to receive on quited timed out.");
}