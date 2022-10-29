use usiagent::rule::{LegalMove, LegalMoveTo, Rule, State};
use usiagent::shogi::{Banmen, KomaKind, Teban};
use usiagent::shogi::KomaKind::{Blank, GOu};


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

        assert!(Rule::is_oute_move(&state,Teban::Sente,
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
