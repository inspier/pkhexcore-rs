#![allow(non_snake_case)]

use deku::prelude::*;

#[allow(non_camel_case_types)]
#[repr(u16)]
/// Species IDs for the corresponding English species name.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, DekuRead, DekuWrite)]
#[deku(type = "u16", ctx = "_endian: deku::ctx::Endian")]
pub enum Species {
    #[deku(id = "0")]
    None,
    #[deku(id = "1")]
    Bulbasaur,
    #[deku(id = "2")]
    Ivysaur,
    #[deku(id = "3")]
    Venusaur,
    #[deku(id = "4")]
    Charmander,
    #[deku(id = "5")]
    Charmeleon,
    #[deku(id = "6")]
    Charizard,
    #[deku(id = "7")]
    Squirtle,
    #[deku(id = "8")]
    Wartortle,
    #[deku(id = "9")]
    Blastoise,
    #[deku(id = "10")]
    Caterpie,
    #[deku(id = "11")]
    Metapod,
    #[deku(id = "12")]
    Butterfree,
    #[deku(id = "13")]
    Weedle,
    #[deku(id = "14")]
    Kakuna,
    #[deku(id = "15")]
    Beedrill,
    #[deku(id = "16")]
    Pidgey,
    #[deku(id = "17")]
    Pidgeotto,
    #[deku(id = "18")]
    Pidgeot,
    #[deku(id = "19")]
    Rattata,
    #[deku(id = "20")]
    Raticate,
    #[deku(id = "21")]
    Spearow,
    #[deku(id = "22")]
    Fearow,
    #[deku(id = "23")]
    Ekans,
    #[deku(id = "24")]
    Arbok,
    #[deku(id = "25")]
    Pikachu,
    #[deku(id = "26")]
    Raichu,
    #[deku(id = "27")]
    Sandshrew,
    #[deku(id = "28")]
    Sandslash,
    #[deku(id = "29")]
    NidoranF,
    #[deku(id = "30")]
    Nidorina,
    #[deku(id = "31")]
    Nidoqueen,
    #[deku(id = "32")]
    NidoranM,
    #[deku(id = "33")]
    Nidorino,
    #[deku(id = "34")]
    Nidoking,
    #[deku(id = "35")]
    Clefairy,
    #[deku(id = "36")]
    Clefable,
    #[deku(id = "37")]
    Vulpix,
    #[deku(id = "38")]
    Ninetales,
    #[deku(id = "39")]
    Jigglypuff,
    #[deku(id = "40")]
    Wigglytuff,
    #[deku(id = "41")]
    Zubat,
    #[deku(id = "42")]
    Golbat,
    #[deku(id = "43")]
    Oddish,
    #[deku(id = "44")]
    Gloom,
    #[deku(id = "45")]
    Vileplume,
    #[deku(id = "46")]
    Paras,
    #[deku(id = "47")]
    Parasect,
    #[deku(id = "48")]
    Venonat,
    #[deku(id = "49")]
    Venomoth,
    #[deku(id = "50")]
    Diglett,
    #[deku(id = "51")]
    Dugtrio,
    #[deku(id = "52")]
    Meowth,
    #[deku(id = "53")]
    Persian,
    #[deku(id = "54")]
    Psyduck,
    #[deku(id = "55")]
    Golduck,
    #[deku(id = "56")]
    Mankey,
    #[deku(id = "57")]
    Primeape,
    #[deku(id = "58")]
    Growlithe,
    #[deku(id = "59")]
    Arcanine,
    #[deku(id = "60")]
    Poliwag,
    #[deku(id = "61")]
    Poliwhirl,
    #[deku(id = "62")]
    Poliwrath,
    #[deku(id = "63")]
    Abra,
    #[deku(id = "64")]
    Kadabra,
    #[deku(id = "65")]
    Alakazam,
    #[deku(id = "66")]
    Machop,
    #[deku(id = "67")]
    Machoke,
    #[deku(id = "68")]
    Machamp,
    #[deku(id = "69")]
    Bellsprout,
    #[deku(id = "70")]
    Weepinbell,
    #[deku(id = "71")]
    Victreebel,
    #[deku(id = "72")]
    Tentacool,
    #[deku(id = "73")]
    Tentacruel,
    #[deku(id = "74")]
    Geodude,
    #[deku(id = "75")]
    Graveler,
    #[deku(id = "76")]
    Golem,
    #[deku(id = "77")]
    Ponyta,
    #[deku(id = "78")]
    Rapidash,
    #[deku(id = "79")]
    Slowpoke,
    #[deku(id = "80")]
    Slowbro,
    #[deku(id = "81")]
    Magnemite,
    #[deku(id = "82")]
    Magneton,
    #[deku(id = "83")]
    Farfetchd,
    #[deku(id = "84")]
    Doduo,
    #[deku(id = "85")]
    Dodrio,
    #[deku(id = "86")]
    Seel,
    #[deku(id = "87")]
    Dewgong,
    #[deku(id = "88")]
    Grimer,
    #[deku(id = "89")]
    Muk,
    #[deku(id = "90")]
    Shellder,
    #[deku(id = "91")]
    Cloyster,
    #[deku(id = "92")]
    Gastly,
    #[deku(id = "93")]
    Haunter,
    #[deku(id = "94")]
    Gengar,
    #[deku(id = "95")]
    Onix,
    #[deku(id = "96")]
    Drowzee,
    #[deku(id = "97")]
    Hypno,
    #[deku(id = "98")]
    Krabby,
    #[deku(id = "99")]
    Kingler,
    #[deku(id = "100")]
    Voltorb,
    #[deku(id = "101")]
    Electrode,
    #[deku(id = "102")]
    Exeggcute,
    #[deku(id = "103")]
    Exeggutor,
    #[deku(id = "104")]
    Cubone,
    #[deku(id = "105")]
    Marowak,
    #[deku(id = "106")]
    Hitmonlee,
    #[deku(id = "107")]
    Hitmonchan,
    #[deku(id = "108")]
    Lickitung,
    #[deku(id = "109")]
    Koffing,
    #[deku(id = "110")]
    Weezing,
    #[deku(id = "111")]
    Rhyhorn,
    #[deku(id = "112")]
    Rhydon,
    #[deku(id = "113")]
    Chansey,
    #[deku(id = "114")]
    Tangela,
    #[deku(id = "115")]
    Kangaskhan,
    #[deku(id = "116")]
    Horsea,
    #[deku(id = "117")]
    Seadra,
    #[deku(id = "118")]
    Goldeen,
    #[deku(id = "119")]
    Seaking,
    #[deku(id = "120")]
    Staryu,
    #[deku(id = "121")]
    Starmie,
    #[deku(id = "122")]
    MrMime,
    #[deku(id = "123")]
    Scyther,
    #[deku(id = "124")]
    Jynx,
    #[deku(id = "125")]
    Electabuzz,
    #[deku(id = "126")]
    Magmar,
    #[deku(id = "127")]
    Pinsir,
    #[deku(id = "128")]
    Tauros,
    #[deku(id = "129")]
    Magikarp,
    #[deku(id = "130")]
    Gyarados,
    #[deku(id = "131")]
    Lapras,
    #[deku(id = "132")]
    Ditto,
    #[deku(id = "133")]
    Eevee,
    #[deku(id = "134")]
    Vaporeon,
    #[deku(id = "135")]
    Jolteon,
    #[deku(id = "136")]
    Flareon,
    #[deku(id = "137")]
    Porygon,
    #[deku(id = "138")]
    Omanyte,
    #[deku(id = "139")]
    Omastar,
    #[deku(id = "140")]
    Kabuto,
    #[deku(id = "141")]
    Kabutops,
    #[deku(id = "142")]
    Aerodactyl,
    #[deku(id = "143")]
    Snorlax,
    #[deku(id = "144")]
    Articuno,
    #[deku(id = "145")]
    Zapdos,
    #[deku(id = "146")]
    Moltres,
    #[deku(id = "147")]
    Dratini,
    #[deku(id = "148")]
    Dragonair,
    #[deku(id = "149")]
    Dragonite,
    #[deku(id = "150")]
    Mewtwo,
    #[deku(id = "151")]
    Mew,
    #[deku(id = "152")]
    Chikorita,
    #[deku(id = "153")]
    Bayleef,
    #[deku(id = "154")]
    Meganium,
    #[deku(id = "155")]
    Cyndaquil,
    #[deku(id = "156")]
    Quilava,
    #[deku(id = "157")]
    Typhlosion,
    #[deku(id = "158")]
    Totodile,
    #[deku(id = "159")]
    Croconaw,
    #[deku(id = "160")]
    Feraligatr,
    #[deku(id = "161")]
    Sentret,
    #[deku(id = "162")]
    Furret,
    #[deku(id = "163")]
    Hoothoot,
    #[deku(id = "164")]
    Noctowl,
    #[deku(id = "165")]
    Ledyba,
    #[deku(id = "166")]
    Ledian,
    #[deku(id = "167")]
    Spinarak,
    #[deku(id = "168")]
    Ariados,
    #[deku(id = "169")]
    Crobat,
    #[deku(id = "170")]
    Chinchou,
    #[deku(id = "171")]
    Lanturn,
    #[deku(id = "172")]
    Pichu,
    #[deku(id = "173")]
    Cleffa,
    #[deku(id = "174")]
    Igglybuff,
    #[deku(id = "175")]
    Togepi,
    #[deku(id = "176")]
    Togetic,
    #[deku(id = "177")]
    Natu,
    #[deku(id = "178")]
    Xatu,
    #[deku(id = "179")]
    Mareep,
    #[deku(id = "180")]
    Flaaffy,
    #[deku(id = "181")]
    Ampharos,
    #[deku(id = "182")]
    Bellossom,
    #[deku(id = "183")]
    Marill,
    #[deku(id = "184")]
    Azumarill,
    #[deku(id = "185")]
    Sudowoodo,
    #[deku(id = "186")]
    Politoed,
    #[deku(id = "187")]
    Hoppip,
    #[deku(id = "188")]
    Skiploom,
    #[deku(id = "189")]
    Jumpluff,
    #[deku(id = "190")]
    Aipom,
    #[deku(id = "191")]
    Sunkern,
    #[deku(id = "192")]
    Sunflora,
    #[deku(id = "193")]
    Yanma,
    #[deku(id = "194")]
    Wooper,
    #[deku(id = "195")]
    Quagsire,
    #[deku(id = "196")]
    Espeon,
    #[deku(id = "197")]
    Umbreon,
    #[deku(id = "198")]
    Murkrow,
    #[deku(id = "199")]
    Slowking,
    #[deku(id = "200")]
    Misdreavus,
    #[deku(id = "201")]
    Unown,
    #[deku(id = "202")]
    Wobbuffet,
    #[deku(id = "203")]
    Girafarig,
    #[deku(id = "204")]
    Pineco,
    #[deku(id = "205")]
    Forretress,
    #[deku(id = "206")]
    Dunsparce,
    #[deku(id = "207")]
    Gligar,
    #[deku(id = "208")]
    Steelix,
    #[deku(id = "209")]
    Snubbull,
    #[deku(id = "210")]
    Granbull,
    #[deku(id = "211")]
    Qwilfish,
    #[deku(id = "212")]
    Scizor,
    #[deku(id = "213")]
    Shuckle,
    #[deku(id = "214")]
    Heracross,
    #[deku(id = "215")]
    Sneasel,
    #[deku(id = "216")]
    Teddiursa,
    #[deku(id = "217")]
    Ursaring,
    #[deku(id = "218")]
    Slugma,
    #[deku(id = "219")]
    Magcargo,
    #[deku(id = "220")]
    Swinub,
    #[deku(id = "221")]
    Piloswine,
    #[deku(id = "222")]
    Corsola,
    #[deku(id = "223")]
    Remoraid,
    #[deku(id = "224")]
    Octillery,
    #[deku(id = "225")]
    Delibird,
    #[deku(id = "226")]
    Mantine,
    #[deku(id = "227")]
    Skarmory,
    #[deku(id = "228")]
    Houndour,
    #[deku(id = "229")]
    Houndoom,
    #[deku(id = "230")]
    Kingdra,
    #[deku(id = "231")]
    Phanpy,
    #[deku(id = "232")]
    Donphan,
    #[deku(id = "233")]
    Porygon2,
    #[deku(id = "234")]
    Stantler,
    #[deku(id = "235")]
    Smeargle,
    #[deku(id = "236")]
    Tyrogue,
    #[deku(id = "237")]
    Hitmontop,
    #[deku(id = "238")]
    Smoochum,
    #[deku(id = "239")]
    Elekid,
    #[deku(id = "240")]
    Magby,
    #[deku(id = "241")]
    Miltank,
    #[deku(id = "242")]
    Blissey,
    #[deku(id = "243")]
    Raikou,
    #[deku(id = "244")]
    Entei,
    #[deku(id = "245")]
    Suicune,
    #[deku(id = "246")]
    Larvitar,
    #[deku(id = "247")]
    Pupitar,
    #[deku(id = "248")]
    Tyranitar,
    #[deku(id = "249")]
    Lugia,
    #[deku(id = "250")]
    HoOh,
    #[deku(id = "251")]
    Celebi,
    #[deku(id = "252")]
    Treecko,
    #[deku(id = "253")]
    Grovyle,
    #[deku(id = "254")]
    Sceptile,
    #[deku(id = "255")]
    Torchic,
    #[deku(id = "256")]
    Combusken,
    #[deku(id = "257")]
    Blaziken,
    #[deku(id = "258")]
    Mudkip,
    #[deku(id = "259")]
    Marshtomp,
    #[deku(id = "260")]
    Swampert,
    #[deku(id = "261")]
    Poochyena,
    #[deku(id = "262")]
    Mightyena,
    #[deku(id = "263")]
    Zigzagoon,
    #[deku(id = "264")]
    Linoone,
    #[deku(id = "265")]
    Wurmple,
    #[deku(id = "266")]
    Silcoon,
    #[deku(id = "267")]
    Beautifly,
    #[deku(id = "268")]
    Cascoon,
    #[deku(id = "269")]
    Dustox,
    #[deku(id = "270")]
    Lotad,
    #[deku(id = "271")]
    Lombre,
    #[deku(id = "272")]
    Ludicolo,
    #[deku(id = "273")]
    Seedot,
    #[deku(id = "274")]
    Nuzleaf,
    #[deku(id = "275")]
    Shiftry,
    #[deku(id = "276")]
    Taillow,
    #[deku(id = "277")]
    Swellow,
    #[deku(id = "278")]
    Wingull,
    #[deku(id = "279")]
    Pelipper,
    #[deku(id = "280")]
    Ralts,
    #[deku(id = "281")]
    Kirlia,
    #[deku(id = "282")]
    Gardevoir,
    #[deku(id = "283")]
    Surskit,
    #[deku(id = "284")]
    Masquerain,
    #[deku(id = "285")]
    Shroomish,
    #[deku(id = "286")]
    Breloom,
    #[deku(id = "287")]
    Slakoth,
    #[deku(id = "288")]
    Vigoroth,
    #[deku(id = "289")]
    Slaking,
    #[deku(id = "290")]
    Nincada,
    #[deku(id = "291")]
    Ninjask,
    #[deku(id = "292")]
    Shedinja,
    #[deku(id = "293")]
    Whismur,
    #[deku(id = "294")]
    Loudred,
    #[deku(id = "295")]
    Exploud,
    #[deku(id = "296")]
    Makuhita,
    #[deku(id = "297")]
    Hariyama,
    #[deku(id = "298")]
    Azurill,
    #[deku(id = "299")]
    Nosepass,
    #[deku(id = "300")]
    Skitty,
    #[deku(id = "301")]
    Delcatty,
    #[deku(id = "302")]
    Sableye,
    #[deku(id = "303")]
    Mawile,
    #[deku(id = "304")]
    Aron,
    #[deku(id = "305")]
    Lairon,
    #[deku(id = "306")]
    Aggron,
    #[deku(id = "307")]
    Meditite,
    #[deku(id = "308")]
    Medicham,
    #[deku(id = "309")]
    Electrike,
    #[deku(id = "310")]
    Manectric,
    #[deku(id = "311")]
    Plusle,
    #[deku(id = "312")]
    Minun,
    #[deku(id = "313")]
    Volbeat,
    #[deku(id = "314")]
    Illumise,
    #[deku(id = "315")]
    Roselia,
    #[deku(id = "316")]
    Gulpin,
    #[deku(id = "317")]
    Swalot,
    #[deku(id = "318")]
    Carvanha,
    #[deku(id = "319")]
    Sharpedo,
    #[deku(id = "320")]
    Wailmer,
    #[deku(id = "321")]
    Wailord,
    #[deku(id = "322")]
    Numel,
    #[deku(id = "323")]
    Camerupt,
    #[deku(id = "324")]
    Torkoal,
    #[deku(id = "325")]
    Spoink,
    #[deku(id = "326")]
    Grumpig,
    #[deku(id = "327")]
    Spinda,
    #[deku(id = "328")]
    Trapinch,
    #[deku(id = "329")]
    Vibrava,
    #[deku(id = "330")]
    Flygon,
    #[deku(id = "331")]
    Cacnea,
    #[deku(id = "332")]
    Cacturne,
    #[deku(id = "333")]
    Swablu,
    #[deku(id = "334")]
    Altaria,
    #[deku(id = "335")]
    Zangoose,
    #[deku(id = "336")]
    Seviper,
    #[deku(id = "337")]
    Lunatone,
    #[deku(id = "338")]
    Solrock,
    #[deku(id = "339")]
    Barboach,
    #[deku(id = "340")]
    Whiscash,
    #[deku(id = "341")]
    Corphish,
    #[deku(id = "342")]
    Crawdaunt,
    #[deku(id = "343")]
    Baltoy,
    #[deku(id = "344")]
    Claydol,
    #[deku(id = "345")]
    Lileep,
    #[deku(id = "346")]
    Cradily,
    #[deku(id = "347")]
    Anorith,
    #[deku(id = "348")]
    Armaldo,
    #[deku(id = "349")]
    Feebas,
    #[deku(id = "350")]
    Milotic,
    #[deku(id = "351")]
    Castform,
    #[deku(id = "352")]
    Kecleon,
    #[deku(id = "353")]
    Shuppet,
    #[deku(id = "354")]
    Banette,
    #[deku(id = "355")]
    Duskull,
    #[deku(id = "356")]
    Dusclops,
    #[deku(id = "357")]
    Tropius,
    #[deku(id = "358")]
    Chimecho,
    #[deku(id = "359")]
    Absol,
    #[deku(id = "360")]
    Wynaut,
    #[deku(id = "361")]
    Snorunt,
    #[deku(id = "362")]
    Glalie,
    #[deku(id = "363")]
    Spheal,
    #[deku(id = "364")]
    Sealeo,
    #[deku(id = "365")]
    Walrein,
    #[deku(id = "366")]
    Clamperl,
    #[deku(id = "367")]
    Huntail,
    #[deku(id = "368")]
    Gorebyss,
    #[deku(id = "369")]
    Relicanth,
    #[deku(id = "370")]
    Luvdisc,
    #[deku(id = "371")]
    Bagon,
    #[deku(id = "372")]
    Shelgon,
    #[deku(id = "373")]
    Salamence,
    #[deku(id = "374")]
    Beldum,
    #[deku(id = "375")]
    Metang,
    #[deku(id = "376")]
    Metagross,
    #[deku(id = "377")]
    Regirock,
    #[deku(id = "378")]
    Regice,
    #[deku(id = "379")]
    Registeel,
    #[deku(id = "380")]
    Latias,
    #[deku(id = "381")]
    Latios,
    #[deku(id = "382")]
    Kyogre,
    #[deku(id = "383")]
    Groudon,
    #[deku(id = "384")]
    Rayquaza,
    #[deku(id = "385")]
    Jirachi,
    #[deku(id = "386")]
    Deoxys,
    #[deku(id = "387")]
    Turtwig,
    #[deku(id = "388")]
    Grotle,
    #[deku(id = "389")]
    Torterra,
    #[deku(id = "390")]
    Chimchar,
    #[deku(id = "391")]
    Monferno,
    #[deku(id = "392")]
    Infernape,
    #[deku(id = "393")]
    Piplup,
    #[deku(id = "394")]
    Prinplup,
    #[deku(id = "395")]
    Empoleon,
    #[deku(id = "396")]
    Starly,
    #[deku(id = "397")]
    Staravia,
    #[deku(id = "398")]
    Staraptor,
    #[deku(id = "399")]
    Bidoof,
    #[deku(id = "400")]
    Bibarel,
    #[deku(id = "401")]
    Kricketot,
    #[deku(id = "402")]
    Kricketune,
    #[deku(id = "403")]
    Shinx,
    #[deku(id = "404")]
    Luxio,
    #[deku(id = "405")]
    Luxray,
    #[deku(id = "406")]
    Budew,
    #[deku(id = "407")]
    Roserade,
    #[deku(id = "408")]
    Cranidos,
    #[deku(id = "409")]
    Rampardos,
    #[deku(id = "410")]
    Shieldon,
    #[deku(id = "411")]
    Bastiodon,
    #[deku(id = "412")]
    Burmy,
    #[deku(id = "413")]
    Wormadam,
    #[deku(id = "414")]
    Mothim,
    #[deku(id = "415")]
    Combee,
    #[deku(id = "416")]
    Vespiquen,
    #[deku(id = "417")]
    Pachirisu,
    #[deku(id = "418")]
    Buizel,
    #[deku(id = "419")]
    Floatzel,
    #[deku(id = "420")]
    Cherubi,
    #[deku(id = "421")]
    Cherrim,
    #[deku(id = "422")]
    Shellos,
    #[deku(id = "423")]
    Gastrodon,
    #[deku(id = "424")]
    Ambipom,
    #[deku(id = "425")]
    Drifloon,
    #[deku(id = "426")]
    Drifblim,
    #[deku(id = "427")]
    Buneary,
    #[deku(id = "428")]
    Lopunny,
    #[deku(id = "429")]
    Mismagius,
    #[deku(id = "430")]
    Honchkrow,
    #[deku(id = "431")]
    Glameow,
    #[deku(id = "432")]
    Purugly,
    #[deku(id = "433")]
    Chingling,
    #[deku(id = "434")]
    Stunky,
    #[deku(id = "435")]
    Skuntank,
    #[deku(id = "436")]
    Bronzor,
    #[deku(id = "437")]
    Bronzong,
    #[deku(id = "438")]
    Bonsly,
    #[deku(id = "439")]
    MimeJr,
    #[deku(id = "440")]
    Happiny,
    #[deku(id = "441")]
    Chatot,
    #[deku(id = "442")]
    Spiritomb,
    #[deku(id = "443")]
    Gible,
    #[deku(id = "444")]
    Gabite,
    #[deku(id = "445")]
    Garchomp,
    #[deku(id = "446")]
    Munchlax,
    #[deku(id = "447")]
    Riolu,
    #[deku(id = "448")]
    Lucario,
    #[deku(id = "449")]
    Hippopotas,
    #[deku(id = "450")]
    Hippowdon,
    #[deku(id = "451")]
    Skorupi,
    #[deku(id = "452")]
    Drapion,
    #[deku(id = "453")]
    Croagunk,
    #[deku(id = "454")]
    Toxicroak,
    #[deku(id = "455")]
    Carnivine,
    #[deku(id = "456")]
    Finneon,
    #[deku(id = "457")]
    Lumineon,
    #[deku(id = "458")]
    Mantyke,
    #[deku(id = "459")]
    Snover,
    #[deku(id = "460")]
    Abomasnow,
    #[deku(id = "461")]
    Weavile,
    #[deku(id = "462")]
    Magnezone,
    #[deku(id = "463")]
    Lickilicky,
    #[deku(id = "464")]
    Rhyperior,
    #[deku(id = "465")]
    Tangrowth,
    #[deku(id = "466")]
    Electivire,
    #[deku(id = "467")]
    Magmortar,
    #[deku(id = "468")]
    Togekiss,
    #[deku(id = "469")]
    Yanmega,
    #[deku(id = "470")]
    Leafeon,
    #[deku(id = "471")]
    Glaceon,
    #[deku(id = "472")]
    Gliscor,
    #[deku(id = "473")]
    Mamoswine,
    #[deku(id = "474")]
    PorygonZ,
    #[deku(id = "475")]
    Gallade,
    #[deku(id = "476")]
    Probopass,
    #[deku(id = "477")]
    Dusknoir,
    #[deku(id = "478")]
    Froslass,
    #[deku(id = "479")]
    Rotom,
    #[deku(id = "480")]
    Uxie,
    #[deku(id = "481")]
    Mesprit,
    #[deku(id = "482")]
    Azelf,
    #[deku(id = "483")]
    Dialga,
    #[deku(id = "484")]
    Palkia,
    #[deku(id = "485")]
    Heatran,
    #[deku(id = "486")]
    Regigigas,
    #[deku(id = "487")]
    Giratina,
    #[deku(id = "488")]
    Cresselia,
    #[deku(id = "489")]
    Phione,
    #[deku(id = "490")]
    Manaphy,
    #[deku(id = "491")]
    Darkrai,
    #[deku(id = "492")]
    Shaymin,
    #[deku(id = "493")]
    Arceus,
    #[deku(id = "494")]
    Victini,
    #[deku(id = "495")]
    Snivy,
    #[deku(id = "496")]
    Servine,
    #[deku(id = "497")]
    Serperior,
    #[deku(id = "498")]
    Tepig,
    #[deku(id = "499")]
    Pignite,
    #[deku(id = "500")]
    Emboar,
    #[deku(id = "501")]
    Oshawott,
    #[deku(id = "502")]
    Dewott,
    #[deku(id = "503")]
    Samurott,
    #[deku(id = "504")]
    Patrat,
    #[deku(id = "505")]
    Watchog,
    #[deku(id = "506")]
    Lillipup,
    #[deku(id = "507")]
    Herdier,
    #[deku(id = "508")]
    Stoutland,
    #[deku(id = "509")]
    Purrloin,
    #[deku(id = "510")]
    Liepard,
    #[deku(id = "511")]
    Pansage,
    #[deku(id = "512")]
    Simisage,
    #[deku(id = "513")]
    Pansear,
    #[deku(id = "514")]
    Simisear,
    #[deku(id = "515")]
    Panpour,
    #[deku(id = "516")]
    Simipour,
    #[deku(id = "517")]
    Munna,
    #[deku(id = "518")]
    Musharna,
    #[deku(id = "519")]
    Pidove,
    #[deku(id = "520")]
    Tranquill,
    #[deku(id = "521")]
    Unfezant,
    #[deku(id = "522")]
    Blitzle,
    #[deku(id = "523")]
    Zebstrika,
    #[deku(id = "524")]
    Roggenrola,
    #[deku(id = "525")]
    Boldore,
    #[deku(id = "526")]
    Gigalith,
    #[deku(id = "527")]
    Woobat,
    #[deku(id = "528")]
    Swoobat,
    #[deku(id = "529")]
    Drilbur,
    #[deku(id = "530")]
    Excadrill,
    #[deku(id = "531")]
    Audino,
    #[deku(id = "532")]
    Timburr,
    #[deku(id = "533")]
    Gurdurr,
    #[deku(id = "534")]
    Conkeldurr,
    #[deku(id = "535")]
    Tympole,
    #[deku(id = "536")]
    Palpitoad,
    #[deku(id = "537")]
    Seismitoad,
    #[deku(id = "538")]
    Throh,
    #[deku(id = "539")]
    Sawk,
    #[deku(id = "540")]
    Sewaddle,
    #[deku(id = "541")]
    Swadloon,
    #[deku(id = "542")]
    Leavanny,
    #[deku(id = "543")]
    Venipede,
    #[deku(id = "544")]
    Whirlipede,
    #[deku(id = "545")]
    Scolipede,
    #[deku(id = "546")]
    Cottonee,
    #[deku(id = "547")]
    Whimsicott,
    #[deku(id = "548")]
    Petilil,
    #[deku(id = "549")]
    Lilligant,
    #[deku(id = "550")]
    Basculin,
    #[deku(id = "551")]
    Sandile,
    #[deku(id = "552")]
    Krokorok,
    #[deku(id = "553")]
    Krookodile,
    #[deku(id = "554")]
    Darumaka,
    #[deku(id = "555")]
    Darmanitan,
    #[deku(id = "556")]
    Maractus,
    #[deku(id = "557")]
    Dwebble,
    #[deku(id = "558")]
    Crustle,
    #[deku(id = "559")]
    Scraggy,
    #[deku(id = "560")]
    Scrafty,
    #[deku(id = "561")]
    Sigilyph,
    #[deku(id = "562")]
    Yamask,
    #[deku(id = "563")]
    Cofagrigus,
    #[deku(id = "564")]
    Tirtouga,
    #[deku(id = "565")]
    Carracosta,
    #[deku(id = "566")]
    Archen,
    #[deku(id = "567")]
    Archeops,
    #[deku(id = "568")]
    Trubbish,
    #[deku(id = "569")]
    Garbodor,
    #[deku(id = "570")]
    Zorua,
    #[deku(id = "571")]
    Zoroark,
    #[deku(id = "572")]
    Minccino,
    #[deku(id = "573")]
    Cinccino,
    #[deku(id = "574")]
    Gothita,
    #[deku(id = "575")]
    Gothorita,
    #[deku(id = "576")]
    Gothitelle,
    #[deku(id = "577")]
    Solosis,
    #[deku(id = "578")]
    Duosion,
    #[deku(id = "579")]
    Reuniclus,
    #[deku(id = "580")]
    Ducklett,
    #[deku(id = "581")]
    Swanna,
    #[deku(id = "582")]
    Vanillite,
    #[deku(id = "583")]
    Vanillish,
    #[deku(id = "584")]
    Vanilluxe,
    #[deku(id = "585")]
    Deerling,
    #[deku(id = "586")]
    Sawsbuck,
    #[deku(id = "587")]
    Emolga,
    #[deku(id = "588")]
    Karrablast,
    #[deku(id = "589")]
    Escavalier,
    #[deku(id = "590")]
    Foongus,
    #[deku(id = "591")]
    Amoonguss,
    #[deku(id = "592")]
    Frillish,
    #[deku(id = "593")]
    Jellicent,
    #[deku(id = "594")]
    Alomomola,
    #[deku(id = "595")]
    Joltik,
    #[deku(id = "596")]
    Galvantula,
    #[deku(id = "597")]
    Ferroseed,
    #[deku(id = "598")]
    Ferrothorn,
    #[deku(id = "599")]
    Klink,
    #[deku(id = "600")]
    Klang,
    #[deku(id = "601")]
    Klinklang,
    #[deku(id = "602")]
    Tynamo,
    #[deku(id = "603")]
    Eelektrik,
    #[deku(id = "604")]
    Eelektross,
    #[deku(id = "605")]
    Elgyem,
    #[deku(id = "606")]
    Beheeyem,
    #[deku(id = "607")]
    Litwick,
    #[deku(id = "608")]
    Lampent,
    #[deku(id = "609")]
    Chandelure,
    #[deku(id = "610")]
    Axew,
    #[deku(id = "611")]
    Fraxure,
    #[deku(id = "612")]
    Haxorus,
    #[deku(id = "613")]
    Cubchoo,
    #[deku(id = "614")]
    Beartic,
    #[deku(id = "615")]
    Cryogonal,
    #[deku(id = "616")]
    Shelmet,
    #[deku(id = "617")]
    Accelgor,
    #[deku(id = "618")]
    Stunfisk,
    #[deku(id = "619")]
    Mienfoo,
    #[deku(id = "620")]
    Mienshao,
    #[deku(id = "621")]
    Druddigon,
    #[deku(id = "622")]
    Golett,
    #[deku(id = "623")]
    Golurk,
    #[deku(id = "624")]
    Pawniard,
    #[deku(id = "625")]
    Bisharp,
    #[deku(id = "626")]
    Bouffalant,
    #[deku(id = "627")]
    Rufflet,
    #[deku(id = "628")]
    Braviary,
    #[deku(id = "629")]
    Vullaby,
    #[deku(id = "630")]
    Mandibuzz,
    #[deku(id = "631")]
    Heatmor,
    #[deku(id = "632")]
    Durant,
    #[deku(id = "633")]
    Deino,
    #[deku(id = "634")]
    Zweilous,
    #[deku(id = "635")]
    Hydreigon,
    #[deku(id = "636")]
    Larvesta,
    #[deku(id = "637")]
    Volcarona,
    #[deku(id = "638")]
    Cobalion,
    #[deku(id = "639")]
    Terrakion,
    #[deku(id = "640")]
    Virizion,
    #[deku(id = "641")]
    Tornadus,
    #[deku(id = "642")]
    Thundurus,
    #[deku(id = "643")]
    Reshiram,
    #[deku(id = "644")]
    Zekrom,
    #[deku(id = "645")]
    Landorus,
    #[deku(id = "646")]
    Kyurem,
    #[deku(id = "647")]
    Keldeo,
    #[deku(id = "648")]
    Meloetta,
    #[deku(id = "649")]
    Genesect,
    #[deku(id = "650")]
    Chespin,
    #[deku(id = "651")]
    Quilladin,
    #[deku(id = "652")]
    Chesnaught,
    #[deku(id = "653")]
    Fennekin,
    #[deku(id = "654")]
    Braixen,
    #[deku(id = "655")]
    Delphox,
    #[deku(id = "656")]
    Froakie,
    #[deku(id = "657")]
    Frogadier,
    #[deku(id = "658")]
    Greninja,
    #[deku(id = "659")]
    Bunnelby,
    #[deku(id = "660")]
    Diggersby,
    #[deku(id = "661")]
    Fletchling,
    #[deku(id = "662")]
    Fletchinder,
    #[deku(id = "663")]
    Talonflame,
    #[deku(id = "664")]
    Scatterbug,
    #[deku(id = "665")]
    Spewpa,
    #[deku(id = "666")]
    Vivillon,
    #[deku(id = "667")]
    Litleo,
    #[deku(id = "668")]
    Pyroar,
    #[deku(id = "669")]
    Flabébé,
    #[deku(id = "670")]
    Floette,
    #[deku(id = "671")]
    Florges,
    #[deku(id = "672")]
    Skiddo,
    #[deku(id = "673")]
    Gogoat,
    #[deku(id = "674")]
    Pancham,
    #[deku(id = "675")]
    Pangoro,
    #[deku(id = "676")]
    Furfrou,
    #[deku(id = "677")]
    Espurr,
    #[deku(id = "678")]
    Meowstic,
    #[deku(id = "679")]
    Honedge,
    #[deku(id = "680")]
    Doublade,
    #[deku(id = "681")]
    Aegislash,
    #[deku(id = "682")]
    Spritzee,
    #[deku(id = "683")]
    Aromatisse,
    #[deku(id = "684")]
    Swirlix,
    #[deku(id = "685")]
    Slurpuff,
    #[deku(id = "686")]
    Inkay,
    #[deku(id = "687")]
    Malamar,
    #[deku(id = "688")]
    Binacle,
    #[deku(id = "689")]
    Barbaracle,
    #[deku(id = "690")]
    Skrelp,
    #[deku(id = "691")]
    Dragalge,
    #[deku(id = "692")]
    Clauncher,
    #[deku(id = "693")]
    Clawitzer,
    #[deku(id = "694")]
    Helioptile,
    #[deku(id = "695")]
    Heliolisk,
    #[deku(id = "696")]
    Tyrunt,
    #[deku(id = "697")]
    Tyrantrum,
    #[deku(id = "698")]
    Amaura,
    #[deku(id = "699")]
    Aurorus,
    #[deku(id = "700")]
    Sylveon,
    #[deku(id = "701")]
    Hawlucha,
    #[deku(id = "702")]
    Dedenne,
    #[deku(id = "703")]
    Carbink,
    #[deku(id = "704")]
    Goomy,
    #[deku(id = "705")]
    Sliggoo,
    #[deku(id = "706")]
    Goodra,
    #[deku(id = "707")]
    Klefki,
    #[deku(id = "708")]
    Phantump,
    #[deku(id = "709")]
    Trevenant,
    #[deku(id = "710")]
    Pumpkaboo,
    #[deku(id = "711")]
    Gourgeist,
    #[deku(id = "712")]
    Bergmite,
    #[deku(id = "713")]
    Avalugg,
    #[deku(id = "714")]
    Noibat,
    #[deku(id = "715")]
    Noivern,
    #[deku(id = "716")]
    Xerneas,
    #[deku(id = "717")]
    Yveltal,
    #[deku(id = "718")]
    Zygarde,
    #[deku(id = "719")]
    Diancie,
    #[deku(id = "720")]
    Hoopa,
    #[deku(id = "721")]
    Volcanion,
    #[deku(id = "722")]
    Rowlet,
    #[deku(id = "723")]
    Dartrix,
    #[deku(id = "724")]
    Decidueye,
    #[deku(id = "725")]
    Litten,
    #[deku(id = "726")]
    Torracat,
    #[deku(id = "727")]
    Incineroar,
    #[deku(id = "728")]
    Popplio,
    #[deku(id = "729")]
    Brionne,
    #[deku(id = "730")]
    Primarina,
    #[deku(id = "731")]
    Pikipek,
    #[deku(id = "732")]
    Trumbeak,
    #[deku(id = "733")]
    Toucannon,
    #[deku(id = "734")]
    Yungoos,
    #[deku(id = "735")]
    Gumshoos,
    #[deku(id = "736")]
    Grubbin,
    #[deku(id = "737")]
    Charjabug,
    #[deku(id = "738")]
    Vikavolt,
    #[deku(id = "739")]
    Crabrawler,
    #[deku(id = "740")]
    Crabominable,
    #[deku(id = "741")]
    Oricorio,
    #[deku(id = "742")]
    Cutiefly,
    #[deku(id = "743")]
    Ribombee,
    #[deku(id = "744")]
    Rockruff,
    #[deku(id = "745")]
    Lycanroc,
    #[deku(id = "746")]
    Wishiwashi,
    #[deku(id = "747")]
    Mareanie,
    #[deku(id = "748")]
    Toxapex,
    #[deku(id = "749")]
    Mudbray,
    #[deku(id = "750")]
    Mudsdale,
    #[deku(id = "751")]
    Dewpider,
    #[deku(id = "752")]
    Araquanid,
    #[deku(id = "753")]
    Fomantis,
    #[deku(id = "754")]
    Lurantis,
    #[deku(id = "755")]
    Morelull,
    #[deku(id = "756")]
    Shiinotic,
    #[deku(id = "757")]
    Salandit,
    #[deku(id = "758")]
    Salazzle,
    #[deku(id = "759")]
    Stufful,
    #[deku(id = "760")]
    Bewear,
    #[deku(id = "761")]
    Bounsweet,
    #[deku(id = "762")]
    Steenee,
    #[deku(id = "763")]
    Tsareena,
    #[deku(id = "764")]
    Comfey,
    #[deku(id = "765")]
    Oranguru,
    #[deku(id = "766")]
    Passimian,
    #[deku(id = "767")]
    Wimpod,
    #[deku(id = "768")]
    Golisopod,
    #[deku(id = "769")]
    Sandygast,
    #[deku(id = "770")]
    Palossand,
    #[deku(id = "771")]
    Pyukumuku,
    #[deku(id = "772")]
    TypeNull,
    #[deku(id = "773")]
    Silvally,
    #[deku(id = "774")]
    Minior,
    #[deku(id = "775")]
    Komala,
    #[deku(id = "776")]
    Turtonator,
    #[deku(id = "777")]
    Togedemaru,
    #[deku(id = "778")]
    Mimikyu,
    #[deku(id = "779")]
    Bruxish,
    #[deku(id = "780")]
    Drampa,
    #[deku(id = "781")]
    Dhelmise,
    #[deku(id = "782")]
    Jangmoo,
    #[deku(id = "783")]
    Hakamoo,
    #[deku(id = "784")]
    Kommoo,
    #[deku(id = "785")]
    TapuKoko,
    #[deku(id = "786")]
    TapuLele,
    #[deku(id = "787")]
    TapuBulu,
    #[deku(id = "788")]
    TapuFini,
    #[deku(id = "789")]
    Cosmog,
    #[deku(id = "790")]
    Cosmoem,
    #[deku(id = "791")]
    Solgaleo,
    #[deku(id = "792")]
    Lunala,
    #[deku(id = "793")]
    Nihilego,
    #[deku(id = "794")]
    Buzzwole,
    #[deku(id = "795")]
    Pheromosa,
    #[deku(id = "796")]
    Xurkitree,
    #[deku(id = "797")]
    Celesteela,
    #[deku(id = "798")]
    Kartana,
    #[deku(id = "799")]
    Guzzlord,
    #[deku(id = "800")]
    Necrozma,
    #[deku(id = "801")]
    Magearna,
    #[deku(id = "802")]
    Marshadow,
    #[deku(id = "803")]
    Poipole,
    #[deku(id = "804")]
    Naganadel,
    #[deku(id = "805")]
    Stakataka,
    #[deku(id = "806")]
    Blacephalon,
    #[deku(id = "807")]
    Zeraora,
    #[deku(id = "808")]
    Meltan,
    #[deku(id = "809")]
    Melmetal,
    #[deku(id = "810")]
    Grookey,
    #[deku(id = "811")]
    Thwackey,
    #[deku(id = "812")]
    Rillaboom,
    #[deku(id = "813")]
    Scorbunny,
    #[deku(id = "814")]
    Raboot,
    #[deku(id = "815")]
    Cinderace,
    #[deku(id = "816")]
    Sobble,
    #[deku(id = "817")]
    Drizzile,
    #[deku(id = "818")]
    Inteleon,
    #[deku(id = "819")]
    Skwovet,
    #[deku(id = "820")]
    Greedent,
    #[deku(id = "821")]
    Rookidee,
    #[deku(id = "822")]
    Corvisquire,
    #[deku(id = "823")]
    Corviknight,
    #[deku(id = "824")]
    Blipbug,
    #[deku(id = "825")]
    Dottler,
    #[deku(id = "826")]
    Orbeetle,
    #[deku(id = "827")]
    Nickit,
    #[deku(id = "828")]
    Thievul,
    #[deku(id = "829")]
    Gossifleur,
    #[deku(id = "830")]
    Eldegoss,
    #[deku(id = "831")]
    Wooloo,
    #[deku(id = "832")]
    Dubwool,
    #[deku(id = "833")]
    Chewtle,
    #[deku(id = "834")]
    Drednaw,
    #[deku(id = "835")]
    Yamper,
    #[deku(id = "836")]
    Boltund,
    #[deku(id = "837")]
    Rolycoly,
    #[deku(id = "838")]
    Carkol,
    #[deku(id = "839")]
    Coalossal,
    #[deku(id = "840")]
    Applin,
    #[deku(id = "841")]
    Flapple,
    #[deku(id = "842")]
    Appletun,
    #[deku(id = "843")]
    Silicobra,
    #[deku(id = "844")]
    Sandaconda,
    #[deku(id = "845")]
    Cramorant,
    #[deku(id = "846")]
    Arrokuda,
    #[deku(id = "847")]
    Barraskewda,
    #[deku(id = "848")]
    Toxel,
    #[deku(id = "849")]
    Toxtricity,
    #[deku(id = "850")]
    Sizzlipede,
    #[deku(id = "851")]
    Centiskorch,
    #[deku(id = "852")]
    Clobbopus,
    #[deku(id = "853")]
    Grapploct,
    #[deku(id = "854")]
    Sinistea,
    #[deku(id = "855")]
    Polteageist,
    #[deku(id = "856")]
    Hatenna,
    #[deku(id = "857")]
    Hattrem,
    #[deku(id = "858")]
    Hatterene,
    #[deku(id = "859")]
    Impidimp,
    #[deku(id = "860")]
    Morgrem,
    #[deku(id = "861")]
    Grimmsnarl,
    #[deku(id = "862")]
    Obstagoon,
    #[deku(id = "863")]
    Perrserker,
    #[deku(id = "864")]
    Cursola,
    #[deku(id = "865")]
    Sirfetchd,
    #[deku(id = "866")]
    MrRime,
    #[deku(id = "867")]
    Runerigus,
    #[deku(id = "868")]
    Milcery,
    #[deku(id = "869")]
    Alcremie,
    #[deku(id = "870")]
    Falinks,
    #[deku(id = "871")]
    Pincurchin,
    #[deku(id = "872")]
    Snom,
    #[deku(id = "873")]
    Frosmoth,
    #[deku(id = "874")]
    Stonjourner,
    #[deku(id = "875")]
    Eiscue,
    #[deku(id = "876")]
    Indeedee,
    #[deku(id = "877")]
    Morpeko,
    #[deku(id = "878")]
    Cufant,
    #[deku(id = "879")]
    Copperajah,
    #[deku(id = "880")]
    Dracozolt,
    #[deku(id = "881")]
    Arctozolt,
    #[deku(id = "882")]
    Dracovish,
    #[deku(id = "883")]
    Arctovish,
    #[deku(id = "884")]
    Duraludon,
    #[deku(id = "885")]
    Dreepy,
    #[deku(id = "886")]
    Drakloak,
    #[deku(id = "887")]
    Dragapult,
    #[deku(id = "888")]
    Zacian,
    #[deku(id = "889")]
    Zamazenta,
    #[deku(id = "890")]
    Eternatus,
    #[deku(id = "891")]
    Kubfu,
    #[deku(id = "892")]
    Urshifu,
    #[deku(id = "893")]
    Zarude,
    #[deku(id = "894")]
    MAX_COUNT,
}

#[allow(non_snake_case)]
impl_from!(Species for u16, i32);

#[allow(unused_macros)]
macro_rules! species {
    ($species:tt) => {
        string_to_enum!(Species, $species)
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn species_macro_test() {
        assert_eq!(Species::Blastoise, species!(Blastoise));
    }
}
