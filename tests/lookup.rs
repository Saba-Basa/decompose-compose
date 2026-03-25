use decompose_compose::lookup;

const R: &str = "abcde";

#[test] fn v_zero()    { assert_eq!(lookup(0.0,  R), Some("a".into())); }
#[test] fn v_half()    { assert_eq!(lookup(0.5,  R), Some("c".into())); }
#[test] fn v_one()     { assert_eq!(lookup(1.0,  R), Some("e".into())); }
#[test] fn v_03()      { assert_eq!(lookup(0.3,  R), Some("b".into())); }
#[test] fn v_neg()     { assert_eq!(lookup(-1.0, R), Some("a".into())); }
#[test] fn v_above()   { assert_eq!(lookup(2.0,  R), Some("e".into())); }
#[test] fn empty()     { assert_eq!(lookup(0.5,  ""), None); }
#[test] fn non_ascii() { assert_eq!(lookup(0.5,  "äöü"), None); }