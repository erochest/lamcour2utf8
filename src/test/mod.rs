use super::recode;
use spectral::prelude::*;
use std::io::Cursor;

fn test_recode(input: &mut Vec<u8>, expected: &str) {
    let mut input = Cursor::new(input);
    let mut actual = Vec::new();
    let expected = expected.to_string().into_bytes();
    let result = recode(&mut input, &mut actual);

    assert_that(&result).is_ok();
    assert_that(&actual).is_equal_to(&expected);
}

#[test]
fn test_recode_recodes_dragonfly() {
    let mut input = b"sne\xb1\xbfk$d\xb5\xdd\xabkt\xb0".to_vec();
    let expected = "sne\u{026a}\u{02d7}k$d\u{0251}\u{0306}\u{02c4}kt\u{0259}";
    test_recode(&mut input, &expected);

    let mut input = b"sne\xb1\xbfk$fi\xbad\xb4".to_vec();
    let expected = "sne\u{026a}\u{02d7}k$fi\u{00b7}d\u{025a}";
    test_recode(&mut input, &expected);

    let mut input = b"\xe5sk\x88i\xbat\x90\xb0\xdd\xf5h\xb7\xbd\xba{\xb0}k".to_vec();
    let expected = "\u{02c8}sk\u{0318}i\u{00b7}t\u{032c}\u{0259}\u{0306}\u{02cc}h\u{0254}\u{02c5}\u{00b7}{\u{0259}}k";
    test_recode(&mut input, &expected);
}
