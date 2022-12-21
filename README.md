<script type="text/javascript" async src="https://cdnjs.cloudflare.com/ajax/libs/mathjax/2.7.7/MathJax.js?config=TeX-MML-AM_CHTML">
</script>
<script type="text/x-mathjax-config">
 MathJax.Hub.Config({
 tex2jax: {
 inlineMath: [['$', '$'] ],
 displayMath: [ ['$$','$$'], ["\\[","\\]"] ]
 }
 });
</script>
# Transform Propositional Logic expression to Conjugate Normal Form and Disjunction Normal Form.
This crate translate propositional logic expression to CNF and DNF. You can use result of translatation via Debug trait.
This crate recursively transform propositional logic expression .

## Step to transform.

* Step1 Eliminate Top and Bottom

* Step2 Eliminate Iff expression

* Step3 Eliminate Implies expression

* Step4 Using De morgan's law to remove outer not

* Step5 Using $ \lnot \lnot A \simeq A $ to get literal.

* Step6-$\alpha$ Using $ A \lor (B \land C) \simeq (A\lor B)\land(A\lor C)  ,$ to transform DNF
 
* Step6-$\beta$ Using $ A \land (B \lor C) \simeq (A\land B)\lor(A\land C)  $ to transform CNF
 