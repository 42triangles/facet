use facet::Facet;
use facet_json::from_str;

#[test]
fn json_read_unit_enum_variant() {
    facet_testhelpers::setup();

    #[derive(Facet, Debug, PartialEq)]
    #[repr(u8)]
    enum FontStyle {
        Italic,
        Oblique,
    }
    // TODO: support rename/rename_all
    let json_italic = r#""Italic""#;
    let json_oblique = r#""Oblique""#;

    let s_italic: FontStyle = match from_str(json_italic) {
        Ok(s) => s,
        Err(e) => panic!("Error deserializing JSON: {}", e),
    };
    assert_eq!(s_italic, FontStyle::Italic);

    let s_oblique: FontStyle = match from_str(json_oblique) {
        Ok(s) => s,
        Err(e) => panic!("Error deserializing JSON: {}", e),
    };
    assert_eq!(s_oblique, FontStyle::Oblique);
}

#[test]
fn json_read_tuple_variant() {
    facet_testhelpers::setup();

    #[derive(Facet, Debug, PartialEq)]
    #[repr(u8)]
    enum Point {
        X(u64),
        Y(String),
    }

    let json_x = r#"{ "X": 123 }"#;
    let json_y = r#"{ "Y": "hello" }"#;

    let p_x: Point = match from_str(json_x) {
        Ok(s) => s,
        Err(e) => panic!("Error deserializing JSON: {}", e),
    };
    assert_eq!(p_x, Point::X(123));

    let p_y: Point = match from_str(json_y) {
        Ok(s) => s,
        Err(e) => panic!("Error deserializing JSON: {}", e),
    };
    assert_eq!(p_y, Point::Y("hello".to_string()));
}

#[test]
fn enum_generic_u8() {
    #[allow(dead_code)]
    #[derive(Facet)]
    #[repr(u8)]
    enum E<'a, T: core::hash::Hash, const C: usize = 3>
    where
        T: std::fmt::Debug,
        [u8; C]: std::fmt::Debug,
    {
        Unit,
        Tuple(T, core::marker::PhantomData<&'a [u8; C]>),
        Record {
            field: T,
            phantom: core::marker::PhantomData<&'a ()>,
            constant_val: [u8; C],
        },
    }
}

#[test]
fn enum_generic_c() {
    #[allow(dead_code)]
    #[derive(Facet)]
    #[repr(C)]
    enum E<'a, T: core::hash::Hash, const C: usize = 3>
    where
        T: std::fmt::Debug,
        [u8; C]: std::fmt::Debug,
    {
        Unit,
        Tuple(T, core::marker::PhantomData<&'a [u8; C]>),
        Record {
            field: T,
            phantom: core::marker::PhantomData<&'a ()>,
            constant_val: [u8; C],
        },
    }
}
