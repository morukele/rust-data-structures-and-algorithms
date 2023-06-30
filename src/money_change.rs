pub fn change_money_basic(mut money: i64) -> i64 {
    let mut number_of_change = 0;

    while money > 0 {
        if money >= 10 {
            money -= 10;
        } else if money >= 5 {
            money -= 5;
        } else {
            money -= 1;
        }
        number_of_change += 1;
    }

    number_of_change
}

pub fn change_money_one_liner(money: i64) -> i64 {
    (money / 10) + ((money % 10) / 5) + money % 5
}
