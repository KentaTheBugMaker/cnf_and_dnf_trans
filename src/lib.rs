pub fn add(left: usize, right: usize) -> usize {
    left + right
}
#[derive(Eq, PartialEq, Clone)]
pub enum Expression<P>
where
    P: Eq,
{
    Var(P),
    Not(Box<Self>),
    And(Box<Self>, Box<Self>),
    Or(Box<Self>, Box<Self>),
    Implies(Box<Self>, Box<Self>),
    Iff(Box<Self>, Box<Self>),
    Top,
    Bottom,
}
impl<P> std::fmt::Debug for Expression<P>
where
    P: std::fmt::Debug + Eq,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Var(var) => write!(f, "{:?}", var),
            Expression::Not(arg0) => write!(f, r"\lnot {:?}", arg0),
            Expression::And(arg0, arg1) => write!(f, r"({:?} \land {:?})", arg0, arg1),
            Expression::Or(arg0, arg1) => write!(f, r"({:?} \lor {:?})", arg0, arg1),
            Expression::Implies(arg0, arg1) => write!(f, r"({:?} \to {:?})", arg0, arg1),
            Expression::Iff(arg0, arg1) => write!(f, r"({:?} \leftrightarrow {:?})", arg0, arg1),
            Expression::Top => write!(f, r"\top"),
            Expression::Bottom => write!(f, r"\bot"),
        }
    }
}
pub fn var<P>(var: P) -> Box<Expression<P>>
where
    P: Eq,
{
    Box::new(Expression::Var(var))
}
pub fn not<P>(exp: impl IntoExpression<P>) -> Box<Expression<P>>
where
    P: Eq,
{
    Box::new(Expression::Not(exp.into()))
}
pub fn and<P>(exp1: impl IntoExpression<P>, exp2: impl IntoExpression<P>) -> Box<Expression<P>>
where
    P: Eq,
{
    Box::new(Expression::And(exp1.into(), exp2.into()))
}
pub fn or<P>(exp1: impl IntoExpression<P>, exp2: impl IntoExpression<P>) -> Box<Expression<P>>
where
    P: Eq,
{
    Box::new(Expression::Or(exp1.into(), exp2.into()))
}
pub fn implies<P>(exp1: impl IntoExpression<P>, exp2: impl IntoExpression<P>) -> Box<Expression<P>>
where
    P: Eq,
{
    Box::new(Expression::Implies(exp1.into(), exp2.into()))
}
pub fn iff<P>(exp1: impl IntoExpression<P>, exp2: impl IntoExpression<P>) -> Box<Expression<P>>
where
    P: Eq,
{
    Box::new(Expression::Iff(exp1.into(), exp2.into()))
}
pub fn top<P>() -> Box<Expression<P>>
where
    P: Eq,
{
    Box::new(Expression::Top)
}
pub fn bottom<P>() -> Box<Expression<P>>
where
    P: Eq,
{
    Box::new(Expression::Bottom)
}

impl<P> Expression<P>
where
    P: Eq + Default + Clone,
{
    /// Eliminate Top or Bottom.
    fn step_1(self: Box<Self>) -> Box<Self> {
        match self.as_ref() {
            Expression::Top => or(not(var(P::default())), var(P::default())),
            Expression::Bottom => or(not(var(P::default())), var(P::default())),
            Expression::And(exp1, exp2) => and(exp1.clone().step_1(), exp2.clone().step_1()),
            Expression::Or(exp1, exp2) => or(exp1.clone().step_1(), exp2.clone().step_1()),
            Expression::Implies(exp1, exp2) => {
                implies(exp1.clone().step_1(), exp2.clone().step_1())
            }
            Expression::Iff(exp1, exp2) => iff(exp1.clone().step_1(), exp2.clone().step_1()),
            Expression::Not(exp1) => not(exp1.clone().step_1()),
            _ => self,
        }
    }
    /// Eliminate Iff
    fn step_2(self: Box<Self>) -> Box<Self> {
        match self.as_ref() {
            Expression::Not(exp) => not(exp.clone().step_2()),
            Expression::And(exp1, exp2) => and(exp1.clone().step_2(), exp2.clone().step_2()),
            Expression::Or(exp1, exp2) => or(exp1.clone().step_2(), exp2.clone().step_2()),
            Expression::Implies(exp1, exp2) => {
                implies(exp1.clone().step_2(), exp2.clone().step_2())
            }
            Expression::Iff(exp1, exp2) => and(
                implies(exp1.clone(), exp2.clone()),
                implies(exp2.clone(), exp1.clone()),
            ),
            _ => self,
        }
    }
    /// Eliminate Implies
    fn step_3(self: Box<Self>) -> Box<Self> {
        match self.as_ref() {
            Expression::Not(exp) => not(exp.clone().step_3()),
            Expression::And(exp1, exp2) => and(exp1.clone().step_3(), exp2.clone().step_3()),
            Expression::Or(exp1, exp2) => or(exp1.clone().step_3(), exp2.clone().step_3()),
            Expression::Implies(exp1, exp2) => {
                or(not(exp1.clone().step_3()), exp2.clone().step_3())
            }
            // iff is already eliminated by step 2.
            //
            _ => self,
        }
    }
    /// transform  not(and(A,B)) ->or(not(A),not(B)),not(or(A,B)) ->and(not(A),not(B)),
    fn step_4(self: Box<Self>) -> Box<Self> {
        match self.as_ref() {
            Expression::Not(exp) => match exp.as_ref() {
                Expression::And(exp1, exp2) => or(not(exp1.clone()), not(exp2.clone())),
                Expression::Or(exp1, exp2) => and(not(exp1.clone()), not(exp2.clone())),
                _ => not(exp.clone()),
            },
            Expression::And(exp1, exp2) => and(exp1.clone().step_4(), exp2.clone().step_4()),
            Expression::Or(exp1, exp2) => or(exp1.clone().step_4(), exp2.clone().step_4()),
            _ => self,
        }
    }
    /// transform not(not(A)) -> A
    fn step_5(self: Box<Self>) -> Box<Self> {
        match self.as_ref() {
            Expression::Not(exp) => match exp.as_ref() {
                Expression::Not(exp) => exp.clone(),
                _ => self,
            },
            Expression::And(exp1, exp2) => and(exp1.clone().step_5(), exp2.clone().step_5()),
            Expression::Or(exp1, exp2) => or(exp1.clone().step_5(), exp2.clone().step_5()),
            _ => self,
        }
    }
    /// introduce Top and Bottom for simplify
    ///
    /// used for finalize process.
    fn top_bottom_rules(self: Box<Self>) -> Box<Self> {
        match self.as_ref() {
            Expression::Not(e) => match e.as_ref() {
                Self::Bottom => top(),
                Self::Top => bottom(),
                _ => e.clone().top_bottom_rules(),
            },
            Expression::And(a, b) => match (a.as_ref(), b.as_ref()) {
                (_, Expression::Top) => a.clone().top_bottom_rules(),
                (_, Expression::Bottom) => bottom(),
                (Expression::Top, _) => b.clone().top_bottom_rules(),
                (Expression::Bottom, _) => bottom(),
                _ => {
                    if a.clone() == !b.clone() {
                        bottom()
                    } else {
                        a.clone().top_bottom_rules() & b.clone().top_bottom_rules()
                    }
                }
            },
            Expression::Or(a, b) => match (a.as_ref(), b.as_ref()) {
                (_, Expression::Top) => top(),
                (_, Expression::Bottom) => a.clone().top_bottom_rules(),
                (Expression::Top, _) => top(),
                (Expression::Bottom, _) => b.clone().top_bottom_rules(),
                _ => {
                    if a.clone() == !b.clone() {
                        top()
                    } else {
                        a.clone().top_bottom_rules() | b.clone().top_bottom_rules()
                    }
                }
            },
            Expression::Implies(a, b) => match (a.as_ref(), b.as_ref()) {
                (_, Expression::Bottom) => not(a.clone().top_bottom_rules()),
                (Expression::Top, _) => b.clone().top_bottom_rules(),
                _ => implies(a.clone().top_bottom_rules(), b.clone().top_bottom_rules()),
            },
            _ => self,
        }
    }
    /// transform to cnf
    fn cnf(self: Box<Self>) -> Box<Self> {
        match self.as_ref() {
            Expression::Not(exp) => not(exp.clone().cnf()),
            Expression::And(exp1, exp2) => and(exp1.clone().cnf(), exp2.clone().cnf()),
            Expression::Or(exp1, exp2) => match exp1.as_ref() {
                Expression::And(exp11, exp12) => and(
                    or(exp11.clone(), exp2.clone()),
                    or(exp12.clone(), exp2.clone()),
                ),
                _ => match exp2.as_ref() {
                    Expression::And(exp21, exp22) => and(
                        or(exp1.clone(), exp22.clone()),
                        or(exp1.clone(), exp21.clone()),
                    ),
                    _ => or(exp1.clone().cnf(), exp2.clone().cnf()),
                },
            },
            _ => self,
        }
    }
    /// transform to dnf
    fn dnf(self: Box<Self>) -> Box<Self> {
        match self.as_ref() {
            Expression::Not(exp) => not(exp.clone().dnf()),
            Expression::Or(exp1, exp2) => or(exp1.clone().dnf(), exp2.clone().dnf()),
            Expression::And(exp1, exp2) => match exp1.as_ref() {
                Expression::Or(exp11, exp12) => or(
                    and(exp11.clone(), exp2.clone()),
                    and(exp12.clone(), exp2.clone()),
                ),
                _ => match exp2.as_ref() {
                    Expression::Or(exp21, exp22) => or(
                        and(exp1.clone(), exp22.clone()),
                        and(exp1.clone(), exp21.clone()),
                    ),
                    _ => and(exp1.clone().dnf(), exp2.clone().dnf()),
                },
            },
            _ => self,
        }
    }
}
impl<P> Expression<P>
where
    P: std::fmt::Debug + Eq + Clone + Default,
{
    pub fn transform_to_cnf(self: Box<Self>) -> Box<Self> {
        let mut expression = self.clone();
        while expression != expression.clone().step_1() {
            println!(" = {:?}", expression.clone().step_1());
            expression = expression.step_1();
        }
        println!(r"\top and \bot eliminated");
        while expression != expression.clone().step_2() {
            println!(" = {:?}", expression.clone().step_2());
            expression = expression.step_2();
        }
        println!(r"iff eliminated");
        while expression != expression.clone().step_3() {
            println!(" = {:?}", expression.clone().step_3());
            expression = expression.step_3();
        }
        println!(r"implies eliminated");
        while expression != expression.clone().step_4() {
            println!("= {:?}", expression.clone().step_4());
            expression = expression.step_4();
        }
        println!(r"apply de morgan");
        while expression != expression.clone().step_5() {
            println!("= {:?}", expression.clone().step_5());
            expression = expression.step_5();
        }
        println!(r"double not");
        while expression != expression.clone().cnf() {
            println!(" = {:?}", expression.clone().cnf());
            expression = expression.cnf();
        }
        expression
    }
    pub fn transform_to_dnf(self: Box<Self>) -> Box<Self> {
        let mut expression = self.clone();
        while expression != expression.clone().step_1() {
            println!(" = {:?}", expression.clone().step_1());
            expression = expression.step_1();
        }
        println!(r"\top and \bot eliminated");
        while expression != expression.clone().step_2() {
            println!(" = {:?}", expression.clone().step_2());
            expression = expression.step_2();
        }
        println!(r"iff eliminated");
        while expression != expression.clone().step_3() {
            println!(" = {:?}", expression.clone().step_3());
            expression = expression.step_3();
        }
        println!(r"implies eliminated");
        while expression != expression.clone().step_4() {
            println!("= {:?}", expression.clone().step_4());
            expression = expression.step_4();
        }
        println!(r"apply de morgan");
        while expression != expression.clone().step_5() {
            println!("= {:?}", expression.clone().step_5());
            expression = expression.step_5();
        }
        println!(r"double not");
        while expression != expression.clone().dnf() {
            println!(" = {:?}", expression.clone().dnf());
            expression = expression.dnf();
        }
        expression
    }
}
//implement some operators for convinience
impl<P> std::ops::BitOr<Box<Expression<P>>> for Box<Expression<P>>
where
    P: Eq,
{
    type Output = Box<Expression<P>>;

    fn bitor(self, rhs: Box<Expression<P>>) -> Self::Output {
        or(self, rhs)
    }
}

impl<P> std::ops::BitAnd<Box<Expression<P>>> for Box<Expression<P>>
where
    P: Eq,
{
    type Output = Box<Expression<P>>;

    fn bitand(self, rhs: Box<Expression<P>>) -> Self::Output {
        and(self, rhs)
    }
}

impl<P> std::ops::Not for Box<Expression<P>>
where
    P: Eq,
{
    type Output = Self;

    fn not(self) -> Self::Output {
        not(self)
    }
}
// Top and Bottom from bool
impl<P> Into<Box<Expression<P>>> for bool
where
    P: Eq,
{
    fn into(self) -> Box<Expression<P>> {
        if self {
            top()
        } else {
            bottom()
        }
    }
}

pub trait IntoExpression<P>
where
    P: Eq,
{
    /// convert to Box\<Expression::Var(self)\>
    fn into(self) -> Box<Expression<P>>;
}

impl<P: Eq> IntoExpression<P> for Box<Expression<P>> {
    fn into(self) -> Box<Expression<P>> {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let exp = implies(not(not(var('P'))), var('Q'));

        assert_eq!(exp.transform_to_cnf(), or(not(var('P')), var('Q')));
        let exp = implies(implies(var('P'), var('Q')), implies(var('R'), var('S')));
        println!("Transforming to CNF");
        exp.clone().transform_to_cnf();
        println!("Transforming to DNF");
        exp.transform_to_dnf();
        let exp = !(implies(var('X'), var('Y') & var('Z')));
        exp.transform_to_dnf();
    }
}
