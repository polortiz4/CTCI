// Solution to Exercise 6.4

/// If you have n ants in the verteces of a n-dim polygon,
/// and they start walking in a random edge adjacent to them,
/// what are the odds that at least two of them will run into each other?
/// 
/// # Answer:
/// 
/// The only way this won't happen is if all ants walk in the same direction
/// P(all ants in same direction) = 0.5^(n-1)
/// P(crash) = 1 - 0.5^(n-1)