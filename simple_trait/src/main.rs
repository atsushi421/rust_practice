#[test]
fn test_jewelrybox() {
    let jewelry_key_no = 30;
    let jewelry_price = 20;
    let jewelrybox = JewelryBox {key_no: jewelry_key_no, price: jewelry_price};

    assert_eq!(jewelrybox.open(jewelry_key_no-1), false);
    assert_eq!(jewelrybox.open(jewelry_key_no), true);
    assert_eq!(jewelrybox.check(), jewelry_price);
}

#[test]
fn test_trapbox() {
    let trap_key_no = 30;
    let trap_damage = -20;
    let trapbox = TrapBox {key_no: trap_key_no, damage: trap_damage};

    assert_eq!(trapbox.open(trap_key_no-1), false);
    assert_eq!(trapbox.open(trap_key_no), true);
    assert_eq!(trapbox.check(), trap_damage);
}