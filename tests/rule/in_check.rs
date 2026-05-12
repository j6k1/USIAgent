use usiagent::rule::{Rule, State, BANMEN_START_POS};
use usiagent::shogi::KomaKind::{
    Blank, GFu, GFuN, GGin, GGinN, GHisha, GHishaN, GKaku, GKakuN, GKei, GKeiN, GKin, GKyou,
    GKyouN, GOu, SFu, SFuN, SGin, SGinN, SHisha, SHishaN, SKaku, SKakuN, SKei, SKeiN, SKin, SKyou,
    SKyouN, SOu,
};
use usiagent::shogi::{
    Banmen, KomaDstPutPosition, KomaDstToPosition, KomaKind, KomaSrcPosition, MochigomaCollections,
    Mochigoma, MochigomaKind, Move, Teban,
};

fn state_with(
    sente_ou: (usize, usize),
    gote_ou: (usize, usize),
    pieces: &[(usize, usize, KomaKind)],
) -> State {
    let mut banmen = Banmen([[Blank; 9]; 9]);

    banmen.0[sente_ou.1][sente_ou.0] = SOu;
    banmen.0[gote_ou.1][gote_ou.0] = GOu;

    for &(x, y, kind) in pieces {
        banmen.0[y][x] = kind;
    }

    State::new(banmen)
}

fn state_for_sente_check(attacker: KomaKind, attacker_pos: (usize, usize)) -> State {
    if attacker == GOu {
        state_with((4, 5), attacker_pos, &[])
    } else {
        state_with(
            (4, 5),
            (8, 0),
            &[(attacker_pos.0, attacker_pos.1, attacker)],
        )
    }
}

fn state_for_gote_check(attacker: KomaKind, attacker_pos: (usize, usize)) -> State {
    if attacker == SOu {
        state_with(attacker_pos, (4, 3), &[])
    } else {
        state_with(
            (8, 8),
            (4, 3),
            &[(attacker_pos.0, attacker_pos.1, attacker)],
        )
    }
}

fn mv_to(from: (usize, usize), to: (usize, usize)) -> Move {
    Move::To(
        KomaSrcPosition(9 - from.0 as u32, from.1 as u32 + 1),
        KomaDstToPosition(9 - to.0 as u32, to.1 as u32 + 1, false),
    )
}

fn mv_put(kind: MochigomaKind, to: (usize, usize)) -> Move {
    Move::Put(
        kind,
        KomaDstPutPosition(9 - to.0 as u32, to.1 as u32 + 1),
    )
}

fn assert_in_check_matches_rebuilt(state: &State, message: &str) {
    let rebuilt = State::new(state.get_banmen().clone());

    assert_eq!(
        Rule::in_check(Teban::Sente, state),
        Rule::in_check(Teban::Sente, &rebuilt),
        "sente {}",
        message
    );
    assert_eq!(
        Rule::in_check(Teban::Gote, state),
        Rule::in_check(Teban::Gote, &rebuilt),
        "gote {}",
        message
    );
}

fn mochigoma_with_gote(kind: MochigomaKind, count: usize) -> MochigomaCollections {
    let sente = Mochigoma::new();
    let mut gote = Mochigoma::new();

    gote.insert(kind, count);

    MochigomaCollections::Pair(sente, gote)
}

#[test]
fn in_check_is_false_at_start_position() {
    let state = State::new(BANMEN_START_POS);

    assert!(!Rule::in_check(Teban::Sente, &state));
    assert!(!Rule::in_check(Teban::Gote, &state));
}

#[test]
fn sente_in_check_from_each_gote_piece_kind() {
    let cases = [
        ("fu", GFu, (4, 4)),
        ("kyou", GKyou, (4, 1)),
        ("kei", GKei, (3, 3)),
        ("gin", GGin, (4, 4)),
        ("kin", GKin, (4, 4)),
        ("tokin", GFuN, (4, 4)),
        ("narikyou", GKyouN, (4, 4)),
        ("narikei", GKeiN, (4, 4)),
        ("narigin", GGinN, (4, 4)),
        ("kaku", GKaku, (2, 3)),
        ("uma", GKakuN, (2, 3)),
        ("hisha", GHisha, (4, 1)),
        ("ryu", GHishaN, (4, 1)),
    ];

    for &(name, kind, pos) in &cases {
        let state = state_for_sente_check(kind, pos);

        assert!(Rule::in_check(Teban::Sente, &state), "{}", name);
        assert!(!Rule::in_check(Teban::Gote, &state), "{}", name);
    }
}

#[test]
fn gote_in_check_from_each_sente_piece_kind() {
    let cases = [
        ("fu", SFu, (4, 4)),
        ("kyou", SKyou, (4, 7)),
        ("kei", SKei, (3, 5)),
        ("gin", SGin, (4, 4)),
        ("kin", SKin, (4, 4)),
        ("tokin", SFuN, (4, 4)),
        ("narikyou", SKyouN, (4, 4)),
        ("narikei", SKeiN, (4, 4)),
        ("narigin", SGinN, (4, 4)),
        ("kaku", SKaku, (2, 5)),
        ("uma", SKakuN, (2, 5)),
        ("hisha", SHisha, (4, 7)),
        ("ryu", SHishaN, (4, 7)),
    ];

    for &(name, kind, pos) in &cases {
        let state = state_for_gote_check(kind, pos);

        assert!(Rule::in_check(Teban::Gote, &state), "{}", name);
        assert!(!Rule::in_check(Teban::Sente, &state), "{}", name);
    }
}

#[test]
fn in_check_from_promoted_piece_additional_king_step_controls() {
    let state = state_with((8, 8), (4, 3), &[(4, 4, SKakuN)]);
    assert!(Rule::in_check(Teban::Gote, &state), "sente horse vertical");

    let state = state_with((8, 8), (4, 3), &[(3, 4, SHishaN)]);
    assert!(Rule::in_check(Teban::Gote, &state), "sente dragon diagonal");

    let state = state_with((4, 5), (8, 0), &[(4, 4, GKakuN)]);
    assert!(Rule::in_check(Teban::Sente, &state), "gote horse vertical");

    let state = state_with((4, 5), (8, 0), &[(3, 4, GHishaN)]);
    assert!(Rule::in_check(Teban::Sente, &state), "gote dragon diagonal");
}

#[test]
fn in_check_is_false_when_pieces_do_not_attack_the_king() {
    let sente_cases = [
        ("fu", GFu, (5, 4)),
        ("kyou", GKyou, (5, 1)),
        ("kei", GKei, (4, 3)),
        ("gin", GGin, (5, 3)),
        ("kin", GKin, (5, 3)),
        ("kaku", GKaku, (1, 3)),
        ("hisha", GHisha, (5, 1)),
    ];

    for &(name, kind, pos) in &sente_cases {
        let state = state_with((4, 5), (8, 0), &[(pos.0, pos.1, kind)]);
        assert!(!Rule::in_check(Teban::Sente, &state), "sente {}", name);
    }

    let gote_cases = [
        ("fu", SFu, (5, 4)),
        ("kyou", SKyou, (5, 7)),
        ("kei", SKei, (4, 5)),
        ("gin", SGin, (5, 5)),
        ("kin", SKin, (5, 5)),
        ("kaku", SKaku, (1, 5)),
        ("hisha", SHisha, (5, 7)),
    ];

    for &(name, kind, pos) in &gote_cases {
        let state = state_with((8, 8), (4, 3), &[(pos.0, pos.1, kind)]);
        assert!(!Rule::in_check(Teban::Gote, &state), "gote {}", name);
    }
}

#[test]
fn in_check_is_false_when_slider_attack_is_blocked() {
    let sente_cases = [
        ("kyou", GKyou, (4, 1), (4, 3, GFu)),
        ("kaku", GKaku, (1, 2), (2, 3, GFu)),
        ("hisha", GHisha, (4, 1), (4, 3, GFu)),
    ];

    for &(name, attacker, attacker_pos, blocker) in &sente_cases {
        let state = state_with(
            (4, 5),
            (8, 0),
            &[(attacker_pos.0, attacker_pos.1, attacker), blocker],
        );
        assert!(!Rule::in_check(Teban::Sente, &state), "sente {}", name);
    }

    let gote_cases = [
        ("kyou", SKyou, (4, 7), (4, 5, SFu)),
        ("kaku", SKaku, (1, 6), (2, 5, SFu)),
        ("hisha", SHisha, (4, 7), (4, 5, SFu)),
    ];

    for &(name, attacker, attacker_pos, blocker) in &gote_cases {
        let state = state_with(
            (8, 8),
            (4, 3),
            &[(attacker_pos.0, attacker_pos.1, attacker), blocker],
        );
        assert!(!Rule::in_check(Teban::Gote, &state), "gote {}", name);
    }
}

#[test]
fn in_check_after_applying_move_that_gives_check() {
    let state = state_with((8, 8), (4, 1), &[(0, 4, SHisha)]);
    let (state, _, _) = Rule::apply_move_none_check(
        &state,
        Teban::Sente,
        &MochigomaCollections::Empty,
        mv_to((0, 4), (4, 4)).to_applied_move(),
    );

    assert!(Rule::in_check(Teban::Gote, &state));
    assert!(!Rule::in_check(Teban::Sente, &state));
}

#[test]
fn in_check_after_applying_move_that_blocks_check() {
    let state = state_with((8, 8), (4, 1), &[(4, 4, SHisha), (0, 3, GKin)]);
    assert!(Rule::in_check(Teban::Gote, &state));

    let (state, _, _) = Rule::apply_move_none_check(
        &state,
        Teban::Gote,
        &MochigomaCollections::Empty,
        mv_to((0, 3), (4, 3)).to_applied_move(),
    );

    assert!(!Rule::in_check(Teban::Gote, &state));
    assert!(!Rule::in_check(Teban::Sente, &state));
}

#[test]
fn in_check_when_kings_are_adjacent_in_initial_state() {
    let state = state_with((4, 5), (4, 4), &[]);

    assert!(Rule::in_check(Teban::Sente, &state));
    assert!(Rule::in_check(Teban::Gote, &state));
}

#[test]
fn in_check_after_applying_king_move_next_to_opponent_king() {
    let state = state_with((4, 5), (8, 0), &[]);
    let (state, _, _) = Rule::apply_move_none_check(
        &state,
        Teban::Gote,
        &MochigomaCollections::Empty,
        mv_to((8, 0), (4, 4)).to_applied_move(),
    );

    assert!(Rule::in_check(Teban::Sente, &state));
    assert!(Rule::in_check(Teban::Gote, &state));
}

#[test]
fn in_check_after_applying_drop_that_blocks_rook_check() {
    let state = state_with((8, 8), (4, 1), &[(4, 4, SHisha)]);
    assert!(Rule::in_check(Teban::Gote, &state));

    let (state, _, _) = Rule::apply_move_none_check(
        &state,
        Teban::Gote,
        &MochigomaCollections::Empty,
        mv_put(MochigomaKind::Kin, (4, 3)).to_applied_move(),
    );

    assert!(!Rule::in_check(Teban::Gote, &state));
    assert!(!Rule::in_check(Teban::Sente, &state));
}

#[test]
fn in_check_incremental_update_matches_rebuilt_state_after_moves() {
    let cases = [
        (
            "move checking rook away",
            state_with((8, 8), (4, 1), &[(4, 4, SHisha)]),
            Teban::Sente,
            mv_to((4, 4), (0, 4)),
        ),
        (
            "capture checking rook",
            state_with((8, 8), (4, 1), &[(4, 4, SHisha), (0, 3, GKin)]),
            Teban::Gote,
            mv_to((0, 3), (4, 4)),
        ),
        (
            "move blocker away and reveal rook check",
            state_with((8, 8), (4, 1), &[(4, 4, SHisha), (4, 3, GKin)]),
            Teban::Gote,
            mv_to((4, 3), (3, 3)),
        ),
        (
            "move king away from rook line",
            state_with((8, 8), (4, 1), &[(4, 4, SHisha)]),
            Teban::Gote,
            mv_to((4, 1), (3, 1)),
        ),
        (
            "move horse into promoted-only check",
            state_with((8, 8), (4, 3), &[(0, 4, SKakuN)]),
            Teban::Sente,
            mv_to((0, 4), (4, 4)),
        ),
        (
            "move dragon into promoted-only check",
            state_with((8, 8), (4, 3), &[(0, 4, SHishaN)]),
            Teban::Sente,
            mv_to((0, 4), (3, 4)),
        ),
    ];

    for &(message, ref state, teban, mv) in &cases {
        let (state, _, _) = Rule::apply_move_none_check(
            state,
            teban,
            &MochigomaCollections::Empty,
            mv.to_applied_move(),
        );

        assert_in_check_matches_rebuilt(&state, message);
    }
}

#[test]
fn in_check_incremental_update_matches_rebuilt_state_after_all_legal_moves() {
    let positions = [
        (
            "rook check with possible responses",
            state_with((8, 8), (4, 1), &[(4, 4, SHisha), (0, 3, GKin), (5, 3, GGin)]),
            Teban::Gote,
            MochigomaCollections::Empty,
        ),
        (
            "discovered rook check candidates",
            state_with((8, 8), (4, 1), &[(4, 4, SHisha), (4, 3, GKin), (3, 2, GGin)]),
            Teban::Gote,
            MochigomaCollections::Empty,
        ),
        (
            "promoted piece additional control candidates",
            state_with((8, 8), (4, 3), &[(0, 4, SKakuN), (1, 4, SHishaN), (3, 6, SKin)]),
            Teban::Sente,
            MochigomaCollections::Empty,
        ),
        (
            "drop responses to rook check",
            state_with((8, 8), (4, 1), &[(4, 4, SHisha)]),
            Teban::Gote,
            mochigoma_with_gote(MochigomaKind::Kin, 1),
        ),
    ];

    for &(message, ref state, teban, ref mc) in &positions {
        for mv in Rule::legal_moves_all(teban, state, mc) {
            let (state, _, _) = Rule::apply_move_none_check(
                state,
                teban,
                mc,
                mv.to_applied_move(),
            );

            assert_in_check_matches_rebuilt(&state, message);
        }
    }
}
