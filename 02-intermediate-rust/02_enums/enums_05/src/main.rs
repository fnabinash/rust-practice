// Implement a program that uses an enum to represent different payment methods (CreditCard, PayPal, Cash) and calculate the total cost based on the payment method.

const CREDIT_CARD_RATE: f32 = 0.03;
const PAYPAL_RATE: f32 = 0.05;
const CASH_RATE: f32 = 0.1;

enum PaymentMethods {
    CreditCard,
    PayPal,
    Cash
}

impl PaymentMethods {
    fn calculate_total_cost(&self, cart_value: f32) -> f32 {
        match self {
            Self::CreditCard => cart_value + cart_value * CREDIT_CARD_RATE,
            Self::PayPal => cart_value + cart_value * PAYPAL_RATE,
            Self::Cash => cart_value + cart_value * CASH_RATE
        }
    }
}

fn truncate(num: f32) -> f32 {
    f32::trunc(num * 100.0) / 100.0
}

fn main() {
    let chocolate: f32 = 15.65;

    let payment_method: PaymentMethods = PaymentMethods::CreditCard;
    let total_cost: f32 = payment_method.calculate_total_cost(chocolate);
    println!("Total cost of buying ${} chocolate using your creadicard will be {}.", chocolate, truncate(total_cost));
    
    let payment_method: PaymentMethods = PaymentMethods::PayPal;
    let total_cost: f32 = payment_method.calculate_total_cost(chocolate);
    println!("Total cost of buying ${} chocolate using your paypal will be {}.", chocolate, truncate(total_cost));
    
    let payment_method: PaymentMethods = PaymentMethods::Cash;
    let total_cost: f32 = payment_method.calculate_total_cost(chocolate);
    println!("Total cost of buying ${} chocolate using your cash will be {}.", chocolate, truncate(total_cost));
}
