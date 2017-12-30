// Simplified version of `Rng` trait in the `rand` crate
pub trait Rng: Clone {
    fn next_f64(&mut self) -> f64;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CoinFlip {
    Heads,
    Tails,
}

pub fn flip_coin<R: Rng>(rng: &mut R) -> CoinFlip {
    let r = rng.next_f64();
    if r < 0.5 {
        CoinFlip::Heads
    } else {
        CoinFlip::Tails
    }
}

#[cfg(test)]
mod tests {
    extern crate double;
    use super::*;

    // Generate mock implementations of collaborators
    mock_trait!(
        MockRng,
        next_f64() -> f64);
    impl Rng for MockRng {
        mock_method!(next_f64(&mut self) -> f64);
    }

    #[test]
    fn test_coin_flipper_yielding_heads() {
        // GIVEN:
        let mut rng = MockRng::default();
        rng.next_f64.return_value(0.25);

        // WHEN:
        let flip = flip_coin(&mut rng);

        // THEN:
        assert_eq!(CoinFlip::Heads, flip);
        assert!(rng.next_f64.called());
        assert_eq!(1, rng.next_f64.num_calls());
    }

    #[test]
    fn test_coin_flipper_yielding_tails() {
        // GIVEN:
        let mut rng = MockRng::default();
        rng.next_f64.return_value(0.75);

        // WHEN:
        let flip = flip_coin(&mut rng);

        // THEN:
        assert_eq!(CoinFlip::Tails, flip);
        assert!(rng.next_f64.called());
        assert_eq!(1, rng.next_f64.num_calls());
    }
}
