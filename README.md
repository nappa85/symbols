# Symbols

## What is this?

![What the Fuck Did You Just Bring Upon This Cursed Land](./img/meme.jpg)

This is an utility to build a proc-macro that connects to a database, retrieves data from given table and populates an enum variants with primary keys values.<br />
It also generates a method for every non-primary-key field, and, when there are multiple primary keys, a costructor for every possible subset of primary keys.

## Replacements

Replacements are done using annotated parameters.<br/>
Basic replacements are written in the form `#[macro(field = "enum")]` or `#[macro(field(type = "enum"))]`, where we are telling to replace string values from `field` with variants from enum `enum`, variant names will be the CamelCase version of field value.<br />
Advanced replacements are done in the form `#[macro(field(type = "bar", fn = "foo"))]`, where we are telling to replace string values from `field` with a call to method `foo` from struct/enum `bar`, method output is expected to be of type `bar`.<br />
*WARNING:* Since all produced methods are `const`, also methods you pass this way must be `const`.

### Cache

To avoid flooding the database with requests, light up macro run times and be able to work offline, data cache files are stored in crate folder under the name of `<table>.cache`.<br />
What does crate folder means? Well, if you're working directly on this crate, you'll find cache files in crate folder. If you're using this crate as a dependency of your project, you'll find cache files in cargo home folder, under git/checkouts folders (e.g. `~/.cargo/git/checkouts/symbols-3627ca5bd9a20120/4e682cf/<table>.cache`).<br />
Quick way to delete it is to run
```bash
find ~/.cargo/git -name *.cache -delete
```

### Examples

You can find a basic example in [example](./example) folder, it uses a mariadb container to load a database, you can run it with:
```bash
docker-compose run rust
```
Example code
```rust
#[example::example(
    table = "best_selling_video_games",
    platforms(type = "Platforms", fn = "from_str"),
    developer = "Developer",
    publisher(type = "Publisher", fn = "from_str")
)]
pub enum BestSellingVideoGame {}
```
would expand to
```rust
pub enum BestSellingVideoGame {
    AnimalCrossingNewHorizons,
    Borderlands2,
    CallOfDutyBlackOps,
    CallOfDutyBlackOpsIi,
    CallOfDutyModernWarfare,
    CallOfDutyModernWarfare2,
    CallOfDutyModernWarfare3,
    DiabloIiiReaperOfSouls,
    DuckHunt,
    Fifa18,
    GrandTheftAutoIv,
    GrandTheftAutoV,
    GrandTheftAutoSanAndreas,
    HumanFallFlat,
    KinectAdventures,
    MarioKart8Deluxe,
    MarioKartDs,
    MarioKartWii,
    Minecraft,
    NewSuperMarioBros,
    NewSuperMarioBrosUDeluxeLuigiU,
    NewSuperMarioBrosWii,
    Nintendogs,
    PacMan,
    PokemonDiamondPearlPlatinum,
    PokemonGoldSilverCrystal,
    PokemonRedGreenBlueYellow,
    PokemonRubySapphireEmerald,
    PokemonSunMoonUltraSunUltraMoon,
    PokemonSwordShield,
    PubgBattlegrounds,
    RedDeadRedemption,
    RedDeadRedemption2,
    SonicTheHedgehog,
    SuperMario64Ds,
    SuperMarioBros,
    SuperMarioBros3,
    SuperMarioOdyssey,
    SuperMarioWorld,
    SuperSmashBrosUltimate,
    Terraria,
    TetrisEa,
    TetrisNintendo,
    TheElderScrollsVSkyrim,
    TheLegendOfZeldaBreathOfTheWild,
    TheWitcher3HeartsOfStoneBloodAndWine,
    WiiFitPlus,
    WiiPlay,
    WiiSports,
    WiiSportsResort,
}

impl BestSellingVideoGame {
    pub const fn initial_release_date(&self) -> &'static str {
        match self {
            BestSellingVideoGame::AnimalCrossingNewHorizons => "March 20, 2020",
            BestSellingVideoGame::Borderlands2 => "September 18, 2012",
            BestSellingVideoGame::CallOfDutyBlackOps => "November 9, 2010",
            BestSellingVideoGame::CallOfDutyBlackOpsIi => "November 12, 2012",
            BestSellingVideoGame::CallOfDutyModernWarfare => "October 25, 2019",
            BestSellingVideoGame::CallOfDutyModernWarfare2 => "November 10, 2009",
            BestSellingVideoGame::CallOfDutyModernWarfare3 => "November 8, 2011",
            BestSellingVideoGame::DiabloIiiReaperOfSouls => "May 16, 2012",
            BestSellingVideoGame::DuckHunt => "April 21, 1984",
            BestSellingVideoGame::Fifa18 => "September 29, 2017",
            BestSellingVideoGame::GrandTheftAutoIv => "April 29, 2008",
            BestSellingVideoGame::GrandTheftAutoV => "September 17, 2013",
            BestSellingVideoGame::GrandTheftAutoSanAndreas => "October 26, 2004",
            BestSellingVideoGame::HumanFallFlat => "July 22, 2016",
            BestSellingVideoGame::KinectAdventures => "November 4, 2010",
            BestSellingVideoGame::MarioKart8Deluxe => "May 29, 2014",
            BestSellingVideoGame::MarioKartDs => "November 14, 2005",
            BestSellingVideoGame::MarioKartWii => "April 10, 2008",
            BestSellingVideoGame::Minecraft => "November 18, 2011",
            BestSellingVideoGame::NewSuperMarioBros => "May 15, 2006",
            BestSellingVideoGame::NewSuperMarioBrosUDeluxeLuigiU => "November 18, 2012",
            BestSellingVideoGame::NewSuperMarioBrosWii => "November 11, 2009",
            BestSellingVideoGame::Nintendogs => "April 21, 2005",
            BestSellingVideoGame::PacMan => "May 22, 1980",
            BestSellingVideoGame::PokemonDiamondPearlPlatinum => "September 28, 2006",
            BestSellingVideoGame::PokemonGoldSilverCrystal => "November 21, 1999",
            BestSellingVideoGame::PokemonRedGreenBlueYellow => "February 27, 1996",
            BestSellingVideoGame::PokemonRubySapphireEmerald => "November 21, 2002",
            BestSellingVideoGame::PokemonSunMoonUltraSunUltraMoon => "November 18, 2016",
            BestSellingVideoGame::PokemonSwordShield => "November 15, 2019",
            BestSellingVideoGame::PubgBattlegrounds => "December 20, 2017",
            BestSellingVideoGame::RedDeadRedemption => "May 18, 2010",
            BestSellingVideoGame::RedDeadRedemption2 => "October 26, 2018",
            BestSellingVideoGame::SonicTheHedgehog => "June 23, 1991",
            BestSellingVideoGame::SuperMario64Ds => "June 23, 1996",
            BestSellingVideoGame::SuperMarioBros => "September 13, 1985",
            BestSellingVideoGame::SuperMarioBros3 => "October 23, 1988",
            BestSellingVideoGame::SuperMarioOdyssey => "October 27, 2017",
            BestSellingVideoGame::SuperMarioWorld => "November 21, 1990",
            BestSellingVideoGame::SuperSmashBrosUltimate => "December 7, 2018",
            BestSellingVideoGame::Terraria => "May 16, 2011",
            BestSellingVideoGame::TetrisEa => "September 12, 2006",
            BestSellingVideoGame::TetrisNintendo => "June 14, 1989",
            BestSellingVideoGame::TheElderScrollsVSkyrim => "November 11, 2011",
            BestSellingVideoGame::TheLegendOfZeldaBreathOfTheWild => "March 3, 2017",
            BestSellingVideoGame::TheWitcher3HeartsOfStoneBloodAndWine => "May 19, 2015",
            BestSellingVideoGame::WiiFitPlus => "December 1, 2007",
            BestSellingVideoGame::WiiPlay => "December 2, 2006",
            BestSellingVideoGame::WiiSports => "November 19, 2006",
            BestSellingVideoGame::WiiSportsResort => "June 25, 2009",
        }
    }
    pub const fn rank(&self) -> i8 {
        match self {
            BestSellingVideoGame::AnimalCrossingNewHorizons => 15,
            BestSellingVideoGame::Borderlands2 => 33,
            BestSellingVideoGame::CallOfDutyBlackOps => 32,
            BestSellingVideoGame::CallOfDutyBlackOpsIi => 38,
            BestSellingVideoGame::CallOfDutyModernWarfare => 20,
            BestSellingVideoGame::CallOfDutyModernWarfare2 => 48,
            BestSellingVideoGame::CallOfDutyModernWarfare3 => 31,
            BestSellingVideoGame::DiabloIiiReaperOfSouls => 20,
            BestSellingVideoGame::DuckHunt => 25,
            BestSellingVideoGame::Fifa18 => 39,
            BestSellingVideoGame::GrandTheftAutoIv => 35,
            BestSellingVideoGame::GrandTheftAutoV => 2,
            BestSellingVideoGame::GrandTheftAutoSanAndreas => 27,
            BestSellingVideoGame::HumanFallFlat => 20,
            BestSellingVideoGame::KinectAdventures => 39,
            BestSellingVideoGame::MarioKart8Deluxe => 7,
            BestSellingVideoGame::MarioKartDs => 44,
            BestSellingVideoGame::MarioKartWii => 16,
            BestSellingVideoGame::Minecraft => 1,
            BestSellingVideoGame::NewSuperMarioBros => 18,
            BestSellingVideoGame::NewSuperMarioBrosUDeluxeLuigiU => 50,
            BestSellingVideoGame::NewSuperMarioBrosWii => 19,
            BestSellingVideoGame::Nintendogs => 42,
            BestSellingVideoGame::PacMan => 13,
            BestSellingVideoGame::PokemonDiamondPearlPlatinum => 36,
            BestSellingVideoGame::PokemonGoldSilverCrystal => 24,
            BestSellingVideoGame::PokemonRedGreenBlueYellow => 8,
            BestSellingVideoGame::PokemonRubySapphireEmerald => 49,
            BestSellingVideoGame::PokemonSunMoonUltraSunUltraMoon => 34,
            BestSellingVideoGame::PokemonSwordShield => 43,
            BestSellingVideoGame::PubgBattlegrounds => 5,
            BestSellingVideoGame::RedDeadRedemption => 46,
            BestSellingVideoGame::RedDeadRedemption2 => 11,
            BestSellingVideoGame::SonicTheHedgehog => 41,
            BestSellingVideoGame::SuperMario64Ds => 47,
            BestSellingVideoGame::SuperMarioBros => 6,
            BestSellingVideoGame::SuperMarioBros3 => 37,
            BestSellingVideoGame::SuperMarioOdyssey => 45,
            BestSellingVideoGame::SuperMarioWorld => 30,
            BestSellingVideoGame::SuperSmashBrosUltimate => 29,
            BestSellingVideoGame::Terraria => 9,
            BestSellingVideoGame::TetrisEa => 3,
            BestSellingVideoGame::TetrisNintendo => 11,
            BestSellingVideoGame::TheElderScrollsVSkyrim => 20,
            BestSellingVideoGame::TheLegendOfZeldaBreathOfTheWild => 28,
            BestSellingVideoGame::TheWitcher3HeartsOfStoneBloodAndWine => 14,
            BestSellingVideoGame::WiiFitPlus => 10,
            BestSellingVideoGame::WiiPlay => 26,
            BestSellingVideoGame::WiiSports => 4,
            BestSellingVideoGame::WiiSportsResort => 17,
        }
    }
    pub const fn sales(&self) -> &'static str {
        match self {
            BestSellingVideoGame::AnimalCrossingNewHorizons => "37,620,000",
            BestSellingVideoGame::Borderlands2 => "26,000,000",
            BestSellingVideoGame::CallOfDutyBlackOps => "26,200,000",
            BestSellingVideoGame::CallOfDutyBlackOpsIi => "24,200,000",
            BestSellingVideoGame::CallOfDutyModernWarfare => "30,000,000",
            BestSellingVideoGame::CallOfDutyModernWarfare2 => "22,700,000",
            BestSellingVideoGame::CallOfDutyModernWarfare3 => "26,500,000",
            BestSellingVideoGame::DiabloIiiReaperOfSouls => "30,000,000",
            BestSellingVideoGame::DuckHunt => "28,300,000",
            BestSellingVideoGame::Fifa18 => "24,000,000",
            BestSellingVideoGame::GrandTheftAutoIv => "25,000,000",
            BestSellingVideoGame::GrandTheftAutoV => "160,000,000",
            BestSellingVideoGame::GrandTheftAutoSanAndreas => "27,500,000",
            BestSellingVideoGame::HumanFallFlat => "30,000,000",
            BestSellingVideoGame::KinectAdventures => "24,000,000",
            BestSellingVideoGame::MarioKart8Deluxe => "51,810,000",
            BestSellingVideoGame::MarioKartDs => "23,600,000",
            BestSellingVideoGame::MarioKartWii => "37,380,000",
            BestSellingVideoGame::Minecraft => "238,000,000",
            BestSellingVideoGame::NewSuperMarioBros => "30,800,000",
            BestSellingVideoGame::NewSuperMarioBrosUDeluxeLuigiU => "21,600,000",
            BestSellingVideoGame::NewSuperMarioBrosWii => "30,320,000",
            BestSellingVideoGame::Nintendogs => "23,960,000",
            BestSellingVideoGame::PacMan => "42,071,635",
            BestSellingVideoGame::PokemonDiamondPearlPlatinum => "24,730,000",
            BestSellingVideoGame::PokemonGoldSilverCrystal => "29,490,000",
            BestSellingVideoGame::PokemonRedGreenBlueYellow => "47,520,000",
            BestSellingVideoGame::PokemonRubySapphireEmerald => "22,540,000",
            BestSellingVideoGame::PokemonSunMoonUltraSunUltraMoon => "25,310,000",
            BestSellingVideoGame::PokemonSwordShield => "23,900,000",
            BestSellingVideoGame::PubgBattlegrounds => "75,000,000",
            BestSellingVideoGame::RedDeadRedemption => "23,000,000",
            BestSellingVideoGame::RedDeadRedemption2 => "43,000,000",
            BestSellingVideoGame::SonicTheHedgehog => "23,982,960",
            BestSellingVideoGame::SuperMario64Ds => "22,960,000",
            BestSellingVideoGame::SuperMarioBros => "58,000,000",
            BestSellingVideoGame::SuperMarioBros3 => "24,430,000",
            BestSellingVideoGame::SuperMarioOdyssey => "23,020,000",
            BestSellingVideoGame::SuperMarioWorld => "26,662,500",
            BestSellingVideoGame::SuperSmashBrosUltimate => "27,400,000",
            BestSellingVideoGame::Terraria => "44,000,000",
            BestSellingVideoGame::TetrisEa => "100,000,000",
            BestSellingVideoGame::TetrisNintendo => "43,000,000",
            BestSellingVideoGame::TheElderScrollsVSkyrim => "30,000,000",
            BestSellingVideoGame::TheLegendOfZeldaBreathOfTheWild => "27,490,000",
            BestSellingVideoGame::TheWitcher3HeartsOfStoneBloodAndWine => "40,000,000",
            BestSellingVideoGame::WiiFitPlus => "43,800,000",
            BestSellingVideoGame::WiiPlay => "28,020,000",
            BestSellingVideoGame::WiiSports => "82,900,000",
            BestSellingVideoGame::WiiSportsResort => "33,140,000",
        }
    }
    pub const fn as_str(&self) -> &'static str {
        match self {
            BestSellingVideoGame::AnimalCrossingNewHorizons => "Animal Crossing: New Horizons",
            BestSellingVideoGame::Borderlands2 => "Borderlands 2",
            BestSellingVideoGame::CallOfDutyBlackOps => "Call of Duty: Black Ops",
            BestSellingVideoGame::CallOfDutyBlackOpsIi => "Call of Duty: Black Ops II",
            BestSellingVideoGame::CallOfDutyModernWarfare => "Call of Duty: Modern Warfare",
            BestSellingVideoGame::CallOfDutyModernWarfare2 => "Call of Duty: Modern Warfare 2",
            BestSellingVideoGame::CallOfDutyModernWarfare3 => "Call of Duty: Modern Warfare 3",
            BestSellingVideoGame::DiabloIiiReaperOfSouls => "Diablo III / Reaper of Souls",
            BestSellingVideoGame::DuckHunt => "Duck Hunt",
            BestSellingVideoGame::Fifa18 => "FIFA 18",
            BestSellingVideoGame::GrandTheftAutoIv => "Grand Theft Auto IV",
            BestSellingVideoGame::GrandTheftAutoV => "Grand Theft Auto V",
            BestSellingVideoGame::GrandTheftAutoSanAndreas => "Grand Theft Auto: San Andreas",
            BestSellingVideoGame::HumanFallFlat => "Human: Fall Flat",
            BestSellingVideoGame::KinectAdventures => "Kinect Adventures!",
            BestSellingVideoGame::MarioKart8Deluxe => "Mario Kart 8 / Deluxe",
            BestSellingVideoGame::MarioKartDs => "Mario Kart DS",
            BestSellingVideoGame::MarioKartWii => "Mario Kart Wii",
            BestSellingVideoGame::Minecraft => "Minecraft",
            BestSellingVideoGame::NewSuperMarioBros => "New Super Mario Bros.",
            BestSellingVideoGame::NewSuperMarioBrosUDeluxeLuigiU => {
                "New Super Mario Bros. U / Deluxe / Luigi U"
            }
            BestSellingVideoGame::NewSuperMarioBrosWii => "New Super Mario Bros. Wii",
            BestSellingVideoGame::Nintendogs => "Nintendogs",
            BestSellingVideoGame::PacMan => "Pac-Man",
            BestSellingVideoGame::PokemonDiamondPearlPlatinum => {
                "Pokemon Diamond / Pearl / Platinum"
            }
            BestSellingVideoGame::PokemonGoldSilverCrystal => "Pokemon Gold / Silver / Crystal",
            BestSellingVideoGame::PokemonRedGreenBlueYellow => {
                "Pokemon Red / Green / Blue / Yellow"
            }
            BestSellingVideoGame::PokemonRubySapphireEmerald => "Pokemon Ruby / Sapphire / Emerald",
            BestSellingVideoGame::PokemonSunMoonUltraSunUltraMoon => {
                "Pokemon Sun / Moon / Ultra Sun / Ultra Moon"
            }
            BestSellingVideoGame::PokemonSwordShield => "Pokemon Sword / Shield",
            BestSellingVideoGame::PubgBattlegrounds => "PUBG: Battlegrounds",
            BestSellingVideoGame::RedDeadRedemption => "Red Dead Redemption",
            BestSellingVideoGame::RedDeadRedemption2 => "Red Dead Redemption 2",
            BestSellingVideoGame::SonicTheHedgehog => "Sonic the Hedgehog",
            BestSellingVideoGame::SuperMario64Ds => "Super Mario 64 / DS",
            BestSellingVideoGame::SuperMarioBros => "Super Mario Bros.",
            BestSellingVideoGame::SuperMarioBros3 => "Super Mario Bros. 3",
            BestSellingVideoGame::SuperMarioOdyssey => "Super Mario Odyssey",
            BestSellingVideoGame::SuperMarioWorld => "Super Mario World",
            BestSellingVideoGame::SuperSmashBrosUltimate => "Super Smash Bros. Ultimate",
            BestSellingVideoGame::Terraria => "Terraria",
            BestSellingVideoGame::TetrisEa => "Tetris (EA)",
            BestSellingVideoGame::TetrisNintendo => "Tetris (Nintendo)",
            BestSellingVideoGame::TheElderScrollsVSkyrim => "The Elder Scrolls V: Skyrim",
            BestSellingVideoGame::TheLegendOfZeldaBreathOfTheWild => {
                "The Legend of Zelda: Breath of the Wild"
            }
            BestSellingVideoGame::TheWitcher3HeartsOfStoneBloodAndWine => {
                "The Witcher 3 / Hearts of Stone / Blood and Wine"
            }
            BestSellingVideoGame::WiiFitPlus => "Wii Fit / Plus",
            BestSellingVideoGame::WiiPlay => "Wii Play",
            BestSellingVideoGame::WiiSports => "Wii Sports",
            BestSellingVideoGame::WiiSportsResort => "Wii Sports Resort",
        }
    }
    pub const fn series(&self) -> &'static str {
        match self {
            BestSellingVideoGame::AnimalCrossingNewHorizons => "Animal Crossing",
            BestSellingVideoGame::Borderlands2 => "Borderlands",
            BestSellingVideoGame::CallOfDutyBlackOps => "Call of Duty",
            BestSellingVideoGame::CallOfDutyBlackOpsIi => "Call of Duty",
            BestSellingVideoGame::CallOfDutyModernWarfare => "Call of Duty",
            BestSellingVideoGame::CallOfDutyModernWarfare2 => "Call of Duty",
            BestSellingVideoGame::CallOfDutyModernWarfare3 => "Call of Duty",
            BestSellingVideoGame::DiabloIiiReaperOfSouls => "Diablo",
            BestSellingVideoGame::DuckHunt => "None",
            BestSellingVideoGame::Fifa18 => "FIFA",
            BestSellingVideoGame::GrandTheftAutoIv => "Grand Theft Auto",
            BestSellingVideoGame::GrandTheftAutoV => "Grand Theft Auto",
            BestSellingVideoGame::GrandTheftAutoSanAndreas => "Grand Theft Auto",
            BestSellingVideoGame::HumanFallFlat => "None",
            BestSellingVideoGame::KinectAdventures => "None",
            BestSellingVideoGame::MarioKart8Deluxe => "Mario Kart",
            BestSellingVideoGame::MarioKartDs => "Mario Kart",
            BestSellingVideoGame::MarioKartWii => "Mario Kart",
            BestSellingVideoGame::Minecraft => "Minecraft",
            BestSellingVideoGame::NewSuperMarioBros => "Super Mario",
            BestSellingVideoGame::NewSuperMarioBrosUDeluxeLuigiU => "Super Mario",
            BestSellingVideoGame::NewSuperMarioBrosWii => "Super Mario",
            BestSellingVideoGame::Nintendogs => "None",
            BestSellingVideoGame::PacMan => "Pac-Man",
            BestSellingVideoGame::PokemonDiamondPearlPlatinum => "Pokemon",
            BestSellingVideoGame::PokemonGoldSilverCrystal => "Pokemon",
            BestSellingVideoGame::PokemonRedGreenBlueYellow => "Pokemon",
            BestSellingVideoGame::PokemonRubySapphireEmerald => "Pokemon",
            BestSellingVideoGame::PokemonSunMoonUltraSunUltraMoon => "Pokemon",
            BestSellingVideoGame::PokemonSwordShield => "Pokemon",
            BestSellingVideoGame::PubgBattlegrounds => "PUBG Universe",
            BestSellingVideoGame::RedDeadRedemption => "Red Dead",
            BestSellingVideoGame::RedDeadRedemption2 => "Red Dead",
            BestSellingVideoGame::SonicTheHedgehog => "Sonic the Hedgehog",
            BestSellingVideoGame::SuperMario64Ds => "Super Mario",
            BestSellingVideoGame::SuperMarioBros => "Super Mario",
            BestSellingVideoGame::SuperMarioBros3 => "Super Mario",
            BestSellingVideoGame::SuperMarioOdyssey => "Super Mario",
            BestSellingVideoGame::SuperMarioWorld => "Super Mario",
            BestSellingVideoGame::SuperSmashBrosUltimate => "Super Smash Bros.",
            BestSellingVideoGame::Terraria => "None",
            BestSellingVideoGame::TetrisEa => "Tetris",
            BestSellingVideoGame::TetrisNintendo => "Tetris",
            BestSellingVideoGame::TheElderScrollsVSkyrim => "The Elder Scrolls",
            BestSellingVideoGame::TheLegendOfZeldaBreathOfTheWild => "The Legend of Zelda",
            BestSellingVideoGame::TheWitcher3HeartsOfStoneBloodAndWine => "The Witcher",
            BestSellingVideoGame::WiiFitPlus => "Wii",
            BestSellingVideoGame::WiiPlay => "Wii",
            BestSellingVideoGame::WiiSports => "Wii",
            BestSellingVideoGame::WiiSportsResort => "Wii",
        }
    }
    pub const fn platforms(&self) -> Platforms {
        match self {
            BestSellingVideoGame::AnimalCrossingNewHorizons => {
                Platforms::from_str("Nintendo Switch")
            }
            BestSellingVideoGame::Borderlands2 => Platforms::from_str("Multi-platform"),
            BestSellingVideoGame::CallOfDutyBlackOps => Platforms::from_str("Multi-platform"),
            BestSellingVideoGame::CallOfDutyBlackOpsIi => Platforms::from_str("Multi-platform"),
            BestSellingVideoGame::CallOfDutyModernWarfare => Platforms::from_str("Multi-platform"),
            BestSellingVideoGame::CallOfDutyModernWarfare2 => Platforms::from_str("Multi-platform"),
            BestSellingVideoGame::CallOfDutyModernWarfare3 => Platforms::from_str("Multi-platform"),
            BestSellingVideoGame::DiabloIiiReaperOfSouls => Platforms::from_str("Multi-platform"),
            BestSellingVideoGame::DuckHunt => Platforms::from_str("NES"),
            BestSellingVideoGame::Fifa18 => Platforms::from_str("Multi-platform"),
            BestSellingVideoGame::GrandTheftAutoIv => Platforms::from_str("Multi-platform"),
            BestSellingVideoGame::GrandTheftAutoV => Platforms::from_str("Multi-platform"),
            BestSellingVideoGame::GrandTheftAutoSanAndreas => Platforms::from_str("Multi-platform"),
            BestSellingVideoGame::HumanFallFlat => Platforms::from_str("Multi-platform"),
            BestSellingVideoGame::KinectAdventures => Platforms::from_str("Xbox 360"),
            BestSellingVideoGame::MarioKart8Deluxe => Platforms::from_str("Wii U / Switch"),
            BestSellingVideoGame::MarioKartDs => Platforms::from_str("Nintendo DS"),
            BestSellingVideoGame::MarioKartWii => Platforms::from_str("Wii"),
            BestSellingVideoGame::Minecraft => Platforms::from_str("Multi-platform"),
            BestSellingVideoGame::NewSuperMarioBros => Platforms::from_str("Nintendo DS"),
            BestSellingVideoGame::NewSuperMarioBrosUDeluxeLuigiU => {
                Platforms::from_str("Wii U / Nintendo Switch")
            }
            BestSellingVideoGame::NewSuperMarioBrosWii => Platforms::from_str("Wii"),
            BestSellingVideoGame::Nintendogs => Platforms::from_str("Nintendo DS"),
            BestSellingVideoGame::PacMan => Platforms::from_str("Multi-platform"),
            BestSellingVideoGame::PokemonDiamondPearlPlatinum => Platforms::from_str("Nintendo DS"),
            BestSellingVideoGame::PokemonGoldSilverCrystal => Platforms::from_str("Game Boy Color"),
            BestSellingVideoGame::PokemonRedGreenBlueYellow => {
                Platforms::from_str("Game Boy / Color")
            }
            BestSellingVideoGame::PokemonRubySapphireEmerald => {
                Platforms::from_str("Game Boy Advance")
            }
            BestSellingVideoGame::PokemonSunMoonUltraSunUltraMoon => {
                Platforms::from_str("Nintendo 3DS")
            }
            BestSellingVideoGame::PokemonSwordShield => Platforms::from_str("Nintendo Switch"),
            BestSellingVideoGame::PubgBattlegrounds => Platforms::from_str("Multi-platform"),
            BestSellingVideoGame::RedDeadRedemption => Platforms::from_str("PS3 / Xbox 360"),
            BestSellingVideoGame::RedDeadRedemption2 => Platforms::from_str("Multi-platform"),
            BestSellingVideoGame::SonicTheHedgehog => Platforms::from_str("Multi-platform"),
            BestSellingVideoGame::SuperMario64Ds => Platforms::from_str("Nintendo 64 / DS"),
            BestSellingVideoGame::SuperMarioBros => Platforms::from_str("Multi-platform"),
            BestSellingVideoGame::SuperMarioBros3 => Platforms::from_str("Multi-platform"),
            BestSellingVideoGame::SuperMarioOdyssey => Platforms::from_str("Nintendo Switch"),
            BestSellingVideoGame::SuperMarioWorld => Platforms::from_str("Multi-platform"),
            BestSellingVideoGame::SuperSmashBrosUltimate => Platforms::from_str("Nintendo Switch"),
            BestSellingVideoGame::Terraria => Platforms::from_str("Multi-platform"),
            BestSellingVideoGame::TetrisEa => Platforms::from_str("Multi-platform"),
            BestSellingVideoGame::TetrisNintendo => Platforms::from_str("Game Boy / NES"),
            BestSellingVideoGame::TheElderScrollsVSkyrim => Platforms::from_str("Multi-platform"),
            BestSellingVideoGame::TheLegendOfZeldaBreathOfTheWild => {
                Platforms::from_str("Wii U / Switch")
            }
            BestSellingVideoGame::TheWitcher3HeartsOfStoneBloodAndWine => {
                Platforms::from_str("Multi-platform")
            }
            BestSellingVideoGame::WiiFitPlus => Platforms::from_str("Wii"),
            BestSellingVideoGame::WiiPlay => Platforms::from_str("Wii"),
            BestSellingVideoGame::WiiSports => Platforms::from_str("Wii"),
            BestSellingVideoGame::WiiSportsResort => Platforms::from_str("Wii"),
        }
    }
    pub const fn publisher(&self) -> Publisher {
        match self {
            BestSellingVideoGame::AnimalCrossingNewHorizons => Publisher::from_str("Nintendo"),
            BestSellingVideoGame::Borderlands2 => Publisher::from_str("2K Games"),
            BestSellingVideoGame::CallOfDutyBlackOps => Publisher::from_str("Activision"),
            BestSellingVideoGame::CallOfDutyBlackOpsIi => Publisher::from_str("Activision"),
            BestSellingVideoGame::CallOfDutyModernWarfare => Publisher::from_str("Activision"),
            BestSellingVideoGame::CallOfDutyModernWarfare2 => Publisher::from_str("Activision"),
            BestSellingVideoGame::CallOfDutyModernWarfare3 => Publisher::from_str("Activision"),
            BestSellingVideoGame::DiabloIiiReaperOfSouls => {
                Publisher::from_str("Blizzard Entertainment")
            }
            BestSellingVideoGame::DuckHunt => Publisher::from_str("Nintendo"),
            BestSellingVideoGame::Fifa18 => Publisher::from_str("EA Sports"),
            BestSellingVideoGame::GrandTheftAutoIv => Publisher::from_str("Rockstar Games"),
            BestSellingVideoGame::GrandTheftAutoV => Publisher::from_str("Rockstar Games"),
            BestSellingVideoGame::GrandTheftAutoSanAndreas => Publisher::from_str("Rockstar Games"),
            BestSellingVideoGame::HumanFallFlat => Publisher::from_str("Curve Digital"),
            BestSellingVideoGame::KinectAdventures => Publisher::from_str("Xbox Game Studios"),
            BestSellingVideoGame::MarioKart8Deluxe => Publisher::from_str("Nintendo"),
            BestSellingVideoGame::MarioKartDs => Publisher::from_str("Nintendo"),
            BestSellingVideoGame::MarioKartWii => Publisher::from_str("Nintendo"),
            BestSellingVideoGame::Minecraft => Publisher::from_str("Xbox Game Studios"),
            BestSellingVideoGame::NewSuperMarioBros => Publisher::from_str("Nintendo"),
            BestSellingVideoGame::NewSuperMarioBrosUDeluxeLuigiU => Publisher::from_str("Nintendo"),
            BestSellingVideoGame::NewSuperMarioBrosWii => Publisher::from_str("Nintendo"),
            BestSellingVideoGame::Nintendogs => Publisher::from_str("Nintendo"),
            BestSellingVideoGame::PacMan => Publisher::from_str("Namco"),
            BestSellingVideoGame::PokemonDiamondPearlPlatinum => {
                Publisher::from_str("Nintendo / The Pokemon Company")
            }
            BestSellingVideoGame::PokemonGoldSilverCrystal => Publisher::from_str("Nintendo"),
            BestSellingVideoGame::PokemonRedGreenBlueYellow => Publisher::from_str("Nintendo"),
            BestSellingVideoGame::PokemonRubySapphireEmerald => {
                Publisher::from_str("Nintendo / The Pokemon Company")
            }
            BestSellingVideoGame::PokemonSunMoonUltraSunUltraMoon => {
                Publisher::from_str("Nintendo / The Pokemon Company")
            }
            BestSellingVideoGame::PokemonSwordShield => {
                Publisher::from_str("Nintendo / The Pokemon Company")
            }
            BestSellingVideoGame::PubgBattlegrounds => Publisher::from_str("PUBG Corporation"),
            BestSellingVideoGame::RedDeadRedemption => Publisher::from_str("Rockstar Games"),
            BestSellingVideoGame::RedDeadRedemption2 => Publisher::from_str("Rockstar Games"),
            BestSellingVideoGame::SonicTheHedgehog => Publisher::from_str("Sega"),
            BestSellingVideoGame::SuperMario64Ds => Publisher::from_str("Nintendo"),
            BestSellingVideoGame::SuperMarioBros => Publisher::from_str("Nintendo"),
            BestSellingVideoGame::SuperMarioBros3 => Publisher::from_str("Nintendo"),
            BestSellingVideoGame::SuperMarioOdyssey => Publisher::from_str("Nintendo"),
            BestSellingVideoGame::SuperMarioWorld => Publisher::from_str("Nintendo"),
            BestSellingVideoGame::SuperSmashBrosUltimate => Publisher::from_str("Nintendo"),
            BestSellingVideoGame::Terraria => Publisher::from_str("Re-Logic / 505 Games"),
            BestSellingVideoGame::TetrisEa => Publisher::from_str("Electronic Arts"),
            BestSellingVideoGame::TetrisNintendo => Publisher::from_str("Nintendo"),
            BestSellingVideoGame::TheElderScrollsVSkyrim => {
                Publisher::from_str("Bethesda Softworks")
            }
            BestSellingVideoGame::TheLegendOfZeldaBreathOfTheWild => {
                Publisher::from_str("Nintendo")
            }
            BestSellingVideoGame::TheWitcher3HeartsOfStoneBloodAndWine => {
                Publisher::from_str("CD Projekt")
            }
            BestSellingVideoGame::WiiFitPlus => Publisher::from_str("Nintendo"),
            BestSellingVideoGame::WiiPlay => Publisher::from_str("Nintendo"),
            BestSellingVideoGame::WiiSports => Publisher::from_str("Nintendo"),
            BestSellingVideoGame::WiiSportsResort => Publisher::from_str("Nintendo"),
        }
    }
    pub const fn developer(&self) -> Developer {
        match self {
            BestSellingVideoGame::AnimalCrossingNewHorizons => Developer::NintendoEpd,
            BestSellingVideoGame::Borderlands2 => Developer::GearboxSoftware,
            BestSellingVideoGame::CallOfDutyBlackOps => Developer::Treyarch,
            BestSellingVideoGame::CallOfDutyBlackOpsIi => Developer::Treyarch,
            BestSellingVideoGame::CallOfDutyModernWarfare => Developer::InfinityWard,
            BestSellingVideoGame::CallOfDutyModernWarfare2 => Developer::InfinityWard,
            BestSellingVideoGame::CallOfDutyModernWarfare3 => Developer::InfinityWardSledgehammer,
            BestSellingVideoGame::DiabloIiiReaperOfSouls => Developer::BlizzardEntertainment,
            BestSellingVideoGame::DuckHunt => Developer::NintendoRD1,
            BestSellingVideoGame::Fifa18 => Developer::EaVancouver,
            BestSellingVideoGame::GrandTheftAutoIv => Developer::RockstarNorth,
            BestSellingVideoGame::GrandTheftAutoV => Developer::RockstarNorth,
            BestSellingVideoGame::GrandTheftAutoSanAndreas => Developer::RockstarNorth,
            BestSellingVideoGame::HumanFallFlat => Developer::NoBrakesGames,
            BestSellingVideoGame::KinectAdventures => Developer::GoodScienceStudio,
            BestSellingVideoGame::MarioKart8Deluxe => Developer::NintendoEad,
            BestSellingVideoGame::MarioKartDs => Developer::NintendoEad,
            BestSellingVideoGame::MarioKartWii => Developer::NintendoEad,
            BestSellingVideoGame::Minecraft => Developer::MojangStudios,
            BestSellingVideoGame::NewSuperMarioBros => Developer::NintendoEad,
            BestSellingVideoGame::NewSuperMarioBrosUDeluxeLuigiU => Developer::NintendoEad,
            BestSellingVideoGame::NewSuperMarioBrosWii => Developer::NintendoEad,
            BestSellingVideoGame::Nintendogs => Developer::NintendoEad,
            BestSellingVideoGame::PacMan => Developer::Namco,
            BestSellingVideoGame::PokemonDiamondPearlPlatinum => Developer::GameFreak,
            BestSellingVideoGame::PokemonGoldSilverCrystal => Developer::GameFreak,
            BestSellingVideoGame::PokemonRedGreenBlueYellow => Developer::GameFreak,
            BestSellingVideoGame::PokemonRubySapphireEmerald => Developer::GameFreak,
            BestSellingVideoGame::PokemonSunMoonUltraSunUltraMoon => Developer::GameFreak,
            BestSellingVideoGame::PokemonSwordShield => Developer::GameFreak,
            BestSellingVideoGame::PubgBattlegrounds => Developer::PubgCorporation,
            BestSellingVideoGame::RedDeadRedemption => Developer::RockstarSanDiego,
            BestSellingVideoGame::RedDeadRedemption2 => Developer::RockstarStudios,
            BestSellingVideoGame::SonicTheHedgehog => Developer::SonicTeam,
            BestSellingVideoGame::SuperMario64Ds => Developer::NintendoEad,
            BestSellingVideoGame::SuperMarioBros => Developer::NintendoRD4,
            BestSellingVideoGame::SuperMarioBros3 => Developer::NintendoEad,
            BestSellingVideoGame::SuperMarioOdyssey => Developer::NintendoEpd,
            BestSellingVideoGame::SuperMarioWorld => Developer::NintendoEad,
            BestSellingVideoGame::SuperSmashBrosUltimate => Developer::BandaiNamcoStudiosSoraLtd,
            BestSellingVideoGame::Terraria => Developer::ReLogic,
            BestSellingVideoGame::TetrisEa => Developer::EaMobile,
            BestSellingVideoGame::TetrisNintendo => Developer::NintendoRD1,
            BestSellingVideoGame::TheElderScrollsVSkyrim => Developer::BethesdaGameStudios,
            BestSellingVideoGame::TheLegendOfZeldaBreathOfTheWild => Developer::NintendoEpd,
            BestSellingVideoGame::TheWitcher3HeartsOfStoneBloodAndWine => Developer::CdProjektRed,
            BestSellingVideoGame::WiiFitPlus => Developer::NintendoEad,
            BestSellingVideoGame::WiiPlay => Developer::NintendoEad,
            BestSellingVideoGame::WiiSports => Developer::NintendoEad,
            BestSellingVideoGame::WiiSportsResort => Developer::NintendoEad,
        }
    }
}

impl<'a> TryFrom<&'a str> for BestSellingVideoGame {
    type Error = String;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        match s {
            "Animal Crossing: New Horizons" => Ok(BestSellingVideoGame::AnimalCrossingNewHorizons),
            "Borderlands 2" => Ok(BestSellingVideoGame::Borderlands2),
            "Call of Duty: Black Ops" => Ok(BestSellingVideoGame::CallOfDutyBlackOps),
            "Call of Duty: Black Ops II" => Ok(BestSellingVideoGame::CallOfDutyBlackOpsIi),
            "Call of Duty: Modern Warfare" => Ok(BestSellingVideoGame::CallOfDutyModernWarfare),
            "Call of Duty: Modern Warfare 2" => Ok(BestSellingVideoGame::CallOfDutyModernWarfare2),
            "Call of Duty: Modern Warfare 3" => Ok(BestSellingVideoGame::CallOfDutyModernWarfare3),
            "Diablo III / Reaper of Souls" => Ok(BestSellingVideoGame::DiabloIiiReaperOfSouls),
            "Duck Hunt" => Ok(BestSellingVideoGame::DuckHunt),
            "FIFA 18" => Ok(BestSellingVideoGame::Fifa18),
            "Grand Theft Auto IV" => Ok(BestSellingVideoGame::GrandTheftAutoIv),
            "Grand Theft Auto V" => Ok(BestSellingVideoGame::GrandTheftAutoV),
            "Grand Theft Auto: San Andreas" => Ok(BestSellingVideoGame::GrandTheftAutoSanAndreas),
            "Human: Fall Flat" => Ok(BestSellingVideoGame::HumanFallFlat),
            "Kinect Adventures!" => Ok(BestSellingVideoGame::KinectAdventures),
            "Mario Kart 8 / Deluxe" => Ok(BestSellingVideoGame::MarioKart8Deluxe),
            "Mario Kart DS" => Ok(BestSellingVideoGame::MarioKartDs),
            "Mario Kart Wii" => Ok(BestSellingVideoGame::MarioKartWii),
            "Minecraft" => Ok(BestSellingVideoGame::Minecraft),
            "New Super Mario Bros." => Ok(BestSellingVideoGame::NewSuperMarioBros),
            "New Super Mario Bros. U / Deluxe / Luigi U" => {
                Ok(BestSellingVideoGame::NewSuperMarioBrosUDeluxeLuigiU)
            }
            "New Super Mario Bros. Wii" => Ok(BestSellingVideoGame::NewSuperMarioBrosWii),
            "Nintendogs" => Ok(BestSellingVideoGame::Nintendogs),
            "Pac-Man" => Ok(BestSellingVideoGame::PacMan),
            "Pokemon Diamond / Pearl / Platinum" => {
                Ok(BestSellingVideoGame::PokemonDiamondPearlPlatinum)
            }
            "Pokemon Gold / Silver / Crystal" => Ok(BestSellingVideoGame::PokemonGoldSilverCrystal),
            "Pokemon Red / Green / Blue / Yellow" => {
                Ok(BestSellingVideoGame::PokemonRedGreenBlueYellow)
            }
            "Pokemon Ruby / Sapphire / Emerald" => {
                Ok(BestSellingVideoGame::PokemonRubySapphireEmerald)
            }
            "Pokemon Sun / Moon / Ultra Sun / Ultra Moon" => {
                Ok(BestSellingVideoGame::PokemonSunMoonUltraSunUltraMoon)
            }
            "Pokemon Sword / Shield" => Ok(BestSellingVideoGame::PokemonSwordShield),
            "PUBG: Battlegrounds" => Ok(BestSellingVideoGame::PubgBattlegrounds),
            "Red Dead Redemption" => Ok(BestSellingVideoGame::RedDeadRedemption),
            "Red Dead Redemption 2" => Ok(BestSellingVideoGame::RedDeadRedemption2),
            "Sonic the Hedgehog" => Ok(BestSellingVideoGame::SonicTheHedgehog),
            "Super Mario 64 / DS" => Ok(BestSellingVideoGame::SuperMario64Ds),
            "Super Mario Bros." => Ok(BestSellingVideoGame::SuperMarioBros),
            "Super Mario Bros. 3" => Ok(BestSellingVideoGame::SuperMarioBros3),
            "Super Mario Odyssey" => Ok(BestSellingVideoGame::SuperMarioOdyssey),
            "Super Mario World" => Ok(BestSellingVideoGame::SuperMarioWorld),
            "Super Smash Bros. Ultimate" => Ok(BestSellingVideoGame::SuperSmashBrosUltimate),
            "Terraria" => Ok(BestSellingVideoGame::Terraria),
            "Tetris (EA)" => Ok(BestSellingVideoGame::TetrisEa),
            "Tetris (Nintendo)" => Ok(BestSellingVideoGame::TetrisNintendo),
            "The Elder Scrolls V: Skyrim" => Ok(BestSellingVideoGame::TheElderScrollsVSkyrim),
            "The Legend of Zelda: Breath of the Wild" => {
                Ok(BestSellingVideoGame::TheLegendOfZeldaBreathOfTheWild)
            }
            "The Witcher 3 / Hearts of Stone / Blood and Wine" => {
                Ok(BestSellingVideoGame::TheWitcher3HeartsOfStoneBloodAndWine)
            }
            "Wii Fit / Plus" => Ok(BestSellingVideoGame::WiiFitPlus),
            "Wii Play" => Ok(BestSellingVideoGame::WiiPlay),
            "Wii Sports" => Ok(BestSellingVideoGame::WiiSports),
            "Wii Sports Resort" => Ok(BestSellingVideoGame::WiiSportsResort),
            _ => Err(format!("Unknown BestSellingVideoGame {}", s)),
        }
    }
}
```
