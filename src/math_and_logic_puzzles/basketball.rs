// Solution to Exercise 6.2

#[derive(Debug)] 
pub enum Game{
    OneHoop,
    ThreeHoops,
    Either,
}

/// Given a probability opf scoring a hoop, what is more likely? To score one hoop, or score two out of three
/// 
/// # Examples:
/// ```
/// use ctci::math_and_logic_puzzles::basketball::basketball;
/// use ctci::math_and_logic_puzzles::basketball::Game;
/// 
/// matches!(basketball(0.4), OneHoop);
/// ```
pub fn basketball(probability: f32) -> Game{
    assert!(probability >= 0.0 && probability <= 1.0);

    match probability{
        x if x==0.0 => Game::Either,
        x if x < 0.5 => Game::OneHoop,
        x if x==0.5 => Game::Either,
        x if x < 1.0 => Game::ThreeHoops,
        x if x==1.0 => Game::Either,
        _ => Game::Either,
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_ball(){
        matches!(basketball(0.0), Game::Either);
        matches!(basketball(0.4), Game::OneHoop);
        matches!(basketball(0.5), Game::Either);
        matches!(basketball(0.6), Game::ThreeHoops);
        matches!(basketball(1.0), Game::Either);
    }
}