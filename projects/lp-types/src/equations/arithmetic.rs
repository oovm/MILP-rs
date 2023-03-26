use super::*;
use std::ops::SubAssign;

impl<T: SubAssign> SubAssign<Self> for LinearExpression<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.constant -= rhs.constant;
        for (k, v) in rhs.coefficients {
            self.coefficients.entry(k).and_modify(|e| *e -= v).or_insert_with(|| -v);
        }
    }
}
