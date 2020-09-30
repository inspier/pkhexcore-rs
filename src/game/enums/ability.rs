#![allow(non_snake_case)]
use deku::prelude::*;

#[allow(non_camel_case_types)]

/// Ability IDs for the corresponding English ability name.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, DekuRead, DekuWrite)]
#[deku(type = "u16", ctx = "_endian: deku::ctx::Endian")]
pub enum Ability {
    #[deku(id = "0")]
    None,
    #[deku(id = "1")]
    Stench,
    #[deku(id = "2")]
    Drizzle,
    #[deku(id = "3")]
    SpeedBoost,
    #[deku(id = "4")]
    BattleArmor,
    #[deku(id = "5")]
    Sturdy,
    #[deku(id = "6")]
    Damp,
    #[deku(id = "7")]
    Limber,
    #[deku(id = "8")]
    SandVeil,
    #[deku(id = "9")]
    Static,
    #[deku(id = "10")]
    VoltAbsorb,
    #[deku(id = "11")]
    WaterAbsorb,
    #[deku(id = "12")]
    Oblivious,
    #[deku(id = "13")]
    CloudNine,
    #[deku(id = "14")]
    CompoundEyes,
    #[deku(id = "15")]
    Insomnia,
    #[deku(id = "16")]
    ColorChange,
    #[deku(id = "17")]
    Immunity,
    #[deku(id = "18")]
    FlashFire,
    #[deku(id = "19")]
    ShieldDust,
    #[deku(id = "20")]
    OwnTempo,
    #[deku(id = "21")]
    SuctionCups,
    #[deku(id = "22")]
    Intimidate,
    #[deku(id = "23")]
    ShadowTag,
    #[deku(id = "24")]
    RoughSkin,
    #[deku(id = "25")]
    WonderGuard,
    #[deku(id = "26")]
    Levitate,
    #[deku(id = "27")]
    EffectSpore,
    #[deku(id = "28")]
    Synchronize,
    #[deku(id = "29")]
    ClearBody,
    #[deku(id = "30")]
    NaturalCure,
    #[deku(id = "31")]
    LightningRod,
    #[deku(id = "32")]
    SereneGrace,
    #[deku(id = "33")]
    SwiftSwim,
    #[deku(id = "34")]
    Chlorophyll,
    #[deku(id = "35")]
    Illuminate,
    #[deku(id = "36")]
    Trace,
    #[deku(id = "37")]
    HugePower,
    #[deku(id = "38")]
    PoisonPoint,
    #[deku(id = "39")]
    InnerFocus,
    #[deku(id = "40")]
    MagmaArmor,
    #[deku(id = "41")]
    WaterVeil,
    #[deku(id = "42")]
    MagnetPull,
    #[deku(id = "43")]
    Soundproof,
    #[deku(id = "44")]
    RainDish,
    #[deku(id = "45")]
    SandStream,
    #[deku(id = "46")]
    Pressure,
    #[deku(id = "47")]
    ThickFat,
    #[deku(id = "48")]
    EarlyBird,
    #[deku(id = "49")]
    FlameBody,
    #[deku(id = "50")]
    RunAway,
    #[deku(id = "51")]
    KeenEye,
    #[deku(id = "52")]
    HyperCutter,
    #[deku(id = "53")]
    Pickup,
    #[deku(id = "54")]
    Truant,
    #[deku(id = "55")]
    Hustle,
    #[deku(id = "56")]
    CuteCharm,
    #[deku(id = "57")]
    Plus,
    #[deku(id = "58")]
    Minus,
    #[deku(id = "59")]
    Forecast,
    #[deku(id = "60")]
    StickyHold,
    #[deku(id = "61")]
    ShedSkin,
    #[deku(id = "62")]
    Guts,
    #[deku(id = "63")]
    MarvelScale,
    #[deku(id = "64")]
    LiquidOoze,
    #[deku(id = "65")]
    Overgrow,
    #[deku(id = "66")]
    Blaze,
    #[deku(id = "67")]
    Torrent,
    #[deku(id = "68")]
    Swarm,
    #[deku(id = "69")]
    RockHead,
    #[deku(id = "70")]
    Drought,
    #[deku(id = "71")]
    ArenaTrap,
    #[deku(id = "72")]
    VitalSpirit,
    #[deku(id = "73")]
    WhiteSmoke,
    #[deku(id = "74")]
    PurePower,
    #[deku(id = "75")]
    ShellArmor,
    #[deku(id = "76")]
    AirLock,
    #[deku(id = "77")]
    TangledFeet,
    #[deku(id = "78")]
    MotorDrive,
    #[deku(id = "79")]
    Rivalry,
    #[deku(id = "80")]
    Steadfast,
    #[deku(id = "81")]
    SnowCloak,
    #[deku(id = "82")]
    Gluttony,
    #[deku(id = "83")]
    AngerPoint,
    #[deku(id = "84")]
    Unburden,
    #[deku(id = "85")]
    Heatproof,
    #[deku(id = "86")]
    Simple,
    #[deku(id = "87")]
    DrySkin,
    #[deku(id = "88")]
    Download,
    #[deku(id = "89")]
    IronFist,
    #[deku(id = "90")]
    PoisonHeal,
    #[deku(id = "91")]
    Adaptability,
    #[deku(id = "92")]
    SkillLink,
    #[deku(id = "93")]
    Hydration,
    #[deku(id = "94")]
    SolarPower,
    #[deku(id = "95")]
    QuickFeet,
    #[deku(id = "96")]
    Normalize,
    #[deku(id = "97")]
    Sniper,
    #[deku(id = "98")]
    MagicGuard,
    #[deku(id = "99")]
    NoGuard,
    #[deku(id = "100")]
    Stall,
    #[deku(id = "101")]
    Technician,
    #[deku(id = "102")]
    LeafGuard,
    #[deku(id = "103")]
    Klutz,
    #[deku(id = "104")]
    MoldBreaker,
    #[deku(id = "105")]
    SuperLuck,
    #[deku(id = "106")]
    Aftermath,
    #[deku(id = "107")]
    Anticipation,
    #[deku(id = "108")]
    Forewarn,
    #[deku(id = "109")]
    Unaware,
    #[deku(id = "110")]
    TintedLens,
    #[deku(id = "111")]
    Filter,
    #[deku(id = "112")]
    SlowStart,
    #[deku(id = "113")]
    Scrappy,
    #[deku(id = "114")]
    StormDrain,
    #[deku(id = "115")]
    IceBody,
    #[deku(id = "116")]
    SolidRock,
    #[deku(id = "117")]
    SnowWarning,
    #[deku(id = "118")]
    HoneyGather,
    #[deku(id = "119")]
    Frisk,
    #[deku(id = "120")]
    Reckless,
    #[deku(id = "121")]
    Multitype,
    #[deku(id = "122")]
    FlowerGift,
    #[deku(id = "123")]
    BadDreams,
    #[deku(id = "124")]
    Pickpocket,
    #[deku(id = "125")]
    SheerForce,
    #[deku(id = "126")]
    Contrary,
    #[deku(id = "127")]
    Unnerve,
    #[deku(id = "128")]
    Defiant,
    #[deku(id = "129")]
    Defeatist,
    #[deku(id = "130")]
    CursedBody,
    #[deku(id = "131")]
    Healer,
    #[deku(id = "132")]
    FriendGuard,
    #[deku(id = "133")]
    WeakArmor,
    #[deku(id = "134")]
    HeavyMetal,
    #[deku(id = "135")]
    LightMetal,
    #[deku(id = "136")]
    Multiscale,
    #[deku(id = "137")]
    ToxicBoost,
    #[deku(id = "138")]
    FlareBoost,
    #[deku(id = "139")]
    Harvest,
    #[deku(id = "140")]
    Telepathy,
    #[deku(id = "141")]
    Moody,
    #[deku(id = "142")]
    Overcoat,
    #[deku(id = "143")]
    PoisonTouch,
    #[deku(id = "144")]
    Regenerator,
    #[deku(id = "145")]
    BigPecks,
    #[deku(id = "146")]
    SandRush,
    #[deku(id = "147")]
    WonderSkin,
    #[deku(id = "148")]
    Analytic,
    #[deku(id = "149")]
    Illusion,
    #[deku(id = "150")]
    Imposter,
    #[deku(id = "151")]
    Infiltrator,
    #[deku(id = "152")]
    Mummy,
    #[deku(id = "153")]
    Moxie,
    #[deku(id = "154")]
    Justified,
    #[deku(id = "155")]
    Rattled,
    #[deku(id = "156")]
    MagicBounce,
    #[deku(id = "157")]
    SapSipper,
    #[deku(id = "158")]
    Prankster,
    #[deku(id = "159")]
    SandForce,
    #[deku(id = "160")]
    IronBarbs,
    #[deku(id = "161")]
    ZenMode,
    #[deku(id = "162")]
    VictoryStar,
    #[deku(id = "163")]
    Turboblaze,
    #[deku(id = "164")]
    Teravolt,
    #[deku(id = "165")]
    AromaVeil,
    #[deku(id = "166")]
    FlowerVeil,
    #[deku(id = "167")]
    CheekPouch,
    #[deku(id = "168")]
    Protean,
    #[deku(id = "169")]
    FurCoat,
    #[deku(id = "170")]
    Magician,
    #[deku(id = "171")]
    Bulletproof,
    #[deku(id = "172")]
    Competitive,
    #[deku(id = "173")]
    StrongJaw,
    #[deku(id = "174")]
    Refrigerate,
    #[deku(id = "175")]
    SweetVeil,
    #[deku(id = "176")]
    StanceChange,
    #[deku(id = "177")]
    GaleWings,
    #[deku(id = "178")]
    MegaLauncher,
    #[deku(id = "179")]
    GrassPelt,
    #[deku(id = "180")]
    Symbiosis,
    #[deku(id = "181")]
    ToughClaws,
    #[deku(id = "182")]
    Pixilate,
    #[deku(id = "183")]
    Gooey,
    #[deku(id = "184")]
    Aerilate,
    #[deku(id = "185")]
    ParentalBond,
    #[deku(id = "186")]
    DarkAura,
    #[deku(id = "187")]
    FairyAura,
    #[deku(id = "188")]
    AuraBreak,
    #[deku(id = "189")]
    PrimordialSea,
    #[deku(id = "190")]
    DesolateLand,
    #[deku(id = "191")]
    DeltaStream,
    #[deku(id = "192")]
    Stamina,
    #[deku(id = "193")]
    WimpOut,
    #[deku(id = "194")]
    EmergencyExit,
    #[deku(id = "195")]
    WaterCompaction,
    #[deku(id = "196")]
    Merciless,
    #[deku(id = "197")]
    ShieldsDown,
    #[deku(id = "198")]
    Stakeout,
    #[deku(id = "199")]
    WaterBubble,
    #[deku(id = "200")]
    Steelworker,
    #[deku(id = "201")]
    Berserk,
    #[deku(id = "202")]
    SlushRush,
    #[deku(id = "203")]
    LongReach,
    #[deku(id = "204")]
    LiquidVoice,
    #[deku(id = "205")]
    Triage,
    #[deku(id = "206")]
    Galvanize,
    #[deku(id = "207")]
    SurgeSurfer,
    #[deku(id = "208")]
    Schooling,
    #[deku(id = "209")]
    Disguise,
    #[deku(id = "210")]
    BattleBond,
    #[deku(id = "211")]
    PowerConstruct,
    #[deku(id = "212")]
    Corrosion,
    #[deku(id = "213")]
    Comatose,
    #[deku(id = "214")]
    QueenlyMajesty,
    #[deku(id = "215")]
    InnardsOut,
    #[deku(id = "216")]
    Dancer,
    #[deku(id = "217")]
    Battery,
    #[deku(id = "218")]
    Fluffy,
    #[deku(id = "219")]
    Dazzling,
    #[deku(id = "220")]
    SoulHeart,
    #[deku(id = "221")]
    TanglingHair,
    #[deku(id = "222")]
    Receiver,
    #[deku(id = "223")]
    PowerofAlchemy,
    #[deku(id = "224")]
    BeastBoost,
    #[deku(id = "225")]
    RKSSystem,
    #[deku(id = "226")]
    ElectricSurge,
    #[deku(id = "227")]
    PsychicSurge,
    #[deku(id = "228")]
    MistySurge,
    #[deku(id = "229")]
    GrassySurge,
    #[deku(id = "230")]
    FullMetalBody,
    #[deku(id = "231")]
    ShadowShield,
    #[deku(id = "232")]
    PrismArmor,
    #[deku(id = "233")]
    Neuroforce,
    #[deku(id = "234")]
    IntrepidSword,
    #[deku(id = "235")]
    DauntlessShield,
    #[deku(id = "236")]
    Libero,
    #[deku(id = "237")]
    BallFetch,
    #[deku(id = "238")]
    CottonDown,
    #[deku(id = "239")]
    PropellerTail,
    #[deku(id = "240")]
    MirrorArmor,
    #[deku(id = "241")]
    GulpMissile,
    #[deku(id = "242")]
    Stalwart,
    #[deku(id = "243")]
    SteamEngine,
    #[deku(id = "244")]
    PunkRock,
    #[deku(id = "245")]
    SandSpit,
    #[deku(id = "246")]
    IceScales,
    #[deku(id = "247")]
    Ripen,
    #[deku(id = "248")]
    IceFace,
    #[deku(id = "249")]
    PowerSpot,
    #[deku(id = "250")]
    Mimicry,
    #[deku(id = "251")]
    ScreenCleaner,
    #[deku(id = "252")]
    SteelySpirit,
    #[deku(id = "253")]
    PerishBody,
    #[deku(id = "254")]
    WanderingSpirit,
    #[deku(id = "255")]
    GorillaTactics,
    #[deku(id = "256")]
    NeutralizingGas,
    #[deku(id = "257")]
    PastelVeil,
    #[deku(id = "258")]
    HungerSwitch,
    #[deku(id = "259")]
    QuickDraw,
    #[deku(id = "260")]
    UnseenFist,
    #[deku(id = "261")]
    MAX_COUNT,
}

impl_from! (Ability for i32);

#[allow(unused_macros)]
macro_rules! ability {
    ($ability:tt) => {
        string_to_enum!(Ability, $ability)
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ability_macro_test() {
        assert_eq!(Ability::IntrepidSword, ability!(IntrepidSword));
    }
}
