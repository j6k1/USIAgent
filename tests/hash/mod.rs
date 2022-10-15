use std::convert::TryFrom;

use usiagent::shogi::*;
use usiagent::rule::{AppliedMove, LegalMove, LegalMoveTo, LegalMovePut};
use usiagent::rule::BANMEN_START_POS;
use usiagent::hash::*;

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

#[test]
fn test_moveto_kinds_sente() {
    let mvs:Vec<((u32,u32),(u32,u32))> = vec![
        ((0,6),(0,5)),
        ((0,8),(0,7)),
        ((1,8),(2,6)),
        ((2,8),(2,7)),
        ((3,8),(3,7)),
        ((1,7),(2,6)),
        ((7,7),(6,7)),
        ((4,8),(4,7))
    ];

    let hasher = KyokumenHash::<u64>::new();

    for &((sx,sy),(dx,dy)) in &mvs {
        let mut banmen = BANMEN_START_POS.clone();
        banmen.0[6][2] = Blank;
        banmen.0[5][3] = SKaku;

        let (mut mhash, mut shash) = hasher.calc_initial_hash(&banmen,&Mochigoma::new(),&Mochigoma::new());

        let expected = {
            let kind = banmen.0[sy as usize][sx as usize];

            let mut banmen = banmen.clone();

            banmen.0[sy as usize][sx as usize] = Blank;
            banmen.0[dy as usize][dx as usize] = kind;

            hasher.calc_initial_hash(&banmen,&Mochigoma::new(),&Mochigoma::new())
        };

        let teban = Teban::Sente;
        let mc = MochigomaCollections::Empty;
        let m = AppliedMove::from(LegalMove::from(LegalMove::To(LegalMoveTo::new(sx*9+sy,dx*9+dy,false,None))));

        mhash = hasher.calc_main_hash(mhash,teban,&banmen,&mc,m,&None);
        shash = hasher.calc_sub_hash(shash,teban,&banmen,&mc,m,&None);

        assert_eq!(expected,(mhash,shash));
    }
}
#[test]
fn test_moveto_nari_kinds_sente() {
    let mvs:Vec<((u32,u32),(u32,u32))> = vec![
        ((0,6),(0,5)),
        ((0,8),(0,7)),
        ((1,8),(1,7)),
        ((2,8),(2,7)),
        ((1,7),(2,6)),
        ((7,7),(6,7)),
    ];

    let hasher = KyokumenHash::<u64>::new();

    for &((sx,sy),(dx,dy)) in &mvs {
        let mut banmen = BANMEN_START_POS.clone();
        banmen.0[6][2] = Blank;
        banmen.0[5][2] = SKaku;
        banmen.0[6][0] = SFuN;
        banmen.0[8][0] = SKyouN;
        banmen.0[8][1] = SKeiN;
        banmen.0[8][2] = SGinN;
        banmen.0[7][1] = SKakuN;
        banmen.0[7][7] = SHishaN;

        let (mut mhash, mut shash) = hasher.calc_initial_hash(&banmen,&Mochigoma::new(),&Mochigoma::new());

        let expected = {
            let kind = banmen.0[sy as usize][sx as usize];

            let mut banmen = banmen.clone();

            banmen.0[sy as usize][sx as usize] = Blank;
            banmen.0[dy as usize][dx as usize] = kind;

            hasher.calc_initial_hash(&banmen,&Mochigoma::new(),&Mochigoma::new())
        };

        let teban = Teban::Sente;
        let mc = MochigomaCollections::Empty;
        let m = AppliedMove::from(LegalMove::To(LegalMoveTo::new(sx*9+sy,dx*9+dy,false,None)));

        mhash = hasher.calc_main_hash(mhash,teban,&banmen,&mc,m,&None);
        shash = hasher.calc_sub_hash(shash,teban,&banmen,&mc,m,&None);

        assert_eq!(expected,(mhash,shash));
    }
}
#[test]
fn test_moveto_kinds_to_nari_sente() {
    let mvs:Vec<((u32,u32),(u32,u32),(u32,u32))> = vec![
        ((0,6),(4,3),(5,2)),
        ((0,8),(4,3),(5,2)),
        ((1,8),(3,4),(4,2)),
        ((2,8),(4,3),(5,2)),
        ((1,7),(2,6),(6,2)),
        ((7,7),(4,3),(4,2))
    ];

    let hasher = KyokumenHash::<u64>::new();

    for &((rx,ry),(sx,sy),(dx,dy)) in &mvs {
        let mut banmen = BANMEN_START_POS.clone();
        banmen.0[6][2] = Blank;
        banmen.0[5][3] = SKaku;
        let kind = banmen.0[ry as usize][rx as usize];
        banmen.0[ry as usize][rx as usize] = Blank;
        banmen.0[sy as usize][sx as usize] = kind;

        let (mut mhash, mut shash) = hasher.calc_initial_hash(&banmen,&Mochigoma::new(),&Mochigoma::new());

        let expected = {
            let mut banmen = banmen.clone();

            banmen.0[sy as usize][sx as usize] = Blank;
            banmen.0[dy as usize][dx as usize] = kind.to_nari();

            hasher.calc_initial_hash(&banmen,&Mochigoma::new(),&Mochigoma::new())
        };

        let teban = Teban::Sente;
        let mc = MochigomaCollections::Empty;
        let m = AppliedMove::from(LegalMove::To(LegalMoveTo::new(sx*9+sy,dx*9+dy,true,None)));

        mhash = hasher.calc_main_hash(mhash,teban,&banmen,&mc,m,&None);
        shash = hasher.calc_sub_hash(shash,teban,&banmen,&mc,m,&None);

        assert_eq!(expected,(mhash,shash));
    }
}
#[test]
fn test_moveto_kinds_obtained_sente() {
    let obtained:Vec<((u32, u32), KomaKind)> = vec![
        ((0,2),KomaKind::GFu),
        ((0,0),KomaKind::GKyou),
        ((1,0),KomaKind::GKei),
        ((2,0),KomaKind::GGin),
        ((3,0),KomaKind::GKin),
        ((1,1),KomaKind::GHisha),
        ((7,1),KomaKind::GKaku),
        ((0,2),KomaKind::GFuN),
        ((0,0),KomaKind::GKyouN),
        ((1,0),KomaKind::GKeiN),
        ((2,0),KomaKind::GGinN),
        ((1,1),KomaKind::GHishaN),
        ((7,1),KomaKind::GKakuN),
    ];

    let hasher = KyokumenHash::<u64>::new();

    for &((rx,ry),kind) in &obtained {
        let mut banmen = BANMEN_START_POS.clone();
        banmen.0[6][2] = Blank;
        banmen.0[5][3] = SKaku;
        banmen.0[ry as usize][rx as usize] = Blank;
        banmen.0[2][4] = kind;
        banmen.0[3][4] = SFu;

        let ms = Mochigoma::new();
        let mut mg = Mochigoma::new();

        mg.insert(MochigomaKind::Fu,1);

        let (mut mhash, mut shash) = hasher.calc_initial_hash(&banmen,&ms,&mg);

        let expected = {
            let mut banmen = banmen.clone();

            banmen.0[3][4] = Blank;
            banmen.0[2][4] = SFu;

            let mut ms = Mochigoma::new();
            ms.insert(MochigomaKind::try_from(kind).unwrap(),1);
            let mut mg = Mochigoma::new();
            mg.insert(MochigomaKind::Fu,1);

            hasher.calc_initial_hash(&banmen,&ms,&mg)
        };

        let teban = Teban::Sente;
        let mc = MochigomaCollections::Pair(ms,mg);

        let obtained = Some(ObtainKind::try_from(kind).unwrap());

        let m = AppliedMove::from(LegalMove::To(LegalMoveTo::new(4*9+3,4*9+2,false,obtained)));

        let obtained = Some(MochigomaKind::try_from(kind).unwrap());

        mhash = hasher.calc_main_hash(mhash,teban,&banmen,&mc,m,&obtained);
        shash = hasher.calc_sub_hash(shash,teban,&banmen,&mc,m,&obtained);

        assert_eq!(expected,(mhash,shash));
    }
}
#[test]
fn test_moveto_kinds_mochigoma_empty_obtained_sente() {
    let obtained:Vec<((u32, u32), KomaKind)> = vec![
        ((0,2),KomaKind::GFu),
        ((0,0),KomaKind::GKyou),
        ((1,0),KomaKind::GKei),
        ((2,0),KomaKind::GGin),
        ((3,0),KomaKind::GKin),
        ((1,1),KomaKind::GHisha),
        ((7,1),KomaKind::GKaku),
        ((0,2),KomaKind::GFuN),
        ((0,0),KomaKind::GKyouN),
        ((1,0),KomaKind::GKeiN),
        ((2,0),KomaKind::GGinN),
        ((1,1),KomaKind::GHishaN),
        ((7,1),KomaKind::GKakuN),
    ];

    let hasher = KyokumenHash::<u64>::new();

    for &((rx,ry),kind) in &obtained {
        let mut banmen = BANMEN_START_POS.clone();
        banmen.0[6][2] = Blank;
        banmen.0[5][3] = SKaku;
        banmen.0[ry as usize][rx as usize] = Blank;
        banmen.0[2][4] = kind;
        banmen.0[3][4] = SFu;
        banmen.0[4][4] = GFu;

        let (mut mhash, mut shash) = hasher.calc_initial_hash(&banmen,&Mochigoma::new(),&Mochigoma::new());

        let expected = {
            let mut banmen = banmen.clone();

            banmen.0[3][4] = Blank;
            banmen.0[2][4] = SFu;

            let mut ms = Mochigoma::new();
            ms.insert(MochigomaKind::try_from(kind).unwrap(),1);
            let mg = Mochigoma::new();

            hasher.calc_initial_hash(&banmen,&ms,&mg)
        };

        let teban = Teban::Sente;
        let mc = MochigomaCollections::Empty;

        let obtained = Some(ObtainKind::try_from(kind).unwrap());

        let m = AppliedMove::from(LegalMove::To(LegalMoveTo::new(4*9+3,4*9+2,false,obtained)));

        let obtained = Some(MochigomaKind::try_from(kind).unwrap());

        mhash = hasher.calc_main_hash(mhash,teban,&banmen,&mc,m,&obtained);
        shash = hasher.calc_sub_hash(shash,teban,&banmen,&mc,m,&obtained);

        assert_eq!(expected,(mhash,shash));
    }
}
#[test]
fn test_moveto_kinds_obtained_2nd_sente() {
    let obtained:Vec<((u32, u32), KomaKind)> = vec![
        ((0,2),KomaKind::GFu),
        ((0,0),KomaKind::GKyou),
        ((1,0),KomaKind::GKei),
        ((2,0),KomaKind::GGin),
        ((3,0),KomaKind::GKin),
        ((1,1),KomaKind::GHisha),
        ((7,1),KomaKind::GKaku),
        ((0,2),KomaKind::GFuN),
        ((0,0),KomaKind::GKyouN),
        ((1,0),KomaKind::GKeiN),
        ((2,0),KomaKind::GGinN),
        ((1,1),KomaKind::GHishaN),
        ((7,1),KomaKind::GKakuN),
    ];

    let hasher = KyokumenHash::<u64>::new();

    for &((rx,ry),kind) in &obtained {
        let mut banmen = BANMEN_START_POS.clone();
        banmen.0[6][2] = Blank;
        banmen.0[5][3] = SKaku;
        banmen.0[ry as usize][rx as usize] = Blank;
        banmen.0[2][4] = kind;
        banmen.0[3][4] = SFu;

        let mut ms = Mochigoma::new();

        for &kind in &MOCHIGOMA_KINDS {
            ms.insert(kind,1);
        }

        let mut mg = Mochigoma::new();

        mg.insert(MochigomaKind::Fu,1);

        let (mhash, shash) = hasher.calc_initial_hash(&banmen,&ms,&mg);

        let expected = {
            let mut banmen = banmen.clone();

            banmen.0[3][4] = Blank;
            banmen.0[2][4] = SFu;

            let mut ms = Mochigoma::new();

            for &kind in &MOCHIGOMA_KINDS {
                ms.insert(kind,1);
            }
            ms.insert(MochigomaKind::try_from(kind).unwrap(),2);

            let mut mg = Mochigoma::new();
            mg.insert(MochigomaKind::Fu,1);

            hasher.calc_initial_hash(&banmen,&ms,&mg)
        };

        let teban = Teban::Sente;
        let mc = MochigomaCollections::Pair(ms,mg);

        let obtained = Some(ObtainKind::try_from(kind).unwrap());

        let m = AppliedMove::from(LegalMove::To(LegalMoveTo::new(4*9+3,4*9+2,false,obtained)));

        let obtained = Some(MochigomaKind::try_from(kind).unwrap());

        let mhash = hasher.calc_main_hash(mhash,teban,&banmen,&mc,m,&obtained);
        let shash = hasher.calc_sub_hash(shash,teban,&banmen,&mc,m,&obtained);

        assert_eq!(expected,(mhash,shash));
    }
}
#[test]
fn test_moveto_kinds_obtained_mochigoma_limit_sente() {
    let hasher = KyokumenHash::<u64>::new();

    let mut banmen = BANMEN_START_POS.clone();
    banmen.0[2][4] = GFu;
    banmen.0[3][4] = SKyou;
    banmen.0[0][8] = Blank;

    for x in 0..9 {
        banmen.0[2][x] = Blank;
        banmen.0[6][x] = Blank;
    }

    let mut ms = Mochigoma::new();
    ms.insert(MochigomaKind::Fu,17);

    let mg = Mochigoma::new();

    let (mut mhash, mut shash) = hasher.calc_initial_hash(&banmen,&ms,&mg);

    let expected = {
        let mut banmen = banmen.clone();

        banmen.0[3][4] = Blank;
        banmen.0[2][4] = SKyou;

        let mut ms = Mochigoma::new();
        ms.insert(MochigomaKind::Fu,18);

        let mg = Mochigoma::new();

        hasher.calc_initial_hash(&banmen,&ms,&mg)
    };

    let teban = Teban::Sente;
    let mc = MochigomaCollections::Pair(ms,mg);

    let obtained = Some(ObtainKind::Fu);

    let m = AppliedMove::from(LegalMove::To(LegalMoveTo::new(4*9+3,4*9+2,false,obtained)));

    let obtained = Some(MochigomaKind::Fu);

    mhash = hasher.calc_main_hash(mhash,teban,&banmen,&mc,m,&obtained);
    shash = hasher.calc_sub_hash(shash,teban,&banmen,&mc,m,&obtained);

    assert_eq!(expected,(mhash,shash));
}
#[test]
fn test_put_sente() {
    let hasher = KyokumenHash::<u64>::new();

    for &kind in &MOCHIGOMA_KINDS {
        let mut banmen = BANMEN_START_POS.clone();
        banmen.0[6][2] = Blank;
        banmen.0[8][0] = Blank;
        banmen.0[8][1] = Blank;
        banmen.0[8][2] = Blank;
        banmen.0[8][3] = Blank;
        banmen.0[7][1] = Blank;
        banmen.0[7][7] = Blank;

        let mut ms = Mochigoma::new();

        for &kind in &MOCHIGOMA_KINDS {
            ms.insert(kind,1);
        }

        let mg = Mochigoma::new();

        let (mhash, shash) = hasher.calc_initial_hash(&banmen,&ms,&mg);

        let expected = {
            let mut banmen = banmen.clone();

            banmen.0[3][4] = KomaKind::from((Teban::Sente,kind));

            let mut ms = Mochigoma::new();

            for &kind in &MOCHIGOMA_KINDS {
                ms.insert(kind,1);
            }
            ms.insert(kind,0);

            let mg = Mochigoma::new();

            hasher.calc_initial_hash(&banmen,&ms,&mg)
        };

        let teban = Teban::Sente;
        let mc = MochigomaCollections::Pair(ms,mg);

        let m = AppliedMove::from(LegalMove::Put(LegalMovePut::new(kind,9*4+3)));

        let mhash = hasher.calc_main_hash(mhash,teban,&banmen,&mc,m,&None);
        let shash = hasher.calc_sub_hash(shash,teban,&banmen,&mc,m,&None);

        assert_eq!(expected,(mhash,shash));
    }
}
#[test]
fn test_put_mochigomacollection_empty_sente() {
    let hasher = KyokumenHash::<u64>::new();

    for &kind in &MOCHIGOMA_KINDS {
        let banmen = BANMEN_START_POS.clone();

        let (mhash, shash) = hasher.calc_initial_hash(&banmen,&Mochigoma::new(),&Mochigoma::new());

        let expected = {
            let banmen = banmen.clone();

            let ms = Mochigoma::new();
            let mg = Mochigoma::new();

            hasher.calc_initial_hash(&banmen,&ms,&mg)
        };

        let teban = Teban::Sente;
        let mc = MochigomaCollections::Empty;

        let m = AppliedMove::from(LegalMove::Put(LegalMovePut::new(kind,9*4+3)));

        let mhash = hasher.calc_main_hash(mhash,teban,&banmen,&mc,m,&None);
        let shash = hasher.calc_sub_hash(shash,teban,&banmen,&mc,m,&None);

        assert_eq!(expected,(mhash,shash));
    }
}
#[test]
fn test_put_mochigomacollection_all_zero_sente() {
    let hasher = KyokumenHash::<u64>::new();

    for &kind in &MOCHIGOMA_KINDS {
        let banmen = BANMEN_START_POS.clone();

        let (mhash, shash) = hasher.calc_initial_hash(&banmen,&Mochigoma::new(),&Mochigoma::new());

        let expected = {
            let banmen = banmen.clone();

            let ms = Mochigoma::new();
            let mg = Mochigoma::new();

            hasher.calc_initial_hash(&banmen,&ms,&mg)
        };

        let teban = Teban::Sente;
        let mc = MochigomaCollections::Pair(Mochigoma::new(),Mochigoma::new());

        let m = AppliedMove::from(LegalMove::Put(LegalMovePut::new(kind,9*4+3)));

        let mhash = hasher.calc_main_hash(mhash,teban,&banmen,&mc,m,&None);
        let shash = hasher.calc_sub_hash(shash,teban,&banmen,&mc,m,&None);

        assert_eq!(expected,(mhash,shash));
    }
}

#[test]
fn test_moveto_kinds_gote() {
    let mvs:Vec<((u32,u32),(u32,u32))> = vec![
        ((0,6),(0,5)),
        ((0,8),(0,7)),
        ((1,8),(2,6)),
        ((2,8),(2,7)),
        ((3,8),(3,7)),
        ((1,7),(2,6)),
        ((7,7),(6,7)),
        ((4,8),(4,7))
    ];

    let hasher = KyokumenHash::<u64>::new();

    for &((sx,sy),(dx,dy)) in &mvs {
        let mut banmen = BANMEN_START_POS.clone();
        banmen.0[2][6] = Blank;
        banmen.0[3][5] = SKaku;

        let (mut mhash, mut shash) = hasher.calc_initial_hash(&banmen,&Mochigoma::new(),&Mochigoma::new());

        let expected = {
            let kind = banmen.0[8 - sy as usize][8 - sx as usize];

            let mut banmen = banmen.clone();

            banmen.0[8 - sy as usize][8 - sx as usize] = Blank;
            banmen.0[8 - dy as usize][8 - dx as usize] = kind;

            hasher.calc_initial_hash(&banmen,&Mochigoma::new(),&Mochigoma::new())
        };

        let teban = Teban::Gote;
        let mc = MochigomaCollections::Empty;
        let m = AppliedMove::from(LegalMove::from(LegalMove::To(LegalMoveTo::new(80-(sx*9+sy),80-(dx*9+dy),false,None))));

        mhash = hasher.calc_main_hash(mhash,teban,&banmen,&mc,m,&None);
        shash = hasher.calc_sub_hash(shash,teban,&banmen,&mc,m,&None);

        assert_eq!(expected,(mhash,shash));
    }
}
#[test]
fn test_moveto_nari_kinds_gote() {
    let mvs:Vec<((u32,u32),(u32,u32))> = vec![
        ((0,6),(0,5)),
        ((0,8),(0,7)),
        ((1,8),(1,7)),
        ((2,8),(2,7)),
        ((1,7),(2,6)),
        ((7,7),(6,7)),
    ];

    let hasher = KyokumenHash::<u64>::new();

    for &((sx,sy),(dx,dy)) in &mvs {
        let mut banmen = BANMEN_START_POS.clone();
        banmen.0[2][6] = Blank;
        banmen.0[3][6] = SKaku;
        banmen.0[2][8] = SFuN;
        banmen.0[0][8] = SKyouN;
        banmen.0[0][7] = SKeiN;
        banmen.0[0][6] = SGinN;
        banmen.0[1][7] = SKakuN;
        banmen.0[1][1] = SHishaN;

        let (mut mhash, mut shash) = hasher.calc_initial_hash(&banmen,&Mochigoma::new(),&Mochigoma::new());

        let expected = {
            let kind = banmen.0[8 - sy as usize][8 - sx as usize];

            let mut banmen = banmen.clone();

            banmen.0[8 - sy as usize][8 - sx as usize] = Blank;
            banmen.0[8 - dy as usize][8 - dx as usize] = kind;

            hasher.calc_initial_hash(&banmen,&Mochigoma::new(),&Mochigoma::new())
        };

        let teban = Teban::Gote;
        let mc = MochigomaCollections::Empty;
        let m = AppliedMove::from(LegalMove::To(LegalMoveTo::new(80-(sx*9+sy),80-(dx*9+dy),false,None)));

        mhash = hasher.calc_main_hash(mhash,teban,&banmen,&mc,m,&None);
        shash = hasher.calc_sub_hash(shash,teban,&banmen,&mc,m,&None);

        assert_eq!(expected,(mhash,shash));
    }
}
#[test]
fn test_moveto_kinds_to_nari_gote() {
    let mvs:Vec<((u32,u32),(u32,u32),(u32,u32))> = vec![
        ((0,6),(4,3),(5,2)),
        ((0,8),(4,3),(5,2)),
        ((1,8),(3,4),(4,2)),
        ((2,8),(4,3),(5,2)),
        ((1,7),(2,6),(6,2)),
        ((7,7),(4,3),(4,2))
    ];

    let hasher = KyokumenHash::<u64>::new();

    for &((rx,ry),(sx,sy),(dx,dy)) in &mvs {
        let mut banmen = BANMEN_START_POS.clone();
        banmen.0[2][6] = Blank;
        banmen.0[3][6] = SKaku;
        let kind = banmen.0[ry as usize][rx as usize];
        banmen.0[8 - ry as usize][8 - rx as usize] = Blank;
        banmen.0[8 - sy as usize][8 - sx as usize] = kind;

        let (mut mhash, mut shash) = hasher.calc_initial_hash(&banmen,&Mochigoma::new(),&Mochigoma::new());

        let expected = {
            let mut banmen = banmen.clone();

            banmen.0[8 - sy as usize][8 - sx as usize] = Blank;
            banmen.0[8 - dy as usize][8 - dx as usize] = kind.to_nari();

            hasher.calc_initial_hash(&banmen,&Mochigoma::new(),&Mochigoma::new())
        };

        let teban = Teban::Gote;
        let mc = MochigomaCollections::Empty;
        let m = AppliedMove::from(LegalMove::To(LegalMoveTo::new(80-(sx*9+sy),80-(dx*9+dy),true,None)));

        mhash = hasher.calc_main_hash(mhash,teban,&banmen,&mc,m,&None);
        shash = hasher.calc_sub_hash(shash,teban,&banmen,&mc,m,&None);

        assert_eq!(expected,(mhash,shash));
    }
}
#[test]
fn test_moveto_kinds_obtained_gote() {
    let obtained:Vec<((u32, u32), KomaKind)> = vec![
        ((0,2),KomaKind::SFu),
        ((0,0),KomaKind::SKyou),
        ((1,0),KomaKind::SKei),
        ((2,0),KomaKind::SGin),
        ((3,0),KomaKind::SKin),
        ((1,1),KomaKind::SHisha),
        ((7,1),KomaKind::SKaku),
        ((0,2),KomaKind::SFuN),
        ((0,0),KomaKind::SKyouN),
        ((1,0),KomaKind::SKeiN),
        ((2,0),KomaKind::SGinN),
        ((1,1),KomaKind::SHishaN),
        ((7,1),KomaKind::SKakuN),
    ];

    let hasher = KyokumenHash::<u64>::new();

    for &((rx,ry),kind) in &obtained {
        let mut banmen = BANMEN_START_POS.clone();
        banmen.0[2][6] = Blank;
        banmen.0[3][6] = GKaku;
        banmen.0[8 - ry as usize][8 - rx as usize] = Blank;
        banmen.0[6][4] = kind;
        banmen.0[5][4] = GFu;

        let mg = Mochigoma::new();
        let mut ms = Mochigoma::new();

        ms.insert(MochigomaKind::Fu,1);

        let (mut mhash, mut shash) = hasher.calc_initial_hash(&banmen,&ms,&mg);

        let expected = {
            let mut banmen = banmen.clone();

            banmen.0[5][4] = Blank;
            banmen.0[6][4] = GFu;

            let mut mg = Mochigoma::new();
            mg.insert(MochigomaKind::try_from(kind).unwrap(),1);
            let mut ms = Mochigoma::new();
            ms.insert(MochigomaKind::Fu,1);

            hasher.calc_initial_hash(&banmen,&ms,&mg)
        };

        let teban = Teban::Gote;
        let mc = MochigomaCollections::Pair(ms,mg);

        let obtained = Some(ObtainKind::try_from(kind).unwrap());

        let m = AppliedMove::from(LegalMove::To(LegalMoveTo::new(80-(4*9+3),80-(4*9+2),false,obtained)));

        let obtained = Some(MochigomaKind::try_from(kind).unwrap());

        mhash = hasher.calc_main_hash(mhash,teban,&banmen,&mc,m,&obtained);
        shash = hasher.calc_sub_hash(shash,teban,&banmen,&mc,m,&obtained);

        assert_eq!(expected,(mhash,shash));
    }
}
#[test]
fn test_moveto_kinds_mochigoma_empty_obtained_gote() {
    let obtained:Vec<((u32, u32), KomaKind)> = vec![
        ((0,2),KomaKind::SFu),
        ((0,0),KomaKind::SKyou),
        ((1,0),KomaKind::SKei),
        ((2,0),KomaKind::SGin),
        ((3,0),KomaKind::SKin),
        ((1,1),KomaKind::SHisha),
        ((7,1),KomaKind::SKaku),
        ((0,2),KomaKind::SFuN),
        ((0,0),KomaKind::SKyouN),
        ((1,0),KomaKind::SKeiN),
        ((2,0),KomaKind::SGinN),
        ((1,1),KomaKind::SHishaN),
        ((7,1),KomaKind::SKakuN),
    ];

    let hasher = KyokumenHash::<u64>::new();

    for &((rx,ry),kind) in &obtained {
        let mut banmen = BANMEN_START_POS.clone();
        banmen.0[2][6] = Blank;
        banmen.0[3][6] = GKaku;
        banmen.0[8 - ry as usize][8 - rx as usize] = Blank;
        banmen.0[6][4] = kind;
        banmen.0[5][4] = GFu;
        banmen.0[4][4] = SFu;

        let (mut mhash, mut shash) = hasher.calc_initial_hash(&banmen,&Mochigoma::new(),&Mochigoma::new());

        let expected = {
            let mut banmen = banmen.clone();

            banmen.0[5][4] = Blank;
            banmen.0[6][4] = GFu;

            let mut mg = Mochigoma::new();
            mg.insert(MochigomaKind::try_from(kind).unwrap(),1);
            let ms = Mochigoma::new();

            hasher.calc_initial_hash(&banmen,&ms,&mg)
        };

        let teban = Teban::Gote;
        let mc = MochigomaCollections::Empty;

        let obtained = Some(ObtainKind::try_from(kind).unwrap());

        let m = AppliedMove::from(LegalMove::To(LegalMoveTo::new(80-(4*9+3),80-(4*9+2),false,obtained)));

        let obtained = Some(MochigomaKind::try_from(kind).unwrap());

        mhash = hasher.calc_main_hash(mhash,teban,&banmen,&mc,m,&obtained);
        shash = hasher.calc_sub_hash(shash,teban,&banmen,&mc,m,&obtained);

        assert_eq!(expected,(mhash,shash));
    }
}
#[test]
fn test_moveto_kinds_obtained_2nd_gote() {
    let obtained:Vec<((u32, u32), KomaKind)> = vec![
        ((0,2),KomaKind::SFu),
        ((0,0),KomaKind::SKyou),
        ((1,0),KomaKind::SKei),
        ((2,0),KomaKind::SGin),
        ((3,0),KomaKind::SKin),
        ((1,1),KomaKind::SHisha),
        ((7,1),KomaKind::SKaku),
        ((0,2),KomaKind::SFuN),
        ((0,0),KomaKind::SKyouN),
        ((1,0),KomaKind::SKeiN),
        ((2,0),KomaKind::SGinN),
        ((1,1),KomaKind::SHishaN),
        ((7,1),KomaKind::SKakuN),
    ];

    let hasher = KyokumenHash::<u64>::new();

    for &((rx,ry),kind) in &obtained {
        let mut banmen = BANMEN_START_POS.clone();
        banmen.0[2][6] = Blank;
        banmen.0[3][6] = GKaku;
        banmen.0[8 - ry as usize][8 - rx as usize] = Blank;
        banmen.0[6][4] = kind;
        banmen.0[5][4] = GFu;

        let mut mg = Mochigoma::new();

        for &kind in &MOCHIGOMA_KINDS {
            mg.insert(kind,1);
        }

        let mut ms = Mochigoma::new();

        ms.insert(MochigomaKind::Fu,1);

        let (mhash, shash) = hasher.calc_initial_hash(&banmen,&ms,&mg);

        let expected = {
            let mut banmen = banmen.clone();

            banmen.0[5][4] = Blank;
            banmen.0[6][4] = GFu;

            let mut mg = Mochigoma::new();

            for &kind in &MOCHIGOMA_KINDS {
                mg.insert(kind,1);
            }
            mg.insert(MochigomaKind::try_from(kind).unwrap(),2);

            let mut ms = Mochigoma::new();
            ms.insert(MochigomaKind::Fu,1);

            hasher.calc_initial_hash(&banmen,&ms,&mg)
        };

        let teban = Teban::Gote;
        let mc = MochigomaCollections::Pair(ms,mg);

        let obtained = Some(ObtainKind::try_from(kind).unwrap());

        let m = AppliedMove::from(LegalMove::To(LegalMoveTo::new(80-(4*9+3),80-(4*9+2),false,obtained)));

        let obtained = Some(MochigomaKind::try_from(kind).unwrap());

        let mhash = hasher.calc_main_hash(mhash,teban,&banmen,&mc,m,&obtained);
        let shash = hasher.calc_sub_hash(shash,teban,&banmen,&mc,m,&obtained);

        assert_eq!(expected,(mhash,shash));
    }
}
#[test]
fn test_moveto_kinds_obtained_mochigoma_limit_gote() {
    let hasher = KyokumenHash::<u64>::new();

    let mut banmen = BANMEN_START_POS.clone();
    banmen.0[6][4] = SFu;
    banmen.0[5][4] = GKyou;
    banmen.0[8][0] = Blank;

    for x in 0..9 {
        banmen.0[6][8 - x] = Blank;
        banmen.0[2][8 - x] = Blank;
    }

    let mut mg = Mochigoma::new();
    mg.insert(MochigomaKind::Fu,17);

    let ms = Mochigoma::new();

    let (mut mhash, mut shash) = hasher.calc_initial_hash(&banmen,&ms,&mg);

    let expected = {
        let mut banmen = banmen.clone();

        banmen.0[5][4] = Blank;
        banmen.0[6][4] = GKyou;

        let mut mg = Mochigoma::new();
        mg.insert(MochigomaKind::Fu,18);

        let ms = Mochigoma::new();

        hasher.calc_initial_hash(&banmen,&ms,&mg)
    };

    let teban = Teban::Gote;
    let mc = MochigomaCollections::Pair(ms,mg);

    let obtained = Some(ObtainKind::Fu);

    let m = AppliedMove::from(LegalMove::To(LegalMoveTo::new(80-(4*9+3),80-(4*9+2),false,obtained)));

    let obtained = Some(MochigomaKind::Fu);

    mhash = hasher.calc_main_hash(mhash,teban,&banmen,&mc,m,&obtained);
    shash = hasher.calc_sub_hash(shash,teban,&banmen,&mc,m,&obtained);

    assert_eq!(expected,(mhash,shash));
}
#[test]
fn test_put_gote() {
    let hasher = KyokumenHash::<u64>::new();

    for &kind in &MOCHIGOMA_KINDS {
        let mut banmen = BANMEN_START_POS.clone();
        banmen.0[2][6] = Blank;
        banmen.0[0][8] = Blank;
        banmen.0[0][7] = Blank;
        banmen.0[0][6] = Blank;
        banmen.0[0][5] = Blank;
        banmen.0[1][7] = Blank;
        banmen.0[1][1] = Blank;

        let mut mg = Mochigoma::new();

        for &kind in &MOCHIGOMA_KINDS {
            mg.insert(kind,1);
        }

        let ms = Mochigoma::new();

        let (mhash, shash) = hasher.calc_initial_hash(&banmen,&ms,&mg);

        let expected = {
            let mut banmen = banmen.clone();

            banmen.0[5][4] = KomaKind::from((Teban::Gote,kind));

            let mut mg = Mochigoma::new();

            for &kind in &MOCHIGOMA_KINDS {
                mg.insert(kind,1);
            }
            mg.insert(kind,0);

            let ms = Mochigoma::new();

            hasher.calc_initial_hash(&banmen,&ms,&mg)
        };

        let teban = Teban::Gote;
        let mc = MochigomaCollections::Pair(ms,mg);

        let m = AppliedMove::from(LegalMove::Put(LegalMovePut::new(kind,80-(9*4+3))));

        let mhash = hasher.calc_main_hash(mhash,teban,&banmen,&mc,m,&None);
        let shash = hasher.calc_sub_hash(shash,teban,&banmen,&mc,m,&None);

        assert_eq!(expected,(mhash,shash));
    }
}
#[test]
fn test_put_mochigomacollection_empty_gote() {
    let hasher = KyokumenHash::<u64>::new();

    for &kind in &MOCHIGOMA_KINDS {
        let banmen = BANMEN_START_POS.clone();

        let (mhash, shash) = hasher.calc_initial_hash(&banmen,&Mochigoma::new(),&Mochigoma::new());

        let expected = {
            let banmen = banmen.clone();

            let mg = Mochigoma::new();
            let ms = Mochigoma::new();

            hasher.calc_initial_hash(&banmen,&ms,&mg)
        };

        let teban = Teban::Gote;
        let mc = MochigomaCollections::Empty;

        let m = AppliedMove::from(LegalMove::Put(LegalMovePut::new(kind,80-(9*4+3))));

        let mhash = hasher.calc_main_hash(mhash,teban,&banmen,&mc,m,&None);
        let shash = hasher.calc_sub_hash(shash,teban,&banmen,&mc,m,&None);

        assert_eq!(expected,(mhash,shash));
    }
}
#[test]
fn test_put_mochigomacollection_all_zero_gote() {
    let hasher = KyokumenHash::<u64>::new();

    for &kind in &MOCHIGOMA_KINDS {
        let banmen = BANMEN_START_POS.clone();

        let (mhash, shash) = hasher.calc_initial_hash(&banmen,&Mochigoma::new(),&Mochigoma::new());

        let expected = {
            let banmen = banmen.clone();

            let mg = Mochigoma::new();
            let ms = Mochigoma::new();

            hasher.calc_initial_hash(&banmen,&ms,&mg)
        };

        let teban = Teban::Gote;
        let mc = MochigomaCollections::Pair(Mochigoma::new(),Mochigoma::new());

        let m = AppliedMove::from(LegalMove::Put(LegalMovePut::new(kind,80-(9*4+3))));

        let mhash = hasher.calc_main_hash(mhash,teban,&banmen,&mc,m,&None);
        let shash = hasher.calc_sub_hash(shash,teban,&banmen,&mc,m,&None);

        assert_eq!(expected,(mhash,shash));
    }
}
