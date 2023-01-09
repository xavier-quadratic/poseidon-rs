use ff::{Field, PrimeField};
use poseidon;
use poseidon::convert::{felts_from_u8s, scalar_from_u64s, scalar_from_u8s, u8s_from_felts};
use poseidon::hash_s128b as hash;
use poseidon::parameters::s128b::F253;

mod common;

fn to_felts(input: &[&str]) -> Vec<F253> {
    common::load_felts::<F253>(input)
}

#[test]
fn test_ff() {
    let b0: F253 = F253::from(0);
    let b1: F253 = F253::from(7);
    let b2: F253 = F253::from(343);

    assert_eq!(F253::ZERO, b0);
    assert_eq!(F253::MULTIPLICATIVE_GENERATOR, b1);
    assert_eq!(b1.pow([3]), b2);
}

#[test]
fn test_ff_conversion() {
    let a1: [u64; 4] = [7, 0, 0, 0];
    let a1: F253 = scalar_from_u64s::<F253>(&a1);
    let b1: F253 = F253::from(7);
    assert_eq!(a1, b1);

    let a2: [u8; 32] = [
        87, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    let a2: F253 = scalar_from_u8s::<F253>(&a2);
    let b2: F253 = F253::from(343);
    assert_eq!(a2, b2);

    let a_felts = vec![b1.clone(), b2.clone()];
    let a_u8s = u8s_from_felts::<F253>(&a_felts);
    let b_u8s: [u8; 64] = [
        7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 87, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ];
    let b_felts = felts_from_u8s::<F253>(&b_u8s);
    assert_eq!(a_u8s, b_u8s);
    assert_eq!(a_felts, b_felts);
}

#[test]
fn test_hash_simple() {
    let input = ["7", "98"];
    let input = to_felts(&input);
    let expected =
        ["11053447091811250430558990262025664436943237361628909971717799705027922243051"];
    let expected = to_felts(&expected);
    let output = hash(&input);
    assert_eq!(output, expected);
}

#[test]
fn test_hash_double() {
    let input = [
        "7",
        "98",
        "9033127700447853090229678702028773675793347128105171639302548972716183808266",
        "5548559894030014093638382051049588462182080648170927883872275099718526588448",
    ];
    let input = to_felts(&input);
    let expected =
        ["14226598713275173058539927170621755455377315018571595090070022785066210289324"];
    let expected = to_felts(&expected);
    let output = hash(&input);
    assert_eq!(output, expected);
}

#[test]
#[should_panic]
fn test_hash_wrong_size() {
    let input = ["0", "7", "98"];
    let input = to_felts(&input);
    hash(&input);
}
