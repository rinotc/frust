use crate::util::currency::Currency;

#[derive(Copy, Clone)]
pub struct Money {
    pub amount: f64,
    pub currency: Currency,
}

impl Money {
    fn new(self, amount: f64, currency: Currency) -> Money {
        Money { amount, currency }
    }
}