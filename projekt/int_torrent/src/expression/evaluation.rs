use super::models::{AExpr, BinaryOperation};
impl AExpr {
    pub fn evaluate(&self) -> i64 {
        // Tukaj lahko predpostavite, da spremenljivke ne obstajajo
        match self {
            AExpr::Num(c) => return *c,
            AExpr::BinOp(iz1, op, iz2) => 
                match op {
                    BinaryOperation::Add => &iz1.evaluate() + &iz2.evaluate(),
                    BinaryOperation::Sub => &iz1.evaluate() - &iz2.evaluate(),
                    BinaryOperation::Mul => &iz1.evaluate() * &iz2.evaluate(),
                    BinaryOperation::Pow => iz1.evaluate().pow(iz2.evaluate() as u32),
                }
        }
    }

    pub fn evaluate_map(
        &self,
        vars: &std::collections::HashMap<String, Option<i64>>,
    ) -> Option<i64> {
        panic!("Implement variable evaluation")
    }
}
