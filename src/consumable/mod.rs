pub mod planets;
pub mod spectrals;
pub mod tarots;

pub use self::planets::*;
pub use self::spectrals::*;
pub use self::tarots::*;

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum Consumable {
    Tarot(Tarot),
    Planet(Planet),
    Spectral(Spectral),
}
