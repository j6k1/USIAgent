use usiagent::rule::{LegalMove, LegalMovePut, LegalMoveTo, Rule, State};
use usiagent::shogi::{Banmen, KomaKind, MochigomaKind, Teban};
use usiagent::shogi::KomaKind::{Blank, GKin, GKyou, GOu, SKin, SKyou, SOu};

#[test]
fn is_oute_moveto_sente() {
    let position_and_kinds = vec![
        (4,5,KomaKind::SFu),
        (2,7,KomaKind::SKei),
        (6,7,KomaKind::SKei),
        (4,5,KomaKind::SGin),
        (3,5,KomaKind::SGin),
        (5,5,KomaKind::SGin),
        (2,1,KomaKind::SGin),
        (6,1,KomaKind::SGin),
        (4,5,KomaKind::SKin),
        (3,5,KomaKind::SKin),
        (5,5,KomaKind::SKin),
        (2,3,KomaKind::SKin),
        (6,3,KomaKind::SKin),
        (4,1,KomaKind::SKin),
        (4,5,KomaKind::SOu),
        (3,5,KomaKind::SOu),
        (5,5,KomaKind::SOu),
        (2,3,KomaKind::SOu),
        (6,3,KomaKind::SOu),
        (2,1,KomaKind::SOu),
        (6,1,KomaKind::SOu),
        (4,1,KomaKind::SOu),
        (4,5,KomaKind::SFuN),
        (3,5,KomaKind::SFuN),
        (5,5,KomaKind::SFuN),
        (2,3,KomaKind::SFuN),
        (6,3,KomaKind::SFuN),
        (4,1,KomaKind::SFuN),
        (4,5,KomaKind::SKyouN),
        (3,5,KomaKind::SKyouN),
        (5,5,KomaKind::SKyouN),
        (2,3,KomaKind::SKyouN),
        (6,3,KomaKind::SKyouN),
        (4,1,KomaKind::SKyouN),
        (4,5,KomaKind::SKeiN),
        (3,5,KomaKind::SKeiN),
        (5,5,KomaKind::SKeiN),
        (2,3,KomaKind::SKeiN),
        (6,3,KomaKind::SKeiN),
        (4,1,KomaKind::SKeiN),
        (4,5,KomaKind::SGinN),
        (3,5,KomaKind::SGinN),
        (5,5,KomaKind::SGinN),
        (2,3,KomaKind::SGinN),
        (6,3,KomaKind::SGinN),
        (4,1,KomaKind::SGinN),
        (4,1,KomaKind::SKakuN),
        (2,3,KomaKind::SKakuN),
        (6,3,KomaKind::SKakuN),
        (4,5,KomaKind::SKakuN),
        (2,1,KomaKind::SHishaN),
        (6,1,KomaKind::SHishaN),
        (2,5,KomaKind::SHishaN),
        (6,5,KomaKind::SHishaN)
    ];

    let mvs = vec![
        ((4,5),(4,4)),
        ((2,7),(3,5)),
        ((6,7),(5,5)),
        ((4,5),(4,4)),
        ((3,5),(3,4)),
        ((5,5),(5,4)),
        ((2,1),(3,2)),
        ((6,1),(5,2)),
        ((4,5),(4,4)),
        ((3,5),(3,4)),
        ((5,5),(5,4)),
        ((2,3),(3,3)),
        ((6,3),(5,3)),
        ((4,1),(4,2)),
        ((4,5),(4,4)),
        ((3,5),(3,4)),
        ((5,5),(5,4)),
        ((2,3),(3,3)),
        ((6,3),(5,3)),
        ((2,1),(3,2)),
        ((6,1),(5,2)),
        ((4,1),(4,2)),
        ((4,5),(4,4)),
        ((3,5),(3,4)),
        ((5,5),(5,4)),
        ((2,3),(3,3)),
        ((6,3),(5,3)),
        ((4,1),(4,2)),
        ((4,5),(4,4)),
        ((3,5),(3,4)),
        ((5,5),(5,4)),
        ((2,3),(3,3)),
        ((6,3),(5,3)),
        ((4,1),(4,2)),
        ((4,5),(4,4)),
        ((3,5),(3,4)),
        ((5,5),(5,4)),
        ((2,3),(3,3)),
        ((6,3),(5,3)),
        ((4,1),(4,2)),
        ((4,5),(4,4)),
        ((3,5),(3,4)),
        ((5,5),(5,4)),
        ((2,3),(3,3)),
        ((6,3),(5,3)),
        ((4,1),(4,2)),
        ((4,1),(4,2)),
        ((2,3),(3,3)),
        ((6,3),(5,3)),
        ((4,5),(4,4)),
        ((2,1),(3,2)),
        ((6,1),(5,2)),
        ((2,5),(3,4)),
        ((6,5),(5,4))
    ];

    for (&m,&(x,y,kind)) in mvs.iter().zip(position_and_kinds.iter()) {
        let mut banmen = Banmen([[Blank; 9]; 9]);

        banmen.0[3][4] = GOu;
        banmen.0[y][x] = kind;

        let state = State::new(banmen);

        assert_eq!(true,Rule::is_oute_move(&state,Teban::Sente,
                                            LegalMove::To(LegalMoveTo::new((m.0).0 * 9 + (m.0).1,
                                                                           (m.1).0 * 9 + (m.1).1,
                                                                           false,None))),"{:?} {:?}",kind,m);
    }
}
#[test]
fn is_oute_not_moveto_sente() {
    let position_and_kinds = vec![
        (4,6,KomaKind::SFu),
        (4,4,KomaKind::SKei),
        (2,4,KomaKind::SGin),
        (6,4,KomaKind::SGin),
        (3,1,KomaKind::SKin),
        (5,1,KomaKind::SKin),
        (4,6,KomaKind::SOu),
        (3,1,KomaKind::SFuN),
        (5,1,KomaKind::SFuN),
        (3,1,KomaKind::SKyouN),
        (5,1,KomaKind::SKyouN),
        (3,1,KomaKind::SKeiN),
        (5,1,KomaKind::SKeiN),
        (3,1,KomaKind::SGinN),
        (5,1,KomaKind::SGinN),
        (4,6,KomaKind::SKakuN),
        (2,0,KomaKind::SHishaN),
        (6,0,KomaKind::SHishaN)
    ];

    let mvs = vec![
        ((4,6),(4,5)),
        ((4,4),(3,2)),
        ((4,4),(5,2)),
        ((2,4),(3,3)),
        ((6,4),(5,3)),
        ((3,1),(3,2)),
        ((5,1),(5,2)),
        ((4,6),(4,5)),
        ((3,1),(3,2)),
        ((5,1),(5,2)),
        ((3,1),(3,2)),
        ((5,1),(5,2)),
        ((3,1),(3,2)),
        ((5,1),(5,2)),
        ((3,1),(3,2)),
        ((5,1),(5,2)),
        ((2,0),(2,1)),
        ((6,0),(6,1))
    ];

    for (&m,&(x,y,kind)) in mvs.iter().zip(position_and_kinds.iter()) {
        let mut banmen = Banmen([[Blank; 9]; 9]);

        banmen.0[3][4] = GOu;
        banmen.0[y][x] = kind;

        let state = State::new(banmen);

        assert_eq!(false,Rule::is_oute_move(&state,Teban::Sente,
                           LegalMove::To(LegalMoveTo::new((m.0).0 * 9 + (m.0).1,
                                                          (m.1).0 * 9 + (m.1).1,
                                                          false,None))),"{:?} {:?}",kind,m);
    }
}
#[test]
fn is_oute_moveto_sente_hisha() {
    let position_and_kinds = vec![
        (3,0),
        (0,2),
        (8,2),
        (3,8)
    ];

    let mvs = vec![
        ((3,0),(4,0)),
        ((0,2),(0,3)),
        ((8,2),(8,3)),
        ((3,8),(4,8))
    ];

    for &kind in [KomaKind::SHisha,KomaKind::SHishaN].iter() {
        for (&m, &(x, y)) in mvs.iter().zip(position_and_kinds.iter()) {
            let mut banmen = Banmen([[Blank; 9]; 9]);

            banmen.0[3][4] = GOu;
            banmen.0[y][x] = kind;

            let state = State::new(banmen);

            assert_eq!(true,Rule::is_oute_move(&state, Teban::Sente,
                                       LegalMove::To(LegalMoveTo::new((m.0).0 * 9 + (m.0).1,
                                                                      (m.1).0 * 9 + (m.1).1,
                                                                      false, None))), "{:?} {:?}", kind, m);
        }
    }
}
#[test]
fn is_oute_not_moveto_sente_hisha() {
    let position_and_kinds = vec![
        (4,0),
        (0,3),
        (8,3),
        (4,8)
    ];

    let mvs = vec![
        ((4,0),(3,0)),
        ((0,3),(0,2)),
        ((8,3),(8,2)),
        ((4,8),(3,8))
    ];

    for &kind in [KomaKind::SHisha,KomaKind::SHishaN].iter() {
        for (&m, &(x, y)) in mvs.iter().zip(position_and_kinds.iter()) {
            let mut banmen = Banmen([[Blank; 9]; 9]);

            banmen.0[3][4] = GOu;
            banmen.0[y][x] = kind;

            let state = State::new(banmen);

            assert_eq!(false,Rule::is_oute_move(&state, Teban::Sente,
                                               LegalMove::To(LegalMoveTo::new((m.0).0 * 9 + (m.0).1,
                                                                              (m.1).0 * 9 + (m.1).1,
                                                                              false, None))), "{:?} {:?}", kind, m);
        }
    }
}
#[test]
fn is_oute_not_moveto_sente_hisha_occupied_self() {
    let position_and_kinds = vec![
        (3,0),
        (0,2),
        (8,2),
        (3,8)
    ];

    let mvs = vec![
        ((3,0),(4,0)),
        ((0,2),(0,3)),
        ((8,2),(8,3)),
        ((3,8),(4,8))
    ];

    for &kind in [KomaKind::SHisha,KomaKind::SHishaN].iter() {
        for (&m, &(x, y)) in mvs.iter().zip(position_and_kinds.iter()) {
            let mut banmen = Banmen([[Blank; 9]; 9]);

            banmen.0[3][4] = GOu;
            banmen.0[y][x] = kind;
            banmen.0[3][3] = KomaKind::SFu;
            banmen.0[3][5] = KomaKind::SFu;
            banmen.0[5][4] = KomaKind::SFu;
            banmen.0[2][4] = KomaKind::SFu;

            let state = State::new(banmen);

            assert_eq!(false,Rule::is_oute_move(&state, Teban::Sente,
                                               LegalMove::To(LegalMoveTo::new((m.0).0 * 9 + (m.0).1,
                                                                              (m.1).0 * 9 + (m.1).1,
                                                                              false, None))), "{:?} {:?}", kind, m);
        }
    }
}
#[test]
fn is_oute_not_moveto_sente_hisha_occupied_opponent() {
    let position_and_kinds = vec![
        (3,0),
        (0,2),
        (8,2),
        (3,8)
    ];

    let mvs = vec![
        ((3,0),(4,0)),
        ((0,2),(0,3)),
        ((8,2),(8,3)),
        ((3,8),(4,8))
    ];

    for &kind in [KomaKind::SHisha,KomaKind::SHishaN].iter() {
        for (&m, &(x, y)) in mvs.iter().zip(position_and_kinds.iter()) {
            let mut banmen = Banmen([[Blank; 9]; 9]);

            banmen.0[3][4] = GOu;
            banmen.0[y][x] = kind;
            banmen.0[3][3] = KomaKind::GFu;
            banmen.0[3][5] = KomaKind::GFu;
            banmen.0[5][4] = KomaKind::GFu;
            banmen.0[2][4] = KomaKind::GFu;

            let state = State::new(banmen);

            assert_eq!(false,Rule::is_oute_move(&state, Teban::Sente,
                                                LegalMove::To(LegalMoveTo::new((m.0).0 * 9 + (m.0).1,
                                                                               (m.1).0 * 9 + (m.1).1,
                                                                               false, None))), "{:?} {:?}", kind, m);
        }
    }
}

#[test]
fn is_oute_moveto_sente_hisha_open_path() {
    let position_and_kinds = vec![
        ((4,0),(4,1)),
        ((0,3),(1,3)),
        ((8,3),(7,3)),
        ((4,8),(4,7))
    ];

    let mvs = vec![
        ((4,1),(3,1)),
        ((1,3),(1,2)),
        ((7,3),(7,4)),
        ((4,7),(5,7))
    ];

    for &kind in [KomaKind::SHisha,KomaKind::SHishaN].iter() {
        for (&m, &((hx, hy),(x,y))) in mvs.iter().zip(position_and_kinds.iter()) {
            let mut banmen = Banmen([[Blank; 9]; 9]);

            banmen.0[3][4] = GOu;
            banmen.0[hy][hx] = kind;
            banmen.0[y][x] = SKin;

            let state = State::new(banmen);

            assert_eq!(true,Rule::is_oute_move(&state, Teban::Sente,
                                               LegalMove::To(LegalMoveTo::new((m.0).0 * 9 + (m.0).1,
                                                                              (m.1).0 * 9 + (m.1).1,
                                                                              false, None))), "{:?} {:?}", SKin, m);
        }
    }
}

#[test]
fn is_oute_moveto_sente_hisha_open_path_occupied() {
    let position_and_kinds = vec![
        ((4,0),(4,1)),
        ((0,3),(1,3)),
        ((8,3),(7,3)),
        ((4,8),(4,7))
    ];

    let mvs = vec![
        ((4,0),(4,1)),
        ((0,3),(1,3)),
        ((8,3),(7,3)),
        ((4,8),(4,7))
    ];

    for &kind in [KomaKind::SHisha,KomaKind::SHishaN].iter() {
        for (&m, &((x, y),(ox,oy))) in mvs.iter().zip(position_and_kinds.iter()) {
            let mut banmen = Banmen([[Blank; 9]; 9]);

            banmen.0[3][4] = GOu;
            banmen.0[y][x] = kind;
            banmen.0[oy][ox] = KomaKind::GFu;

            let state = State::new(banmen);

            assert_eq!(true,Rule::is_oute_move(&state, Teban::Sente,
                                               LegalMove::To(LegalMoveTo::new((m.0).0 * 9 + (m.0).1,
                                                                              (m.1).0 * 9 + (m.1).1,
                                                                              false, None))), "{:?} {:?}", kind, m);
        }
    }
}
#[test]
fn is_oute_moveto_sente_kaku() {
    let position_and_kinds = vec![
        (0,1),
        (8,1),
        (1,8),
        (7,8)
    ];

    let mvs = vec![
        ((0,1),(1,0)),
        ((8,1),(7,0)),
        ((1,8),(0,7)),
        ((7,8),(8,7))
    ];

    for &kind in [KomaKind::SKaku,KomaKind::SKakuN].iter() {
        for (&m, &(x, y)) in mvs.iter().zip(position_and_kinds.iter()) {
            let mut banmen = Banmen([[Blank; 9]; 9]);

            banmen.0[3][4] = GOu;
            banmen.0[y][x] = kind;

            let state = State::new(banmen);

            assert_eq!(true,Rule::is_oute_move(&state, Teban::Sente,
                                               LegalMove::To(LegalMoveTo::new((m.0).0 * 9 + (m.0).1,
                                                                              (m.1).0 * 9 + (m.1).1,
                                                                              false, None))), "{:?} {:?}", kind, m);
        }
    }
}
#[test]
fn is_oute_not_moveto_sente_kaku() {
    let position_and_kinds = vec![
        (1,0),
        (7,0),
        (8,7),
        (0,7)
    ];

    let mvs = vec![
        ((1,0),(0,1)),
        ((7,0),(8,1)),
        ((8,7),(7,8)),
        ((0,7),(1,8))
    ];

    for &kind in [KomaKind::SKaku,KomaKind::SKakuN].iter() {
        for (&m, &(x, y)) in mvs.iter().zip(position_and_kinds.iter()) {
            let mut banmen = Banmen([[Blank; 9]; 9]);

            banmen.0[3][4] = GOu;
            banmen.0[y][x] = kind;

            let state = State::new(banmen);

            assert_eq!(false,Rule::is_oute_move(&state, Teban::Sente,
                                                LegalMove::To(LegalMoveTo::new((m.0).0 * 9 + (m.0).1,
                                                                               (m.1).0 * 9 + (m.1).1,
                                                                               false, None))), "{:?} {:?}", kind, m);
        }
    }
}
#[test]
fn is_oute_not_moveto_sente_kaku_occupied_self() {
    let position_and_kinds = vec![
        (0,1),
        (8,1),
        (1,8),
        (7,8)
    ];

    let mvs = vec![
        ((0,1),(1,0)),
        ((8,1),(7,0)),
        ((1,8),(0,7)),
        ((7,8),(8,7))
    ];

    for &kind in [KomaKind::SKaku,KomaKind::SKakuN].iter() {
        for (&m, &(x, y)) in mvs.iter().zip(position_and_kinds.iter()) {
            let mut banmen = Banmen([[Blank; 9]; 9]);

            banmen.0[3][4] = GOu;
            banmen.0[y][x] = kind;
            banmen.0[1][2] = KomaKind::SFu;
            banmen.0[1][6] = KomaKind::SFu;
            banmen.0[6][1] = KomaKind::SFu;
            banmen.0[6][7] = KomaKind::SFu;

            let state = State::new(banmen);

            assert_eq!(false,Rule::is_oute_move(&state, Teban::Sente,
                                                LegalMove::To(LegalMoveTo::new((m.0).0 * 9 + (m.0).1,
                                                                               (m.1).0 * 9 + (m.1).1,
                                                                               false, None))), "{:?} {:?}", kind, m);
        }
    }
}
#[test]
fn is_oute_not_moveto_sente_kaku_occupied_opponent() {
    let position_and_kinds = vec![
        (0,1),
        (8,1),
        (1,8),
        (7,8)
    ];

    let mvs = vec![
        ((0,1),(1,0)),
        ((8,1),(7,0)),
        ((1,8),(0,7)),
        ((7,8),(8,7))
    ];

    for &kind in [KomaKind::SKaku,KomaKind::SKakuN].iter() {
        for (&m, &(x, y)) in mvs.iter().zip(position_and_kinds.iter()) {
            let mut banmen = Banmen([[Blank; 9]; 9]);

            banmen.0[3][4] = GOu;
            banmen.0[y][x] = kind;
            banmen.0[1][2] = KomaKind::GFu;
            banmen.0[1][6] = KomaKind::GFu;
            banmen.0[6][1] = KomaKind::GFu;
            banmen.0[6][7] = KomaKind::GFu;

            let state = State::new(banmen);

            assert_eq!(false,Rule::is_oute_move(&state, Teban::Sente,
                                                LegalMove::To(LegalMoveTo::new((m.0).0 * 9 + (m.0).1,
                                                                               (m.1).0 * 9 + (m.1).1,
                                                                               false, None))), "{:?} {:?}", kind, m);
        }
    }
}
#[test]
fn is_oute_moveto_sente_kaku_open_path() {
    let position_and_kinds = vec![
        ((1,0),(2,1)),
        ((7,0),(6,1)),
        ((0,7),(1,6)),
        ((8,7),(7,6))
    ];

    let mvs = vec![
        ((2,1),(3,1)),
        ((6,1),(7,1)),
        ((1,6),(2,6)),
        ((7,6),(6,6))
    ];

    for &kind in [KomaKind::SKaku,KomaKind::SKakuN].iter() {
        for (&m, &((kx, ky),(x,y))) in mvs.iter().zip(position_and_kinds.iter()) {
            let mut banmen = Banmen([[Blank; 9]; 9]);

            banmen.0[3][4] = GOu;
            banmen.0[ky][kx] = kind;
            banmen.0[y][x] = SKin;

            let state = State::new(banmen);

            assert_eq!(true,Rule::is_oute_move(&state, Teban::Sente,
                                               LegalMove::To(LegalMoveTo::new((m.0).0 * 9 + (m.0).1,
                                                                              (m.1).0 * 9 + (m.1).1,
                                                                              false, None))), "{:?} {:?}", SKin, m);
        }
    }
}
#[test]
fn is_oute_moveto_sente_kaku_open_path_occupied() {
    let position_and_kinds = vec![
        ((1,0),(2,1)),
        ((7,0),(6,1)),
        ((0,7),(1,6)),
        ((8,7),(7,6))
    ];

    let mvs = vec![
        ((1,0),(2,1)),
        ((7,0),(6,1)),
        ((0,7),(1,6)),
        ((8,7),(7,6))
    ];

    for &kind in [KomaKind::SKaku,KomaKind::SKakuN].iter() {
        for (&m, &((x, y),(ox,oy))) in mvs.iter().zip(position_and_kinds.iter()) {
            let mut banmen = Banmen([[Blank; 9]; 9]);

            banmen.0[3][4] = GOu;
            banmen.0[y][x] = kind;
            banmen.0[oy][ox] = KomaKind::GFu;

            let state = State::new(banmen);

            assert_eq!(true,Rule::is_oute_move(&state, Teban::Sente,
                                               LegalMove::To(LegalMoveTo::new((m.0).0 * 9 + (m.0).1,
                                                                              (m.1).0 * 9 + (m.1).1,
                                                                              false, None))), "{:?} {:?}", kind, m);
        }
    }
}
#[test]
fn is_oute_moveto_sente_kyou() {
    let position_and_kinds = vec![
        (4,8)
    ];

    let mvs = vec![
        ((4,8),(4,7))
    ];

    for (&m, &(x, y)) in mvs.iter().zip(position_and_kinds.iter()) {
        let mut banmen = Banmen([[Blank; 9]; 9]);

        banmen.0[3][4] = GOu;
        banmen.0[7][4] = KomaKind::GFu;
        banmen.0[y][x] = KomaKind::SKyou;

        let state = State::new(banmen);

        assert_eq!(true,Rule::is_oute_move(&state, Teban::Sente,
                                           LegalMove::To(LegalMoveTo::new((m.0).0 * 9 + (m.0).1,
                                                                          (m.1).0 * 9 + (m.1).1,
                                                                          false, None))), "{:?} {:?}", SKyou, m);
    }
}
#[test]
fn is_oute_not_moveto_sente_kyou() {
    let position_and_kinds = vec![
        (3,8),
        (5,8)
    ];

    let mvs = vec![
        ((3,8),(3,7)),
        ((5,8),(5,7))
    ];

    for (&m, &(x, y)) in mvs.iter().zip(position_and_kinds.iter()) {
        let mut banmen = Banmen([[Blank; 9]; 9]);

        banmen.0[3][4] = GOu;
        banmen.0[y][x] = SKyou;

        let state = State::new(banmen);

        assert_eq!(false,Rule::is_oute_move(&state, Teban::Sente,
                                            LegalMove::To(LegalMoveTo::new((m.0).0 * 9 + (m.0).1,
                                                                           (m.1).0 * 9 + (m.1).1,
                                                                           false, None))), "{:?} {:?}", SKyou, m);
    }
}
#[test]
fn is_oute_not_moveto_sente_kyou_occupied_self() {
    let position_and_kinds = vec![
        (4,8)
    ];

    let mvs = vec![
        ((4,8),(4,7))
    ];

    for (&m, &(x, y)) in mvs.iter().zip(position_and_kinds.iter()) {
        let mut banmen = Banmen([[Blank; 9]; 9]);

        banmen.0[3][4] = GOu;
        banmen.0[y][x] = SKyou;
        banmen.0[6][4] = KomaKind::SFu;

        let state = State::new(banmen);

        assert_eq!(false,Rule::is_oute_move(&state, Teban::Sente,
                                            LegalMove::To(LegalMoveTo::new((m.0).0 * 9 + (m.0).1,
                                                                           (m.1).0 * 9 + (m.1).1,
                                                                           false, None))), "{:?} {:?}", SKyou, m);
    }
}
#[test]
fn is_oute_not_moveto_sente_kyou_occupied_opponent() {
    let position_and_kinds = vec![
        (4,8)
    ];

    let mvs = vec![
        ((4,8),(4,7))
    ];

    for (&m, &(x, y)) in mvs.iter().zip(position_and_kinds.iter()) {
        let mut banmen = Banmen([[Blank; 9]; 9]);

        banmen.0[3][4] = GOu;
        banmen.0[y][x] = SKyou;
        banmen.0[6][4] = KomaKind::GFu;

        let state = State::new(banmen);

        assert_eq!(false,Rule::is_oute_move(&state, Teban::Sente,
                                            LegalMove::To(LegalMoveTo::new((m.0).0 * 9 + (m.0).1,
                                                                           (m.1).0 * 9 + (m.1).1,
                                                                           false, None))), "{:?} {:?}", SKyou, m);
    }
}
#[test]
fn is_oute_moveto_sente_kyou_open_path() {
    let position_and_kinds = vec![
        ((4,8),(4,7))
    ];

    let mvs = vec![
        ((4,7),(5,7))
    ];

    for (&m, &((kx, ky),(x,y))) in mvs.iter().zip(position_and_kinds.iter()) {
        let mut banmen = Banmen([[Blank; 9]; 9]);

        banmen.0[3][4] = GOu;
        banmen.0[ky][kx] = SKyou;
        banmen.0[y][x] = SKin;

        let state = State::new(banmen);

        assert_eq!(true,Rule::is_oute_move(&state, Teban::Sente,
                                           LegalMove::To(LegalMoveTo::new((m.0).0 * 9 + (m.0).1,
                                                                          (m.1).0 * 9 + (m.1).1,
                                                                          false, None))), "{:?} {:?}", SKin, m);
    }
}
#[test]
fn is_oute_moveto_sente_kyou_open_path_occupied() {
    let position_and_kinds = vec![
        ((4,8),(4,7))
    ];

    let mvs = vec![
        ((4,8),(4,7))
    ];

    for (&m, &((x, y),(ox,oy))) in mvs.iter().zip(position_and_kinds.iter()) {
        let mut banmen = Banmen([[Blank; 9]; 9]);

        banmen.0[3][4] = GOu;
        banmen.0[y][x] = SKyou;
        banmen.0[oy][ox] = KomaKind::GFu;

        let state = State::new(banmen);

        assert_eq!(true,Rule::is_oute_move(&state, Teban::Sente,
                                           LegalMove::To(LegalMoveTo::new((m.0).0 * 9 + (m.0).1,
                                                                          (m.1).0 * 9 + (m.1).1,
                                                                          false, None))), "{:?} {:?}", SKyou, m);
    }
}
#[test]
fn is_oute_moveput_sente() {
    let mvs = vec![
        (4,4,MochigomaKind::Fu),
        (4,8,MochigomaKind::Kyou),
        (3,5,MochigomaKind::Kei),
        (5,5,MochigomaKind::Kei),
        (3,4,MochigomaKind::Gin),
        (4,4,MochigomaKind::Gin),
        (5,4,MochigomaKind::Gin),
        (3,2,MochigomaKind::Gin),
        (5,2,MochigomaKind::Gin),
        (3,4,MochigomaKind::Kin),
        (4,4,MochigomaKind::Kin),
        (5,4,MochigomaKind::Kin),
        (3,3,MochigomaKind::Kin),
        (5,3,MochigomaKind::Kin),
        (4,2,MochigomaKind::Kin),
        (0,7,MochigomaKind::Kaku),
        (8,7,MochigomaKind::Kaku),
        (1,0,MochigomaKind::Kaku),
        (7,0,MochigomaKind::Kaku),
        (4,8,MochigomaKind::Hisha),
        (0,3,MochigomaKind::Hisha),
        (8,3,MochigomaKind::Hisha),
        (4,0,MochigomaKind::Hisha)
    ];

    for &(x,y,kind) in mvs.iter() {
        let mut banmen = Banmen([[Blank; 9]; 9]);

        banmen.0[3][4] = GOu;

        let state = State::new(banmen);

        assert_eq!(true,Rule::is_oute_move(&state, Teban::Sente,
                                           LegalMove::Put(LegalMovePut::new(kind,
                                                                          x * 9 + y))), "{:?} {},{}", kind, x, y);
    }
}
#[test]
fn is_oute_not_moveput_sente() {
    let mvs = vec![
        (4,5,MochigomaKind::Fu),
        (5,3,MochigomaKind::Fu),
        (5,3,MochigomaKind::Kyou),
        (5,3,MochigomaKind::Kei),
        (4,5,MochigomaKind::Gin),
        (2,5,MochigomaKind::Gin),
        (6,5,MochigomaKind::Gin),
        (3,3,MochigomaKind::Gin),
        (5,3,MochigomaKind::Gin),
        (2,1,MochigomaKind::Gin),
        (6,1,MochigomaKind::Gin),
        (4,2,MochigomaKind::Gin),
        (4,5,MochigomaKind::Kin),
        (2,5,MochigomaKind::Kin),
        (6,5,MochigomaKind::Kin),
        (2,3,MochigomaKind::Kin),
        (6,3,MochigomaKind::Kin),
        (3,2,MochigomaKind::Kin),
        (5,2,MochigomaKind::Kin),
        (4,1,MochigomaKind::Kin),
        (3,1,MochigomaKind::Kaku),
        (5,1,MochigomaKind::Kaku),
        (3,5,MochigomaKind::Kaku),
        (5,5,MochigomaKind::Kaku),
        (5,0,MochigomaKind::Hisha),
        (0,4,MochigomaKind::Hisha),
        (8,4,MochigomaKind::Hisha),
        (3,0,MochigomaKind::Hisha)
    ];

    for &(x,y,kind) in mvs.iter() {
        let mut banmen = Banmen([[Blank; 9]; 9]);

        banmen.0[3][4] = GOu;

        let state = State::new(banmen);

        assert_eq!(false,Rule::is_oute_move(&state, Teban::Sente,
                                           LegalMove::Put(LegalMovePut::new(kind,
                                                                            x * 9 + y))), "{:?} {},{}", kind, x, y);
    }
}
#[test]
fn is_oute_not_moveput_sente_kaku_self_occupied() {
    let mvs = vec![
        (0,7,MochigomaKind::Kaku),
        (8,7,MochigomaKind::Kaku),
        (1,0,MochigomaKind::Kaku),
        (7,0,MochigomaKind::Kaku),
    ];

    let occ = vec![
        (1,6),
        (7,6),
        (2,1),
        (6,1)
    ];

    for (&(x,y,kind),&(ox,oy)) in mvs.iter().zip(occ.iter()) {
        let mut banmen = Banmen([[Blank; 9]; 9]);

        banmen.0[3][4] = GOu;
        banmen.0[oy][ox] = KomaKind::SFu;

        let state = State::new(banmen);

        assert_eq!(false,Rule::is_oute_move(&state, Teban::Sente,
                                           LegalMove::Put(LegalMovePut::new(kind,
                                                                            x * 9 + y))), "{:?} {},{}", kind, x, y);
    }
}
#[test]
fn is_oute_not_moveput_sente_kaku_opponent_occupied() {
    let mvs = vec![
        (0,7,MochigomaKind::Kaku),
        (8,7,MochigomaKind::Kaku),
        (1,0,MochigomaKind::Kaku),
        (7,0,MochigomaKind::Kaku),
    ];

    let occ = vec![
        (1,6),
        (7,6),
        (2,1),
        (6,1)
    ];

    for (&(x,y,kind),&(ox,oy)) in mvs.iter().zip(occ.iter()) {
        let mut banmen = Banmen([[Blank; 9]; 9]);

        banmen.0[3][4] = GOu;
        banmen.0[oy][ox] = KomaKind::GFu;

        let state = State::new(banmen);

        assert_eq!(false,Rule::is_oute_move(&state, Teban::Sente,
                                           LegalMove::Put(LegalMovePut::new(kind,
                                                                            x * 9 + y))), "{:?} {},{}", kind, x, y);
    }
}
#[test]
fn is_oute_not_moveput_sente_hisha_self_occupied() {
    let mvs = vec![
        (4,8,MochigomaKind::Hisha),
        (0,3,MochigomaKind::Hisha),
        (8,3,MochigomaKind::Hisha),
        (4,0,MochigomaKind::Hisha)
    ];

    let occ = vec![
        (4,7),
        (1,3),
        (7,3),
        (4,1)
    ];

    for (&(x,y,kind),&(ox,oy)) in mvs.iter().zip(occ.iter()) {
        let mut banmen = Banmen([[Blank; 9]; 9]);

        banmen.0[3][4] = GOu;
        banmen.0[oy][ox] = KomaKind::SFu;

        let state = State::new(banmen);

        assert_eq!(false,Rule::is_oute_move(&state, Teban::Sente,
                                           LegalMove::Put(LegalMovePut::new(kind,
                                                                            x * 9 + y))), "{:?} {},{}", kind, x, y);
    }
}
#[test]
fn is_oute_not_moveput_sente_hisha_opponent_occupied() {
    let mvs = vec![
        (4,8,MochigomaKind::Hisha),
        (0,3,MochigomaKind::Hisha),
        (8,3,MochigomaKind::Hisha),
        (4,0,MochigomaKind::Hisha)
    ];

    let occ = vec![
        (4,7),
        (1,3),
        (7,3),
        (4,1)
    ];

    for (&(x,y,kind),&(ox,oy)) in mvs.iter().zip(occ.iter()) {
        let mut banmen = Banmen([[Blank; 9]; 9]);

        banmen.0[3][4] = GOu;
        banmen.0[oy][ox] = KomaKind::GFu;

        let state = State::new(banmen);

        assert_eq!(false,Rule::is_oute_move(&state, Teban::Sente,
                                           LegalMove::Put(LegalMovePut::new(kind,
                                                                            x * 9 + y))), "{:?} {},{}", kind, x, y);
    }
}
#[test]
fn is_oute_not_moveput_sente_kyou_self_occupied() {
    let mvs = vec![
        (4,8,MochigomaKind::Kyou)
    ];

    let occ = vec![
        (4,7)
    ];

    for (&(x,y,kind),&(ox,oy)) in mvs.iter().zip(occ.iter()) {
        let mut banmen = Banmen([[Blank; 9]; 9]);

        banmen.0[3][4] = GOu;
        banmen.0[oy][ox] = KomaKind::SFu;

        let state = State::new(banmen);

        assert_eq!(false,Rule::is_oute_move(&state, Teban::Sente,
                                           LegalMove::Put(LegalMovePut::new(kind,
                                                                            x * 9 + y))), "{:?} {},{}", kind, x, y);
    }
}
#[test]
fn is_oute_not_moveput_sente_kyou_opponent_occupied() {
    let mvs = vec![
        (4,8,MochigomaKind::Kyou)
    ];

    let occ = vec![
        (4,7)
    ];

    for (&(x,y,kind),&(ox,oy)) in mvs.iter().zip(occ.iter()) {
        let mut banmen = Banmen([[Blank; 9]; 9]);

        banmen.0[3][4] = GOu;
        banmen.0[oy][ox] = KomaKind::GFu;

        let state = State::new(banmen);

        assert_eq!(false,Rule::is_oute_move(&state, Teban::Sente,
                                           LegalMove::Put(LegalMovePut::new(kind,
                                                                            x * 9 + y))), "{:?} {},{}", kind, x, y);
    }
}
#[test]
fn is_oute_moveto_gote() {
    let position_and_kinds = vec![
        (4,5,KomaKind::SFu),
        (2,7,KomaKind::SKei),
        (6,7,KomaKind::SKei),
        (4,5,KomaKind::SGin),
        (3,5,KomaKind::SGin),
        (5,5,KomaKind::SGin),
        (2,1,KomaKind::SGin),
        (6,1,KomaKind::SGin),
        (4,5,KomaKind::SKin),
        (3,5,KomaKind::SKin),
        (5,5,KomaKind::SKin),
        (2,3,KomaKind::SKin),
        (6,3,KomaKind::SKin),
        (4,1,KomaKind::SKin),
        (4,5,KomaKind::SOu),
        (3,5,KomaKind::SOu),
        (5,5,KomaKind::SOu),
        (2,3,KomaKind::SOu),
        (6,3,KomaKind::SOu),
        (2,1,KomaKind::SOu),
        (6,1,KomaKind::SOu),
        (4,1,KomaKind::SOu),
        (4,5,KomaKind::SFuN),
        (3,5,KomaKind::SFuN),
        (5,5,KomaKind::SFuN),
        (2,3,KomaKind::SFuN),
        (6,3,KomaKind::SFuN),
        (4,1,KomaKind::SFuN),
        (4,5,KomaKind::SKyouN),
        (3,5,KomaKind::SKyouN),
        (5,5,KomaKind::SKyouN),
        (2,3,KomaKind::SKyouN),
        (6,3,KomaKind::SKyouN),
        (4,1,KomaKind::SKyouN),
        (4,5,KomaKind::SKeiN),
        (3,5,KomaKind::SKeiN),
        (5,5,KomaKind::SKeiN),
        (2,3,KomaKind::SKeiN),
        (6,3,KomaKind::SKeiN),
        (4,1,KomaKind::SKeiN),
        (4,5,KomaKind::SGinN),
        (3,5,KomaKind::SGinN),
        (5,5,KomaKind::SGinN),
        (2,3,KomaKind::SGinN),
        (6,3,KomaKind::SGinN),
        (4,1,KomaKind::SGinN),
        (4,1,KomaKind::SKakuN),
        (2,3,KomaKind::SKakuN),
        (6,3,KomaKind::SKakuN),
        (4,5,KomaKind::SKakuN),
        (2,1,KomaKind::SHishaN),
        (6,1,KomaKind::SHishaN),
        (2,5,KomaKind::SHishaN),
        (6,5,KomaKind::SHishaN)
    ];

    let mvs = vec![
        ((4,5),(4,4)),
        ((2,7),(3,5)),
        ((6,7),(5,5)),
        ((4,5),(4,4)),
        ((3,5),(3,4)),
        ((5,5),(5,4)),
        ((2,1),(3,2)),
        ((6,1),(5,2)),
        ((4,5),(4,4)),
        ((3,5),(3,4)),
        ((5,5),(5,4)),
        ((2,3),(3,3)),
        ((6,3),(5,3)),
        ((4,1),(4,2)),
        ((4,5),(4,4)),
        ((3,5),(3,4)),
        ((5,5),(5,4)),
        ((2,3),(3,3)),
        ((6,3),(5,3)),
        ((2,1),(3,2)),
        ((6,1),(5,2)),
        ((4,1),(4,2)),
        ((4,5),(4,4)),
        ((3,5),(3,4)),
        ((5,5),(5,4)),
        ((2,3),(3,3)),
        ((6,3),(5,3)),
        ((4,1),(4,2)),
        ((4,5),(4,4)),
        ((3,5),(3,4)),
        ((5,5),(5,4)),
        ((2,3),(3,3)),
        ((6,3),(5,3)),
        ((4,1),(4,2)),
        ((4,5),(4,4)),
        ((3,5),(3,4)),
        ((5,5),(5,4)),
        ((2,3),(3,3)),
        ((6,3),(5,3)),
        ((4,1),(4,2)),
        ((4,5),(4,4)),
        ((3,5),(3,4)),
        ((5,5),(5,4)),
        ((2,3),(3,3)),
        ((6,3),(5,3)),
        ((4,1),(4,2)),
        ((4,1),(4,2)),
        ((2,3),(3,3)),
        ((6,3),(5,3)),
        ((4,5),(4,4)),
        ((2,1),(3,2)),
        ((6,1),(5,2)),
        ((2,5),(3,4)),
        ((6,5),(5,4))
    ];

    for (&m,&(x,y,kind)) in mvs.iter().zip(position_and_kinds.iter()) {
        let (x,y) = (8-x,8-y);

        let m = match m {
            ((sx,sy),(dx,dy)) => {
                ((8-sx,8-sy),(8-dx,8-dy))
            }
        };

        let mut banmen = Banmen([[Blank; 9]; 9]);

        banmen.0[8-3][8-4] = SOu;
        banmen.0[y][x] = kind;

        let state = State::new(banmen);

        assert_eq!(true,Rule::is_oute_move(&state,Teban::Gote,
                                           LegalMove::To(LegalMoveTo::new((m.0).0 * 9 + (m.0).1,
                                                                          (m.1).0 * 9 + (m.1).1,
                                                                          false,None))),"{:?} {:?}",kind,m);
    }
}
#[test]
fn is_oute_not_moveto_gote() {
    let position_and_kinds = vec![
        (4,6,KomaKind::GFu),
        (4,4,KomaKind::GKei),
        (2,4,KomaKind::GGin),
        (6,4,KomaKind::GGin),
        (3,1,KomaKind::GKin),
        (5,1,KomaKind::GKin),
        (4,6,KomaKind::GOu),
        (3,1,KomaKind::GFuN),
        (5,1,KomaKind::GFuN),
        (3,1,KomaKind::GKyouN),
        (5,1,KomaKind::GKyouN),
        (3,1,KomaKind::GKeiN),
        (5,1,KomaKind::GKeiN),
        (3,1,KomaKind::GGinN),
        (5,1,KomaKind::GGinN),
        (4,6,KomaKind::GKakuN),
        (2,0,KomaKind::GHishaN),
        (6,0,KomaKind::GHishaN)
    ];

    let mvs = vec![
        ((4,6),(4,5)),
        ((4,4),(3,2)),
        ((4,4),(5,2)),
        ((2,4),(3,3)),
        ((6,4),(5,3)),
        ((3,1),(3,2)),
        ((5,1),(5,2)),
        ((4,6),(4,5)),
        ((3,1),(3,2)),
        ((5,1),(5,2)),
        ((3,1),(3,2)),
        ((5,1),(5,2)),
        ((3,1),(3,2)),
        ((5,1),(5,2)),
        ((3,1),(3,2)),
        ((5,1),(5,2)),
        ((2,0),(2,1)),
        ((6,0),(6,1))
    ];

    for (&m,&(x,y,kind)) in mvs.iter().zip(position_and_kinds.iter()) {
        let (x,y) = (8-x,8-y);

        let m = match m {
            ((sx,sy),(dx,dy)) => {
                ((8-sx,8-sy),(8-dx,8-dy))
            }
        };

        let mut banmen = Banmen([[Blank; 9]; 9]);

        banmen.0[8-3][8-4] = SOu;
        banmen.0[y][x] = kind;

        let state = State::new(banmen);

        assert_eq!(false,Rule::is_oute_move(&state,Teban::Gote,
                                            LegalMove::To(LegalMoveTo::new((m.0).0 * 9 + (m.0).1,
                                                                           (m.1).0 * 9 + (m.1).1,
                                                                           false,None))),"{:?} {:?}",kind,m);
    }
}
#[test]
fn is_oute_moveto_gote_hisha() {
    let position_and_kinds = vec![
        (3,0),
        (0,2),
        (8,2),
        (3,8)
    ];

    let mvs = vec![
        ((3,0),(4,0)),
        ((0,2),(0,3)),
        ((8,2),(8,3)),
        ((3,8),(4,8))
    ];

    for &kind in [KomaKind::GHisha,KomaKind::GHishaN].iter() {
        for (&m, &(x, y)) in mvs.iter().zip(position_and_kinds.iter()) {
            let (x,y) = (8-x,8-y);

            let m = match m {
                ((sx,sy),(dx,dy)) => {
                    ((8-sx,8-sy),(8-dx,8-dy))
                }
            };

            let mut banmen = Banmen([[Blank; 9]; 9]);

            banmen.0[8-3][8-4] = SOu;
            banmen.0[y][x] = kind;

            let state = State::new(banmen);

            assert_eq!(true,Rule::is_oute_move(&state, Teban::Gote,
                                               LegalMove::To(LegalMoveTo::new((m.0).0 * 9 + (m.0).1,
                                                                              (m.1).0 * 9 + (m.1).1,
                                                                              false, None))), "{:?} {:?}", kind, m);
        }
    }
}
#[test]
fn is_oute_not_moveto_gote_hisha() {
    let position_and_kinds = vec![
        (4,0),
        (0,3),
        (8,3),
        (4,8)
    ];

    let mvs = vec![
        ((4,0),(3,0)),
        ((0,3),(0,2)),
        ((8,3),(8,2)),
        ((4,8),(3,8))
    ];

    for &kind in [KomaKind::GHisha,KomaKind::GHishaN].iter() {
        for (&m, &(x, y)) in mvs.iter().zip(position_and_kinds.iter()) {
            let (x,y) = (8-x,8-y);

            let m = match m {
                ((sx,sy),(dx,dy)) => {
                    ((8-sx,8-sy),(8-dx,8-dy))
                }
            };

            let mut banmen = Banmen([[Blank; 9]; 9]);

            banmen.0[8-3][8-4] = SOu;
            banmen.0[y][x] = kind;

            let state = State::new(banmen);

            assert_eq!(false,Rule::is_oute_move(&state, Teban::Gote,
                                                LegalMove::To(LegalMoveTo::new((m.0).0 * 9 + (m.0).1,
                                                                               (m.1).0 * 9 + (m.1).1,
                                                                               false, None))), "{:?} {:?}", kind, m);
        }
    }
}
#[test]
fn is_oute_not_moveto_sente_gote_occupied_self() {
    let position_and_kinds = vec![
        (3,0),
        (0,2),
        (8,2),
        (3,8)
    ];

    let mvs = vec![
        ((3,0),(4,0)),
        ((0,2),(0,3)),
        ((8,2),(8,3)),
        ((3,8),(4,8))
    ];

    for &kind in [KomaKind::GHisha,KomaKind::GHishaN].iter() {
        for (&m, &(x, y)) in mvs.iter().zip(position_and_kinds.iter()) {
            let (x,y) = (8-x,8-y);

            let m = match m {
                ((sx,sy),(dx,dy)) => {
                    ((8-sx,8-sy),(8-dx,8-dy))
                }
            };

            let mut banmen = Banmen([[Blank; 9]; 9]);

            banmen.0[8-3][8-4] = SOu;
            banmen.0[y][x] = kind;
            banmen.0[8-3][8-3] = KomaKind::GFu;
            banmen.0[8-3][8-5] = KomaKind::GFu;
            banmen.0[8-5][8-4] = KomaKind::GFu;
            banmen.0[8-2][8-4] = KomaKind::GFu;

            let state = State::new(banmen);

            assert_eq!(false,Rule::is_oute_move(&state, Teban::Gote,
                                                LegalMove::To(LegalMoveTo::new((m.0).0 * 9 + (m.0).1,
                                                                               (m.1).0 * 9 + (m.1).1,
                                                                               false, None))), "{:?} {:?}", kind, m);
        }
    }
}
#[test]
fn is_oute_not_moveto_gote_hisha_occupied_opponent() {
    let position_and_kinds = vec![
        (3,0),
        (0,2),
        (8,2),
        (3,8)
    ];

    let mvs = vec![
        ((3,0),(4,0)),
        ((0,2),(0,3)),
        ((8,2),(8,3)),
        ((3,8),(4,8))
    ];

    for &kind in [KomaKind::GHisha,KomaKind::GHishaN].iter() {
        for (&m, &(x, y)) in mvs.iter().zip(position_and_kinds.iter()) {
            let (x,y) = (8-x,8-y);

            let m = match m {
                ((sx,sy),(dx,dy)) => {
                    ((8-sx,8-sy),(8-dx,8-dy))
                }
            };

            let mut banmen = Banmen([[Blank; 9]; 9]);

            banmen.0[8-3][8-4] = SOu;
            banmen.0[y][x] = kind;
            banmen.0[8-3][8-3] = KomaKind::SFu;
            banmen.0[8-3][8-5] = KomaKind::SFu;
            banmen.0[8-5][8-4] = KomaKind::SFu;
            banmen.0[8-2][8-4] = KomaKind::SFu;

            let state = State::new(banmen);

            assert_eq!(false,Rule::is_oute_move(&state, Teban::Gote,
                                                LegalMove::To(LegalMoveTo::new((m.0).0 * 9 + (m.0).1,
                                                                               (m.1).0 * 9 + (m.1).1,
                                                                               false, None))), "{:?} {:?}", kind, m);
        }
    }
}

#[test]
fn is_oute_moveto_gote_hisha_open_path() {
    let position_and_kinds = vec![
        ((4,0),(4,1)),
        ((0,3),(1,3)),
        ((8,3),(7,3)),
        ((4,8),(4,7))
    ];

    let mvs = vec![
        ((4,1),(3,1)),
        ((1,3),(1,2)),
        ((7,3),(7,4)),
        ((4,7),(5,7))
    ];

    for &kind in [KomaKind::GHisha,KomaKind::GHishaN].iter() {
        for (&m, &((hx, hy),(x,y))) in mvs.iter().zip(position_and_kinds.iter()) {
            let (x,y) = (8-x,8-y);
            let (hx,hy) = (8-hx,8-hy);

            let m = match m {
                ((sx,sy),(dx,dy)) => {
                    ((8-sx,8-sy),(8-dx,8-dy))
                }
            };

            let mut banmen = Banmen([[Blank; 9]; 9]);

            banmen.0[8-3][8-4] = SOu;
            banmen.0[hy][hx] = kind;
            banmen.0[y][x] = GKin;

            let state = State::new(banmen);

            assert_eq!(true,Rule::is_oute_move(&state, Teban::Gote,
                                               LegalMove::To(LegalMoveTo::new((m.0).0 * 9 + (m.0).1,
                                                                              (m.1).0 * 9 + (m.1).1,
                                                                              false, None))), "{:?} {:?}", GKin, m);
        }
    }
}

#[test]
fn is_oute_moveto_gote_hisha_open_path_occupied() {
    let position_and_kinds = vec![
        ((4,0),(4,1)),
        ((0,3),(1,3)),
        ((8,3),(7,3)),
        ((4,8),(4,7))
    ];

    let mvs = vec![
        ((4,0),(4,1)),
        ((0,3),(1,3)),
        ((8,3),(7,3)),
        ((4,8),(4,7))
    ];

    for &kind in [KomaKind::GHisha,KomaKind::GHishaN].iter() {
        for (&m, &((x, y),(ox,oy))) in mvs.iter().zip(position_and_kinds.iter()) {
            let (x,y) = (8-x,8-y);
            let (ox,oy) = (8-ox,8-oy);

            let m = match m {
                ((sx,sy),(dx,dy)) => {
                    ((8-sx,8-sy),(8-dx,8-dy))
                }
            };

            let mut banmen = Banmen([[Blank; 9]; 9]);

            banmen.0[8-3][8-4] = SOu;
            banmen.0[y][x] = kind;
            banmen.0[oy][ox] = KomaKind::SFu;

            let state = State::new(banmen);

            assert_eq!(true,Rule::is_oute_move(&state, Teban::Gote,
                                               LegalMove::To(LegalMoveTo::new((m.0).0 * 9 + (m.0).1,
                                                                              (m.1).0 * 9 + (m.1).1,
                                                                              false, None))), "{:?} {:?}", kind, m);
        }
    }
}
#[test]
fn is_oute_moveto_gote_kaku() {
    let position_and_kinds = vec![
        (0,1),
        (8,1),
        (1,8),
        (7,8)
    ];

    let mvs = vec![
        ((0,1),(1,0)),
        ((8,1),(7,0)),
        ((1,8),(0,7)),
        ((7,8),(8,7))
    ];

    for &kind in [KomaKind::GKaku,KomaKind::GKakuN].iter() {
        for (&m, &(x, y)) in mvs.iter().zip(position_and_kinds.iter()) {
            let (x,y) = (8-x,8-y);

            let m = match m {
                ((sx,sy),(dx,dy)) => {
                    ((8-sx,8-sy),(8-dx,8-dy))
                }
            };

            let mut banmen = Banmen([[Blank; 9]; 9]);

            banmen.0[8-3][8-4] = SOu;
            banmen.0[y][x] = kind;

            let state = State::new(banmen);

            assert_eq!(true,Rule::is_oute_move(&state, Teban::Gote,
                                               LegalMove::To(LegalMoveTo::new((m.0).0 * 9 + (m.0).1,
                                                                              (m.1).0 * 9 + (m.1).1,
                                                                              false, None))), "{:?} {:?}", kind, m);
        }
    }
}
#[test]
fn is_oute_not_moveto_gote_kaku() {
    let position_and_kinds = vec![
        (1,0),
        (7,0),
        (8,7),
        (0,7)
    ];

    let mvs = vec![
        ((1,0),(0,1)),
        ((7,0),(8,1)),
        ((8,7),(7,8)),
        ((0,7),(1,8))
    ];

    for &kind in [KomaKind::GKaku,KomaKind::GKakuN].iter() {
        for (&m, &(x, y)) in mvs.iter().zip(position_and_kinds.iter()) {
            let (x,y) = (8-x,8-y);

            let m = match m {
                ((sx,sy),(dx,dy)) => {
                    ((8-sx,8-sy),(8-dx,8-dy))
                }
            };

            let mut banmen = Banmen([[Blank; 9]; 9]);

            banmen.0[8-3][8-4] = SOu;
            banmen.0[y][x] = kind;

            let state = State::new(banmen);

            assert_eq!(false,Rule::is_oute_move(&state, Teban::Gote,
                                                LegalMove::To(LegalMoveTo::new((m.0).0 * 9 + (m.0).1,
                                                                               (m.1).0 * 9 + (m.1).1,
                                                                               false, None))), "{:?} {:?}", kind, m);
        }
    }
}
#[test]
fn is_oute_not_moveto_gote_kaku_occupied_self() {
    let position_and_kinds = vec![
        (0,1),
        (8,1),
        (1,8),
        (7,8)
    ];

    let mvs = vec![
        ((0,1),(1,0)),
        ((8,1),(7,0)),
        ((1,8),(0,7)),
        ((7,8),(8,7))
    ];

    for &kind in [KomaKind::GKaku,KomaKind::GKakuN].iter() {
        for (&m, &(x, y)) in mvs.iter().zip(position_and_kinds.iter()) {
            let (x,y) = (8-x,8-y);

            let m = match m {
                ((sx,sy),(dx,dy)) => {
                    ((8-sx,8-sy),(8-dx,8-dy))
                }
            };

            let mut banmen = Banmen([[Blank; 9]; 9]);

            banmen.0[8-3][8-4] = SOu;
            banmen.0[y][x] = kind;
            banmen.0[8-1][8-2] = KomaKind::GFu;
            banmen.0[8-1][8-6] = KomaKind::GFu;
            banmen.0[8-6][8-1] = KomaKind::GFu;
            banmen.0[8-6][8-7] = KomaKind::GFu;

            let state = State::new(banmen);

            assert_eq!(false,Rule::is_oute_move(&state, Teban::Gote,
                                                LegalMove::To(LegalMoveTo::new((m.0).0 * 9 + (m.0).1,
                                                                               (m.1).0 * 9 + (m.1).1,
                                                                               false, None))), "{:?} {:?}", kind, m);
        }
    }
}
#[test]
fn is_oute_not_moveto_gote_kaku_occupied_opponent() {
    let position_and_kinds = vec![
        (0,1),
        (8,1),
        (1,8),
        (7,8)
    ];

    let mvs = vec![
        ((0,1),(1,0)),
        ((8,1),(7,0)),
        ((1,8),(0,7)),
        ((7,8),(8,7))
    ];

    for &kind in [KomaKind::GKaku,KomaKind::GKakuN].iter() {
        for (&m, &(x, y)) in mvs.iter().zip(position_and_kinds.iter()) {
            let (x,y) = (8-x,8-y);

            let m = match m {
                ((sx,sy),(dx,dy)) => {
                    ((8-sx,8-sy),(8-dx,8-dy))
                }
            };

            let mut banmen = Banmen([[Blank; 9]; 9]);

            banmen.0[8-3][8-4] = SOu;
            banmen.0[y][x] = kind;
            banmen.0[8-1][8-2] = KomaKind::SFu;
            banmen.0[8-1][8-6] = KomaKind::SFu;
            banmen.0[8-6][8-1] = KomaKind::SFu;
            banmen.0[8-6][8-7] = KomaKind::SFu;

            let state = State::new(banmen);

            assert_eq!(false,Rule::is_oute_move(&state, Teban::Gote,
                                                LegalMove::To(LegalMoveTo::new((m.0).0 * 9 + (m.0).1,
                                                                               (m.1).0 * 9 + (m.1).1,
                                                                               false, None))), "{:?} {:?}", kind, m);
        }
    }
}
#[test]
fn is_oute_moveto_gote_kaku_open_path() {
    let position_and_kinds = vec![
        ((1,0),(2,1)),
        ((7,0),(6,1)),
        ((0,7),(1,6)),
        ((8,7),(7,6))
    ];

    let mvs = vec![
        ((2,1),(3,1)),
        ((6,1),(7,1)),
        ((1,6),(2,6)),
        ((7,6),(6,6))
    ];

    for &kind in [KomaKind::GKaku,KomaKind::GKakuN].iter() {
        for (&m, &((kx, ky),(x,y))) in mvs.iter().zip(position_and_kinds.iter()) {
            let (kx,ky) = (8-kx,8-ky);
            let (x,y) = (8-x,8-y);

            let m = match m {
                ((sx,sy),(dx,dy)) => {
                    ((8-sx,8-sy),(8-dx,8-dy))
                }
            };

            let mut banmen = Banmen([[Blank; 9]; 9]);

            banmen.0[8-3][8-4] = SOu;
            banmen.0[ky][kx] = kind;
            banmen.0[y][x] = GKin;

            let state = State::new(banmen);

            assert_eq!(true,Rule::is_oute_move(&state, Teban::Gote,
                                               LegalMove::To(LegalMoveTo::new((m.0).0 * 9 + (m.0).1,
                                                                              (m.1).0 * 9 + (m.1).1,
                                                                              false, None))), "{:?} {:?}", GKin, m);
        }
    }
}
#[test]
fn is_oute_moveto_gote_kaku_open_path_occupied() {
    let position_and_kinds = vec![
        ((1,0),(2,1)),
        ((7,0),(6,1)),
        ((0,7),(1,6)),
        ((8,7),(7,6))
    ];

    let mvs = vec![
        ((1,0),(2,1)),
        ((7,0),(6,1)),
        ((0,7),(1,6)),
        ((8,7),(7,6))
    ];

    for &kind in [KomaKind::GKaku,KomaKind::GKakuN].iter() {
        for (&m, &((x, y),(ox,oy))) in mvs.iter().zip(position_and_kinds.iter()) {
            let (x,y) = (8-x,8-y);
            let (ox,oy) = (8-ox,8-oy);

            let m = match m {
                ((sx,sy),(dx,dy)) => {
                    ((8-sx,8-sy),(8-dx,8-dy))
                }
            };

            let mut banmen = Banmen([[Blank; 9]; 9]);

            banmen.0[8-3][8-4] = SOu;
            banmen.0[y][x] = kind;
            banmen.0[oy][ox] = KomaKind::SFu;

            let state = State::new(banmen);

            assert_eq!(true,Rule::is_oute_move(&state, Teban::Gote,
                                               LegalMove::To(LegalMoveTo::new((m.0).0 * 9 + (m.0).1,
                                                                              (m.1).0 * 9 + (m.1).1,
                                                                              false, None))), "{:?} {:?}", kind, m);
        }
    }
}
#[test]
fn is_oute_moveto_gote_kyou() {
    let position_and_kinds = vec![
        (4,8)
    ];

    let mvs = vec![
        ((4,8),(4,7))
    ];

    for (&m, &(x, y)) in mvs.iter().zip(position_and_kinds.iter()) {
        let (x,y) = (8-x,8-y);

        let m = match m {
            ((sx,sy),(dx,dy)) => {
                ((8-sx,8-sy),(8-dx,8-dy))
            }
        };

        let mut banmen = Banmen([[Blank; 9]; 9]);

        banmen.0[8-3][8-4] = SOu;
        banmen.0[8-7][8-4] = KomaKind::SFu;
        banmen.0[y][x] = KomaKind::GKyou;

        let state = State::new(banmen);

        assert_eq!(true,Rule::is_oute_move(&state, Teban::Gote,
                                           LegalMove::To(LegalMoveTo::new((m.0).0 * 9 + (m.0).1,
                                                                          (m.1).0 * 9 + (m.1).1,
                                                                          false, None))), "{:?} {:?}", GKyou, m);
    }
}
#[test]
fn is_oute_not_moveto_gote_kyou() {
    let position_and_kinds = vec![
        (3,8),
        (5,8)
    ];

    let mvs = vec![
        ((3,8),(3,7)),
        ((5,8),(5,7))
    ];

    for (&m, &(x, y)) in mvs.iter().zip(position_and_kinds.iter()) {
        let (x,y) = (8-x,8-y);

        let m = match m {
            ((sx,sy),(dx,dy)) => {
                ((8-sx,8-sy),(8-dx,8-dy))
            }
        };

        let mut banmen = Banmen([[Blank; 9]; 9]);

        banmen.0[8-3][8-4] = SOu;
        banmen.0[y][x] = GKyou;

        let state = State::new(banmen);

        assert_eq!(false,Rule::is_oute_move(&state, Teban::Gote,
                                            LegalMove::To(LegalMoveTo::new((m.0).0 * 9 + (m.0).1,
                                                                           (m.1).0 * 9 + (m.1).1,
                                                                           false, None))), "{:?} {:?}", GKyou, m);
    }
}
#[test]
fn is_oute_not_moveto_gote_kyou_occupied_self() {
    let position_and_kinds = vec![
        (4,8)
    ];

    let mvs = vec![
        ((4,8),(4,7))
    ];

    for (&m, &(x, y)) in mvs.iter().zip(position_and_kinds.iter()) {
        let (x,y) = (8-x,8-y);

        let m = match m {
            ((sx,sy),(dx,dy)) => {
                ((8-sx,8-sy),(8-dx,8-dy))
            }
        };

        let mut banmen = Banmen([[Blank; 9]; 9]);

        banmen.0[8-3][8-4] = SOu;
        banmen.0[y][x] = GKyou;
        banmen.0[8-6][8-4] = KomaKind::GFu;

        let state = State::new(banmen);

        assert_eq!(false,Rule::is_oute_move(&state, Teban::Gote,
                                            LegalMove::To(LegalMoveTo::new((m.0).0 * 9 + (m.0).1,
                                                                           (m.1).0 * 9 + (m.1).1,
                                                                           false, None))), "{:?} {:?}", GKyou, m);
    }
}
#[test]
fn is_oute_not_moveto_gote_kyou_occupied_opponent() {
    let position_and_kinds = vec![
        (4,8)
    ];

    let mvs = vec![
        ((4,8),(4,7))
    ];

    for (&m, &(x, y)) in mvs.iter().zip(position_and_kinds.iter()) {
        let (x,y) = (8-x,8-y);

        let m = match m {
            ((sx,sy),(dx,dy)) => {
                ((8-sx,8-sy),(8-dx,8-dy))
            }
        };

        let mut banmen = Banmen([[Blank; 9]; 9]);

        banmen.0[8-3][8-4] = SOu;
        banmen.0[y][x] = GKyou;
        banmen.0[8-6][8-4] = KomaKind::SFu;

        let state = State::new(banmen);

        assert_eq!(false,Rule::is_oute_move(&state, Teban::Gote,
                                            LegalMove::To(LegalMoveTo::new((m.0).0 * 9 + (m.0).1,
                                                                           (m.1).0 * 9 + (m.1).1,
                                                                           false, None))), "{:?} {:?}", GKyou, m);
    }
}
#[test]
fn is_oute_moveto_gote_kyou_open_path() {
    let position_and_kinds = vec![
        ((4,8),(4,7))
    ];

    let mvs = vec![
        ((4,7),(5,7))
    ];

    for (&m, &((kx, ky),(x,y))) in mvs.iter().zip(position_and_kinds.iter()) {
        let (kx,ky) = (8-kx,8-ky);
        let (x,y) = (8-x,8-y);

        let m = match m {
            ((sx,sy),(dx,dy)) => {
                ((8-sx,8-sy),(8-dx,8-dy))
            }
        };

        let mut banmen = Banmen([[Blank; 9]; 9]);

        banmen.0[8-3][8-4] = SOu;
        banmen.0[ky][kx] = GKyou;
        banmen.0[y][x] = GKin;

        let state = State::new(banmen);

        assert_eq!(true,Rule::is_oute_move(&state, Teban::Gote,
                                           LegalMove::To(LegalMoveTo::new((m.0).0 * 9 + (m.0).1,
                                                                          (m.1).0 * 9 + (m.1).1,
                                                                          false, None))), "{:?} {:?}", GKin, m);
    }
}
#[test]
fn is_oute_moveto_gote_kyou_open_path_occupied() {
    let position_and_kinds = vec![
        ((4,8),(4,7))
    ];

    let mvs = vec![
        ((4,8),(4,7))
    ];

    for (&m, &((x, y),(ox,oy))) in mvs.iter().zip(position_and_kinds.iter()) {
        let (x,y) = (8-x,8-y);
        let (ox,oy) = (8-ox,8-oy);

        let m = match m {
            ((sx,sy),(dx,dy)) => {
                ((8-sx,8-sy),(8-dx,8-dy))
            }
        };

        let mut banmen = Banmen([[Blank; 9]; 9]);

        banmen.0[8-3][8-4] = SOu;
        banmen.0[y][x] = GKyou;
        banmen.0[oy][ox] = KomaKind::SFu;

        let state = State::new(banmen);

        assert_eq!(true,Rule::is_oute_move(&state, Teban::Gote,
                                           LegalMove::To(LegalMoveTo::new((m.0).0 * 9 + (m.0).1,
                                                                          (m.1).0 * 9 + (m.1).1,
                                                                          false, None))), "{:?} {:?}", GKyou, m);
    }
}
#[test]
fn is_oute_moveput_gote() {
    let mvs = vec![
        (4,4,MochigomaKind::Fu),
        (4,8,MochigomaKind::Kyou),
        (3,5,MochigomaKind::Kei),
        (5,5,MochigomaKind::Kei),
        (3,4,MochigomaKind::Gin),
        (4,4,MochigomaKind::Gin),
        (5,4,MochigomaKind::Gin),
        (3,2,MochigomaKind::Gin),
        (5,2,MochigomaKind::Gin),
        (3,4,MochigomaKind::Kin),
        (4,4,MochigomaKind::Kin),
        (5,4,MochigomaKind::Kin),
        (3,3,MochigomaKind::Kin),
        (5,3,MochigomaKind::Kin),
        (4,2,MochigomaKind::Kin),
        (0,7,MochigomaKind::Kaku),
        (8,7,MochigomaKind::Kaku),
        (1,0,MochigomaKind::Kaku),
        (7,0,MochigomaKind::Kaku),
        (4,8,MochigomaKind::Hisha),
        (0,3,MochigomaKind::Hisha),
        (8,3,MochigomaKind::Hisha),
        (4,0,MochigomaKind::Hisha)
    ];

    for &(x,y,kind) in mvs.iter() {
        let (x,y) = (8-x,8-y);

        let mut banmen = Banmen([[Blank; 9]; 9]);

        banmen.0[8-3][8-4] = SOu;

        let state = State::new(banmen);

        assert_eq!(true,Rule::is_oute_move(&state, Teban::Gote,
                                           LegalMove::Put(LegalMovePut::new(kind,
                                                                            x * 9 + y))), "{:?} {},{}", kind, x, y);
    }
}
#[test]
fn is_oute_not_moveput_gote() {
    let mvs = vec![
        (4,5,MochigomaKind::Fu),
        (5,3,MochigomaKind::Fu),
        (5,3,MochigomaKind::Kyou),
        (5,3,MochigomaKind::Kei),
        (4,5,MochigomaKind::Gin),
        (2,5,MochigomaKind::Gin),
        (6,5,MochigomaKind::Gin),
        (3,3,MochigomaKind::Gin),
        (5,3,MochigomaKind::Gin),
        (2,1,MochigomaKind::Gin),
        (6,1,MochigomaKind::Gin),
        (4,2,MochigomaKind::Gin),
        (4,5,MochigomaKind::Kin),
        (2,5,MochigomaKind::Kin),
        (6,5,MochigomaKind::Kin),
        (2,3,MochigomaKind::Kin),
        (6,3,MochigomaKind::Kin),
        (3,2,MochigomaKind::Kin),
        (5,2,MochigomaKind::Kin),
        (4,1,MochigomaKind::Kin),
        (3,1,MochigomaKind::Kaku),
        (5,1,MochigomaKind::Kaku),
        (3,5,MochigomaKind::Kaku),
        (5,5,MochigomaKind::Kaku),
        (5,0,MochigomaKind::Hisha),
        (0,4,MochigomaKind::Hisha),
        (8,4,MochigomaKind::Hisha),
        (3,0,MochigomaKind::Hisha)
    ];

    for &(x,y,kind) in mvs.iter() {
        let (x,y) = (8-x,8-y);

        let mut banmen = Banmen([[Blank; 9]; 9]);

        banmen.0[8-3][8-4] = SOu;

        let state = State::new(banmen);

        assert_eq!(false,Rule::is_oute_move(&state, Teban::Gote,
                                            LegalMove::Put(LegalMovePut::new(kind,
                                                                             x * 9 + y))), "{:?} {},{}", kind, x, y);
    }
}
#[test]
fn is_oute_not_moveput_gote_kaku_self_occupied() {
    let mvs = vec![
        (0,7,MochigomaKind::Kaku),
        (8,7,MochigomaKind::Kaku),
        (1,0,MochigomaKind::Kaku),
        (7,0,MochigomaKind::Kaku),
    ];

    let occ = vec![
        (1,6),
        (7,6),
        (2,1),
        (6,1)
    ];

    for (&(x,y,kind),&(ox,oy)) in mvs.iter().zip(occ.iter()) {
        let (x,y) = (8-x,8-y);
        let (ox,oy) = (8-ox,8-oy);

        let mut banmen = Banmen([[Blank; 9]; 9]);

        banmen.0[8-3][8-4] = SOu;
        banmen.0[oy][ox] = KomaKind::GFu;

        let state = State::new(banmen);

        assert_eq!(false,Rule::is_oute_move(&state, Teban::Gote,
                                            LegalMove::Put(LegalMovePut::new(kind,
                                                                             x * 9 + y))), "{:?} {},{}", kind, x, y);
    }
}
#[test]
fn is_oute_not_moveput_gote_kaku_opponent_occupied() {
    let mvs = vec![
        (0,7,MochigomaKind::Kaku),
        (8,7,MochigomaKind::Kaku),
        (1,0,MochigomaKind::Kaku),
        (7,0,MochigomaKind::Kaku),
    ];

    let occ = vec![
        (1,6),
        (7,6),
        (2,1),
        (6,1)
    ];

    for (&(x,y,kind),&(ox,oy)) in mvs.iter().zip(occ.iter()) {
        let (x,y) = (8-x,8-y);
        let (ox,oy) = (8-ox,8-oy);

        let mut banmen = Banmen([[Blank; 9]; 9]);

        banmen.0[8-3][8-4] = SOu;
        banmen.0[oy][ox] = KomaKind::SFu;

        let state = State::new(banmen);

        assert_eq!(false,Rule::is_oute_move(&state, Teban::Gote,
                                            LegalMove::Put(LegalMovePut::new(kind,
                                                                             x * 9 + y))), "{:?} {},{}", kind, x, y);
    }
}
#[test]
fn is_oute_not_moveput_gote_hisha_self_occupied() {
    let mvs = vec![
        (4,8,MochigomaKind::Hisha),
        (0,3,MochigomaKind::Hisha),
        (8,3,MochigomaKind::Hisha),
        (4,0,MochigomaKind::Hisha)
    ];

    let occ = vec![
        (4,7),
        (1,3),
        (7,3),
        (4,1)
    ];

    for (&(x,y,kind),&(ox,oy)) in mvs.iter().zip(occ.iter()) {
        let (x,y) = (8-x,8-y);
        let (ox,oy) = (8-ox,8-oy);

        let mut banmen = Banmen([[Blank; 9]; 9]);

        banmen.0[8-3][8-4] = SOu;
        banmen.0[oy][ox] = KomaKind::GFu;

        let state = State::new(banmen);

        assert_eq!(false,Rule::is_oute_move(&state, Teban::Gote,
                                            LegalMove::Put(LegalMovePut::new(kind,
                                                                             x * 9 + y))), "{:?} {},{}", kind, x, y);
    }
}
#[test]
fn is_oute_not_moveput_gote_hisha_opponent_occupied() {
    let mvs = vec![
        (4,8,MochigomaKind::Hisha),
        (0,3,MochigomaKind::Hisha),
        (8,3,MochigomaKind::Hisha),
        (4,0,MochigomaKind::Hisha)
    ];

    let occ = vec![
        (4,7),
        (1,3),
        (7,3),
        (4,1)
    ];

    for (&(x,y,kind),&(ox,oy)) in mvs.iter().zip(occ.iter()) {
        let (x,y) = (8-x,8-y);
        let (ox,oy) = (8-ox,8-oy);

        let mut banmen = Banmen([[Blank; 9]; 9]);

        banmen.0[8-3][8-4] = SOu;
        banmen.0[oy][ox] = KomaKind::SFu;

        let state = State::new(banmen);

        assert_eq!(false,Rule::is_oute_move(&state, Teban::Gote,
                                            LegalMove::Put(LegalMovePut::new(kind,
                                                                             x * 9 + y))), "{:?} {},{}", kind, x, y);
    }
}
#[test]
fn is_oute_not_moveput_gote_kyou_self_occupied() {
    let mvs = vec![
        (4,8,MochigomaKind::Kyou)
    ];

    let occ = vec![
        (4,7)
    ];

    for (&(x,y,kind),&(ox,oy)) in mvs.iter().zip(occ.iter()) {
        let (x,y) = (8-x,8-y);
        let (ox,oy) = (8-ox,8-oy);

        let mut banmen = Banmen([[Blank; 9]; 9]);

        banmen.0[8-3][8-4] = SOu;
        banmen.0[oy][ox] = KomaKind::GFu;

        let state = State::new(banmen);

        assert_eq!(false,Rule::is_oute_move(&state, Teban::Gote,
                                            LegalMove::Put(LegalMovePut::new(kind,
                                                                             x * 9 + y))), "{:?} {},{}", kind, x, y);
    }
}
#[test]
fn is_oute_not_moveput_gote_kyou_opponent_occupied() {
    let mvs = vec![
        (4,8,MochigomaKind::Kyou)
    ];

    let occ = vec![
        (4,7)
    ];

    for (&(x,y,kind),&(ox,oy)) in mvs.iter().zip(occ.iter()) {
        let (x,y) = (8-x,8-y);
        let (ox,oy) = (8-ox,8-oy);

        let mut banmen = Banmen([[Blank; 9]; 9]);

        banmen.0[8-3][8-4] = SOu;
        banmen.0[oy][ox] = KomaKind::SFu;

        let state = State::new(banmen);

        assert_eq!(false,Rule::is_oute_move(&state, Teban::Gote,
                                            LegalMove::Put(LegalMovePut::new(kind,
                                                                             x * 9 + y))), "{:?} {},{}", kind, x, y);
    }
}
