use super::*;

impl Debug for MixedLinearSolver {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut variables = self.variables.values().collect::<Vec<_>>();
        f.debug_struct("LinearSolver")
            .field("type", &DisplayWrapper::new(&"f64"))
            .field("direct", &self.direct)
            .field("variables", &DisplayList::new(&variables))
            .field("constraints", &DisplayList::new(&self.constraints))
            .field("epsilon", &self.epsilon)
            .finish()
    }
}
