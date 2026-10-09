use super::*;
#[test]
fn from_numbers_limit() {
    for i in 0..=255_u8 {
        let result = square::Square::try_from(i);
        if i < 64 {
            assert!(result.is_ok());
        } else {
            assert!(result.is_err());
        }
    }
}

// Testing for memory transmute errors, not exhaustive
#[test]
fn from_numbers_specific() {
    assert_eq!(Ok(Square::A1), Square::try_from(0));
    assert_eq!(Ok(Square::B1), Square::try_from(1));
    assert_eq!(Ok(Square::H1), Square::try_from(7));
    assert_eq!(Ok(Square::E4), Square::try_from(28));
    assert_eq!(Ok(Square::C6), Square::try_from(42));
    assert_eq!(Ok(Square::A8), Square::try_from(56));
    assert_eq!(Ok(Square::H8), Square::try_from(63));
    assert!(Square::try_from(64).is_err());
    assert!(Square::try_from(87).is_err());
    assert!(Square::try_from(105).is_err());
    assert!(Square::try_from(254).is_err());
    assert!(Square::try_from(255).is_err());
}

#[test]
fn from_str() {
    assert_eq!(Ok(Square::A1), "a1".parse());
    assert_eq!(Ok(Square::B1), "b1".parse());
    assert_eq!(Ok(Square::H1), "h1".parse());
    assert_eq!(Ok(Square::E4), "e4".parse());
    assert_eq!(Ok(Square::C6), "c6".parse());
    assert!("Hello world!".parse::<Square>().is_err());
    assert!("a0".parse::<Square>().is_err());
    assert!("a9".parse::<Square>().is_err());
    assert!("i1".parse::<Square>().is_err());
    assert!("i8".parse::<Square>().is_err());
}
