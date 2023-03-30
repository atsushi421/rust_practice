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

impl Add for Coffee {
    type Output = Coffee;
    fn add(self, rhs: Self) -> Self::Output {
        Coffee {ml: rhs.ml + self.ml, weight_g: rhs.weight_g + self.weight_g}
    }
}

fn my_add<T: Add<Output = T>>(right: T, left: T) -> T {
    right + left
}

# [test]
fn test_add_coffee() {
    let coffee1 = Coffee {ml: 500, weight_g: 300};
    let coffee2 = Coffee {ml: 150, weight_g: 150};
    let added_coffee = my_add::<Coffee>(coffee1, coffee2);

    assert_eq!(added_coffee.ml, 650);
    assert_eq!(added_coffee.weight_g, 450);
}

# [test]
fn test_my_add_trait_bound() {
    struct NotImplAdd {
        tmp: i32,
    }
    let tmp1 = NotImplAdd {tmp: 10};
    let tmp2 = NotImplAdd {tmp: 5};

    my_add::<NotImplAdd>(tmp1, tmp2);
}