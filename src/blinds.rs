#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BossBlind {
    TheHook,
    TheOx,
    TheHouse,
    TheWall,
    TheWheel,
    TheArm,
    TheClub,
    TheFish,
    ThePsychic,
    TheGoad,
    TheWater,
    TheWindow,
    TheManacle,
    TheEye,
    TheMouth,
    ThePlant,
    TheSerpent,
    ThePillar,
    TheNeedle,
    TheHead,
    TheTooth,
    TheFlint,
    TheMark,
}

enum ShowDownBossBlind {
    AmberAcorn,
    VerdantLeaf,
    VioletVessel,
    CrimsonHeart,
    CeruleanBell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Blind {
    Small,
    Big,
    Boss,
}

impl Blind {
    pub fn next(self) -> Self {
        match self {
            Blind::Small => Blind::Big,
            Blind::Big => Blind::Boss,
            Blind::Boss => Blind::Small,
        }
    }
}
