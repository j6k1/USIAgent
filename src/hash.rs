//! 局面のハッシュ表現を取り扱う
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Add;
use std::ops::Sub;
use std::ops::BitXor;
use rand::{prelude, Rng, SeedableRng};
use rand::prelude::{Distribution};
use rand_xorshift::XorShiftRng;
use std::num::Wrapping;
use rand::distributions::Standard;

use shogi::*;
use rule::AppliedMove;

/// 主キーとサブキーの二つのキーを用いたマップ
pub struct TwoKeyHashMap<K,T> where K: Eq + Hash {
	map:HashMap<K,Vec<(K,T)>>
}
impl<T,K> TwoKeyHashMap<K,T> where K: Eq + Hash {
	/// `TwoKeyHashMap`の生成
	pub fn new() -> TwoKeyHashMap<K,T> {
		let map:HashMap<K,Vec<(K,T)>> = HashMap::new();

		TwoKeyHashMap {
			map:map
		}
	}

	/// 主キーとサブキーに一致する項目の変更不能な参照を取得
	///
	/// # Arguments
	/// * `k` - 主キー
	/// * `sk` - サブキー
	pub fn get(&self,k:&K,sk:&K) -> Option<&T> {
		match self.map.get(k) {
			Some(v) if v.len() == 1 && v[0].0 == *sk => {
				Some(&v[0].1)
			},
			Some(v) if v.len() > 1 => {
				for e in v {
					if e.0 == *sk {
						return Some(&e.1);
					}
				}
				None
			},
			_ => None,
		}
	}

	/// 主キーとサブキーに一致する項目の変更可能な参照を取得
	///
	/// # Arguments
	/// * `k` - 主キー
	/// * `sk` - サブキー
	pub fn get_mut(&mut self,k:&K,sk:&K) -> Option<&mut T> {
		match self.map.get_mut(k) {
			Some(v) => {
				if v.len() == 1 && v[0].0 == *sk {
					Some(&mut v[0].1)
				} else if v.len() > 1 {
					for e in v {
						if e.0 == *sk {
							return Some(&mut e.1);
						}
					}
					None
				} else {
					None
				}
			},
			_ => None,
		}
	}

	/// 主キーとサブキーを指定して項目を挿入する
	///
	/// # Arguments
	/// * `k` - 主キー
	/// * `sk` - サブキー
	/// * `nv` - キーに対応する新しい項目（対応するキーがある場合は上書き、ない場合は追加される）
	pub fn insert(&mut self,k:K,sk:K,nv:T) -> Option<T> {
		match self.map.get_mut(&k) {
			Some(ref mut v) if v.len() == 1 => {
				if v[0].0 == sk {
					let old = v.remove(0).1;
					v.push((sk,nv));
					return Some(old);
				} else {
					v.push((sk,nv));
					return None;
				}
			},
			Some(ref mut v) if v.len() > 1 => {
				for i in 0..v.len() {
					if v[i].0 == sk {
						let old = v.remove(i).1;
						v.insert(i,(sk,nv));
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

	/// 指定された主キーとサブキーの項目を削除する
	///
	/// # Arguments
	/// * `k` - 主キー
	/// * `sk` - サブキー
	pub fn remove(&mut self,k:&K,sk:&K) -> Option<T> {
		let (s,old) = match self.map.get_mut(&k) {
			Some(ref mut v) if v.len() == 1 => {
				if v[0].0 == *sk {
					let old = v.remove(0).1;
					(true,Some(old))
				} else {
					return None;
				}
			},
			Some(ref mut v) if v.len() > 1 => {
				let mut r = None;
				for i in 0..v.len() {
					if v[i].0 == *sk {
						let old = v.remove(i).1;
						r = Some(old);
						break;
					}
				}

				if r.is_some() {
					(false,r)
				} else {
					return None;
				}
			},
			_ => (false,None),
		};

		if s {
			self.map.remove(k);
		}

		old
	}

	/// マップをクリア
	pub fn clear(&mut self) {
		self.map.clear();
	}
}
impl<T,K> Clone for TwoKeyHashMap<K,T> where K: Eq + Hash + Clone, T: Clone {
	fn clone(&self) -> TwoKeyHashMap<K,T> {
		TwoKeyHashMap {
			map:self.map.clone()
		}
	}
}
/// 先手と後手の局面を`TwoKeyHashMap`を使って管理する
pub struct KyokumenMap<K,T> where K: Eq + Hash + Clone, T: Clone {
	sente_map:TwoKeyHashMap<K,T>,
	gote_map:TwoKeyHashMap<K,T>
}
impl<T,K> KyokumenMap<K,T> where K: Eq + Hash + Clone, T: Clone {
	/// `KyokumenMap`の生成
	pub fn new()
		-> KyokumenMap<K,T> {

		KyokumenMap {
			sente_map:TwoKeyHashMap::new(),
			gote_map:TwoKeyHashMap::new(),
		}
	}

	/// 指定された手番の主キーとサブキーに一致する項目の変更不能な参照を取得
	///
	/// # Arguments
	/// * `teban` - 手番
	/// * `k` - 主キー
	/// * `sk` - サブキー
	pub fn get(&self,teban:Teban,k:&K,sk:&K) -> Option<&T> {
		match teban {
			Teban::Sente => self.sente_map.get(k,sk),
			Teban::Gote => self.gote_map.get(k,sk),
		}
	}

	/// 指定された手番の主キーとサブキーに一致する項目の変更可能な参照を取得
	///
	/// # Arguments
	/// * `teban` - 手番
	/// * `k` - 主キー
	/// * `sk` - サブキー
	pub fn get_mut(&mut self,teban:Teban,k:&K,sk:&K) -> Option<&mut T> {
		match teban {
			Teban::Sente => self.sente_map.get_mut(k,sk),
			Teban::Gote => self.gote_map.get_mut(k,sk),
		}
	}

	/// 手番と主キーとサブキーを指定して項目を挿入する
	///
	/// # Arguments
	/// * `teban` - 手番
	/// * `k` - 主キー
	/// * `sk` - サブキー
	/// * `nv` - キーに対応する新しい項目（対応するキーがある場合は上書き、ない場合は追加される）
	pub fn insert(&mut self,teban:Teban,k:K,sk:K,nv:T) -> Option<T> {
		match teban {
			Teban::Sente => self.sente_map.insert(k,sk,nv),
			Teban::Gote => self.gote_map.insert(k,sk,nv),
		}
	}

	/// 指定された主キーとサブキーの項目を削除する
	///
	/// # Arguments
	/// * `teban` - 手番
	/// * `k` - 主キー
	/// * `sk` - サブキー
	pub fn remove(&mut self,teban:Teban,k:&K,sk:&K) -> Option<T> {
		match teban {
			Teban::Sente => self.sente_map.remove(k,sk),
			Teban::Gote => self.gote_map.remove(k,sk),
		}
	}

	/// 手番を指定してマップをクリア
	///
	/// # Arguments
	/// * `teban` - 手番
	pub fn clear(&mut self,teban:Teban) {
		match teban {
			Teban::Sente => self.sente_map.clear(),
			Teban::Gote => self.gote_map.clear(),
		}
	}
}
impl<K,T> Clone for KyokumenMap<K,T> where K: Eq + Hash + Clone, T: Clone, TwoKeyHashMap<K,T>: Clone {
	fn clone(&self) -> KyokumenMap<K,T> {
		KyokumenMap {
			sente_map:self.sente_map.clone(),
			gote_map:self.gote_map.clone(),
		}
	}
}
const KOMA_KIND_MAX:usize = KomaKind::Blank as usize;
const MOCHIGOMA_KIND_MAX:usize = MochigomaKind::Hisha as usize;
const MOCHIGOMA_MAX:usize = 18;
const SUJI_MAX:usize = 9;
const DAN_MAX:usize = 9;

/// 型に対応するハッシュの初期値
pub trait InitialHash {
	const INITIAL_HASH:Self;
}
impl InitialHash for u128 {
	const INITIAL_HASH:u128 = 0;
}
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
/// 局面のハッシュ値を計算する。差分計算対応
pub struct KyokumenHash<T>
	where T: Add + Sub + BitXor<Output = T> + Copy + InitialHash,
			Wrapping<T>: Add<Output = Wrapping<T>> + Sub<Output = Wrapping<T>> + BitXor<Output = Wrapping<T>> + Copy,
		    Standard: Distribution<T> {
	kyokumen_hash_seeds:[[T; SUJI_MAX * DAN_MAX]; KOMA_KIND_MAX + 1],
	mochigoma_hash_seeds:[[[T; MOCHIGOMA_KIND_MAX + 1]; MOCHIGOMA_MAX]; 2],
}
impl<T> KyokumenHash<T>
	where T: Add + Sub + BitXor<Output = T> + Copy + InitialHash,
			Wrapping<T>: Add<Output = Wrapping<T>> + Sub<Output = Wrapping<T>> + BitXor<Output = Wrapping<T>> + Copy,
		    Standard: Distribution<T> {
	/// `KyokumenHash`の生成
	pub fn new() -> KyokumenHash<T> {
		let mut rnd = prelude::thread_rng();
		let mut rnd = XorShiftRng::from_seed(rnd.gen::<[u8;16]>());

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

	fn calc_hash<AF,PF>(&self,h:T,t:Teban,b:&Banmen,mc:&MochigomaCollections,
												m:AppliedMove,obtained:&Option<MochigomaKind>,add:AF,pull:PF)
		-> T where AF: Fn(T,T) -> T, PF: Fn(T,T) -> T {
		match b {
			&Banmen(ref kinds) => {
				match m {
					AppliedMove::To(m) => {
						let from = m.src();
						let to = m.dst();
						let sx = from as usize / 9;
						let sy = from as usize - sx as usize * 9;
						let dx = to as usize / 9;
						let dy = to as usize - dx as usize * 9;
						let n = m.is_nari();

						let mut hash = h;
						let k = kinds[sy][sx];

						hash =  pull(hash,self.kyokumen_hash_seeds[k as usize][sy * 9 + sx]);
						hash = add(hash,self.kyokumen_hash_seeds[KomaKind::Blank as usize][sy * 9 + sx]);

						let dk = kinds[dy][dx] as usize;

						hash =  pull(hash,self.kyokumen_hash_seeds[dk][dy * 9 + dx]);

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

						hash = add(hash,self.kyokumen_hash_seeds[k][dy * 9 + dx]);

						hash = match obtained  {
								&None => hash,
								&Some(obtained) => {
									let c =  match t {
										Teban::Sente => {
											match mc {
												&MochigomaCollections::Pair(ref mc,_) => {
													mc.get(obtained)
												},
												&MochigomaCollections::Empty => 0,
											}
										},
										Teban::Gote => {
											match mc {
												&MochigomaCollections::Pair(_,ref mc) => {
													mc.get(obtained)
												},
												&MochigomaCollections::Empty => 0,
											}
										}
									};

									let k = obtained as usize;

									match t {
										Teban::Sente => {
											hash = add(hash,self.mochigoma_hash_seeds[0][c][k]);
										},
										Teban::Gote => {
											hash = add(hash,self.mochigoma_hash_seeds[1][c][k]);
										}
									}
									hash
								}
						};

						hash
					},
					AppliedMove::Put(m) => {
						let to = m.dst();
						let mk = m.kind();
						let dx = to as usize / 9;
						let dy = to as usize - dx as usize * 9;

						let mut hash = h;

						let c = match t {
							Teban::Sente => {
								match mc {
									&MochigomaCollections::Pair(ref mc,_) => {
										match mc.get(mk) {
											0 => {
												return hash;
											}
											c => c,
										}
									},
									&MochigomaCollections::Empty => {
										return hash;
									}
								}
							},
							Teban::Gote => {
								match mc {
									&MochigomaCollections::Pair(_,ref mc) => {
										match mc.get(mk) {
											0 => {
												return hash;
											}
											c => c,
										}
									},
									&MochigomaCollections::Empty => {
										return hash;
									}
								}
							}
						};

						let k = mk as usize;

						match t {
							Teban::Sente => {
								hash = pull(hash,self.mochigoma_hash_seeds[0][c-1][k]);
							},
							Teban::Gote => {
								hash = pull(hash,self.mochigoma_hash_seeds[1][c-1][k]);
							}
						}

						let dk = kinds[dy][dx] as usize;

						hash = pull(hash,self.kyokumen_hash_seeds[dk as usize][dy * 9 + dx]);

						let k = KomaKind::from((t,mk)) as usize;

						hash = add(hash,self.kyokumen_hash_seeds[k as usize][dy * 9 + dx]);
						hash
					}
				}
			}
		}
	}

	/// メインハッシュを計算
	///
	/// # Arguments
	/// * `h` - 現在のハッシュ
	/// * `t` - 手番
	/// * `b` - 手の適用前の盤面
	/// * `mc` - 手の適用前の持ち駒
	/// * `m` - 現在のハッシュに対して適用する指し手
	/// * `obtained` - 獲った駒
	pub fn calc_main_hash(&self,h:T,t:Teban,b:&Banmen,mc:&MochigomaCollections,m:AppliedMove,obtained:&Option<MochigomaKind>) -> T {
		self.calc_hash(h,t,b,mc,m,obtained,|h,v| h ^ v, |h,v| h ^ v)
	}

	/// サブハッシュを計算
	///
	/// # Arguments
	/// * `h` - 現在のハッシュ
	/// * `b` - 手の適用前の盤面
	/// * `t` - 手番
	/// * `mc` - 手の適用前の持ち駒
	/// * `m` - 現在のハッシュに対して適用する指し手
	/// * `obtained` - 獲った駒
	pub fn calc_sub_hash(&self,h:T,t:Teban,b:&Banmen,mc:&MochigomaCollections,m:AppliedMove,obtained:&Option<MochigomaKind>) -> T {
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

	/// ハッシュの初期値を計算
	///
	/// # Arguments
	/// * `b` - 盤面
	/// * `ms` - 先手の持ち駒
	/// * `mg` - 後手の持ち駒
	pub fn calc_initial_hash(&self,b:&Banmen,ms:&Mochigoma,mg:&Mochigoma) -> (T,T) {
		let mut mhash:T = T::INITIAL_HASH;
		let mut shash:Wrapping<T> = Wrapping(T::INITIAL_HASH);

		match b {
			&Banmen(ref kinds) => {
				for y in 0..9 {
					for x in 0..9 {
						let k = kinds[y][x] as usize;
						mhash = mhash ^ self.kyokumen_hash_seeds[k][y * 9 + x];
						shash = shash + Wrapping(self.kyokumen_hash_seeds[k][y * 9 + x]);
					}
				}
			}
		}
		for &k in &MOCHIGOMA_KINDS {
			for i in 0..(ms.get(k)) {
				mhash = mhash ^ self.mochigoma_hash_seeds[0][i][k as usize];
				shash = shash + Wrapping(self.mochigoma_hash_seeds[0][i][k as usize]);
			}

			for i in 0..(mg.get(k)) {
				mhash = mhash ^ self.mochigoma_hash_seeds[1][i][k as usize];
				shash = shash + Wrapping(self.mochigoma_hash_seeds[1][i][k as usize]);
			}
		}

		(mhash,shash.0)
	}
}