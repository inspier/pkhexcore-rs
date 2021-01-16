/// Game Version ID enum shared between actual Version IDs and lumped version
/// groupings.
#[non_exhaustive]
pub enum GameVersion {
    // Indicators for method empty arguments & result indication. Not stored
    // values.
    Invalid = -2,
    Any = -1,
    Unknown = 0,
    // End Indicators

    // The following values are IDs stored within PKM data, and can also
    // identify individual games.

    // Gen3
    /// Pokémon Sapphire (GBA)
    S = 1,

    /// Pokémon Ruby (GBA)
    R = 2,

    /// Pokémon Emerald (GBA)
    E = 3,

    /// Pokémon FireRed (GBA)
    FR = 4,

    /// Pokémon LeafGreen (GBA)
    LG = 5,

    /// Pokémon Colosseum & Pokémon XD (GameCube)
    CXD = 15,
    // End Gen3

    // Gen4
    /// Pokémon Diamond (NDS)
    D = 10,

    /// Pokémon Pearl (NDS)
    P = 11,

    /// Pokémon Platinum (NDS)
    Pt = 12,

    /// Pokémon Heart Gold (NDS)
    HG = 7,

    /// Pokémon Soul Silver (NDS)
    SS = 8,
    // End Gen4

    // Gen5
    /// Pokémon White (NDS)
    W = 20,

    /// Pokémon Black (NDS)
    B = 21,

    /// Pokémon White 2 (NDS)
    W2 = 22,

    /// Pokémon Black 2 (NDS)
    B2 = 23,
    // End Gen5

    // Gen6
    /// Pokémon X (3DS)
    X = 24,

    /// Pokémon Y (3DS)
    Y = 25,

    /// Pokémon Alpha Sapphire (3DS)
    AS = 26,

    /// Pokémon Omega Ruby (3DS)
    OR = 27,
    // End Gen6

    // Gen7
    /// Pokémon Sun (3DS)
    SN = 30,

    /// Pokémon Moon (3DS)
    MN = 31,

    /// Pokémon Ultra Sun (3DS)
    US = 32,

    /// Pokémon Ultra Moon (3DS)
    UM = 33,
    // End Gen7
    /// Pokémon GO (GO -> Lets Go transfers)
    GO = 34,

    // Virtual Console (3DS) Gen1
    /// Pokémon Red (3DS Virtual Console)
    RD = 35,

    /// Pokémon Green[JP]/Blue[INT] (3DS Virtual Console)
    GN = 36,

    /// Pokémon Blue[JP] (3DS Virtual Console)
    BU = 37,

    /// Pokémon Yellow [JP] (3DS Virtual Console)
    YW = 38,
    // End Virtual Console (3DS) Gen1

    // Virtual Console (3DS) Gen2
    /// Pokémon Gold (3DS Virtual Console)
    GD = 39,

    /// Pokémon Silver (3DS Virtual Console)
    SV = 40,

    /// Pokémon Crystal (3DS Virtual Console)
    C = 41,
    // End Virtual Console (3DS) Gen2

    // Nintendo Switch
    /// Pokémon Let's Go Pikachu (NX)
    GP = 42,

    /// Pokémon Let's Go Eevee (NX)
    GE = 43,

    /// Pokémon Sword (NX)
    SW = 44,

    /// Pokémon Shield (NX)
    SH = 45,
    // End Nintendo Switch

    // The following values are not actually stored values in pkm data,
    // These values are assigned within PKHeX as properties for various logic
    // branching.

    // Game Groupings (SaveFile type, roughly)
    /// Pokémon Red & Blue SAV1 identifier.
    RB,

    /// Pokémon Red/Blue/Yellow SAV1 identifier.
    RBY,

    /// Pokémon Gold & Silver SAV2 identifier.
    GS,

    /// Pokémon Gold/Silver/Crystal SAV2 identifier.
    GSC,

    /// Pokémon Ruby & Sapphire SAV3 identifier.
    RS,

    /// Pokémon Ruby/Sapphire/Emerald SAV3 identifier.
    RSE,

    /// Pokémon FireRed/LeafGreen SAV3 identifier.
    FRLG,

    /// Pokémon Box Ruby & Sapphire SAV3RSBox identifier.
    RSBOX,

    /// Pokémon Colosseum SAV3Colosseum identifier.
    COLO,

    /// Pokémon XD SAV3XD identifier.
    XD,

    /// Pokémon Diamond & Pearl SAV4 identifier.
    DP,

    /// Pokémon Diamond/Pearl/Platinum version group.
    // Used to lump data from the associated games as data assets are shared
    DPPt,

    /// Pokémon Heart Gold & Soul Silver SAV4 identifier.
    HGSS,

    /// Pokémon Battle Revolution SAV4BR identifier.
    BATREV,

    /// Pokémon Black & White version group.
    // Used to lump data from the associated games as data assets are shared
    BW,

    /// Pokémon Black 2 & White 2 version group.
    // Used to lump data from the associated games as data assets are shared.
    B2W2,

    /// Pokémon X & Y
    // Used to lump data from the associated games as data assets are shared.
    XY,

    /// Pokémon Omega Ruby & Alpha Sapphire Demo SAV6 identifier.
    ORASDEMO,

    /// Pokémon Omega Ruby & Alpha Sapphire version group.
    // Used to lump data from the associated games as data assets are shared.
    ORAS,

    /// Pokémon Sun & Moon
    // Used to lump data from the associated games as data assets are shared.
    SM,

    /// Pokémon Ultra Sun & Ultra Moon
    // Used to lump data from the associated games as data assets are shared.
    USUM,

    /// Pokémon Let's Go Pikachu & Eevee
    // Used to lump data from the associated games as data assets are shared.
    GG,

    /// Pokémon Sword & Shield
    // Used to lump data from the associated games as data assets are shared.
    SWSH,

    /// Generation 1 Games
    Gen1,

    /// Generation 2 Games
    Gen2,

    /// Generation 3 Games
    Gen3,

    /// Generation 4 Games
    Gen4,

    /// Generation 5 Games
    Gen5,

    /// Generation 6 Games
    Gen6,

    /// Generation 7 Games
    Gen7,

    /// Generation 8 Games
    Gen8,

    /// Generation 1/2 Game Boy Cartridge Era Only
    // Any special encounters (event data) can only be
    // allowed if the savedata originated from that era.
    GBCartEraOnly,

    /// Pokémon Stadium data origin identifier
    Stadium,

    /// Pokémon Stadium 2 data origin identifier
    Stadium2,

    /// Generation 1 Game Boy Cartridge Era Only data origin identifier
    EventsGBGen1,

    /// Generation 2 Game Boy Cartridge Era Only data origin identifier
    EventsGBGen2,

    /// Generation 1/2 3DS Virtual Console data origin identifier
    VCEvents,
    // End Game Groupings (SaveFile type, roughly)
}
