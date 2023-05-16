use usiagent::rule::Rule;

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
fn test_sente_is_possible_nari_possible() {
    let positions = [
        (SFu,0,3,0,2),
        (SFu,8,3,8,2),
        (SKyou,0,5,0,2),
        (SKyou,8,5,8,2),
        (SKei,1,4,0,2),
        (SKei,7,4,8,2),
        (SGin,0,3,0,2),
        (SGin,8,3,8,2),
        (SHisha,0,5,0,2),
        (SHisha,8,5,8,2),
        (SKaku,2,4,0,2),
        (SKaku,6,4,8,2),
        (SGin,0,2,0,3),
        (SGin,8,2,8,3),
        (SHisha,0,2,0,5),
        (SHisha,8,2,8,5),
        (SKaku,0,2,2,4),
        (SKaku,8,2,6,4)
    ];

    for &(kind,sx,sy,dx,dy) in positions.iter() {
        assert_eq!(Rule::is_possible_nari(kind,9 * sx + sy, 9 * dx + dy),true);
    }
}
#[test]
fn test_gote_is_possible_nari_possible() {
    let positions = [
        (GFu,0,3,0,2),
        (GFu,8,3,8,2),
        (GKyou,0,5,0,2),
        (GKyou,8,5,8,2),
        (GKei,1,4,0,2),
        (GKei,7,4,8,2),
        (GGin,0,3,0,2),
        (GGin,8,3,8,2),
        (GHisha,0,5,0,2),
        (GHisha,8,5,8,2),
        (GKaku,2,4,0,2),
        (GKaku,6,4,8,2),
        (GGin,0,2,0,3),
        (GGin,8,2,8,3),
        (GHisha,0,2,0,5),
        (GHisha,8,2,8,5),
        (GKaku,0,2,2,4),
        (GKaku,8,2,6,4)
    ];

    for &(kind,sx,sy,dx,dy) in positions.iter() {
        let sx = 8 - sx;
        let sy = 8 - sy;
        let dx = 8 - dx;
        let dy = 8 - dy;

        assert_eq!(Rule::is_possible_nari(kind,9 * sx + sy, 9 * dx + dy),true);
    }
}
#[test]
fn test_sente_is_possible_nari_impossible() {
    let positions = [
        (SFu,0,4,0,3),
        (SFu,8,4,8,3),
        (SFu,0,6,0,5),
        (SFu,8,6,8,5),
        (SKyou,0,5,0,3),
        (SKyou,8,5,8,3),
        (SKyou,0,6,0,5),
        (SKyou,8,6,8,5),
        (SKei,1,5,0,3),
        (SKei,7,5,8,3),
        (SKei,0,6,1,4),
        (SKei,8,6,7,4),
        (SGin,0,4,0,3),
        (SGin,8,4,8,3),
        (SGin,0,6,0,5),
        (SGin,8,6,8,5),
        (SKaku,1,4,0,3),
        (SKaku,7,4,8,3),
        (SHisha,0,5,0,3),
        (SHisha,8,5,8,3),
        (SHisha,0,6,0,5),
        (SHisha,8,6,8,5),
        (SFuN,0,3,0,2),
        (SFuN,8,3,8,2),
        (SFuN,0,4,0,3),
        (SFuN,8,4,8,3),
        (SFuN,0,6,0,5),
        (SFuN,8,6,8,5),
        (SKyouN,0,3,0,2),
        (SKyouN,8,3,8,2),
        (SKyouN,0,4,0,3),
        (SKyouN,8,4,8,3),
        (SKyouN,0,6,0,5),
        (SKyouN,8,6,8,5),
        (SKeiN,0,3,0,2),
        (SKeiN,8,3,8,2),
        (SKeiN,0,4,0,3),
        (SKeiN,8,4,8,3),
        (SKeiN,0,6,0,5),
        (SKeiN,8,6,8,5),
        (SGinN,0,3,0,2),
        (SGinN,8,3,8,2),
        (SGinN,0,4,0,3),
        (SGinN,8,4,8,3),
        (SGinN,0,6,0,5),
        (SGinN,8,6,8,5),
        (SHishaN,0,3,0,2),
        (SHishaN,8,3,8,2),
        (SHishaN,0,4,0,3),
        (SHishaN,8,4,8,3),
        (SHishaN,0,6,0,5),
        (SHishaN,8,6,8,5),
        (SKakuN,0,3,0,2),
        (SKakuN,8,3,8,2),
        (SKakuN,0,4,0,3),
        (SKakuN,8,4,8,3),
        (SKakuN,0,6,0,5),
        (SKakuN,8,6,8,5),
    ];

    for (i,&(kind,sx,sy,dx,dy)) in positions.iter().enumerate() {
        assert_eq!(Rule::is_possible_nari(kind,9 * sx + sy, 9 * dx + dy),false,"testcase = {}",i);
    }
}
#[test]
fn test_gote_is_possible_nari_impossible() {
    let positions = [
        (GFu,0,4,0,3),
        (GFu,8,4,8,3),
        (GFu,0,6,0,5),
        (GFu,8,6,8,5),
        (GKyou,0,5,0,3),
        (GKyou,8,5,8,3),
        (GKyou,0,6,0,5),
        (GKyou,8,6,8,5),
        (GKei,1,5,0,3),
        (GKei,7,5,8,3),
        (GKei,0,6,1,4),
        (GKei,8,6,7,4),
        (GGin,0,4,0,3),
        (GGin,8,4,8,3),
        (GGin,0,6,0,5),
        (GGin,8,6,8,5),
        (GKaku,1,4,0,3),
        (GKaku,7,4,8,3),
        (GHisha,0,5,0,3),
        (GHisha,8,5,8,3),
        (GHisha,0,6,0,5),
        (GHisha,8,6,8,5),
        (GFuN,0,3,0,2),
        (GFuN,8,3,8,2),
        (GFuN,0,4,0,3),
        (GFuN,8,4,8,3),
        (GFuN,0,6,0,5),
        (GFuN,8,6,8,5),
        (GKyouN,0,3,0,2),
        (GKyouN,8,3,8,2),
        (GKyouN,0,4,0,3),
        (GKyouN,8,4,8,3),
        (GKyouN,0,6,0,5),
        (GKyouN,8,6,8,5),
        (GKeiN,0,3,0,2),
        (GKeiN,8,3,8,2),
        (GKeiN,0,4,0,3),
        (GKeiN,8,4,8,3),
        (GKeiN,0,6,0,5),
        (GKeiN,8,6,8,5),
        (GGinN,0,3,0,2),
        (GGinN,8,3,8,2),
        (GGinN,0,4,0,3),
        (GGinN,8,4,8,3),
        (GGinN,0,6,0,5),
        (GGinN,8,6,8,5),
        (GHishaN,0,3,0,2),
        (GHishaN,8,3,8,2),
        (GHishaN,0,4,0,3),
        (GHishaN,8,4,8,3),
        (GHishaN,0,6,0,5),
        (GHishaN,8,6,8,5),
        (GKakuN,0,3,0,2),
        (GKakuN,8,3,8,2),
        (GKakuN,0,4,0,3),
        (GKakuN,8,4,8,3),
        (GKakuN,0,6,0,5),
        (GKakuN,8,6,8,5),
    ];

    for (i,&(kind,sx,sy,dx,dy)) in positions.iter().enumerate() {
        let sx = 8 - sx;
        let sy = 8 - sy;
        let dx = 8 - dx;
        let dy = 8 - dy;

        assert_eq!(Rule::is_possible_nari(kind,9 * sx + sy, 9 * dx + dy),false,"testcase = {}",i);
    }
}
