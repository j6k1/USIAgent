use usiagent::rule::{Rule, State};
use usiagent::shogi::*;
use usiagent::shogi::KomaKind::*;

#[inline]
fn set_piece(b:&mut Banmen, x:u32, y:u32, k:KomaKind) {
	b.0[y as usize][x as usize] = k;
}

#[inline]
fn blank() -> Banmen {
	Banmen([[Blank; 9]; 9])
}

#[test]
fn sente_ou_surrounding_threats_count_center_counts_only_gote() {
	let mut b = blank();
	set_piece(&mut b, 4,4, SOu);

	set_piece(&mut b, 3,3, GFu);
	set_piece(&mut b, 4,3, GGin);
	set_piece(&mut b, 5,3, GKin);
	set_piece(&mut b, 3,4, GKaku);
	set_piece(&mut b, 5,5, GHisha);
	set_piece(&mut b, 4,5, SFu);
	set_piece(&mut b, 0,0, GHisha);

	let s = State::new(b);
	let got = Rule::sente_ou_surrounding_threats_count(s.get_part());

	assert_eq!(got, 5);
}

#[test]
fn sente_ou_surrounding_threats_count_bottom_edge() {
	let mut b = blank();
	set_piece(&mut b, 4,8, SOu);

	set_piece(&mut b, 3,7, GFu);
	set_piece(&mut b, 4,7, GGin);
	set_piece(&mut b, 5,7, GKin);
	set_piece(&mut b, 3,8, GKaku);
	set_piece(&mut b, 5,8, GHisha);
	set_piece(&mut b, 4,6, GKei);

	let s = State::new(b);
	let got = Rule::sente_ou_surrounding_threats_count(s.get_part());

	assert_eq!(got, 5);
}

#[test]
fn gote_ou_surrounding_threats_count_top_edge_counts_only_sente() {
	let mut b = blank();
	set_piece(&mut b, 4,0, GOu);

	set_piece(&mut b, 3,0, SFu);
	set_piece(&mut b, 5,0, SGin);
	set_piece(&mut b, 3,1, SKin);
	set_piece(&mut b, 4,1, SKaku);
	set_piece(&mut b, 5,1, GKyou);
	set_piece(&mut b, 4,2, SHisha);

	let s = State::new(b);
	let got = Rule::gote_ou_surrounding_threats_count(s.get_part());

	assert_eq!(got, 4);
}
