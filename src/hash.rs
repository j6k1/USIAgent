use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Add;
use std::ops::Sub;
use std::ops::BitXor;
use rand;
use rand::Rng;
use rand::Rand;
use std::num::Wrapping;

use shogi::*;

pub struct TwoKeyHashMap<K,T> where K: Eq + Hash + Clone, T: Clone {
	map:HashMap<K,Vec<(K,T)>>
}

impl<T,K> TwoKeyHashMap<K,T> where K: Eq + Hash + Clone, T: Clone {
	pub fn new() -> TwoKeyHashMap<K,T> {
		let map:HashMap<K,Vec<(K,T)>> = HashMap::new();

		TwoKeyHashMap {
			map:map
		}
	}

	pub fn get(&self,k:&K,sk:&K) -> Option<T> {
		match self.map.get(k) {
			Some(v) if v.len() == 1 => {
				Some(v[0].1.clone())
			},
			Some(v) if v.len() > 1 => {
				for e in v {
					if e.0 == *sk {
						return Some(e.1.clone());
					}
				}
				None
			},
			_ => None,
		}
	}

	pub fn insert(&mut self,k:K,sk:K,nv:T) -> Option<T> {
		match self.map.get_mut(&k) {
			Some(ref mut v) if v.len() == 1 => {
				if v[0].0 == sk {
					let old = v[0].1.clone();
					v[0] = (sk,nv);
					return Some(old);
				} else {
					v.push((sk,nv));
					return None;
				}
			},
			Some(ref mut v) if v.len() > 1 => {
				for i in 0..v.len() {
					if v[i].0 == sk {
						let old = v[i].1.clone();
						v[i] = (sk,nv);
						return Some(old);
					}
				}
				v.push((sk,nv));
				return None;
			},
			_ => (),
		}
		let mut v:Vec<(K,T)> = Vec::new();
		v.push((sk,nv));
		self.map.insert(k,v);
		None
	}

	pub fn clear(&mut self) {
		self.map.clear();
	}
}
impl<K,T> Clone for TwoKeyHashMap<K,T> where K: Eq + Hash + Clone, T: Clone {
	fn clone(&self) -> TwoKeyHashMap<K,T> {
		TwoKeyHashMap {
			map:self.map.clone()
		}
	}
}
const KOMA_KIND_MAX:usize = KomaKind::Blank as usize;
const MOCHIGOMA_KIND_MAX:usize = MochigomaKind::Hisha as usize;
const MOCHIGOMA_MAX:usize = 18;
const SUJI_MAX:usize = 9;
const DAN_MAX:usize = 9;

pub trait InitialHash {
	const INITIAL_HASH:Self;
}
/*
impl InitialHash for u128 {
	const INITIAL_HASH:u128 = 0;
}
*/
impl InitialHash for u64 {
	const INITIAL_HASH:u64 = 0;
}
impl InitialHash for u32 {
	const INITIAL_HASH:u32 = 0;
}
impl InitialHash for u16 {
	const INITIAL_HASH:u16 = 0;
}
impl InitialHash for u8 {
	const INITIAL_HASH:u8 = 0;
}
pub struct KyokumenHash<T>
	where T: Add + Sub + BitXor<Output = T> + Rand + Copy + InitialHash,
			Wrapping<T>: Add<Output = Wrapping<T>> + Sub<Output = Wrapping<T>> + BitXor<Output = Wrapping<T>> + Copy {
	kyokumen_hash_seeds:[[T; SUJI_MAX * DAN_MAX]; KOMA_KIND_MAX + 1],
	mochigoma_hash_seeds:[[[T; MOCHIGOMA_KIND_MAX + 1]; MOCHIGOMA_MAX]; 2],
}
impl<T> KyokumenHash<T>
	where T: Add + Sub + BitXor<Output = T> + Rand + Copy + InitialHash,
			Wrapping<T>: Add<Output = Wrapping<T>> + Sub<Output = Wrapping<T>> + BitXor<Output = Wrapping<T>> + Copy {
	pub fn new() -> KyokumenHash<T> {
		let mut rnd = rand::XorShiftRng::new_unseeded();

		let mut kyokumen_hash_seeds:[[T; SUJI_MAX * DAN_MAX]; KOMA_KIND_MAX + 1] = [[T::INITIAL_HASH; SUJI_MAX * DAN_MAX]; KOMA_KIND_MAX + 1];
		let mut mochigoma_hash_seeds:[[[T; MOCHIGOMA_KIND_MAX + 1]; MOCHIGOMA_MAX]; 2] = [[[T::INITIAL_HASH; MOCHIGOMA_KIND_MAX + 1]; MOCHIGOMA_MAX]; 2];

		for i in 0..(KOMA_KIND_MAX + 1) {
			for j in 0..(SUJI_MAX * DAN_MAX) {
				kyokumen_hash_seeds[i][j] = rnd.gen();
			}
		}

		for i in 0..MOCHIGOMA_MAX {
			for j in 0..(MOCHIGOMA_KIND_MAX + 1) {
				mochigoma_hash_seeds[0][i][j] = rnd.gen();
				mochigoma_hash_seeds[1][i][j] = rnd.gen();
			}
		}

		KyokumenHash {
			kyokumen_hash_seeds:kyokumen_hash_seeds,
			mochigoma_hash_seeds:mochigoma_hash_seeds,
		}
	}

	fn calc_hash<AF,PF>(&self,h:T,t:&Teban,b:&Banmen,mc:&MochigomaCollections,
												m:&Move,obtained:&Option<MochigomaKind>,add:AF,pull:PF)
		-> T where AF: Fn(T,T) -> T, PF: Fn(T,T) -> T {
		match b {
			&Banmen(ref kinds) => {
				match m {
					&Move::To(KomaSrcPosition(sx,sy), KomaDstToPosition(dx, dy, n)) => {
						let sx = (9 - sx) as usize;
						let sy = (sy - 1) as usize;
						let dx = (9 - dx) as usize;
						let dy = dy as usize - 1;

						let mut hash = h;
						let k = kinds[sy][sx];

						hash =  pull(hash,self.kyokumen_hash_seeds[k as usize][sy * 8 + sx]);
						hash = add(hash,self.kyokumen_hash_seeds[KomaKind::Blank as usize][sy * 8 + sx]);

						let dk = kinds[dy][dx] as usize;

						hash =  pull(hash,self.kyokumen_hash_seeds[dk][dy * 8 + dx]);

						let k = if n {
							match k {
								KomaKind::SFu => KomaKind::SFuN,
								KomaKind::SKyou => KomaKind::SKyouN,
								KomaKind::SKei => KomaKind::SKeiN,
								KomaKind::SGin => KomaKind::SGinN,
								KomaKind::SKaku => KomaKind::SKakuN,
								KomaKind::SHisha => KomaKind::SHishaN,
								KomaKind::GFu => KomaKind::GFuN,
								KomaKind::GKyou => KomaKind::GKyouN,
								KomaKind::GKei => KomaKind::GKeiN,
								KomaKind::GGin => KomaKind::GGinN,
								KomaKind::GKaku => KomaKind::GKakuN,
								KomaKind::GHisha => KomaKind::GHishaN,
								k => k,
							}
						} else {
							k
						} as usize;

						hash = add(hash,self.kyokumen_hash_seeds[k][dy * 8 + dx]);

						hash = match obtained  {
								&None => hash,
								&Some(ref obtained) => {
									let c =  match t {
										&Teban::Sente => {
											match mc {
												&MochigomaCollections::Pair(ref mc,_) => {
													match mc.get(obtained) {
														Some(c) => *c as usize,
														None => 0,
													}
												},
												&MochigomaCollections::Empty => 0,
											}
										},
										&Teban::Gote => {
											match mc {
												&MochigomaCollections::Pair(_,ref mc) => {
													match mc.get(obtained) {
														Some(c) => *c as usize,
														None => 0,
													}
												},
												&MochigomaCollections::Empty => 0,
											}
										}
									};

									let k = *obtained as usize;

									match t {
										&Teban::Sente => {
											hash = add(hash,self.mochigoma_hash_seeds[0][c][k]);
										},
										&Teban::Gote => {
											hash = add(hash,self.mochigoma_hash_seeds[1][c][k]);
										}
									}
									hash
								}
						};

						hash
					},
					&Move::Put(ref mk, ref md) => {
						let mut hash = h;

						let c = match t {
							&Teban::Sente => {
								match mc {
									&MochigomaCollections::Pair(ref mc,_) => {
										match mc.get(&mk) {
											None | Some(&0) => {
												return hash;
											}
											Some(c) => *c as usize,
										}
									},
									&MochigomaCollections::Empty => {
										return hash;
									}
								}
							},
							&Teban::Gote => {
								match mc {
									&MochigomaCollections::Pair(_,ref mc) => {
										match mc.get(&mk) {
											None | Some(&0) => {
												return hash;
											}
											Some(c) => *c as usize,
										}
									},
									&MochigomaCollections::Empty => {
										return hash;
									}
								}
							}
						};

						let k = *mk as usize;

						match t {
							&Teban::Sente => {
								hash = pull(hash,self.mochigoma_hash_seeds[0][c-1][k]);
							},
							&Teban::Gote => {
								hash = pull(hash,self.mochigoma_hash_seeds[1][c-1][k]);
							}
						}

						let dx = 9 - md.0 as usize;
						let dy = md.1 as usize - 1;

						let dk = kinds[dy][dx] as usize;

						hash = pull(hash,self.kyokumen_hash_seeds[dk as usize][dy * 8 + dx]);

						let k = KomaKind::from((*t,*mk)) as usize;

						hash = add(hash,self.kyokumen_hash_seeds[k as usize][dy * 8 + dx]);
						hash
					}
				}
			}
		}
	}

	pub fn calc_main_hash(&self,h:T,t:&Teban,b:&Banmen,mc:&MochigomaCollections,m:&Move,obtained:&Option<MochigomaKind>) -> T {
		self.calc_hash(h,t,b,mc,m,obtained,|h,v| h ^ v, |h,v| h ^ v)
	}

	pub fn calc_sub_hash(&self,h:T,t:&Teban,b:&Banmen,mc:&MochigomaCollections,m:&Move,obtained:&Option<MochigomaKind>) -> T {
		self.calc_hash(h,t,b,mc,m,obtained,|h,v| {
			let h = Wrapping(h);
			let v = Wrapping(v);
			(h + v).0
		}, |h,v| {
			let h = Wrapping(h);
			let v = Wrapping(v);
			(h - v).0
		})
	}


	pub fn calc_initial_hash(&self,b:&Banmen,
		ms:&HashMap<MochigomaKind,u32>,mg:&HashMap<MochigomaKind,u32>) -> (T,T) {
		let mut mhash:T = T::INITIAL_HASH;
		let mut shash:Wrapping<T> = Wrapping(T::INITIAL_HASH);

		match b {
			&Banmen(ref kinds) => {
				for y in 0..9 {
					for x in 0..9 {
						let k = kinds[y][x] as usize;
						mhash = mhash ^ self.kyokumen_hash_seeds[k][y * 8 + x];
						shash = shash + Wrapping(self.kyokumen_hash_seeds[k][y * 8 + x]);
					}
				}
			}
		}
		for k in &MOCHIGOMA_KINDS {
			match ms.get(&k) {
				Some(c) => {
					for i in 0..(*c as usize) {
						mhash = mhash ^ self.mochigoma_hash_seeds[0][i][*k as usize];
						shash = shash + Wrapping(self.mochigoma_hash_seeds[0][i][*k as usize]);
					}
				},
				None => (),
			}
			match mg.get(&k) {
				Some(c) => {
					for i in 0..(*c as usize) {
						mhash = mhash ^ self.mochigoma_hash_seeds[1][i][*k as usize];
						shash = shash + Wrapping(self.mochigoma_hash_seeds[1][i][*k as usize]);
					}
				},
				None => (),
			}
		}

		(mhash,shash.0)
	}
}