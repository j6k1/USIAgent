use usiagent::shogi::*;
use usiagent::rule::Rule;
use usiagent::rule::State;

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
fn test_sente_fu_has_control() {
    let positions = [(4,5)];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for &(x,y) in positions.iter() {
        let mut banmen = blank_banmen.clone();

        banmen.0[y][x] = SFu;

        assert_eq!(Rule::control_count(Teban::Sente,&State::new(banmen),4 * 9 + 4),1);
    }
}
#[test]
fn test_sente_kyou_has_control() {
    let positions = [(4,8)];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for &(x,y) in positions.iter() {
        let mut banmen = blank_banmen.clone();

        banmen.0[y][x] = SKyou;

        assert_eq!(Rule::control_count(Teban::Sente,&State::new(banmen),4 * 9 + 4),1);
    }
}
#[test]
fn test_sente_kei_has_control() {
    let positions = [vec![(3,6),(5,6)]];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for it in positions.iter() {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[y][x] = SKei;
        }

        assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), 4 * 9 + 4), 2);
    }
}
#[test]
fn test_sente_kei_has_control_edge() {
    let positions = [(0,6),(8,6)];
    let target_positions = [(1,4),(7,4)];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for (&(x,y),&(tx,ty)) in positions.iter().zip(target_positions.iter()) {
        let mut banmen = blank_banmen.clone();

        banmen.0[y][x] = SKei;

        assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), tx * 9 + ty), 1);
    }
}
#[test]
fn test_sente_gin_has_control() {
    let positions = [vec![(4,5),(3,3),(5,3)],vec![(3,5),(5,5)]];
    let answers = [3,2];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for (it,&answer) in positions.iter().zip(answers.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[y][x] = SGin;
        }

        assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), 4 * 9 + 4), answer);
    }
}
#[test]
fn test_sente_kin_has_control() {
    let positions = [vec![(4,5),(3,5),(5,5)],vec![(3,4),(5,4)],vec![(4,3)]];
    let answers = [3,2,1];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for (it,&answer) in positions.iter().zip(answers.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[y][x] = SKin;
        }

        assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), 4 * 9 + 4), answer);
    }
}
#[test]
fn test_sente_nari_kin_has_control() {
    let positions = [vec![(4,5),(3,5),(5,5)],vec![(3,4),(5,4)],vec![(4,3)]];
    let answers = [3,2,1];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for &kind in [SFuN,SKyouN,SKeiN,SGinN].iter() {
        for (it, &answer) in positions.iter().zip(answers.iter()) {
            let mut banmen = blank_banmen.clone();

            for &(x, y) in it.iter() {
                banmen.0[y][x] = kind;
            }

            assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), 4 * 9 + 4), answer);
        }
    }
}
#[test]
fn test_sente_kaku_has_control() {
    let positions = [vec![(3,3),(5,3),(3,5),(5,5)],vec![(0,0),(0,8)],vec![(8,0),(8,8)]];
    let answers = [4,2,2];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for (it,&answer) in positions.iter().zip(answers.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[y][x] = SKaku;
        }

        assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), 4 * 9 + 4), answer);
    }
}
#[test]
fn test_sente_hisha_has_control() {
    let positions = [vec![(3,4),(5,4),(4,3),(4,5)],vec![(0,4),(4,0)],vec![(8,4),(4,8)]];
    let answers = [4,2,2];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for (it,&answer) in positions.iter().zip(answers.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[y][x] = SHisha;
        }

        assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), 4 * 9 + 4), answer);
    }
}
#[test]
fn test_sente_ou_has_control() {
    let positions = [
        vec![(4,5),(3,5),(5,5)],
        vec![(3,4),(5,4)],
        vec![(4,3)],
        vec![(3,3),(5,3)],
        vec![(3,5)],
        vec![(4,5)],
        vec![(5,5)],
        vec![(3,4)],
        vec![(5,4)],
        vec![(3,3)],
        vec![(5,3)],
    ];
    let answers = [3,2,1,2,1,1,1,1,1,1,1];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for (it,&answer) in positions.iter().zip(answers.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[y][x] = SOu;
        }

        assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), 4 * 9 + 4), answer);
    }
}
#[test]
fn test_sente_kaku_nari_has_control() {
    let positions = [vec![(3,3),(5,3),(3,5),(5,5)],vec![(0,0),(0,8)],vec![(8,0),(8,8)],vec![(3,4),(4,3),(5,4),(4,5)]];
    let answers = [4,2,2,4];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for (it,&answer) in positions.iter().zip(answers.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[y][x] = SKakuN;
        }

        assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), 4 * 9 + 4), answer);
    }
}
#[test]
fn test_sente_hisha_nari_has_control() {
    let positions = [vec![(3,4),(5,4),(4,3),(4,5)],vec![(0,4),(4,0)],vec![(8,4),(4,8)],vec![(3,5),(3,3),(5,3),(5,5)]];
    let answers = [4,2,2,4];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for (it,&answer) in positions.iter().zip(answers.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[y][x] = SHishaN;
        }

        assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), 4 * 9 + 4), answer);
    }
}
#[test]
fn test_gote_fu_has_control() {
    let positions = [(4,5)];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for &(x,y) in positions.iter() {
        let mut banmen = blank_banmen.clone();

        banmen.0[8-y][8-x] = GFu;

        assert_eq!(Rule::control_count(Teban::Gote,&State::new(banmen),4 * 9 + 4),1);
    }
}
#[test]
fn test_gote_kyou_has_control() {
    let positions = [(4,8)];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for &(x,y) in positions.iter() {
        let mut banmen = blank_banmen.clone();

        banmen.0[8-y][8-x] = GKyou;

        assert_eq!(Rule::control_count(Teban::Gote,&State::new(banmen),4 * 9 + 4),1);
    }
}
#[test]
fn test_gote_kei_has_control() {
    let positions = [vec![(3,6),(5,6)]];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for it in positions.iter() {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[8-y][8-x] = GKei;
        }

        assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), 4 * 9 + 4), 2);
    }
}
#[test]
fn test_gote_kei_has_control_edge() {
    let positions = [(0,6),(8,6)];
    let target_positions = [(1,4),(7,4)];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for (&(x,y),&(tx,ty)) in positions.iter().zip(target_positions.iter()) {
        let mut banmen = blank_banmen.clone();

        banmen.0[8-y][8-x] = GKei;

        assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), 80 - (tx * 9 + ty)), 1);
    }
}
#[test]
fn test_gote_gin_has_control() {
    let positions = [vec![(4,5),(3,3),(5,3)],vec![(3,5),(5,5)]];
    let answers = [3,2];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for (it,&answer) in positions.iter().zip(answers.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[8-y][8-x] = GGin;
        }

        assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), 4 * 9 + 4), answer);
    }
}
#[test]
fn test_gote_kin_has_control() {
    let positions = [vec![(4,5),(3,5),(5,5)],vec![(3,4),(5,4)],vec![(4,3)]];
    let answers = [3,2,1];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for (it,&answer) in positions.iter().zip(answers.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[8-y][8-x] = GKin;
        }

        assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), 4 * 9 + 4), answer);
    }
}
#[test]
fn test_gote_nari_kin_has_control() {
    let positions = [vec![(4,5),(3,5),(5,5)],vec![(3,4),(5,4)],vec![(4,3)]];
    let answers = [3,2,1];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for &kind in [GFuN,GKyouN,GKeiN,GGinN].iter() {
        for (it, &answer) in positions.iter().zip(answers.iter()) {
            let mut banmen = blank_banmen.clone();

            for &(x, y) in it.iter() {
                banmen.0[8-y][8-x] = kind;
            }

            assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), 4 * 9 + 4), answer);
        }
    }
}
#[test]
fn test_gote_kaku_has_control() {
    let positions = [vec![(3,3),(5,3),(3,5),(5,5)],vec![(0,0),(0,8)],vec![(8,0),(8,8)]];
    let answers = [4,2,2];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for (it,&answer) in positions.iter().zip(answers.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[8-y][8-x] = GKaku;
        }

        assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), 4 * 9 + 4), answer);
    }
}
#[test]
fn test_gote_hisha_has_control() {
    let positions = [vec![(3,4),(5,4),(4,3),(4,5)],vec![(0,4),(4,0)],vec![(8,4),(4,8)]];
    let answers = [4,2,2];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for (it,&answer) in positions.iter().zip(answers.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[8-y][8-x] = GHisha;
        }

        assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), 4 * 9 + 4), answer);
    }
}
#[test]
fn test_gote_ou_has_control() {
    let positions = [
        vec![(4,5),(3,5),(5,5)],
        vec![(3,4),(5,4)],
        vec![(4,3)],
        vec![(3,3),(5,3)],
        vec![(3,5)],
        vec![(4,5)],
        vec![(5,5)],
        vec![(3,4)],
        vec![(5,4)],
        vec![(3,3)],
        vec![(5,3)],
    ];
    let answers = [3,2,1,2,1,1,1,1,1,1,1];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for (it,&answer) in positions.iter().zip(answers.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[8-y][8-x] = GOu;
        }

        assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), 4 * 9 + 4), answer);
    }
}
#[test]
fn test_gote_kaku_nari_has_control() {
    let positions = [vec![(3,3),(5,3),(3,5),(5,5)],vec![(0,0),(0,8)],vec![(8,0),(8,8)],vec![(3,4),(4,3),(5,4),(4,5)]];
    let answers = [4,2,2,4];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for (it,&answer) in positions.iter().zip(answers.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[8-y][8-x] = GKakuN;
        }

        assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), 4 * 9 + 4), answer);
    }
}
#[test]
fn test_gote_hisha_nari_has_control() {
    let positions = [vec![(3,4),(5,4),(4,3),(4,5)],vec![(0,4),(4,0)],vec![(8,4),(4,8)],vec![(3,5),(3,3),(5,3),(5,5)]];
    let answers = [4,2,2,4];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for (it,&answer) in positions.iter().zip(answers.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[8-y][8-x] = GHishaN;
        }

        assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), 4 * 9 + 4), answer);
    }
}
#[test]
fn test_sente_fu_none_control() {
    let positions = [(3,5),(5,5),(4,6),(3,4)];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for &(x,y) in positions.iter() {
        let mut banmen = blank_banmen.clone();

        banmen.0[y][x] = SFu;

        assert_eq!(Rule::control_count(Teban::Sente,&State::new(banmen),4 * 9 + 4),0);
    }
}
#[test]
fn test_sente_kyou_none_control() {
    let positions = [(3,8),(5,8),(4,0)];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for &(x,y) in positions.iter() {
        let mut banmen = blank_banmen.clone();

        banmen.0[y][x] = SKyou;

        assert_eq!(Rule::control_count(Teban::Sente,&State::new(banmen),4 * 9 + 4),0);
    }
}
#[test]
fn test_sente_kei_none_control() {
    let positions = [vec![(3,5),(5,5),(3,3),(5,3)]];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for it in positions.iter() {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[y][x] = SKei;
        }

        assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), 4 * 9 + 4), 0);
    }
}
#[test]
fn test_sente_gin_none_control() {
    let positions = [vec![(4,6),(2,2),(6,3)],vec![(3,4),(5,4),(4,3)]];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for it in positions.iter() {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[y][x] = SGin;
        }

        assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), 4 * 9 + 4), 0);
    }
}
#[test]
fn test_sente_kin_none_control() {
    let positions = [vec![(4,6),(2,4),(6,4)],vec![(4,2),(2,2),(6,2)],vec![(3,3),(5,3)]];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for it in positions.iter() {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[y][x] = SKin;
        }

        assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), 4 * 9 + 4), 0);
    }
}
#[test]
fn test_sente_nari_kin_none_control() {
    let positions = [vec![(4,6),(2,4),(6,4)],vec![(4,2),(2,2),(6,2)],vec![(3,3),(5,3)]];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for &kind in [SFuN,SKyouN,SKeiN,SGinN].iter() {
        for it in positions.iter() {
            let mut banmen = blank_banmen.clone();

            for &(x, y) in it.iter() {
                banmen.0[y][x] = kind;
            }

            assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), 4 * 9 + 4), 0);
        }
    }
}
#[test]
fn test_sente_kaku_none_control() {
    let positions = [vec![(4,5),(4,3)],vec![(3,4),(5,4)],vec![(0,1),(7,0),(1,8),(8,7)]];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for it in positions.iter() {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[y][x] = SKaku;
        }

        assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), 4 * 9 + 4), 0);
    }
}
#[test]
fn test_sente_hisha_none_control() {
    let positions = [vec![(3,5),(5,5)],vec![(3,3),(5,3)],vec![(0,5),(0,3)],vec![(8,5),(8,3)],vec![(3,0),(5,0)],vec![(3,8),(5,8)]];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for it in positions.iter() {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[y][x] = SHisha;
        }

        assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), 4 * 9 + 4), 0);
    }
}
#[test]
fn test_sente_ou_none_control() {
    let positions = [
        vec![(2,2),(4,6),(6,6)],
        vec![(2,4),(6,4)],
        vec![(4,2)],
        vec![(2,2),(6,2)],
        vec![(2,6)],
        vec![(3,6)],
        vec![(4,6)],
        vec![(5,6)],
        vec![(6,6)],
        vec![(2,5)],
        vec![(2,4)],
        vec![(2,3)],
        vec![(6,5)],
        vec![(6,4)],
        vec![(6,3)],
        vec![(2,2)],
        vec![(3,2)],
        vec![(5,2)],
        vec![(6,2)]
    ];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for it in positions.iter() {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[y][x] = SOu;
        }

        assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), 4 * 9 + 4), 0);
    }
}
#[test]
fn test_sente_kaku_nari_none_control() {
    let positions = [vec![(2,4),(6,4),(4,2),(4,6)],vec![(0,1),(1,8)],vec![(8,1),(7,8)]];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for it in positions.iter() {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[y][x] = SKakuN;
        }

        assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), 4 * 9 + 4), 0);
    }
}
#[test]
fn test_sente_hisha_nari_none_control() {
    let positions = [vec![(2,6),(6,6),(2,2),(6,2)],vec![(0,3),(5,0)],vec![(8,5),(3,8)]];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for it in positions.iter() {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[y][x] = SHishaN;
        }

        assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), 4 * 9 + 4), 0);
    }
}
#[test]
fn test_gote_fu_none_control() {
    let positions = [(3,5),(5,5),(4,6),(3,4)];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for &(x,y) in positions.iter() {
        let mut banmen = blank_banmen.clone();

        banmen.0[8-y][8-x] = GFu;

        assert_eq!(Rule::control_count(Teban::Gote,&State::new(banmen),4 * 9 + 4),0);
    }
}
#[test]
fn test_gote_kyou_none_control() {
    let positions = [(3,8),(5,8),(4,0)];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for &(x,y) in positions.iter() {
        let mut banmen = blank_banmen.clone();

        banmen.0[8-y][8-x] = GKyou;

        assert_eq!(Rule::control_count(Teban::Gote,&State::new(banmen),4 * 9 + 4),0);
    }
}
#[test]
fn test_gote_kei_none_control() {
    let positions = [vec![(3,5),(5,5),(3,3),(5,3)]];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for it in positions.iter() {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[8-y][8-x] = GKei;
        }

        assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), 4 * 9 + 4), 0);
    }
}
#[test]
fn test_gote_gin_none_control() {
    let positions = [vec![(4,6),(2,2),(6,3)],vec![(3,4),(5,4),(4,3)]];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for it in positions.iter() {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[8-y][8-x] = SGin;
        }

        assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), 4 * 9 + 4), 0);
    }
}
#[test]
fn test_gote_kin_none_control() {
    let positions = [vec![(4,6),(2,4),(6,4)],vec![(4,2),(2,2),(6,2)],vec![(3,3),(5,3)]];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for it in positions.iter() {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[8-y][8-x] = GKin;
        }

        assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), 4 * 9 + 4), 0);
    }
}
#[test]
fn test_gote_nari_kin_none_control() {
    let positions = [vec![(4,6),(2,4),(6,4)],vec![(4,2),(2,2),(6,2)],vec![(3,3),(5,3)]];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for &kind in [GFuN,GKyouN,GKeiN,GGinN].iter() {
        for it in positions.iter() {
            let mut banmen = blank_banmen.clone();

            for &(x, y) in it.iter() {
                banmen.0[8-y][8-x] = kind;
            }

            assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), 4 * 9 + 4), 0);
        }
    }
}
#[test]
fn test_gote_kaku_none_control() {
    let positions = [vec![(4,5),(4,3)],vec![(3,4),(5,4)],vec![(0,1),(7,0),(1,8),(8,7)]];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for it in positions.iter() {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[8-y][8-x] = GKaku;
        }

        assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), 4 * 9 + 4), 0);
    }
}
#[test]
fn test_gote_hisha_none_control() {
    let positions = [vec![(3,5),(5,5)],vec![(3,3),(5,3)],vec![(0,5),(0,3)],vec![(8,5),(8,3)],vec![(3,0),(5,0)],vec![(3,8),(5,8)]];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for it in positions.iter() {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[y][x] = GHisha;
        }

        assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), 4 * 9 + 4), 0);
    }
}
#[test]
fn test_gote_ou_none_control() {
    let positions = [
        vec![(2,2),(4,6),(6,6)],
        vec![(2,4),(6,4)],
        vec![(4,2)],
        vec![(2,2),(6,2)],
        vec![(2,6)],
        vec![(3,6)],
        vec![(4,6)],
        vec![(5,6)],
        vec![(6,6)],
        vec![(2,5)],
        vec![(2,4)],
        vec![(2,3)],
        vec![(6,5)],
        vec![(6,4)],
        vec![(6,3)],
        vec![(2,2)],
        vec![(3,2)],
        vec![(5,2)],
        vec![(6,2)]
    ];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for it in positions.iter() {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[8-y][8-x] = GOu;
        }

        assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), 4 * 9 + 4), 0);
    }
}
#[test]
fn test_gote_kaku_nari_none_control() {
    let positions = [vec![(2,4),(6,4),(4,2),(4,6)],vec![(0,1),(1,8)],vec![(8,1),(7,8)]];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for it in positions.iter() {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[8-y][8-x] = GKakuN;
        }

        assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), 4 * 9 + 4), 0);
    }
}
#[test]
fn test_gote_hisha_nari_none_control() {
    let positions = [vec![(2,6),(6,6),(2,2),(6,2)],vec![(0,3),(5,0)],vec![(8,5),(3,8)]];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for it in positions.iter() {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[8-y][8-x] = GHishaN;
        }

        assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), 4 * 9 + 4), 0);
    }
}
#[test]
fn test_sente_fu_mix() {
    let positions = [vec![(4,5),(3,5),(5,5)],vec![(4,6)]];
    let answers = [1,0];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for (it,&answer) in positions.iter().zip(answers.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[y][x] = SFu;
        }

        assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), 4 * 9 + 4), answer);
    }
}
#[test]
fn test_sente_kyou_mix() {
    let positions = [vec![(4,8),(3,8),(5,8)],vec![(4,8),(4,3)]];
    let answers = [1,1];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for (it,&answer) in positions.iter().zip(answers.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[y][x] = SKyou;
        }

        assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), 4 * 9 + 4), answer);
    }
}
#[test]
fn test_sente_kei_mix() {
    let positions = [vec![(3,6),(5,6),(3,7),(5,7)],vec![(3,6),(5,6),(3,5),(5,5)]];
    let answers = [2,2];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for (it,&answer) in positions.iter().zip(answers.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[y][x] = SKei;
        }

        assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), 4 * 9 + 4), answer);
    }
}
#[test]
fn test_sente_gin_mix() {
    let positions = [vec![(4,5),(3,3),(5,3),(3,4),(5,4)],vec![(3,5),(5,5),(4,3)]];
    let answers = [3,2];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for (it,&answer) in positions.iter().zip(answers.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[y][x] = SGin;
        }

        assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), 4 * 9 + 4), answer);
    }
}
#[test]
fn test_sente_kin_mix() {
    let positions = [vec![(4,5),(3,5),(5,5),(4,6),(3,3),(5,3)],vec![(3,4),(5,4),(4,2)],vec![(4,3),(4,2)]];
    let answers = [3,2,1];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for (it,&answer) in positions.iter().zip(answers.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[y][x] = SKin;
        }

        assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), 4 * 9 + 4), answer);
    }
}
#[test]
fn test_sente_nari_kin_mix() {
    let positions = [vec![(4,5),(3,5),(5,5),(4,6),(3,3),(5,3)],vec![(3,4),(5,4),(4,2)],vec![(4,3),(4,2)]];
    let answers = [3,2,1];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for &kind in [SFuN,SKyouN,SKeiN,SGinN].iter() {
        for (it, &answer) in positions.iter().zip(answers.iter()) {
            let mut banmen = blank_banmen.clone();

            for &(x, y) in it.iter() {
                banmen.0[y][x] = kind;
            }

            assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), 4 * 9 + 4), answer);
        }
    }
}
#[test]
fn test_sente_kaku_mix() {
    let positions = [vec![(3,3),(5,3),(3,5),(5,5),(2,3),(6,3),(4,5),(5,6)],vec![(0,0),(0,8),(0,4),(8,4)],vec![(8,0),(8,8),(4,0),(4,8)]];
    let answers = [4,2,2];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for (it,&answer) in positions.iter().zip(answers.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[y][x] = SKaku;
        }

        assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), 4 * 9 + 4), answer);
    }
}
#[test]
fn test_sente_hisha_mix() {
    let positions = [vec![(3,4),(5,4),(4,3),(4,5),(3,3),(5,3),(3,5),(5,5)],vec![(0,4),(4,0),(3,3),(5,3),(3,5),(5,5)],vec![(8,4),(4,8),(3,3),(5,3),(3,5),(5,5)]];
    let answers = [4,2,2];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for (it,&answer) in positions.iter().zip(answers.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[y][x] = SHisha;
        }

        assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), 4 * 9 + 4), answer);
    }
}
#[test]
fn test_sente_ou_has_mix() {
    let positions = [vec![(4,5),(3,5),(5,5),(4,6),(3,6),(5,6)],vec![(3,4),(5,4),(2,4),(6,4)],vec![(4,3),(4,2)],vec![(3,3),(5,3),(2,2),(6,2)]];
    let answers = [3,2,1,2];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for (it,&answer) in positions.iter().zip(answers.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[y][x] = SOu;
        }

        assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), 4 * 9 + 4), answer);
    }
}
#[test]
fn test_sente_kaku_nari_mix() {
    let positions = [
        vec![(3,3),(5,3),(3,5),(5,5),(2,4),(6,4)],
        vec![(0,0),(0,8),(0,4),(4,0)],
        vec![(8,0),(8,8),(8,4),(8,0)],
        vec![(3,4),(4,3),(5,4),(4,5),(2,4),(4,2),(6,4),(4,6)]
    ];
    let answers = [4,2,2,4];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for (it,&answer) in positions.iter().zip(answers.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[y][x] = SKakuN;
        }

        assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), 4 * 9 + 4), answer);
    }
}
#[test]
fn test_sente_hisha_nari_mix() {
    let positions = [
        vec![(3,4),(5,4),(4,3),(4,5),(2,6),(6,6),(2,2),(6,2)],
        vec![(0,4),(4,0),(0,5),(5,0)],
        vec![(8,4),(4,8),(8,5),(5,8)],
        vec![(3,5),(3,3),(5,3),(5,5),(2,6),(2,2),(6,2),(6,6)]
    ];
    let answers = [4,2,2,4];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for (it,&answer) in positions.iter().zip(answers.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[y][x] = SHishaN;
        }

        assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), 4 * 9 + 4), answer);
    }
}
#[test]
fn test_gote_fu_mix() {
    let positions = [vec![(4,5),(3,5),(5,5)],vec![(4,6)]];
    let answers = [1,0];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for (it,&answer) in positions.iter().zip(answers.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[8-y][8-x] = GFu;
        }

        assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), 4 * 9 + 4), answer);
    }
}
#[test]
fn test_gote_kyou_mix() {
    let positions = [vec![(4,8),(3,8),(5,8)],vec![(4,8),(4,3)]];
    let answers = [1,1];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for (it,&answer) in positions.iter().zip(answers.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[8-y][8-x] = GKyou;
        }

        assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), 4 * 9 + 4), answer);
    }
}
#[test]
fn test_gote_kei_mix() {
    let positions = [vec![(3,6),(5,6),(3,7),(5,7)],vec![(3,6),(5,6),(3,5),(5,5)]];
    let answers = [2,2];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for (it,&answer) in positions.iter().zip(answers.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[8-y][8-x] = GKei;
        }

        assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), 4 * 9 + 4), answer);
    }
}
#[test]
fn test_gote_gin_mix() {
    let positions = [vec![(4,5),(3,3),(5,3),(3,4),(5,4)],vec![(3,5),(5,5),(4,3)]];
    let answers = [3,2];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for (it,&answer) in positions.iter().zip(answers.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[8-y][8-x] = GGin;
        }

        assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), 4 * 9 + 4), answer);
    }
}
#[test]
fn test_gote_kin_mix() {
    let positions = [vec![(4,5),(3,5),(5,5),(4,6),(3,3),(5,3)],vec![(3,4),(5,4),(4,2)],vec![(4,3),(4,2)]];
    let answers = [3,2,1];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for (it,&answer) in positions.iter().zip(answers.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[8-y][8-x] = SKin;
        }

        assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), 4 * 9 + 4), answer);
    }
}
#[test]
fn test_gote_nari_kin_mix() {
    let positions = [vec![(4,5),(3,5),(5,5),(4,6),(3,3),(5,3)],vec![(3,4),(5,4),(4,2)],vec![(4,3),(4,2)]];
    let answers = [3,2,1];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for &kind in [GFuN,GKyouN,GKeiN,GGinN].iter() {
        for (it, &answer) in positions.iter().zip(answers.iter()) {
            let mut banmen = blank_banmen.clone();

            for &(x, y) in it.iter() {
                banmen.0[8-y][8-x] = kind;
            }

            assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), 4 * 9 + 4), answer);
        }
    }
}
#[test]
fn test_gote_kaku_mix() {
    let positions = [vec![(3,3),(5,3),(3,5),(5,5),(2,3),(6,3),(4,5),(5,6)],vec![(0,0),(0,8),(0,4),(8,4)],vec![(8,0),(8,8),(4,0),(4,8)]];
    let answers = [4,2,2];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for (it,&answer) in positions.iter().zip(answers.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[8-y][8-x] = GKaku;
        }

        assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), 4 * 9 + 4), answer);
    }
}
#[test]
fn test_gote_hisha_mix() {
    let positions = [vec![(3,4),(5,4),(4,3),(4,5),(3,3),(5,3),(3,5),(5,5)],vec![(0,4),(4,0),(3,3),(5,3),(3,5),(5,5)],vec![(8,4),(4,8),(3,3),(5,3),(3,5),(5,5)]];
    let answers = [4,2,2];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for (it,&answer) in positions.iter().zip(answers.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[8-y][8-x] = GHisha;
        }

        assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), 4 * 9 + 4), answer);
    }
}
#[test]
fn test_gote_ou_has_mix() {
    let positions = [vec![(4,5),(3,5),(5,5),(4,6),(3,6),(5,6)],vec![(3,4),(5,4),(2,4),(6,4)],vec![(4,3),(4,2)],vec![(3,3),(5,3),(2,2),(6,2)]];
    let answers = [3,2,1,2];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for (it,&answer) in positions.iter().zip(answers.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[y][x] = GOu;
        }

        assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), 4 * 9 + 4), answer);
    }
}
#[test]
fn test_gote_kaku_nari_mix() {
    let positions = [
        vec![(3,3),(5,3),(3,5),(5,5),(2,4),(6,4)],
        vec![(0,0),(0,8),(0,4),(4,0)],
        vec![(8,0),(8,8),(8,4),(8,0)],
        vec![(3,4),(4,3),(5,4),(4,5),(2,4),(4,2),(6,4),(4,6)]
    ];
    let answers = [4,2,2,4];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for (it,&answer) in positions.iter().zip(answers.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[8-y][8-x] = GKakuN;
        }

        assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), 4 * 9 + 4), answer);
    }
}
#[test]
fn test_gote_hisha_nari_mix() {
    let positions = [
        vec![(3,4),(5,4),(4,3),(4,5),(2,6),(6,6),(2,2),(6,2)],
        vec![(0,4),(4,0),(0,5),(5,0)],
        vec![(8,4),(4,8),(8,5),(5,8)],
        vec![(3,5),(3,3),(5,3),(5,5),(2,6),(2,2),(6,2),(6,6)]
    ];
    let answers = [4,2,2,4];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for (it,&answer) in positions.iter().zip(answers.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[8-y][8-x] = GHishaN;
        }

        assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), 4 * 9 + 4), answer);
    }
}
#[test]
fn test_sente_kyou_has_control_with_target_positions() {
    let positions = [vec![(3,8)],vec![(4,8)],vec![(5,8)]];
    let target_positions = [(3,4),(4,4),(5,4)];
    let answers = [1,1,1];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for ((it,&answer),&(tx,ty)) in positions.iter().zip(answers.iter()).zip(target_positions.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[y][x] = SKyou;
        }

        assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), tx * 9 + ty), answer);
    }
}
#[test]
fn test_sente_kaku_has_control_with_target_positions() {
    let positions = [
        vec![(1,0),(7,0)],
        vec![(0,1),(0,7)],
        vec![(1,8),(7,8)],
        vec![(8,1),(8,7)]
    ];
    let target_positions = [(4,3),(3,4),(4,5),(5,4)];
    let answers = [2,2,2,2];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for ((it,&answer),&(tx,ty)) in positions.iter().zip(answers.iter()).zip(target_positions.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[y][x] = SKaku;
        }

        assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), tx * 9 + ty), answer);
    }
}
#[test]
fn test_sente_hisha_has_control_with_target_positions() {
    let positions = [
        vec![(0,3),(3,0)],
        vec![(0,5),(3,0)],
        vec![(5,8),(8,5)],
        vec![(5,0),(8,3)]
    ];
    let target_positions = [(3,3),(3,5),(5,5),(5,3)];
    let answers = [2,2,2,2];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for ((it,&answer),&(tx,ty)) in positions.iter().zip(answers.iter()).zip(target_positions.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[y][x] = SHisha;
        }

        assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), tx * 9 + ty), answer);
    }
}
#[test]
fn test_sente_kyou_mix_with_target_positions() {
    let positions = [vec![(3,8),(4,8),(5,8)],vec![(3,8),(4,8),(5,8)],vec![(3,8),(4,8),(5,8)]];
    let target_positions = [(3,4),(4,4),(5,4)];
    let answers = [1,1,1];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for ((it,&answer),&(tx,ty)) in positions.iter().zip(answers.iter()).zip(target_positions.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[y][x] = SKyou;
        }

        assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), tx * 9 + ty), answer);
    }
}
#[test]
fn test_sente_kaku_mix_with_target_positions() {
    let positions = [
        vec![(1,0),(7,0),(1,8),(7,8)],
        vec![(0,1),(0,7),(8,1),(8,7)],
        vec![(1,8),(7,8),(1,0),(7,0)],
        vec![(8,1),(8,7),(0,1),(0,7)]
    ];
    let target_positions = [(4,3),(3,4),(4,5),(5,4)];
    let answers = [2,2,2,2];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for ((it,&answer),&(tx,ty)) in positions.iter().zip(answers.iter()).zip(target_positions.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[y][x] = SKaku;
        }

        assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), tx * 9 + ty), answer);
    }
}
#[test]
fn test_sente_hisha_mix_with_target_positions() {
    let positions = [
        vec![(0,3),(3,0),(5,8),(8,5)],
        vec![(0,5),(3,0),(5,0),(8,3)],
        vec![(5,8),(8,5),(0,3),(3,0)],
        vec![(5,0),(8,3),(0,5),(3,0)]
    ];
    let target_positions = [(3,3),(3,5),(5,5),(5,3)];
    let answers = [2,2,2,2];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for ((it,&answer),&(tx,ty)) in positions.iter().zip(answers.iter()).zip(target_positions.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[y][x] = SHisha;
        }

        assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), tx * 9 + ty), answer);
    }
}
#[test]
fn test_gote_kyou_has_control_with_target_positions() {
    let positions = [vec![(3,8)],vec![(4,8)],vec![(5,8)]];
    let target_positions = [(3,4),(4,4),(5,4)];
    let answers = [1,1,1];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for ((it,&answer),&(tx,ty)) in positions.iter().zip(answers.iter()).zip(target_positions.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[8-y][8-x] = GKyou;
        }

        let tx = 8 - tx;
        let ty = 8 - ty;

        assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), tx * 9 + ty), answer);
    }
}
#[test]
fn test_gote_kaku_has_control_with_target_positions() {
    let positions = [
        vec![(1,0),(7,0)],
        vec![(0,1),(0,7)],
        vec![(1,8),(7,8)],
        vec![(8,1),(8,7)]
    ];
    let target_positions = [(4,3),(3,4),(4,5),(5,4)];
    let answers = [2,2,2,2];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for ((it,&answer),&(tx,ty)) in positions.iter().zip(answers.iter()).zip(target_positions.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[8-y][8-x] = GKaku;
        }

        let tx = 8 - tx;
        let ty = 8 - ty;

        assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), tx * 9 + ty), answer);
    }
}
#[test]
fn test_gote_hisha_has_control_with_target_positions() {
    let positions = [
        vec![(0,3),(3,0)],
        vec![(0,5),(3,0)],
        vec![(5,8),(8,5)],
        vec![(5,0),(8,3)]
    ];
    let target_positions = [(3,3),(3,5),(5,5),(5,3)];
    let answers = [2,2,2,2];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for ((it,&answer),&(tx,ty)) in positions.iter().zip(answers.iter()).zip(target_positions.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[8-y][8-x] = GHisha;
        }

        let tx = 8 - tx;
        let ty = 8 - ty;

        assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), tx * 9 + ty), answer);
    }
}
#[test]
fn test_gote_kyou_mix_with_target_positions() {
    let positions = [vec![(3,8),(4,8),(5,8)],vec![(3,8),(4,8),(5,8)],vec![(3,8),(4,8),(5,8)]];
    let target_positions = [(3,4),(4,4),(5,4)];
    let answers = [1,1,1];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for ((it,&answer),&(tx,ty)) in positions.iter().zip(answers.iter()).zip(target_positions.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[8-y][8-x] = GKyou;
        }

        let tx = 8 - tx;
        let ty = 8 - ty;

        assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), tx * 9 + ty), answer);
    }
}
#[test]
fn test_gote_kaku_mix_with_target_positions() {
    let positions = [
        vec![(1,0),(7,0),(1,8),(7,8)],
        vec![(0,1),(0,7),(8,1),(8,7)],
        vec![(1,8),(7,8),(1,0),(7,0)],
        vec![(8,1),(8,7),(0,1),(0,7)]
    ];
    let target_positions = [(4,3),(3,4),(4,5),(5,4)];
    let answers = [2,2,2,2];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for ((it,&answer),&(tx,ty)) in positions.iter().zip(answers.iter()).zip(target_positions.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[8-y][8-x] = GKaku;
        }

        let tx = 8 - tx;
        let ty = 8 - ty;

        assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), tx * 9 + ty), answer);
    }
}
#[test]
fn test_gote_hisha_mix_with_target_positions() {
    let positions = [
        vec![(0,3),(3,0),(5,8),(8,5)],
        vec![(0,5),(3,0),(5,0),(8,3)],
        vec![(5,8),(8,5),(0,3),(3,0)],
        vec![(5,0),(8,3),(0,5),(3,0)]
    ];
    let target_positions = [(3,3),(3,5),(5,5),(5,3)];
    let answers = [2,2,2,2];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for ((it,&answer),&(tx,ty)) in positions.iter().zip(answers.iter()).zip(target_positions.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[8-y][8-x] = GHisha;
        }

        let tx = 8 - tx;
        let ty = 8 - ty;

        assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), tx * 9 + ty), answer);
    }
}
#[test]
fn test_sente_kyou_none_control_occupied_self() {
    let positions = [vec![(3,8)],vec![(5,8)]];
    let occ_positions = [vec![(3,7)],vec![(5,7)]];
    let target_positions = [(3,4),(5,4)];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for ((it,oit),&(tx,ty)) in positions.iter()
                                                                            .zip(occ_positions.iter())
                                                                            .zip(target_positions.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[y][x] = SKyou;
        }

        for &(ox,oy) in oit.iter() {
            banmen.0[oy][ox] = SFu;
        }

        assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), tx * 9 + ty), 0);
    }
}
#[test]
fn test_sente_kaku_none_control_occupied_self() {
    let positions = [vec![(1,0),(7,0)],vec![(0,1),(0,7)],vec![(8,1),(7,8)],vec![(8,1),(8,7)]];
    let occ_positions = [vec![(2,1),(6,1)],vec![(1,2),(1,6)],vec![(7,2),(6,7)],vec![(7,2),(7,6)]];
    let target_positions = [(4,3),(3,4),(4,5),(5,4)];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for ((it,oit),&(tx,ty)) in positions.iter()
        .zip(occ_positions.iter())
        .zip(target_positions.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[y][x] = SKaku;
        }

        for &(ox,oy) in oit.iter() {
            banmen.0[oy][ox] = SFu;
        }

        assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), tx * 9 + ty), 0);
    }
}
#[test]
fn test_sente_hisha_none_control_occupied_self() {
    let positions = [vec![(0,3),(3,0)],vec![(0,5),(3,8)],vec![(5,8),(8,5)],vec![(5,0),(8,3)]];
    let occ_positions = [vec![(1,3),(3,1)],vec![(1,5),(3,7)],vec![(5,7),(7,5)],vec![(5,1),(7,3)]];
    let target_positions = [(3,3),(3,5),(5,5),(5,3)];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for ((it,oit),&(tx,ty)) in positions.iter()
        .zip(occ_positions.iter())
        .zip(target_positions.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[y][x] = SHisha;
        }

        for &(ox,oy) in oit.iter() {
            banmen.0[oy][ox] = SFu;
        }

        assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), tx * 9 + ty), 0);
    }
}
#[test]
fn test_gote_kyou_none_control_occupied_self() {
    let positions = [vec![(3,8)],vec![(5,8)]];
    let occ_positions = [vec![(3,7)],vec![(5,7)]];
    let target_positions = [(3,4),(5,4)];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for ((it,oit),&(tx,ty)) in positions.iter()
        .zip(occ_positions.iter())
        .zip(target_positions.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[8-y][8-x] = GKyou;
        }

        for &(ox,oy) in oit.iter() {
            banmen.0[8-oy][8-ox] = GFu;
        }

        let tx = 8 - tx;
        let ty = 8 - ty;

        assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), tx * 9 + ty), 0);
    }
}
#[test]
fn test_gote_kaku_none_control_occupied_self() {
    let positions = [vec![(1,0),(7,0)],vec![(0,1),(0,7)],vec![(8,1),(7,8)],vec![(8,1),(8,7)]];
    let occ_positions = [vec![(2,1),(6,1)],vec![(1,2),(1,6)],vec![(7,2),(6,7)],vec![(7,2),(7,6)]];
    let target_positions = [(4,3),(3,4),(4,5),(5,4)];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for ((it,oit),&(tx,ty)) in positions.iter()
        .zip(occ_positions.iter())
        .zip(target_positions.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[8-y][8-x] = GKaku;
        }

        for &(ox,oy) in oit.iter() {
            banmen.0[8-oy][8-ox] = GFu;
        }

        let tx = 8 - tx;
        let ty = 8 - ty;

        assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), tx * 9 + ty), 0);
    }
}
#[test]
fn test_gote_hisha_none_control_occupied_self() {
    let positions = [vec![(0,3),(3,0)],vec![(0,5),(3,8)],vec![(5,8),(8,5)],vec![(5,0),(8,3)]];
    let occ_positions = [vec![(1,3),(3,1)],vec![(1,5),(3,7)],vec![(5,7),(7,5)],vec![(5,1),(7,3)]];
    let target_positions = [(3,3),(3,5),(5,5),(5,3)];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for ((it,oit),&(tx,ty)) in positions.iter()
        .zip(occ_positions.iter())
        .zip(target_positions.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[8-y][8-x] = GHisha;
        }

        for &(ox,oy) in oit.iter() {
            banmen.0[8-oy][8-ox] = GFu;
        }

        let tx = 8 - tx;
        let ty = 8 - ty;

        assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), tx * 9 + ty), 0);
    }
}
#[test]
fn test_sente_kyou_none_control_occupied_opponent() {
    let positions = [vec![(3,8)],vec![(5,8)]];
    let occ_positions = [vec![(3,7)],vec![(5,7)]];
    let target_positions = [(3,4),(5,4)];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for ((it,oit),&(tx,ty)) in positions.iter()
        .zip(occ_positions.iter())
        .zip(target_positions.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[y][x] = SKyou;
        }

        for &(ox,oy) in oit.iter() {
            banmen.0[oy][ox] = GFu;
        }

        assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), tx * 9 + ty), 0);
    }
}
#[test]
fn test_sente_kaku_none_control_occupied_opponent() {
    let positions = [vec![(1,0),(7,0)],vec![(0,1),(0,7)],vec![(8,1),(7,8)],vec![(8,1),(8,7)]];
    let occ_positions = [vec![(2,1),(6,1)],vec![(1,2),(1,6)],vec![(7,2),(6,7)],vec![(7,2),(7,6)]];
    let target_positions = [(4,3),(3,4),(4,5),(5,4)];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for ((it,oit),&(tx,ty)) in positions.iter()
        .zip(occ_positions.iter())
        .zip(target_positions.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[y][x] = SKaku;
        }

        for &(ox,oy) in oit.iter() {
            banmen.0[oy][ox] = GFu;
        }

        assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), tx * 9 + ty), 0);
    }
}
#[test]
fn test_sente_hisha_none_control_occupied_opponent() {
    let positions = [vec![(0,3),(3,0)],vec![(0,5),(3,8)],vec![(5,8),(8,5)],vec![(5,0),(8,3)]];
    let occ_positions = [vec![(1,3),(3,1)],vec![(1,5),(3,7)],vec![(5,7),(7,5)],vec![(5,1),(7,3)]];
    let target_positions = [(3,3),(3,5),(5,5),(5,3)];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for ((it,oit),&(tx,ty)) in positions.iter()
        .zip(occ_positions.iter())
        .zip(target_positions.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[y][x] = SHisha;
        }

        for &(ox,oy) in oit.iter() {
            banmen.0[oy][ox] = GFu;
        }

        assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), tx * 9 + ty), 0);
    }
}
#[test]
fn test_gote_kyou_none_control_occupied_opponent() {
    let positions = [vec![(3,8)],vec![(5,8)]];
    let occ_positions = [vec![(3,7)],vec![(5,7)]];
    let target_positions = [(3,4),(5,4)];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for ((it,oit),&(tx,ty)) in positions.iter()
        .zip(occ_positions.iter())
        .zip(target_positions.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[8-y][8-x] = GKyou;
        }

        for &(ox,oy) in oit.iter() {
            banmen.0[8-oy][8-ox] = SFu;
        }

        let tx = 8 - tx;
        let ty = 8 - ty;

        assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), tx * 9 + ty), 0);
    }
}
#[test]
fn test_gote_kaku_none_control_occupied_opponent() {
    let positions = [vec![(1,0),(7,0)],vec![(0,1),(0,7)],vec![(8,1),(7,8)],vec![(8,1),(8,7)]];
    let occ_positions = [vec![(2,1),(6,1)],vec![(1,2),(1,6)],vec![(7,2),(6,7)],vec![(7,2),(7,6)]];
    let target_positions = [(4,3),(3,4),(4,5),(5,4)];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for ((it,oit),&(tx,ty)) in positions.iter()
        .zip(occ_positions.iter())
        .zip(target_positions.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[8-y][8-x] = GKaku;
        }

        for &(ox,oy) in oit.iter() {
            banmen.0[8-oy][8-ox] = SFu;
        }

        let tx = 8 - tx;
        let ty = 8 - ty;

        assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), tx * 9 + ty), 0);
    }
}
#[test]
fn test_gote_hisha_none_control_occupied_opponent() {
    let positions = [vec![(0,3),(3,0)],vec![(0,5),(3,8)],vec![(5,8),(8,5)],vec![(5,0),(8,3)]];
    let occ_positions = [vec![(1,3),(3,1)],vec![(1,5),(3,7)],vec![(5,7),(7,5)],vec![(5,1),(7,3)]];
    let target_positions = [(3,3),(3,5),(5,5),(5,3)];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for ((it,oit),&(tx,ty)) in positions.iter()
        .zip(occ_positions.iter())
        .zip(target_positions.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[8-y][8-x] = GHisha;
        }

        for &(ox,oy) in oit.iter() {
            banmen.0[8-oy][8-ox] = SFu;
        }

        let tx = 8 - tx;
        let ty = 8 - ty;

        assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), tx * 9 + ty), 0);
    }
}
#[test]
fn test_sente_kyou_has_control_none_occupied_self() {
    let positions = [vec![(3,8)],vec![(5,8)]];
    let occ_positions = [vec![(3,3)],vec![(5,3)]];
    let target_positions = [(3,4),(5,4)];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for ((it,oit),&(tx,ty)) in positions.iter()
        .zip(occ_positions.iter())
        .zip(target_positions.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[y][x] = SKyou;
        }

        for &(ox,oy) in oit.iter() {
            banmen.0[oy][ox] = SFu;
        }

        assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), tx * 9 + ty), 1);
    }
}
#[test]
fn test_sente_kaku_has_control_none_occupied_self() {
    let positions = [vec![(2,1),(6,1)],vec![(1,2),(1,6)],vec![(7,2),(6,7)],vec![(7,2),(7,6)]];
    let occ_positions = [vec![(1,0),(7,0)],vec![(0,1),(0,7)],vec![(8,1),(7,8)],vec![(8,1),(8,7)]];
    let target_positions = [(4,3),(3,4),(4,5),(5,4)];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for ((it,oit),&(tx,ty)) in positions.iter()
        .zip(occ_positions.iter())
        .zip(target_positions.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[y][x] = SKaku;
        }

        for &(ox,oy) in oit.iter() {
            banmen.0[oy][ox] = SFu;
        }

        assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), tx * 9 + ty), 2);
    }
}
#[test]
fn test_sente_hisha_has_control_none_occupied_self() {
    let positions = [vec![(1,3),(3,1)],vec![(1,5),(3,7)],vec![(5,7),(7,5)],vec![(5,1),(7,3)]];
    let occ_positions = [vec![(0,3),(3,0)],vec![(0,5),(3,8)],vec![(5,8),(8,5)],vec![(5,0),(8,3)]];
    let target_positions = [(3,3),(3,5),(5,5),(5,3)];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for ((it,oit),&(tx,ty)) in positions.iter()
        .zip(occ_positions.iter())
        .zip(target_positions.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[y][x] = SHisha;
        }

        for &(ox,oy) in oit.iter() {
            banmen.0[oy][ox] = SFu;
        }

        assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), tx * 9 + ty), 2);
    }
}
#[test]
fn test_gote_kyou_has_control_none_occupied_self() {
    let positions = [vec![(3,8)],vec![(5,8)]];
    let occ_positions = [vec![(3,3)],vec![(5,3)]];
    let target_positions = [(3,4),(5,4)];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for ((it,oit),&(tx,ty)) in positions.iter()
        .zip(occ_positions.iter())
        .zip(target_positions.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[8-y][8-x] = GKyou;
        }

        for &(ox,oy) in oit.iter() {
            banmen.0[8-oy][8-ox] = GFu;
        }

        let tx = 8 - tx;
        let ty = 8 - ty;

        assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), tx * 9 + ty), 1);
    }
}
#[test]
fn test_gote_kaku_has_control_none_occupied_self() {
    let positions = [vec![(2,1),(6,1)],vec![(1,2),(1,6)],vec![(7,2),(6,7)],vec![(7,2),(7,6)]];
    let occ_positions = [vec![(1,0),(7,0)],vec![(0,1),(0,7)],vec![(8,1),(7,8)],vec![(8,1),(8,7)]];
    let target_positions = [(4,3),(3,4),(4,5),(5,4)];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for ((it,oit),&(tx,ty)) in positions.iter()
        .zip(occ_positions.iter())
        .zip(target_positions.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[8-y][8-x] = GKaku;
        }

        for &(ox,oy) in oit.iter() {
            banmen.0[8-oy][8-ox] = GFu;
        }

        let tx = 8 - tx;
        let ty = 8 - ty;

        assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), tx * 9 + ty), 2);
    }
}
#[test]
fn test_gote_hisha_has_control_none_occupied_self() {
    let positions = [vec![(1,3),(3,1)],vec![(1,5),(3,7)],vec![(5,7),(7,5)],vec![(5,1),(7,3)]];
    let occ_positions = [vec![(0,3),(3,0)],vec![(0,5),(3,8)],vec![(5,8),(8,5)],vec![(5,0),(8,3)]];
    let target_positions = [(3,3),(3,5),(5,5),(5,3)];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for ((it,oit),&(tx,ty)) in positions.iter()
        .zip(occ_positions.iter())
        .zip(target_positions.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[8-y][8-x] = GHisha;
        }

        for &(ox,oy) in oit.iter() {
            banmen.0[8-oy][8-ox] = GFu;
        }

        let tx = 8 - tx;
        let ty = 8 - ty;

        assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), tx * 9 + ty), 2);
    }
}
#[test]
fn test_sente_kyou_has_control_none_occupied_opponent() {
    let positions = [vec![(3,8)],vec![(5,8)]];
    let occ_positions = [vec![(3,3)],vec![(5,3)]];
    let target_positions = [(3,4),(5,4)];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for ((it,oit),&(tx,ty)) in positions.iter()
        .zip(occ_positions.iter())
        .zip(target_positions.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[y][x] = SKyou;
        }

        for &(ox,oy) in oit.iter() {
            banmen.0[oy][ox] = GFu;
        }

        assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), tx * 9 + ty), 1);
    }
}
#[test]
fn test_sente_kaku_has_control_none_occupied_opponent() {
    let positions = [vec![(2,1),(6,1)],vec![(1,2),(1,6)],vec![(7,2),(6,7)],vec![(7,2),(7,6)]];
    let occ_positions = [vec![(1,0),(7,0)],vec![(0,1),(0,7)],vec![(8,1),(7,8)],vec![(8,1),(8,7)]];
    let target_positions = [(4,3),(3,4),(4,5),(5,4)];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for ((it,oit),&(tx,ty)) in positions.iter()
        .zip(occ_positions.iter())
        .zip(target_positions.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[y][x] = SKaku;
        }

        for &(ox,oy) in oit.iter() {
            banmen.0[oy][ox] = GFu;
        }

        assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), tx * 9 + ty), 2);
    }
}
#[test]
fn test_sente_hisha_has_control_none_occupied_opponent() {
    let positions = [vec![(1,3),(3,1)],vec![(1,5),(3,7)],vec![(5,7),(7,5)],vec![(5,1),(7,3)]];
    let occ_positions = [vec![(0,3),(3,0)],vec![(0,5),(3,8)],vec![(5,8),(8,5)],vec![(5,0),(8,3)]];
    let target_positions = [(3,3),(3,5),(5,5),(5,3)];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for ((it,oit),&(tx,ty)) in positions.iter()
        .zip(occ_positions.iter())
        .zip(target_positions.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[y][x] = SHisha;
        }

        for &(ox,oy) in oit.iter() {
            banmen.0[oy][ox] = GFu;
        }

        assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), tx * 9 + ty), 2);
    }
}
#[test]
fn test_gote_kyou_has_control_none_occupied_opponent() {
    let positions = [vec![(3,8)],vec![(5,8)]];
    let occ_positions = [vec![(3,3)],vec![(5,3)]];
    let target_positions = [(3,4),(5,4)];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for ((it,oit),&(tx,ty)) in positions.iter()
        .zip(occ_positions.iter())
        .zip(target_positions.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[8-y][8-x] = GKyou;
        }

        for &(ox,oy) in oit.iter() {
            banmen.0[8-oy][8-ox] = SFu;
        }

        let tx = 8 - tx;
        let ty = 8 - ty;

        assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), tx * 9 + ty), 1);
    }
}
#[test]
fn test_gote_kaku_has_control_none_occupied_opponent() {
    let positions = [vec![(2,1),(6,1)],vec![(1,2),(1,6)],vec![(7,2),(6,7)],vec![(7,2),(7,6)]];
    let occ_positions = [vec![(1,0),(7,0)],vec![(0,1),(0,7)],vec![(8,1),(7,8)],vec![(8,1),(8,7)]];
    let target_positions = [(4,3),(3,4),(4,5),(5,4)];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for ((it,oit),&(tx,ty)) in positions.iter()
        .zip(occ_positions.iter())
        .zip(target_positions.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[8-y][8-x] = GKaku;
        }

        for &(ox,oy) in oit.iter() {
            banmen.0[8-oy][8-ox] = SFu;
        }

        let tx = 8 - tx;
        let ty = 8 - ty;

        assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), tx * 9 + ty), 2);
    }
}
#[test]
fn test_gote_hisha_has_control_none_occupied_opponent() {
    let positions = [vec![(1,3),(3,1)],vec![(1,5),(3,7)],vec![(5,7),(7,5)],vec![(5,1),(7,3)]];
    let occ_positions = [vec![(0,3),(3,0)],vec![(0,5),(3,8)],vec![(5,8),(8,5)],vec![(5,0),(8,3)]];
    let target_positions = [(3,3),(3,5),(5,5),(5,3)];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for ((it,oit),&(tx,ty)) in positions.iter()
        .zip(occ_positions.iter())
        .zip(target_positions.iter()) {
        let mut banmen = blank_banmen.clone();

        for &(x,y) in it.iter() {
            banmen.0[8-y][8-x] = GHisha;
        }

        for &(ox,oy) in oit.iter() {
            banmen.0[8-oy][8-ox] = SFu;
        }

        let tx = 8 - tx;
        let ty = 8 - ty;

        assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), tx * 9 + ty), 2);
    }
}
#[test]
fn test_sente_mix() {
    let positions = [
        vec![(SHisha,0,3),(SKyou,3,8),(SKaku,0,0),(SKaku,8,8)],
        vec![(SHisha,0,6),(SKaku,0,3),(SHisha,3,0),(SKyou,3,8)],
        vec![(SHishaN,4,5),(SHishaN,7,4),(SKakuN,5,7),(SKakuN,5,4)],
        vec![(SKei,4,5),(SGin,6,2),(SKin,4,3),(SGin,5,2)],
        vec![(SKei,4,5),(SGin,6,2),(SFuN,4,3),(SGin,5,2)],
        vec![(SKei,4,5),(SGin,6,2),(SKyouN,4,3),(SGin,5,2)],
        vec![(SKei,4,5),(SGin,6,2),(SKeiN,4,3),(SGin,5,2)],
        vec![(SKei,4,5),(SGin,6,2),(SGinN,4,3),(SGin,5,2)],
    ];
    let occ_positions = [
        vec![(GFu,3,7),(SFu,7,7)],
        vec![(GFu,3,1)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ];
    let target_positions = [(3,3),(3,6),(5,6),(5,3),(5,3),(5,3),(5,3),(5,3)];
    let answers = [2,3,2,3,3,3,3,3];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for (i,(((it,oit),&(tx,ty)),&answer)) in positions.iter()
        .zip(occ_positions.iter())
        .zip(target_positions.iter()).zip(answers.iter()).enumerate() {
        let mut banmen = blank_banmen.clone();

        for &(kind,x,y) in it.iter() {
            banmen.0[y][x] = kind;
        }

        for &(kind,ox,oy) in oit.iter() {
            banmen.0[oy][ox] = kind;
        }

        assert_eq!(Rule::control_count(Teban::Sente, &State::new(banmen), tx * 9 + ty), answer,
            "testcase = {}",i
        );
    }
}
#[test]
fn test_gote_mix() {
    let positions = [
        vec![(GHisha,0,3),(GKyou,3,8),(GKaku,0,0),(GKaku,8,8)],
        vec![(GHisha,0,6),(GKaku,0,3),(GHisha,3,0),(GKyou,3,8)],
        vec![(GHishaN,4,5),(GHishaN,7,4),(GKakuN,5,7),(GKakuN,5,4)],
        vec![(GKei,4,5),(GGin,6,2),(GKin,4,3),(GGin,5,2)],
        vec![(GKei,4,5),(GGin,6,2),(GFuN,4,3),(GGin,5,2)],
        vec![(GKei,4,5),(GGin,6,2),(GKyouN,4,3),(GGin,5,2)],
        vec![(GKei,4,5),(GGin,6,2),(GKeiN,4,3),(GGin,5,2)],
        vec![(GKei,4,5),(GGin,6,2),(GGinN,4,3),(GGin,5,2)],
    ];
    let occ_positions = [
        vec![(SFu,3,7),(GFu,7,7)],
        vec![(SFu,3,1)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ];
    let target_positions = [(3,3),(3,6),(5,6),(5,3),(5,3),(5,3),(5,3),(5,3)];
    let answers = [2,3,2,3,3,3,3,3];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for (i,(((it,oit),&(tx,ty)),&answer)) in positions.iter()
        .zip(occ_positions.iter())
        .zip(target_positions.iter()).zip(answers.iter()).enumerate() {
        let mut banmen = blank_banmen.clone();

        for &(kind,x,y) in it.iter() {
            banmen.0[8-y][8-x] = kind;
        }

        for &(kind,ox,oy) in oit.iter() {
            banmen.0[8-oy][8-ox] = kind;
        }

        let tx = 8 - tx;
        let ty = 8 - ty;

        assert_eq!(Rule::control_count(Teban::Gote, &State::new(banmen), tx * 9 + ty), answer,
                   "testcase = {}",i
        );
    }
}
