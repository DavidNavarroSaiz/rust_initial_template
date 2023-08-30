use testrust::add;
use testrust::sub;

fn test_add() {
    assert_eq!(add(1, 2), 3);
}

fn test_sub() {
    assert_eq!(sub(3, 2), 1);
}
