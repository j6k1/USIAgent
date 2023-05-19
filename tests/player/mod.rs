use std::sync::{Arc, mpsc, Mutex};
use std::time::Duration;

use usiagent::player::*;
use usiagent::shogi::*;
use usiagent::hash::*;
use usiagent::{OnErrorHandler, rule};
use usiagent::rule::*;

#[allow(unused)]
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
use usiagent::command::UsiInfoSubCommand;

#[test]
fn test_apply_moves() {
	let (pms1,_) = mpsc::channel();
	let (pns1,_) = mpsc::channel();

	let player = MockPlayer::new(pms1,pns1,
		ConsumedIterator::new(vec![]),
		ConsumedIterator::new(vec![]),
		ConsumedIterator::new(vec![]),
		ConsumedIterator::new(vec![]),
		ConsumedIterator::new(vec![]),
		ConsumedIterator::new(vec![])
	);
	let mut after_banmen = BANMEN_START_POS.clone();

	after_banmen.0[6][8] = Blank;
	after_banmen.0[4][8] = SFu;
	after_banmen.0[8][1] = Blank;
	after_banmen.0[6][2] = SKei;
	after_banmen.0[5][2] = SFu;

	after_banmen.0[8-6][8-8] = Blank;
	after_banmen.0[8-4][8-8] = GFu;
	after_banmen.0[8-8][8-1] = Blank;
	after_banmen.0[8-6][8-2] = GKei;
	after_banmen.0[8-5][8-2] = GFu;

	let mvs:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![
		((8,6),(8,5,false),None),
		((8-8,8-6),(8-8,8-5,false),None),
		((8,5),(8,4,false),None),
		((8-8,8-5),(8-8,8-4,false),None),
		((2,6),(2,5,false),None),
		((8-2,8-6),(8-2,8-5,false),None),
		((1,8),(2,6,false),None),
		((8-1,8-8),(8-2,8-6,false),None),
	];

	let hasher = KyokumenHash::new();

	let (imhash, ishash) = hasher.calc_initial_hash(&BANMEN_START_POS,&Mochigoma::new(),&Mochigoma::new());

	let mvs = mvs.into_iter().map(|m| {
		rule::AppliedMove::from(Move::To(KomaSrcPosition(9-(m.0).0,(m.0).1+1),KomaDstToPosition(9-(m.1).0,(m.1).1+1,(m.1).2)))
	}).collect::<Vec<rule::AppliedMove>>();

	let state = State::new(BANMEN_START_POS.clone());
	let teban = Teban::Sente;
	let mc = MochigomaCollections::Empty;

	let (_, _, _, r) = player.apply_moves(state,
							teban,
							mc,
							&mvs,
							(imhash,ishash),
							|_,teban,banmen,mc,m,o,r:(u64,u64)| {
			let (mhash,shash) = r;

			if let Some(m) = m {
				let mhash = hasher.calc_main_hash(mhash,teban,banmen,mc,*m,o);
				let shash = hasher.calc_sub_hash(shash,teban,banmen,mc,*m,o);

				(mhash,shash)
			} else {
				(mhash,shash)
			}
		 });

	let (mhash,shash) = r;
	let (amhash, ashash) = hasher.calc_initial_hash(&after_banmen,&Mochigoma::new(),&Mochigoma::new());

	assert_eq!(amhash,mhash);
	assert_eq!(ashash,shash);

	assert!(mhash != imhash);
	assert!(mhash != ishash);
}
#[test]
fn test_send_immediate() {
	let logger = StdErrorLogger::new();

	let on_error_handler = OnErrorHandler::new( Arc::new(Mutex::new(logger)));
	let on_error_handler = Arc::new(Mutex::new(on_error_handler));

	let (output_writer,r) = {
		let (s,r) = mpsc::channel();

		let output_writer = MockOutputWriter::new(s);
		(output_writer,r)
	};

	let (info_send_worker,on_info_send_worker_quit_receiver) = {
		let (notifier,receiver) = mpsc::channel();

		(InfoSendWorker::new(Arc::new(Mutex::new(output_writer)),notifier,on_error_handler.clone()),receiver)
	};

	let mut info_sender = {
		let info_send_worker = info_send_worker.clone();
		USIInfoSender::new(info_send_worker)
	};

	let h = std::thread::spawn(move || {
		for i in 1..10000 {
			let mut commands: Vec<UsiInfoSubCommand> = Vec::new();
			commands.push(UsiInfoSubCommand::Str(format!("send {} timnes!",i)));

			info_sender.send(commands).unwrap();
		}

		let mut commands: Vec<UsiInfoSubCommand> = Vec::new();
		commands.push(UsiInfoSubCommand::Str(String::from("send immediate.")));
		info_sender.send_immediate(commands).unwrap();
	});

	h.join().unwrap();
	info_send_worker.quit(on_info_send_worker_quit_receiver).unwrap();

	let mut found = false;

	while !found {
		let res = r.recv_timeout(Duration::from_millis(300)).expect("attempt to receive send_immediate result timed out.");

		if &*res == "info string send immediate." {
			found = true;
			break;
		}
	}

	assert!(found);
}