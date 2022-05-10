use std::fmt;

use sea_orm::{EnumIter, Iterable};

#[example::example(
    table = "best_selling_video_games",
    platforms(type = "Platforms", fn = "from_str"),
    developer = "Developer",
    publisher(type = "Publisher", fn = "from_str")
)]
#[derive(Debug, EnumIter)]
pub enum BestSellingVideoGame {}

#[derive(Debug)]
pub enum Platform {
    MultiPlatform,
    NintendoSwitch,
    Nes,
    Xbox360,
    NintendoDs,
    Wii,
    WiiU,
    Ps2,
    GameBoyColor,
    GameBoyAdvance,
    Nintendo3ds,
    Nintendo64,
    GameBoy,
}

pub struct Platforms(&'static [Platform]);

impl Platforms {
    const fn from_str(s: &'static str) -> Self {
        match s.as_bytes() {
            b"Game Boy Advance" => Platforms(&[Platform::GameBoyAdvance]),
            b"Game Boy / Color" => Platforms(&[Platform::GameBoy, Platform::GameBoyColor]),
            b"Game Boy Color" => Platforms(&[Platform::GameBoyColor]),
            b"Game Boy / NES" => Platforms(&[Platform::GameBoy, Platform::Nes]),
            b"Multi-platform" => Platforms(&[Platform::MultiPlatform]),
            b"NES" => Platforms(&[Platform::Nes]),
            b"Nintendo 3DS" => Platforms(&[Platform::Nintendo3ds]),
            b"Nintendo 64 / DS" => Platforms(&[Platform::Nintendo64, Platform::NintendoDs]),
            b"Nintendo DS" => Platforms(&[Platform::NintendoDs]),
            b"Nintendo Switch" => Platforms(&[Platform::NintendoSwitch]),
            b"PS3 / Xbox 360" => Platforms(&[Platform::Ps2, Platform::Xbox360]),
            b"Wii" => Platforms(&[Platform::Wii]),
            b"Wii U / Nintendo Switch" | b"Wii U / Switch" => {
                Platforms(&[Platform::WiiU, Platform::NintendoSwitch])
            }
            b"Xbox 360" => Platforms(&[Platform::Xbox360]),
            _ => panic!("{}", s),
        }
    }
}

impl fmt::Debug for Platforms {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[derive(Debug)]
pub enum Developer {
    BandaiNamcoStudiosSoraLtd,
    BethesdaGameStudios,
    BlizzardEntertainment,
    CdProjektRed,
    EaMobile,
    EaVancouver,
    GameFreak,
    GearboxSoftware,
    GoodScienceStudio,
    InfinityWard,
    InfinityWardSledgehammer,
    MojangStudios,
    Namco,
    NintendoEad,
    NintendoEpd,
    NintendoRD1,
    NintendoRD4,
    NoBrakesGames,
    PubgCorporation,
    ReLogic,
    RockstarNorth,
    RockstarSanDiego,
    RockstarStudios,
    SonicTeam,
    Treyarch,
}

#[derive(Debug)]
pub enum Publisher {
    TwoKGames,
    Activision,
    BethesdaSoftworks,
    BlizzardEntertainment,
    CdProjekt,
    CurveDigital,
    EaSports,
    ElectronicArts,
    Namco,
    Nintendo,
    NintendoThePokemonCompany,
    PubgCorporation,
    ReLogic505Games,
    RockstarGames,
    Sega,
    XboxGameStudios,
}

impl Publisher {
    const fn from_str(s: &'static str) -> Self {
        match s.as_bytes() {
            b"2K Games" => Publisher::TwoKGames,
            b"Activision" => Publisher::Activision,
            b"Bethesda Softworks" => Publisher::BethesdaSoftworks,
            b"Blizzard Entertainment" => Publisher::BlizzardEntertainment,
            b"CD Projekt" => Publisher::CdProjekt,
            b"Curve Digital" => Publisher::CurveDigital,
            b"EA Sports" => Publisher::EaSports,
            b"Electronic Arts" => Publisher::ElectronicArts,
            b"Namco" => Publisher::Namco,
            b"Nintendo" => Publisher::Nintendo,
            b"Nintendo / The Pokemon Company" => Publisher::NintendoThePokemonCompany,
            b"PUBG Corporation" => Publisher::PubgCorporation,
            b"Re-Logic / 505 Games" => Publisher::ReLogic505Games,
            b"Rockstar Games" => Publisher::RockstarGames,
            b"Sega" => Publisher::Sega,
            b"Xbox Game Studios" => Publisher::XboxGameStudios,
            _ => panic!("{}", s),
        }
    }
}

#[test]
fn it_works() {
    for game in BestSellingVideoGame::iter() {
        println!(
            "rank: {}, name: {} sales: {} series: {} platforms: {:?} initial release date: {} developer: {:?} publisher: {:?}",
            game.rank(),
            game.as_str(),
            game.sales(),
            game.series(),
            game.platforms(),
            game.initial_release_date(),
            game.developer(),
            game.publisher()
        );
    }
}
