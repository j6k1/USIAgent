use std::thread;
use std::sync::mpsc;

use usiagent::UsiAgent;

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
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![]),
										ConsumedIterator::new(vec![])
		);
		let agent = UsiAgent::new(player);

		let _ = agent.start(input_reader,output_writer,logger,|h,e| {
			if let Some(h) = h {
				let _ = h.lock().map(|h| h.call(e));
			}
		});
	});
}