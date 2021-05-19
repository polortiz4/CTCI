// Solution to Exercise 6.1

/// Given 20 bottles with pills where 19 are filled with 1.0g pills and one is filled with 1.1g pills,
/// how can I identify the bottle with heavier weight?
pub fn simulate(expected: u8) -> u8 {
    assert!(expected < 20);
    assert!(expected > 0);

    let mut bottle_to_measure: Vec<f32> = Vec::with_capacity(20);
    // Fill empty bottle with pills. The number of pills for a given bottle is equal to its index
    for n_pills in 1..21 {
        if n_pills == expected {
            bottle_to_measure.push(1.1 * n_pills as f32);
        } else {
            bottle_to_measure.push(n_pills as f32);
        }
    }

    // Simulate the scale
    let total_weight = measure_weight(&bottle_to_measure);

    heavy_pill(total_weight)
}
fn heavy_pill(weight: f32) -> u8 {
    const N_BOTTLES: u16 = 20;
    const BASE_WEIGHT: f32 = 1.0;
    const HEAVY_WEIGHT: f32 = 1.1;
    const DIFF: f32 = HEAVY_WEIGHT - BASE_WEIGHT;

    let extra_weight = weight - ((N_BOTTLES * (N_BOTTLES + 1)) as f32 / 2.0) * BASE_WEIGHT;

    (extra_weight / DIFF).round() as u8
}
fn measure_weight(bottle: &Vec<f32>) -> f32 {
    return bottle.iter().sum();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_heavy_pill() {
        let expected = 8;  // bottle number 8 gets indexed as 7
        assert_eq!(simulate(expected), expected);
        assert_eq!(simulate(1), 1);
        assert_eq!(simulate(19), 19);
    }
}
