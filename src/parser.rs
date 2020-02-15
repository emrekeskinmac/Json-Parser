use nom::{
    alt, char,
    character::complete::{alphanumeric1, digit1},
    delimited, named, separated_list, separated_pair, tag,
};

named!(json_key, delimited!(char!('"'), alphanumeric1, char!('"')));

named!(
    json_value,
    alt!(digit1 | delimited!(char!('"'), alphanumeric1, char!('"')))
);

named!(
    json_kv<(&[u8], &[u8])>,
    separated_pair!(json_key, char!(':'), json_value)
);

named!(
    json_kv_multi<Vec<(&[u8], &[u8])>>,
    separated_list!(char!(','), json_kv)
);

named!(pub json_parser<(Vec<(&[u8], &[u8])>)>,
    delimited!(
        tag!("{"),
        json_kv_multi,
        tag!("}")
    )
);

#[cfg(test)]
mod parser_tests {
    use super::*;

    #[test]
    fn test_json_key() {
        assert_eq!(json_key(&b"\"test\""[..]), Ok((&b""[..], &b"test"[..])));
    }

    #[test]
    fn test_json_value() {
        // String value
        assert_eq!(
            json_value(&b"\"12test\""[..]),
            Ok((&b""[..], &b"12test"[..]))
        );
        assert_eq!(json_value(&b"\"test\""[..]), Ok((&b""[..], &b"test"[..])));
        assert_eq!(
            json_value(&b"\"te12st\""[..]),
            Ok((&b""[..], &b"te12st"[..]))
        );
        assert_eq!(
            json_value(&b"\"test12\""[..]),
            Ok((&b""[..], &b"test12"[..]))
        );

        //Number value
        assert_eq!(json_value(&b"12345"[..]), Ok((&b""[..], &b"12345"[..])));
    }

    #[test]
    fn test_json_kv() {
        //Number value string key
        assert_eq!(
            json_kv(&b"\"key\":123"[..]),
            Ok((&b""[..], (&b"key"[..], &b"123"[..])))
        );
        //String value number key
        assert_eq!(
            json_kv(&b"\"123\":\"value\""[..]),
            Ok((&b""[..], (&b"123"[..], &b"value"[..])))
        );
    }

    /* FIXME: Err(Incomplete(Size(1)))
    #[test]
    fn test_json_kv_multi() {
      assert_eq!(json_kv_multi(&b"\"key1\":\"value1\",\"key2\":\"value2\""[..]),
      Ok(( &b""[..], vec![
              (&b"key1"[..], &b"value1"[..]),
              (&b"key2"[..], &b"value2"[..])
      ])));
    }
    */

    #[test]
    fn test_json_parser() {
        assert_eq!(
            json_parser(&b"{\"key1\":\"value1\",\"key2\":\"value2\"}"[..]),
            Ok((
                &b""[..],
                vec![
                    (&b"key1"[..], &b"value1"[..]),
                    (&b"key2"[..], &b"value2"[..])
                ]
            ))
        );
    }
}
