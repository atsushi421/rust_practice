/*
pub trait Add<Rhs = Self> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}
*/
use std::ops::Add;

struct Coffee {
    ml: i32,
    weight_g: i32,
}

# [test]
fn test_add_coffee() {
    let coffee1 = Coffee {ml: 500, weight_g: 300};
    let coffee2 = Coffee {ml: 150, weight_g: 150};
    let added_coffee = my_add::<Coffee>(coffee1, coffee2);

    assert_eq!(added_coffee.ml, 650);
    assert_eq!(added_coffee.weight_g, 450);
}