use ch11::adder;

#[test]
fn it_adder() {
    assert_eq!(4, adder::add(2, 2))
}
