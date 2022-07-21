use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Pick {
    pub fightname: String, // ex: "Israel Adesanya vs Jared Cannonier"
    pub fav_odds: String,  // ex: "Israel Adesanya 87%"
    pub und_odds: String,  // ex: "Jared Cannonier 13%"
    pub ko_odds: String,   // ex: "KO 40%"
    pub sub_odds: String,  // ex: "SUB 10%"
    pub dec_odds: String,  // ex: "DEC 50%""
}

pub const CONFIG: Item<Config> = Item::new("config");

pub const HISTORY: Map<(String, String), Pick> = Map::new("history");
// String 1 = Name of card like "UFC 279" / Make sure they aren't different since entries won't be deletable
// String 2 = Name of fight like "Khabib vs Mcgregor"
// Pick = Struct containing Selina's odds of outcome, see line X

// Name of Card
// Fight on card
// Picks for fight

// Trait implementations, move to own mod
//pub trait FtNt {
//    fn new() -> Self;
//}
//
//impl FtNt for FightNight {
//
//    fn new() -> FightNight {
//        return FightNight {
//            fight_1: None,
//            fight_2: None,
//            fight_3: None,
//            fight_4: None,
//            fight_5: None,
//        }
//    }
//
//}
//
//pub trait StructToArray {
//    fn as_array(&self) -> [Option<Pick>; 5];
//}
//
//impl StructToArray for FightNight {
//    fn as_array(&self) -> [Option<Pick>; 5] {
//        [
//            self.fight_1.clone(),
//            self.fight_2.clone(),
//            self.fight_3.clone(),
//            self.fight_4.clone(),
//            self.fight_5.clone(),
//        ]
//
//    }
//}
//
