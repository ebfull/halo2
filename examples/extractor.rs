use halo2::arithmetic::{compute_inner_product, eval_polynomial, Field, FieldExt};
use halo2::pasta::pallas::Scalar;

const K: usize = 3;
const N: usize = 1 << K;

#[derive(Clone, PartialEq, Debug)]
struct GroupElement([Scalar; N + 1]);

impl GroupElement {
    fn zero() -> Self {
        GroupElement([Scalar::zero(); N + 1])
    }

    fn mul(&self, by: Scalar) -> Self {
        let mut tmp = self.clone();
        for e in tmp.0.iter_mut() {
            *e = *e * by;
        }
        tmp
    }

    fn add(&self, to: &Self) -> Self {
        let mut tmp = self.clone();
        for (a, b) in tmp.0.iter_mut().zip(to.0.iter()) {
            *a = *a + *b;
        }
        tmp
    }
}

fn main() {
    let urs = (0..(N + 1))
        .map(|i| {
            let mut tmp = [Scalar::zero(); N + 1];
            tmp[i] = Scalar::one();
            GroupElement(tmp)
        })
        .collect::<Vec<_>>();
    let g = &urs[0..N];
    let u = &urs[N];

    let multiexp =
        |bases: &[GroupElement], base_v: &GroupElement, scalars: &[Scalar], v: Scalar| {
            assert_eq!(bases.len(), scalars.len());
            let mut acc = GroupElement::zero();
            for (base, scalar) in bases.iter().zip(scalars.iter()) {
                acc = acc.add(&base.mul(*scalar));
            }
            acc = acc.add(&base_v.mul(v));

            acc
        };

    let mut a = (0..N).map(|_| Scalar::rand()).collect::<Vec<_>>();
    let mut p = multiexp(g, u, &a, Field::zero());
    let x = Scalar::rand();
    let mut b = vec![];
    {
        let mut cur = Scalar::one();
        for _ in 0..N {
            b.push(cur);
            cur *= x;
        }
    }
    let orig_b = b.clone();

    let v = eval_polynomial(&a, x);
    assert_eq!(v, compute_inner_product(&a, &b));

    p.0[0] -= v;
    a[0] -= v;

    let mut g = g.to_vec();

    let mut rounds = vec![];
    let mut challenges = vec![];
    let mut challenges_bare = vec![];
    for k in (1..=K).rev() {
        assert_eq!(a.len(), b.len());
        assert_eq!(g.len(), a.len());
        let half = 1 << (k - 1);
        let l = multiexp(
            &g[0..half],
            u,
            &a[half..],
            compute_inner_product(&a[half..], &b[0..half]),
        );
        let r = multiexp(
            &g[half..],
            u,
            &a[0..half],
            compute_inner_product(&a[0..half], &b[half..]),
        );
        rounds.push((l, r));
        let challenge = Scalar::rand();
        let challenge_inv = challenge.invert().unwrap();
        challenges.push((challenge, challenge_inv));
        challenges_bare.push(challenge);

        for i in 0..half {
            a[i] = a[i] + &(a[i + half] * &challenge_inv);
            b[i] = b[i] + &(b[i + half] * &challenge);
            g[i] = g[i].add(&g[i + half].mul(challenge));
        }
        a.truncate(half);
        b.truncate(half);
        g.truncate(half);
    }

    assert_eq!(a.len(), b.len());
    assert_eq!(g.len(), a.len());
    assert_eq!(a.len(), 1);

    let mut acc = p.clone();
    for ((l, r), (challenge, challenge_inv)) in rounds.iter().zip(challenges.iter()) {
        acc = acc.add(&l.mul(*challenge_inv));
        acc = acc.add(&r.mul(*challenge));
    }

    assert_eq!(b[0], compute_b(x, &challenges_bare));
    let s = compute_s(&challenges_bare, Field::one());
    assert_eq!(b[0], compute_inner_product(&s, &orig_b));

    let expected = g[0].mul(a[0]).add(&u.mul(a[0] * b[0]));
    assert_eq!(acc, expected);

    {
        for i in 0..N {
            let mut acc = p.0[i];
            for ((l, r), (challenge, challenge_inv)) in rounds.iter().zip(challenges.iter()) {
                acc += l.0[i] * challenge_inv;
                acc += r.0[i] * challenge;
            }
            assert_eq!(acc, a[0] * s[i]);
        }
    }

    {
        for i in 0..(N >> 1) {
            let mut acc = p.0[i << 1];
            for ((l, r), (challenge, challenge_inv)) in rounds.iter().zip(challenges.iter()) {
                acc += l.0[i << 1] * challenge_inv;
                acc += r.0[i << 1] * challenge;
            }
            assert_eq!(acc, a[0] * s[i << 1]);
        }

        for i in 0..(N >> 1) {
            let mut acc = p.0[(i << 1) + 1];
            for ((l, r), (challenge, challenge_inv)) in rounds.iter().zip(challenges.iter()) {
                acc += l.0[(i << 1) + 1] * challenge_inv;
                acc += r.0[(i << 1) + 1] * challenge;
            }
            assert_eq!(acc, a[0] * *challenges_bare.last().unwrap() * s[i << 1]);
        }

        for i in 0..(N >> 1) {
            let (l, r) = rounds.last().unwrap();
            assert_eq!(l.0[(i << 1) + 1], Scalar::zero());
            assert_eq!(r.0[(i << 1)], Scalar::zero());
        }

        // Substituting for p, we obtain
        let mut lhs = p.0[N];
        for ((l, r), (challenge, challenge_inv)) in rounds.iter().zip(challenges.iter()) {
            lhs += l.0[N] * challenge_inv;
            lhs += r.0[N] * challenge;
        }

        for i in 0..(N >> 1) {
            let mut rhs = p.0[(i << 1) + 1];
            let rounds = &rounds[0..(rounds.len() - 1)];
            for ((l, r), (challenge, challenge_inv)) in rounds.iter().zip(challenges.iter()) {
                rhs += l.0[(i << 1) + 1] * challenge_inv;
                rhs += r.0[(i << 1) + 1] * challenge;
            }
            rhs *= challenges.last().unwrap().1; // this is the inverse
            rhs += p.0[i << 1];
            for ((l, r), (challenge, challenge_inv)) in rounds.iter().zip(challenges.iter()) {
                rhs += l.0[i << 1] * challenge_inv;
                rhs += r.0[i << 1] * challenge;
            }
            rhs *= b[0] * (s[i << 1].invert().unwrap())
        }
    }

    {
        let expected_val = b[0]
            * &((Scalar::one() + x * challenges_bare.last().unwrap())
                .invert()
                .unwrap());
        for i in 0..(1 << (K - 1)) {
            let expected_val = expected_val * (s[i << 1].invert().unwrap());
            let mut term = p.0[(i << 1) + 1];
            {
                let rounds = &rounds[0..(rounds.len() - 1)];
                for ((l, r), (challenge, challenge_inv)) in rounds.iter().zip(challenges.iter()) {
                    term += l.0[(i << 1) + 1] * challenge_inv;
                    term += r.0[(i << 1) + 1] * challenge;
                }
            }
            let expected_val = expected_val * term;
            assert_eq!(rounds.last().unwrap().0 .0[N], expected_val);
        }
    }

    // R_k-1,U
    {
        let expected_val = x
            * b[0]
            * &((Scalar::one() + x * challenges_bare.last().unwrap())
                .invert()
                .unwrap());
        for i in 0..(1 << (K - 1)) {
            let expected_val = expected_val * (s[i << 1].invert().unwrap());
            let mut term = p.0[i << 1];
            {
                let rounds = &rounds[0..(rounds.len() - 1)];
                for ((l, r), (challenge, challenge_inv)) in rounds.iter().zip(challenges.iter()) {
                    term += l.0[i << 1] * challenge_inv;
                    term += r.0[i << 1] * challenge;
                }
            }
            let expected_val = expected_val * term;
            assert_eq!(rounds.last().unwrap().1 .0[N], expected_val);
        }
    }

    {
        let mut lhs = p.0[N];
        {
            let rounds = &rounds[0..(rounds.len() - 1)];
            for ((l, r), (challenge, challenge_inv)) in rounds.iter().zip(challenges.iter()) {
                lhs += l.0[N] * challenge_inv;
                lhs += r.0[N] * challenge;
            }
        }
        let expected_val = b[0]
            * &((Scalar::one() + x * challenges_bare.last().unwrap())
                .invert()
                .unwrap());
        for i in 0..(1 << (K - 1)) {
            let expected_val = expected_val * (s[i << 1].invert().unwrap());
            let mut tmp = p.0[(i << 1) + 1];
            {
                let rounds = &rounds[0..(rounds.len() - 1)];
                for ((l, r), (challenge, challenge_inv)) in rounds.iter().zip(challenges.iter()) {
                    tmp += l.0[(i << 1) + 1] * challenge_inv;
                    tmp += r.0[(i << 1) + 1] * challenge;
                }
            }
            tmp = tmp * x;
            tmp = tmp + p.0[i << 1];
            {
                let rounds = &rounds[0..(rounds.len() - 1)];
                for ((l, r), (challenge, challenge_inv)) in rounds.iter().zip(challenges.iter()) {
                    tmp += l.0[i << 1] * challenge_inv;
                    tmp += r.0[i << 1] * challenge;
                }
            }
            let expected_val = expected_val * tmp;
            assert_eq!(lhs, expected_val);
        }
    }

    {
        let mut tmp = p.0[N];
        {
            let rounds = &rounds[0..(rounds.len() - 1)];
            for ((l, r), (challenge, challenge_inv)) in rounds.iter().zip(challenges.iter()) {
                tmp += l.0[N] * challenge_inv;
                tmp += r.0[N] * challenge;
            }
        }
        assert_eq!(tmp, x * rounds.last().unwrap().0.0[N] + x.invert().unwrap() * rounds.last().unwrap().1.0[N]);
    }

    // {
    //     let s = s.clone();
    //     let mut challenges_bare = challenges_bare.clone();
    //     //let mut new_s = vec![];
    //     let ch = challenges_bare.pop().unwrap();
    //     for i in 0..(1 << (K - 1)) {
    //         assert_eq!(s[(i << 1) + 1], s[i << 1] * ch);
    //         //new_s.push(s[i << 1]);
    //     }
    //     //let s = new_s;
    //     //let mut new_s = vec![];
    //     let ch = challenges_bare.pop().unwrap();
    //     for i in 0..(1 << (K - 2)) {
    //         assert_eq!(s[(i << 2) + 2], s[i << 2] * ch);
    //         //new_s.push(s[i << 1]);
    //     }
    //     let ch = challenges_bare.pop().unwrap();
    //     for i in 0..(1 << (K - 3)) {
    //         assert_eq!(s[(i << 2) + 2], s[i << 2] * ch);
    //         //new_s.push(s[i << 1]);
    //     }
    // }

    {
        assert_eq!(s[0] * challenges_bare[0], s[4]);
        assert_eq!(s[1] * challenges_bare[0], s[5]);
        assert_eq!(s[2] * challenges_bare[0], s[6]);
        assert_eq!(s[3] * challenges_bare[0], s[7]);
    }

    for r in (0..K).rev() {
        for i in 0..(1 << r) {
            {
                assert_eq!(1 << r, (1 << K) / (1 << (K - r)));
                let lhs = ((1 << (K - r)) * i) + (1 << (K - 1 - r));
                let rhs = ((1 << (K - r)) * i);
                println!("r = {}, i = {}, lhs = {}, rhs = {}", r, i, lhs, rhs);
                assert_eq!(s[lhs], s[rhs] * challenges_bare[r]);
                assert_eq!(s[lhs].invert().unwrap(), s[rhs].invert().unwrap() * challenges_bare[r].invert().unwrap());
            }
            // if (i % 2) == 0 {
            //     assert_eq(
            //         s[(1 << (K - 1 - r)) * i],
            //         s[(1 << (K - r)) * i]
            //     );
            // } else {
            //     assert_eq(
            //         s[(1 << (K - 1 - r)) * i],
            //         s[(1 << (K - r)) * i]
            //     );
            // }
        }
    }

    // {
    //     let s = s.clone();
    //     for r in (0..3).rev() {
    //         for i in 0..(1 << K) {
    //             let lhs = ((1 << (K - r)) * i + (K - r)) % (1 << K);
    //             let rhs = ((1 << (K - r)) * i) % (1 << K);
    //             println!("r = {}, i = {}, lhs = {}, rhs = {}", r, i, lhs, rhs);
    //             assert_eq!(s[lhs], s[rhs] * challenges_bare[r]);
    //         }
    //     }
    // }

    // {
    //     for i in 0..(1 << (K - 1)) {
    //         assert_eq!(rounds[0].0.0[i << 1], p.0[(i << 1) + 1]);
    //     }
    // }
}

/// Computes $\prod\limits_{i=0}^{k-1} (1 + u_i x^{2^i})$.
fn compute_b<F: Field>(x: F, challenges: &[F]) -> F {
    let mut tmp = F::one();
    let mut cur = x;
    for challenge in challenges.iter().rev() {
        tmp *= F::one() + &(*challenge * &cur);
        cur *= cur;
    }
    tmp
}

/// Computes the coefficients of $g(X) = \prod\limits_{i=0}^{k-1} (1 + u_i X^{2^i})$.
fn compute_s<F: Field>(challenges: &[F], init: F) -> Vec<F> {
    assert!(!challenges.is_empty());
    let mut v = vec![F::zero(); 1 << challenges.len()];
    v[0] = init;

    for (len, challenge) in challenges
        .iter()
        .rev()
        .enumerate()
        .map(|(i, challenge)| (1 << i, challenge))
    {
        let (left, right) = v.split_at_mut(len);
        let right = &mut right[0..len];
        right.copy_from_slice(&left);
        for v in right {
            *v *= challenge;
        }
    }

    v
}
