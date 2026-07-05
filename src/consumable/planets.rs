use crate::consumable::core::Consumable;
use crate::game::delta::GameDelta;
use crate::game::state::GameState;
use strum_macros::Display;

#[derive(Debug, Copy, Clone, PartialEq, Display)]
#[repr(u8)]
pub enum Planet {
    Mercury,
    Venus,
    Earth,
    Mars,
    Jupiter,
    Saturn,
    Uranus,
    Neptune,
    Pluto,
    PlanetX,
    Ceres,
    Eris,
}

pub fn use_planet_to_deltas(p: Planet, game_state: &GameState) -> Vec<GameDelta> {
    let mut res = Vec::new();

    res.push(GameDelta::Planet { planet: p, diff: 1 });

    // TODO constellation logic

    res
}

pub fn use_planet(game_state: &mut GameState, planet: Planet) {
    let index = planet as usize;

    game_state.planet_levels[index] += 1;
    game_state.last_used = Consumable::Planet(planet);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decks::Deck;
    use crate::game::state::create_game_state;

    #[test]
    fn test_use_planet_increases_level() {
        let mut state = create_game_state(Deck::Red);
        let initial_level = state.planet_levels[Planet::Earth as usize];

        use_planet(&mut state, Planet::Earth);

        assert_eq!(
            state.planet_levels[Planet::Earth as usize],
            initial_level + 1
        );
        assert_eq!(state.last_used, Consumable::Planet(Planet::Earth));
    }

    #[test]
    fn test_use_planet_multiple_times() {
        let mut state = create_game_state(Deck::Red);
        let initial_level = state.planet_levels[Planet::Mars as usize];

        use_planet(&mut state, Planet::Mars);
        use_planet(&mut state, Planet::Mars);
        use_planet(&mut state, Planet::Mars);

        assert_eq!(
            state.planet_levels[Planet::Mars as usize],
            initial_level + 3
        );
        assert_eq!(state.last_used, Consumable::Planet(Planet::Mars));
    }

    #[test]
    fn test_use_planet_does_not_affect_others() {
        let mut state = create_game_state(Deck::Red);
        let initial_venus = state.planet_levels[Planet::Venus as usize];
        let initial_jupiter = state.planet_levels[Planet::Jupiter as usize];

        use_planet(&mut state, Planet::Venus);

        assert_eq!(
            state.planet_levels[Planet::Venus as usize],
            initial_venus + 1
        );
        assert_eq!(
            state.planet_levels[Planet::Jupiter as usize],
            initial_jupiter
        );
    }

    #[test]
    fn test_last_used_planet() {
        let mut state = create_game_state(Deck::Red);

        use_planet(&mut state, Planet::Venus);
        assert_eq!(state.last_used, Consumable::Planet(Planet::Venus));

        use_planet(&mut state, Planet::Earth);
        assert_eq!(state.last_used, Consumable::Planet(Planet::Earth));

        use_planet(&mut state, Planet::Pluto);
        assert_eq!(state.last_used, Consumable::Planet(Planet::Pluto));
    }
}
