# Transform Propositional Logic expression to Conjugate Normal Form and Disjunction Normal Form

This crate translate propositional logic expression to CNF and DNF. You can use result of translatation via Debug trait.
Debug trait implemented to emit $\LaTeX$ form.
This crate recursively transform propositional logic expression .

## Step to transform

* Step1 Eliminate Top and Bottom.
using these rules.
  * $\bot \simeq A\land \lnot A$
  * $\top \simeq A\lor \lnot A$
* Step2 Eliminate Iff expression
using this rule.
  * $A \leftrightarrow B \simeq (A\to B)\land(B\to A)$
* Step3 Eliminate Implies expression
using this rule.
  * $A\to B \simeq (\lnot A \lor B)$  
* Step4 Using De morgan's law to remove outer not
  using these rules.
  * $\lnot(A\lor B) \simeq (\lnot A \land \lnot B) $
  * $\lnot(A\land B) \simeq (\lnot A\lor \lnot B)$
* Step5 eliminate double not.
  using this rule.
  * $\lnot\lnot A \simeq A$
* Step6- $\alpha$ transform to DNF.
  using these rules.
  * $A \lor (B \land C) \simeq (A\lor B)\land(A\lor C) $
  * $(A\land B )\lor  C \simeq (A\lor C)\land(B\lor C) $
* Step6- $\beta$ transform to CNF.
  using these rules.
  * $A \land (B \lor C) \simeq (A\land B)\lor(A\land C) $
  * $(A\lor B )\land  C \simeq (A\land C)\lor(B\land C) $
* Step7 simplyfy.
  using $\top$ and $\bot$ rule to simplyfy expression.

## Example

```rust
    impl IntoExpression<char> for char {
        fn into(self) -> Box<Expression<char>> {
            var(self)
        }
    }
    fn main(){
        let exp = implies(!!var('P'), 'Q');
        assert_eq!(exp.transform_to_cnf(), !var('P') | 'Q');
    }
```

## Including result to documents

### $\LaTeX$

in preamble we need to import amssmyb and amsmath.

```latex
\usepackage{amssymb}
\usepackage{amsmath}
```

### Markdown

on github dialect just paste it in environment.
