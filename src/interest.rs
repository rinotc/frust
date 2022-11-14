use crate::util::money::Money;
use crate::util::rate::Rate;
use crate::util::time::Time;
use crate::util::currency::Currency::JPY;

struct InterestCalculator {
    initial: Money,
    rate: Rate,
}

impl InterestCalculator {

    fn simple_interest(self, t: Time) -> Money {
        let v = (1.0 + self.rate.0 * t.0) * self.initial.amount;
        Money {
            amount: v,
            currency: self.initial.currency
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_interest() {
        let si = InterestCalculator {
            initial: Money { amount: 1000.0, currency: JPY },
            rate: Rate(0.01),
        };

        let result = si.simple_interest(Time(10.0));

        assert_eq!(result.amount, 1100f64)
    }
}
