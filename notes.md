
------

------

------

Observe that $\mathbf{s}_{2i + 1} = u_{k-1} \mathbf{s}_{2i}$ for all $i \in [0, n/2)$. This gives us the $n$ equalities

$$
\begin{array}{rl}
\sum\limits_{i=0}^{k - 1} u_j^{-1} [L_j]^{\mathbf{G}}_{2i} + [P']^{\mathbf{G}}_{2i} + \sum\limits_{i=0}^{k - 1} u_j [R_j]^{\mathbf{G}}_{2i} &= c \mathbf{s}_{2i} \\
\sum\limits_{i=0}^{k - 1} u_j^{-1} [L_j]^{\mathbf{G}}_{2i + 1} + [P']^{\mathbf{G}}_{2i + 1} + \sum\limits_{i=0}^{k - 1} u_j [R_j]^{\mathbf{G}}_{2i + 1} &= c u_{k - 1} \mathbf{s}_{2i}
\end{array}
$$

Replace $u_{k-1}$ with formal indeterminate $X$ and rearrange to obtain

$$
\begin{array}{rl}
X^{-1} [L_{k-1}]^{\mathbf{G}}_{2i} + X [R_{k-1}]^{\mathbf{G}}_{2i} + \left( \sum\limits_{i=0}^{k - 2} u_j^{-1} [L_j]^{\mathbf{G}}_{2i} + [P']^{\mathbf{G}}_{2i} + \sum\limits_{i=0}^{k - 2} u_j [R_j]^{\mathbf{G}}_{2i} \right) &= c \mathbf{s}_{2i} \\
X^{-2} [L_{k-1}]^{\mathbf{G}}_{2i+1} + [R_{k-1}]^{\mathbf{G}}_{2i+1} + X^{-1} \left( \sum\limits_{i=0}^{k - 2} u_j^{-1} [L_j]^{\mathbf{G}}_{2i + 1} + [P']^{\mathbf{G}}_{2i + 1} + \sum\limits_{i=0}^{k - 2} u_j [R_j]^{\mathbf{G}}_{2i + 1} \right) &= c \mathbf{s}_{2i}
\end{array}
$$

The left hand sides are two (Laurent) polynomials with coefficients determined prior to the choice of $u_{k - 1}$, and so the probability that any one of these pairs of polynomials would be distinct and yet agree at $u_{k-1}$ (to the value $c \mathbf{s}_{2i}$) is, by the Schwartz-Zippel lemma and the union bound, at most $\frac{3n/2}{|\ch|}$. We will later bound $\badch_{\tr_{u_{k - 1}}}$ appropriately, and so we can conclude an equality of polynomials and so by equality of coefficients we obtain

$$
\begin{array}{ll}
[L_{k-1}]^{\mathbf{G}}_{2i+1} &= 0\\
[R_{k-1}]^{\mathbf{G}}_{2i} &= 0\\
[L_{k-1}]^{\mathbf{G}}_{2i} &= \sum\limits_{i=0}^{k - 2} u_j^{-1} [L_j]^{\mathbf{G}}_{2i + 1} + [P']^{\mathbf{G}}_{2i + 1} + \sum\limits_{i=0}^{k - 2} u_j [R_j]^{\mathbf{G}}_{2i + 1} \\
[R_{k-1}]^{\mathbf{G}}_{2i+1} &= \sum\limits_{i=0}^{k - 2} u_j^{-1} [L_j]^{\mathbf{G}}_{2i} + [P']^{\mathbf{G}}_{2i} + \sum\limits_{i=0}^{k - 2} u_j [R_j]^{\mathbf{G}}_{2i}
\end{array}
$$

as expected from the honest prover. By substitution we also obtain the $n / 2$ equalities

$$
\begin{array}{ll}
&\sum\limits_{i=0}^{k - 2} u_j^{-1} [L_j]^{\mathbf{G}}_{2i} + [P']^{\mathbf{G}}_{2i} + \sum\limits_{i=0}^{k - 2} u_j [R_j]^{\mathbf{G}}_{2i} \\
&+ u_{k - 1}^{-1} \left( \sum\limits_{i=0}^{k - 2} u_j^{-1} [L_j]^{\mathbf{G}}_{2i + 1} + [P']^{\mathbf{G}}_{2i + 1} + \sum\limits_{i=0}^{k - 2} u_j [R_j]^{\mathbf{G}}_{2i + 1} \right) \\
&= c \mathbf{s}_{2i}
\end{array}
$$

and by substituting for $c$ in our other equality we have that

$$
\begin{array}{ll}
&\sum_{i=0}^{k - 1} u_j^{-1} [L_j]^U + [P']^U + \sum_{i=0}^{k - 1} u_j [R_j]^U \\
&= \mathbf{s}_{2i}^{-1} z \prod_{j=0}^{k - 1} (1 + u_{k - 1 - j} x_3^{2^j}) \\
&\cdot \left( \sum\limits_{i=0}^{k - 2} u_j^{-1} [L_j]^{\mathbf{G}}_{2i} + [P']^{\mathbf{G}}_{2i} + \sum\limits_{i=0}^{k - 2} u_j [R_j]^{\mathbf{G}}_{2i} + u_{k - 1}^{-1} \left( \sum\limits_{i=0}^{k - 2} u_j^{-1} [L_j]^{\mathbf{G}}_{2i + 1} + [P']^{\mathbf{G}}_{2i + 1} + \sum\limits_{i=0}^{k - 2} u_j [R_j]^{\mathbf{G}}_{2i + 1} \right) \right)
\end{array}
$$

holds for $i \in [0, n/2)$. Again replacing $u_{k-1}$ with formal indeterminate $X$ we obtain

$$
\begin{array}{ll}
& X^{-1} [L_{k-1}]^U + \left( \sum_{i=0}^{k - 2} u_j^{-1} [L_j]^U + [P']^U + \sum_{i=0}^{k - 2} u_j [R_j]^U \right) + X [R_{k-1}]^U \\
&= \mathbf{s}_{2i}^{-1} z \left(1 + X x_3 \right) \prod_{j=1}^{k - 1} (1 + u_{k - 1 - j} x_3^{2^j}) \\
&\cdot \left( \sum\limits_{i=0}^{k - 2} u_j^{-1} [L_j]^{\mathbf{G}}_{2i} + [P']^{\mathbf{G}}_{2i} + \sum\limits_{i=0}^{k - 2} u_j [R_j]^{\mathbf{G}}_{2i} + X^{-1} \left( \sum\limits_{i=0}^{k - 2} u_j^{-1} [L_j]^{\mathbf{G}}_{2i + 1} + [P']^{\mathbf{G}}_{2i + 1} + \sum\limits_{i=0}^{k - 2} u_j [R_j]^{\mathbf{G}}_{2i + 1} \right) \right)
\end{array}
$$

Again obtaining on each side (Laurent) polynomials with coefficients determined prior to the choice of $u_{k - 1}$. The probability that any of these $n/2$ pairs of polynomials are distinct and yet agree at $u_{k - 1}$ is $\frac{n}{|\ch|}$ and so by applying the union bound with the probability from before we have $|\badch_{\tr_{u_{k - 1}}}| \leq \frac{5n/2}{|\ch|} \leq \epsilon$. We conclude an equality of polynomials and, comparing the constant terms, we have that

$$
\begin{array}{ll}
&\sum_{i=0}^{k - 2} u_j^{-1} [L_j]^U + [P']^U + \sum_{i=0}^{k - 2} u_j [R_j]^U \\
=&\mathbf{s}_{2i}^{-1} z \prod_{j=1}^{k - 1} (1 + u_{k - 1 - j} x_3^{2^j}) \\
&\cdot \left( \sum\limits_{i=0}^{k - 2} u_j^{-1} [L_j]^{\mathbf{G}}_{2i} + [P']^{\mathbf{G}}_{2i} + \sum\limits_{i=0}^{k - 2} u_j [R_j]^{\mathbf{G}}_{2i} + x_3 \left( \sum\limits_{i=0}^{k - 2} u_j^{-1} [L_j]^{\mathbf{G}}_{2i + 1} + [P']^{\mathbf{G}}_{2i + 1} + \sum\limits_{i=0}^{k - 2} u_j [R_j]^{\mathbf{G}}_{2i + 1} \right) \right)
\end{array}
$$

holds for all $i \in [0, n/2)$. This can be rewritten as

$$
\begin{array}{ll}
&\sum_{i=0}^{k - 2} u_j^{-1} [L_j]^U + [P']^U + \sum_{i=0}^{k - 2} u_j [R_j]^U \\
=&\mathbf{s}_{2i}^{-1} z \prod_{j=1}^{k - 1} (1 + u_{k - 1 - j} x_3^{2^j}) \\
&\cdot \sum\limits_{i=0}^{k - 2} \left( \sum\limits_{j=0}^{2i - 1} x_3^j \left(
  u_j^{-1} [L_j]^{\mathbf{G}}_{2i + j} + [P']^{\mathbf{G}}_{2i + j} + u_j [R_j]^{\mathbf{G}}_{2i + j} \right) \right)
\end{array}
$$

and by substituting $k - 2$ with $r$ we obtain the equivalent

$$
\begin{array}{ll}
&\sum_{i=0}^{r} u_j^{-1} [L_j]^U + [P']^U + \sum_{i=0}^{r} u_j [R_j]^U \\
=&\mathbf{s}_{2^{k - 1 - r} i}^{-1} z \prod_{j=k - 1 - r}^{k - 1} (1 + u_{k - 1 - j} x_3^{2^j}) \\
&\cdot \sum\limits_{i=0}^{r} \left( \sum\limits_{j=0}^{2^{k - 1 - r} - 1} x_3^j \left(
  u_j^{-1} [L_j]^{\mathbf{G}}_{2^{k - 1 - r} + j} + [P']^{\mathbf{G}}_{2^{k - 1 - r} + j} + u_j [R_j]^{\mathbf{G}}_{2^{k - 1 - r} + j} \right) \right)
\end{array}
$$

------------

------------

Observe that for all $i \in [0, n/4)$ we have that $\mathbf{s}_{4i + 2}^{-1} = \mathbf{s}_{4i}^{-1} u_{k - 2}^{-1}$ and so we obtain the $n/4$ equalities

$$
\begin{array}{ll}
&\sum_{i=0}^{k - 2} u_j^{-1} [L_j]^U + [P']^U + \sum_{i=0}^{k - 2} u_j [R_j]^U \\
=&\mathbf{s}_{4i}^{-1} z \prod_{j=1}^{k - 1} (1 + u_{k - 1 - j} x_3^{2^j}) \\
&\cdot \left( \sum\limits_{i=0}^{k - 2} u_j^{-1} [L_j]^{\mathbf{G}}_{4i} + [P']^{\mathbf{G}}_{4i} + \sum\limits_{i=0}^{k - 2} u_j [R_j]^{\mathbf{G}}_{4i} + x_3 \left( \sum\limits_{i=0}^{k - 2} u_j^{-1} [L_j]^{\mathbf{G}}_{4i + 1} + [P']^{\mathbf{G}}_{4i + 1} + \sum\limits_{i=0}^{k - 2} u_j [R_j]^{\mathbf{G}}_{4i + 1} \right) \right) \\

=&\mathbf{s}_{4i}^{-1} u_{k - 2}^{-1} z \prod_{j=1}^{k - 1} (1 + u_{k - 1 - j} x_3^{2^j}) \\
&\cdot \left( \sum\limits_{i=0}^{k - 2} u_j^{-1} [L_j]^{\mathbf{G}}_{4i + 2} + [P']^{\mathbf{G}}_{4i + 2} + \sum\limits_{i=0}^{k - 2} u_j [R_j]^{\mathbf{G}}_{4i + 2} + x_3 \left( \sum\limits_{i=0}^{k - 2} u_j^{-1} [L_j]^{\mathbf{G}}_{4i + 3} + [P']^{\mathbf{G}}_{4i + 3} + \sum\limits_{i=0}^{k - 2} u_j [R_j]^{\mathbf{G}}_{4i + 3} \right) \right)
\end{array}
$$

Replacing $u_{k - 2}$ with formal indeterminate $X$ we obtain the $n/4$ equalities

$$
\begin{array}{ll}
&X^{-1} [L_{k - 2}]^U + \sum_{i=0}^{k - 2} u_j^{-1} [L_j]^U + [P']^U + \sum_{i=0}^{k - 3} u_j [R_j]^U + X [L_{k - 2}]^U \\

=&\mathbf{s}_{4i}^{-1} z (1 + X x_3^2) \prod_{j=2}^{k - 1} (1 + u_{k - 1 - j} x_3^{2^j}) \\
&\cdot \left( X^{-1} [L_{k-2}]^{\mathbf{G}}_{4i} + \sum\limits_{i=0}^{k - 3} u_j^{-1} [L_j]^{\mathbf{G}}_{4i} + [P']^{\mathbf{G}}_{4i} + \sum\limits_{i=0}^{k - 3} u_j [R_j]^{\mathbf{G}}_{4i} + X [R_{k-2}]^{\mathbf{G}}_{4i} + x_3 \left( X^{-1} [L_{k-2}]^{\mathbf{G}}_{4i + 1} + \sum\limits_{i=0}^{k - 3} u_j^{-1} [L_j]^{\mathbf{G}}_{4i + 1} + [P']^{\mathbf{G}}_{4i + 1} + \sum\limits_{i=0}^{k - 3} u_j [R_j]^{\mathbf{G}}_{4i + 1} + X [R_{k-2}]^{\mathbf{G}}_{4i + 1} \right) \right) \\

=&\mathbf{s}_{4i}^{-1} z (X^{-1} + x_3^2) \prod_{j=2}^{k - 1} (1 + u_{k - 1 - j} x_3^{2^j}) \\
&\cdot \left( X^{-1} [L_{k - 2}]^{\mathbf{G}}_{4i + 2} + \sum\limits_{i=0}^{k - 3} u_j^{-1} [L_j]^{\mathbf{G}}_{4i + 2} + [P']^{\mathbf{G}}_{4i + 2} + \sum\limits_{i=0}^{k - 3} u_j [R_j]^{\mathbf{G}}_{4i + 2} + X [R_{k-2}]^{\mathbf{G}}_{4i + 2} + x_3 \left( X^{-1} [L_{k-2}]^{\mathbf{G}}_{4i + 3} + \sum\limits_{i=0}^{k - 3} u_j^{-1} [L_j]^{\mathbf{G}}_{4i + 3} + [P']^{\mathbf{G}}_{4i + 3} + \sum\limits_{i=0}^{k - 3} u_j [R_j]^{\mathbf{G}}_{4i + 3} + X [R_{k-2}]^{\mathbf{G}}_{4i + 3} \right) \right)
\end{array}
$$

This gives two (Laurent) polynomials with coefficients determined prior to the choice of $u_{k - 2}$. The probability that any of these $n / 4$ pairs of polynomials agree at $u_{k - 2}$ but are distinct is $\frac{n}{|\ch|}$ which will inform our later bound. Assuming equality of polynomials by comparing coefficients we obtain the equalities

$$
[R_{k-2}]^{\mathbf{G}}_{4i} = [R_{k-2}]^{\mathbf{G}}_{4i + 1} = [L_{k - 2}]^{\mathbf{G}}_{4i + 2} = [L_{k-2}]^{\mathbf{G}}_{4i + 3} = 0
$$

which allows us to simplify

$$
\begin{array}{ll}
&X^{-1} [L_{k - 2}]^U + \sum_{i=0}^{k - 2} u_j^{-1} [L_j]^U + [P']^U + \sum_{i=0}^{k - 3} u_j [R_j]^U + X [L_{k - 2}]^U \\

=&\mathbf{s}_{4i}^{-1} z (1 + X x_3^2) \prod_{j=2}^{k - 1} (1 + u_{k - 1 - j} x_3^{2^j}) \\
&\cdot \left( X^{-1} [L_{k-2}]^{\mathbf{G}}_{4i} + \sum\limits_{i=0}^{k - 3} u_j^{-1} [L_j]^{\mathbf{G}}_{4i} + [P']^{\mathbf{G}}_{4i} + \sum\limits_{i=0}^{k - 3} u_j [R_j]^{\mathbf{G}}_{4i} + x_3 \left( X^{-1} [L_{k-2}]^{\mathbf{G}}_{4i + 1} + \sum\limits_{i=0}^{k - 3} u_j^{-1} [L_j]^{\mathbf{G}}_{4i + 1} + [P']^{\mathbf{G}}_{4i + 1} + \sum\limits_{i=0}^{k - 3} u_j [R_j]^{\mathbf{G}}_{4i + 1} \right) \right) \\

=&\mathbf{s}_{4i}^{-1} z (X^{-1} + x_3^2) \prod_{j=2}^{k - 1} (1 + u_{k - 1 - j} x_3^{2^j}) \\
&\cdot \left( \sum\limits_{i=0}^{k - 3} u_j^{-1} [L_j]^{\mathbf{G}}_{4i + 2} + [P']^{\mathbf{G}}_{4i + 2} + \sum\limits_{i=0}^{k - 3} u_j [R_j]^{\mathbf{G}}_{4i + 2} + X [R_{k-2}]^{\mathbf{G}}_{4i + 2} + x_3 \left( \sum\limits_{i=0}^{k - 3} u_j^{-1} [L_j]^{\mathbf{G}}_{4i + 3} + [P']^{\mathbf{G}}_{4i + 3} + \sum\limits_{i=0}^{k - 3} u_j [R_j]^{\mathbf{G}}_{4i + 3} + X [R_{k-2}]^{\mathbf{G}}_{4i + 3} \right) \right)
\end{array}
$$