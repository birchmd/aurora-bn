use super::FieldElement;
use rand::{rngs::StdRng, Rng, SeedableRng};

fn can_invert<F: FieldElement>() {
    let mut a = F::one();

    for _ in 0..10000 {
        assert_eq!(a * a.inverse().unwrap(), F::one());

        a = a + F::one();
    }

    a = -F::one();
    for _ in 0..10000 {
        assert_eq!(a * a.inverse().unwrap(), F::one());

        a = a - F::one();
    }

    assert_eq!(F::zero().inverse(), None);
}

fn rand_element_eval<F: FieldElement, R: Rng>(rng: &mut R) {
    for _ in 0..100 {
        let a = F::random(rng);
        let b = F::random(rng);
        let c = F::random(rng);
        let d = F::random(rng);

        assert_eq!((a + b) * (c + d), (a * c) + (b * c) + (a * d) + (b * d));
    }
}

fn rand_element_squaring<F: FieldElement, R: Rng>(rng: &mut R) {
    for _ in 0..100 {
        let a = F::random(rng);

        assert!(a * a == a.squared());
    }

    let mut cur = F::zero();
    for _ in 0..100 {
        assert_eq!(cur.squared(), cur * cur);

        cur = cur + F::one();
    }
}

fn rand_element_addition_and_negation<F: FieldElement, R: Rng>(rng: &mut R) {
    for _ in 0..100 {
        let a = F::random(rng);

        assert_eq!(a + (-a), F::zero());
    }

    for _ in 0..100 {
        let mut a = F::random(rng);
        let r = F::random(rng);
        let mut b = a + r;

        for _ in 0..10 {
            let r = F::random(rng);
            a = a + r;
            b = b + r;

            let r = F::random(rng);
            a = a - r;
            b = b - r;

            let r = F::random(rng);
            a = a + (-(-r));
            b = b + (-(-r));

            let r = F::random(rng);
            a = a - r;
            b = b + (-r);

            let r = F::random(rng);
            a = a + (-r);
            b = b - r;
        }

        b = b - r;
        assert_eq!(a, b);
    }
}

fn rand_element_inverse<F: FieldElement, R: Rng>(rng: &mut R) {
    for _ in 0..10000 {
        let a = F::random(rng);
        assert!(a.inverse().unwrap() * a == F::one());
        let b = F::random(rng);
        assert_eq!((a * b) * (a.inverse().unwrap()), b);
    }
}

fn rand_element_multiplication<F: FieldElement, R: Rng>(rng: &mut R) {
    // If field is not associative under multiplication, 1/8 of all triplets a, b, c
    // will fail the test (a*b)*c = a*(b*c).

    for _ in 0..250 {
        let a = F::random(rng);
        let b = F::random(rng);
        let c = F::random(rng);

        assert_eq!((a * b) * c, a * (b * c));
    }
}

pub fn field_trials<F: FieldElement>() {
    can_invert::<F>();

    assert_eq!(-F::zero(), F::zero());
    assert_eq!(-F::one() + F::one(), F::zero());
    assert_eq!(F::zero() - F::zero(), F::zero());

    let seed: [u8; 32] = [
        0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x93, 0x4d, 0x0, 0x0, 0x0, 0x0, 0x0, 0x2, 0xed, 0xb2, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x5, 0x0d, 0x0, 0x0, 0x0, 0x0, 0x0, 0x2, 0xee, 0x67,
    ];
    let mut rng = StdRng::from_seed(seed);

    rand_element_squaring::<F, StdRng>(&mut rng);
    rand_element_addition_and_negation::<F, StdRng>(&mut rng);
    rand_element_multiplication::<F, StdRng>(&mut rng);
    rand_element_inverse::<F, StdRng>(&mut rng);
    rand_element_eval::<F, StdRng>(&mut rng);
}
