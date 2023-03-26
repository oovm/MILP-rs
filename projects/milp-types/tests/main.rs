use milp_types::{MixedConstraint, MixedEquation, MixedLinearDescriptor, MixedVariable};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    let mut problem = MixedLinearDescriptor::new();
    let mut e1 = MixedEquation::new(MixedConstraint::le(1.0)).unwrap();
    e1.add_coefficient(1.0, "x");
    e1.add_coefficient(1.0, "y");
    problem.add_equation(e1);

    let mut e2 = MixedEquation::new(MixedConstraint::le(2.0)).unwrap();
    e2.add_coefficient(1.0, "x");
    e2.add_coefficient(1.0, "z");
    problem.add_equation(e2);

    problem.add_variable(MixedVariable::le("x", 1.0));
    problem.add_variable(MixedVariable::le("y", 1.0));
    problem.add_variable(MixedVariable::le("z", 1.0));

    println!("{:#?}", problem);
}
