use super::*;

impl Debug for FloatLinearSolver {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut variables = self.variables.values().collect::<Vec<_>>();
        f.debug_struct("LinearSolver")
            .field("type", &"f64")
            .field("direct", &self.direct)
            .field("variables", &DisplayVector::new(&variables))
            .field("constraints", &DisplayVector::new(&self.constraints))
            .field("epsilon", &self.epsilon)
            .finish()
    }
}
