#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Coordinate {
    #[prost(uint32, optional, tag = "1")]
    pub x: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub y: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub z: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Appearances {
    #[prost(message, repeated, tag = "1")]
    pub object: ::prost::alloc::vec::Vec<Appearance>,
    #[prost(message, repeated, tag = "2")]
    pub outfit: ::prost::alloc::vec::Vec<Appearance>,
    #[prost(message, repeated, tag = "3")]
    pub effect: ::prost::alloc::vec::Vec<Appearance>,
    #[prost(message, repeated, tag = "4")]
    pub missile: ::prost::alloc::vec::Vec<Appearance>,
    #[prost(message, optional, tag = "5")]
    pub special_meaning_appearance_ids: ::core::option::Option<
        SpecialMeaningAppearanceIds,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpritePhase {
    #[prost(uint32, optional, tag = "1")]
    pub duration_min: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub duration_max: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpriteAnimation {
    #[prost(uint32, optional, tag = "1")]
    pub default_start_phase: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "2")]
    pub synchronized: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "3")]
    pub random_start_phase: ::core::option::Option<bool>,
    #[prost(enumeration = "AnimationLoopType", optional, tag = "4")]
    pub loop_type: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "5")]
    pub loop_count: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "6")]
    pub sprite_phase: ::prost::alloc::vec::Vec<SpritePhase>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Box {
    #[prost(uint32, optional, tag = "1")]
    pub x: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub y: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub width: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub height: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpriteInfo {
    #[prost(uint32, optional, tag = "1")]
    pub pattern_width: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub pattern_height: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub pattern_depth: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub layers: ::core::option::Option<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "5")]
    pub sprite_id: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, optional, tag = "7")]
    pub bounding_square: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "6")]
    pub animation: ::core::option::Option<SpriteAnimation>,
    #[prost(bool, optional, tag = "8")]
    pub is_opaque: ::core::option::Option<bool>,
    #[prost(message, repeated, tag = "9")]
    pub bounding_box_per_direction: ::prost::alloc::vec::Vec<Box>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FrameGroup {
    #[prost(enumeration = "FixedFrameGroup", optional, tag = "1")]
    pub fixed_frame_group: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "2")]
    pub id: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "3")]
    pub sprite_info: ::core::option::Option<SpriteInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Appearance {
    #[prost(uint32, optional, tag = "1")]
    pub id: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "2")]
    pub frame_group: ::prost::alloc::vec::Vec<FrameGroup>,
    #[prost(message, optional, tag = "3")]
    pub flags: ::core::option::Option<AppearanceFlags>,
    #[prost(string, optional, tag = "4")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes, optional, tag = "5")]
    pub description: ::core::option::Option<Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppearanceFlags {
    #[prost(message, optional, tag = "1")]
    pub bank: ::core::option::Option<AppearanceFlagBank>,
    #[prost(bool, optional, tag = "2")]
    pub clip: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "3")]
    pub bottom: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "4")]
    pub top: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "5")]
    pub container: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "6")]
    pub cumulative: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "7")]
    pub usable: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "8")]
    pub forceuse: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "9")]
    pub multiuse: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "10")]
    pub write: ::core::option::Option<AppearanceFlagWrite>,
    #[prost(message, optional, tag = "11")]
    pub write_once: ::core::option::Option<AppearanceFlagWriteOnce>,
    #[prost(bool, optional, tag = "12")]
    pub liquidpool: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "13")]
    pub unpass: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "14")]
    pub unmove: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "15")]
    pub unsight: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "16")]
    pub avoid: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "17")]
    pub no_movement_animation: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "18")]
    pub take: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "19")]
    pub liquidcontainer: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "20")]
    pub hang: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "21")]
    pub hook: ::core::option::Option<AppearanceFlagHook>,
    #[prost(bool, optional, tag = "22")]
    pub rotate: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "23")]
    pub light: ::core::option::Option<AppearanceFlagLight>,
    #[prost(bool, optional, tag = "24")]
    pub dont_hide: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "25")]
    pub translucent: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "26")]
    pub shift: ::core::option::Option<AppearanceFlagShift>,
    #[prost(message, optional, tag = "27")]
    pub height: ::core::option::Option<AppearanceFlagHeight>,
    #[prost(bool, optional, tag = "28")]
    pub lying_object: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "29")]
    pub animate_always: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "30")]
    pub automap: ::core::option::Option<AppearanceFlagAutomap>,
    #[prost(message, optional, tag = "31")]
    pub lenshelp: ::core::option::Option<AppearanceFlagLenshelp>,
    #[prost(bool, optional, tag = "32")]
    pub fullbank: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "33")]
    pub ignore_look: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "34")]
    pub clothes: ::core::option::Option<AppearanceFlagClothes>,
    #[prost(message, optional, tag = "35")]
    pub default_action: ::core::option::Option<AppearanceFlagDefaultAction>,
    #[prost(message, optional, tag = "36")]
    pub market: ::core::option::Option<AppearanceFlagMarket>,
    #[prost(bool, optional, tag = "37")]
    pub wrap: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "38")]
    pub unwrap: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "39")]
    pub topeffect: ::core::option::Option<bool>,
    #[prost(message, repeated, tag = "40")]
    pub npcsaledata: ::prost::alloc::vec::Vec<AppearanceFlagNpc>,
    #[prost(message, optional, tag = "41")]
    pub changedtoexpire: ::core::option::Option<AppearanceFlagChangedToExpire>,
    #[prost(bool, optional, tag = "42")]
    pub corpse: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "43")]
    pub player_corpse: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "44")]
    pub cyclopediaitem: ::core::option::Option<AppearanceFlagCyclopedia>,
    #[prost(bool, optional, tag = "45")]
    pub ammo: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "46")]
    pub show_off_socket: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "47")]
    pub reportable: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "48")]
    pub upgradeclassification: ::core::option::Option<
        AppearanceFlagUpgradeClassification,
    >,
    #[prost(bool, optional, tag = "49")]
    pub reverse_addons_east: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "50")]
    pub reverse_addons_west: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "51")]
    pub reverse_addons_south: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "52")]
    pub reverse_addons_north: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "53")]
    pub wearout: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "54")]
    pub clockexpire: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "55")]
    pub expire: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "56")]
    pub expirestop: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "57")]
    pub deco_kit: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppearanceFlagUpgradeClassification {
    #[prost(uint32, optional, tag = "1")]
    pub upgrade_classification: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppearanceFlagBank {
    #[prost(uint32, optional, tag = "1")]
    pub waypoints: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppearanceFlagWrite {
    #[prost(uint32, optional, tag = "1")]
    pub max_text_length: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppearanceFlagWriteOnce {
    #[prost(uint32, optional, tag = "1")]
    pub max_text_length_once: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppearanceFlagLight {
    #[prost(uint32, optional, tag = "1")]
    pub brightness: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub color: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppearanceFlagHeight {
    #[prost(uint32, optional, tag = "1")]
    pub elevation: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppearanceFlagShift {
    #[prost(uint32, optional, tag = "1")]
    pub x: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub y: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppearanceFlagClothes {
    #[prost(uint32, optional, tag = "1")]
    pub slot: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppearanceFlagDefaultAction {
    #[prost(enumeration = "PlayerAction", optional, tag = "1")]
    pub action: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppearanceFlagMarket {
    #[prost(enumeration = "ItemCategory", optional, tag = "1")]
    pub category: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "2")]
    pub trade_as_object_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub show_as_object_id: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "4")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration = "PlayerProfession", repeated, packed = "false", tag = "5")]
    pub restrict_to_profession: ::prost::alloc::vec::Vec<i32>,
    #[prost(uint32, optional, tag = "6")]
    pub minimum_level: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppearanceFlagNpc {
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub location: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "3")]
    pub sale_price: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub buy_price: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub currency_object_type_id: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "6")]
    pub currency_quest_flag_display_name: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppearanceFlagAutomap {
    #[prost(uint32, optional, tag = "1")]
    pub color: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppearanceFlagHook {
    #[prost(enumeration = "HookType", optional, tag = "1")]
    pub south: ::core::option::Option<i32>,
    #[prost(enumeration = "HookType", optional, tag = "2")]
    pub east: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppearanceFlagLenshelp {
    #[prost(uint32, optional, tag = "1")]
    pub id: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppearanceFlagChangedToExpire {
    #[prost(uint32, optional, tag = "1")]
    pub former_object_typeid: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppearanceFlagCyclopedia {
    #[prost(uint32, optional, tag = "1")]
    pub cyclopedia_type: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpecialMeaningAppearanceIds {
    #[prost(uint32, optional, tag = "1")]
    pub gold_coin_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub platinum_coin_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub crystal_coin_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub tibia_coin_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub stamped_letter_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub supply_stash_id: ::core::option::Option<u32>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PlayerAction {
    None = 0,
    Look = 1,
    Use = 2,
    Open = 3,
    AutowalkHighlight = 4,
}
impl PlayerAction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PlayerAction::None => "PLAYER_ACTION_NONE",
            PlayerAction::Look => "PLAYER_ACTION_LOOK",
            PlayerAction::Use => "PLAYER_ACTION_USE",
            PlayerAction::Open => "PLAYER_ACTION_OPEN",
            PlayerAction::AutowalkHighlight => "PLAYER_ACTION_AUTOWALK_HIGHLIGHT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PLAYER_ACTION_NONE" => Some(Self::None),
            "PLAYER_ACTION_LOOK" => Some(Self::Look),
            "PLAYER_ACTION_USE" => Some(Self::Use),
            "PLAYER_ACTION_OPEN" => Some(Self::Open),
            "PLAYER_ACTION_AUTOWALK_HIGHLIGHT" => Some(Self::AutowalkHighlight),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ItemCategory {
    Armors = 1,
    Amulets = 2,
    Boots = 3,
    Containers = 4,
    Decoration = 5,
    Food = 6,
    HelmetsHats = 7,
    Legs = 8,
    Others = 9,
    Potions = 10,
    Rings = 11,
    Runes = 12,
    Shields = 13,
    Tools = 14,
    Valuables = 15,
    Ammunition = 16,
    Axes = 17,
    Clubs = 18,
    DistanceWeapons = 19,
    Swords = 20,
    WandsRods = 21,
    PremiumScrolls = 22,
    TibiaCoins = 23,
    CreatureProducts = 24,
}
impl ItemCategory {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ItemCategory::Armors => "ITEM_CATEGORY_ARMORS",
            ItemCategory::Amulets => "ITEM_CATEGORY_AMULETS",
            ItemCategory::Boots => "ITEM_CATEGORY_BOOTS",
            ItemCategory::Containers => "ITEM_CATEGORY_CONTAINERS",
            ItemCategory::Decoration => "ITEM_CATEGORY_DECORATION",
            ItemCategory::Food => "ITEM_CATEGORY_FOOD",
            ItemCategory::HelmetsHats => "ITEM_CATEGORY_HELMETS_HATS",
            ItemCategory::Legs => "ITEM_CATEGORY_LEGS",
            ItemCategory::Others => "ITEM_CATEGORY_OTHERS",
            ItemCategory::Potions => "ITEM_CATEGORY_POTIONS",
            ItemCategory::Rings => "ITEM_CATEGORY_RINGS",
            ItemCategory::Runes => "ITEM_CATEGORY_RUNES",
            ItemCategory::Shields => "ITEM_CATEGORY_SHIELDS",
            ItemCategory::Tools => "ITEM_CATEGORY_TOOLS",
            ItemCategory::Valuables => "ITEM_CATEGORY_VALUABLES",
            ItemCategory::Ammunition => "ITEM_CATEGORY_AMMUNITION",
            ItemCategory::Axes => "ITEM_CATEGORY_AXES",
            ItemCategory::Clubs => "ITEM_CATEGORY_CLUBS",
            ItemCategory::DistanceWeapons => "ITEM_CATEGORY_DISTANCE_WEAPONS",
            ItemCategory::Swords => "ITEM_CATEGORY_SWORDS",
            ItemCategory::WandsRods => "ITEM_CATEGORY_WANDS_RODS",
            ItemCategory::PremiumScrolls => "ITEM_CATEGORY_PREMIUM_SCROLLS",
            ItemCategory::TibiaCoins => "ITEM_CATEGORY_TIBIA_COINS",
            ItemCategory::CreatureProducts => "ITEM_CATEGORY_CREATURE_PRODUCTS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ITEM_CATEGORY_ARMORS" => Some(Self::Armors),
            "ITEM_CATEGORY_AMULETS" => Some(Self::Amulets),
            "ITEM_CATEGORY_BOOTS" => Some(Self::Boots),
            "ITEM_CATEGORY_CONTAINERS" => Some(Self::Containers),
            "ITEM_CATEGORY_DECORATION" => Some(Self::Decoration),
            "ITEM_CATEGORY_FOOD" => Some(Self::Food),
            "ITEM_CATEGORY_HELMETS_HATS" => Some(Self::HelmetsHats),
            "ITEM_CATEGORY_LEGS" => Some(Self::Legs),
            "ITEM_CATEGORY_OTHERS" => Some(Self::Others),
            "ITEM_CATEGORY_POTIONS" => Some(Self::Potions),
            "ITEM_CATEGORY_RINGS" => Some(Self::Rings),
            "ITEM_CATEGORY_RUNES" => Some(Self::Runes),
            "ITEM_CATEGORY_SHIELDS" => Some(Self::Shields),
            "ITEM_CATEGORY_TOOLS" => Some(Self::Tools),
            "ITEM_CATEGORY_VALUABLES" => Some(Self::Valuables),
            "ITEM_CATEGORY_AMMUNITION" => Some(Self::Ammunition),
            "ITEM_CATEGORY_AXES" => Some(Self::Axes),
            "ITEM_CATEGORY_CLUBS" => Some(Self::Clubs),
            "ITEM_CATEGORY_DISTANCE_WEAPONS" => Some(Self::DistanceWeapons),
            "ITEM_CATEGORY_SWORDS" => Some(Self::Swords),
            "ITEM_CATEGORY_WANDS_RODS" => Some(Self::WandsRods),
            "ITEM_CATEGORY_PREMIUM_SCROLLS" => Some(Self::PremiumScrolls),
            "ITEM_CATEGORY_TIBIA_COINS" => Some(Self::TibiaCoins),
            "ITEM_CATEGORY_CREATURE_PRODUCTS" => Some(Self::CreatureProducts),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PlayerProfession {
    Any = -1,
    None = 0,
    Knight = 1,
    Paladin = 2,
    Sorcerer = 3,
    Druid = 4,
    Promoted = 10,
}
impl PlayerProfession {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PlayerProfession::Any => "PLAYER_PROFESSION_ANY",
            PlayerProfession::None => "PLAYER_PROFESSION_NONE",
            PlayerProfession::Knight => "PLAYER_PROFESSION_KNIGHT",
            PlayerProfession::Paladin => "PLAYER_PROFESSION_PALADIN",
            PlayerProfession::Sorcerer => "PLAYER_PROFESSION_SORCERER",
            PlayerProfession::Druid => "PLAYER_PROFESSION_DRUID",
            PlayerProfession::Promoted => "PLAYER_PROFESSION_PROMOTED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PLAYER_PROFESSION_ANY" => Some(Self::Any),
            "PLAYER_PROFESSION_NONE" => Some(Self::None),
            "PLAYER_PROFESSION_KNIGHT" => Some(Self::Knight),
            "PLAYER_PROFESSION_PALADIN" => Some(Self::Paladin),
            "PLAYER_PROFESSION_SORCERER" => Some(Self::Sorcerer),
            "PLAYER_PROFESSION_DRUID" => Some(Self::Druid),
            "PLAYER_PROFESSION_PROMOTED" => Some(Self::Promoted),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AnimationLoopType {
    Pingpong = -1,
    Infinite = 0,
    Counted = 1,
}
impl AnimationLoopType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AnimationLoopType::Pingpong => "ANIMATION_LOOP_TYPE_PINGPONG",
            AnimationLoopType::Infinite => "ANIMATION_LOOP_TYPE_INFINITE",
            AnimationLoopType::Counted => "ANIMATION_LOOP_TYPE_COUNTED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ANIMATION_LOOP_TYPE_PINGPONG" => Some(Self::Pingpong),
            "ANIMATION_LOOP_TYPE_INFINITE" => Some(Self::Infinite),
            "ANIMATION_LOOP_TYPE_COUNTED" => Some(Self::Counted),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HookType {
    South = 1,
    East = 2,
}
impl HookType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            HookType::South => "HOOK_TYPE_SOUTH",
            HookType::East => "HOOK_TYPE_EAST",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "HOOK_TYPE_SOUTH" => Some(Self::South),
            "HOOK_TYPE_EAST" => Some(Self::East),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FixedFrameGroup {
    OutfitIdle = 0,
    OutfitMoving = 1,
    ObjectInitial = 2,
}
impl FixedFrameGroup {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FixedFrameGroup::OutfitIdle => "FIXED_FRAME_GROUP_OUTFIT_IDLE",
            FixedFrameGroup::OutfitMoving => "FIXED_FRAME_GROUP_OUTFIT_MOVING",
            FixedFrameGroup::ObjectInitial => "FIXED_FRAME_GROUP_OBJECT_INITIAL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FIXED_FRAME_GROUP_OUTFIT_IDLE" => Some(Self::OutfitIdle),
            "FIXED_FRAME_GROUP_OUTFIT_MOVING" => Some(Self::OutfitMoving),
            "FIXED_FRAME_GROUP_OBJECT_INITIAL" => Some(Self::ObjectInitial),
            _ => None,
        }
    }
}
