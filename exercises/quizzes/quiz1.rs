// This is a quiz for the following sections:
// - Variables
// - Functions
// - If
//
// Mary is buying apples. The price of an apple is calculated as follows:
// - An apple costs 2 rustbucks.
// - However, if Mary buys more than 40 apples, the price of each apple in the
// entire order is reduced to only 1 rustbuck!

// TODO: Write a function that calculates the price of an order of apples given
// the quantity bought.
// fn calculate_price_of_apples(???) -> ??? { ??? }
fn calculate_price_of_apples(quantity: u32) -> u32 {
    if quantity > 40 {
        quantity * 1  // 超过40个时，全部苹果单价1锈币
    } else {
        quantity * 2  // 40个及以下时，单价2锈币
    }
}
fn main() {
    // 可以添加一些测试代码，但通常不需要
    println!("Hello, this is a placeholder main function!");
}
// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test() {
        assert_eq!(calculate_price_of_apples(35), 70);
        assert_eq!(calculate_price_of_apples(40), 80);
        assert_eq!(calculate_price_of_apples(41), 41);
        assert_eq!(calculate_price_of_apples(65), 65);
    }
}
