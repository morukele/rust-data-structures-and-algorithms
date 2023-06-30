use puzzle::money_change::{change_money_basic, change_money_one_liner};

fn main() {
    let res_1 = change_money_basic(100);
    let res_2 = change_money_one_liner(100);
    println!("output_1: {:?}", res_1);
    println!("output_2: {:?}", res_2);
}
