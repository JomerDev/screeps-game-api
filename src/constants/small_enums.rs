//! Various constants translated as small enums.
use std::{borrow::Cow, fmt};

use enum_iterator::Sequence;
use js_sys::JsString;
use num_derive::FromPrimitive;
use serde::{
    de::{Error as _, Unexpected},
    Deserialize, Serialize,
};
use serde_repr::{Deserialize_repr, Serialize_repr};
use wasm_bindgen::prelude::*;

use super::{macros::named_enum_serialize_deserialize, InvalidConstantString};
use crate::{
    constants::find::{Exit, Find},
    prelude::*,
};

/// Translates non-OK return codes.
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum ErrorCode {
    NotOwner = -1,
    NoPath = -2,
    NameExists = -3,
    Busy = -4,
    NotFound = -5,
    NotEnough = -6,
    InvalidTarget = -7,
    Full = -8,
    NotInRange = -9,
    InvalidArgs = -10,
    Tired = -11,
    NoBodypart = -12,
    RclNotEnough = -14,
    GclNotEnough = -15,
}

impl FromReturnCode for ErrorCode {
    type Error = Self;

    fn result_from_i8(val: i8) -> Result<(), Self::Error> {
        match val {
            0 => Ok(()),
            -1 => Err(ErrorCode::NotOwner),
            -2 => Err(ErrorCode::NoPath),
            -3 => Err(ErrorCode::NameExists),
            -4 => Err(ErrorCode::Busy),
            -5 => Err(ErrorCode::NotFound),
            -6 => Err(ErrorCode::NotEnough),
            -7 => Err(ErrorCode::InvalidTarget),
            -8 => Err(ErrorCode::Full),
            -9 => Err(ErrorCode::NotInRange),
            -10 => Err(ErrorCode::InvalidArgs),
            -11 => Err(ErrorCode::Tired),
            -12 => Err(ErrorCode::NoBodypart),
            -14 => Err(ErrorCode::RclNotEnough),
            -15 => Err(ErrorCode::GclNotEnough),
            // SAFETY: Return codes must always be one of the values already covered
            #[cfg(feature = "unsafe-return-conversion")]
            _ => unsafe { std::hint::unreachable_unchecked() },
            #[cfg(not(feature = "unsafe-return-conversion"))]
            _ => unreachable!(),
        }
    }

    fn try_result_from_i8(val: i8) -> Option<Result<(), Self::Error>> {
        match val {
            0 => Some(Ok(())),
            -1 => Some(Err(ErrorCode::NotOwner)),
            -2 => Some(Err(ErrorCode::NoPath)),
            -3 => Some(Err(ErrorCode::NameExists)),
            -4 => Some(Err(ErrorCode::Busy)),
            -5 => Some(Err(ErrorCode::NotFound)),
            -6 => Some(Err(ErrorCode::NotEnough)),
            -7 => Some(Err(ErrorCode::InvalidTarget)),
            -8 => Some(Err(ErrorCode::Full)),
            -9 => Some(Err(ErrorCode::NotInRange)),
            -10 => Some(Err(ErrorCode::InvalidArgs)),
            -11 => Some(Err(ErrorCode::Tired)),
            -12 => Some(Err(ErrorCode::NoBodypart)),
            -14 => Some(Err(ErrorCode::RclNotEnough)),
            -15 => Some(Err(ErrorCode::GclNotEnough)),
            _ => None,
        }
    }
}

/// Translates direction constants.
#[wasm_bindgen]
#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Hash,
    FromPrimitive,
    Serialize_repr,
    Deserialize_repr,
    Sequence,
)]
#[repr(u8)]
pub enum Direction {
    Top = 1,
    TopRight = 2,
    Right = 3,
    BottomRight = 4,
    Bottom = 5,
    BottomLeft = 6,
    Left = 7,
    TopLeft = 8,
}

impl From<Direction> for (i32, i32) {
    /// Returns the change in (x, y) when moving in each direction.
    #[inline]
    fn from(direction: Direction) -> (i32, i32) {
        match direction {
            Direction::Top => (0, -1),
            Direction::TopRight => (1, -1),
            Direction::Right => (1, 0),
            Direction::BottomRight => (1, 1),
            Direction::Bottom => (0, 1),
            Direction::BottomLeft => (-1, 1),
            Direction::Left => (-1, 0),
            Direction::TopLeft => (-1, -1),
        }
    }
}

impl ::std::ops::Neg for Direction {
    type Output = Direction;

    /// Negates this direction. Top goes to Bottom, TopRight goes to BottomLeft,
    /// etc.
    ///
    /// Example usage:
    ///
    /// ```
    /// use screeps::Direction::*;
    ///
    /// assert_eq!(-Top, Bottom);
    /// assert_eq!(-BottomRight, TopLeft);
    /// assert_eq!(-Left, Right);
    /// ```
    #[inline]
    fn neg(self) -> Direction {
        use Direction::*;

        match self {
            Top => Bottom,
            TopRight => BottomLeft,
            Right => Left,
            BottomRight => TopLeft,
            Bottom => Top,
            BottomLeft => TopRight,
            Left => Right,
            TopLeft => BottomRight,
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch = match *self {
            Direction::Top => "↑",
            Direction::TopRight => "↗",
            Direction::Right => "→",
            Direction::BottomRight => "↘",
            Direction::Bottom => "↓",
            Direction::BottomLeft => "↙",
            Direction::Left => "←",
            Direction::TopLeft => "↖",
        };
        f.write_str(ch)
    }
}

/// Type used for when the game returns a direction to an exit.
///
/// Restricted more than `Direction` in that it can't be diagonal. Used as the
/// result of [`Room::find_exit_to`].
///
/// Can be converted to [`Find`] for immediate use of [`Room::find`]
/// and [`Direction`].
///
/// [`Room::find`]: crate::objects::Room::find
/// [`Room::find_exit_to`]: crate::objects::Room::find_exit_to
#[wasm_bindgen]
#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Hash,
    FromPrimitive,
    Serialize_repr,
    Deserialize_repr,
    Sequence,
)]
#[repr(u8)]
pub enum ExitDirection {
    Top = 1,
    Right = 3,
    Bottom = 5,
    Left = 7,
}

impl From<ExitDirection> for Find {
    #[inline]
    fn from(dir: ExitDirection) -> Self {
        match dir {
            ExitDirection::Top => Find::ExitTop,
            ExitDirection::Right => Find::ExitRight,
            ExitDirection::Bottom => Find::ExitBottom,
            ExitDirection::Left => Find::ExitLeft,
        }
    }
}

impl From<ExitDirection> for Direction {
    #[inline]
    fn from(dir: ExitDirection) -> Self {
        match dir {
            ExitDirection::Top => Direction::Top,
            ExitDirection::Right => Direction::Right,
            ExitDirection::Bottom => Direction::Bottom,
            ExitDirection::Left => Direction::Left,
        }
    }
}

impl From<ExitDirection> for Exit {
    fn from(value: ExitDirection) -> Self {
        match value {
            ExitDirection::Top => Exit::Top,
            ExitDirection::Right => Exit::Right,
            ExitDirection::Bottom => Exit::Bottom,
            ExitDirection::Left => Exit::Left,
        }
    }
}

/// Translates `COLOR_*` and `COLORS_ALL` constants.
#[wasm_bindgen]
#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    FromPrimitive,
    Hash,
    Deserialize_repr,
    Serialize_repr,
    Sequence,
)]
#[repr(u8)]
pub enum Color {
    Red = 1,
    Purple = 2,
    Blue = 3,
    Cyan = 4,
    Green = 5,
    Yellow = 6,
    Orange = 7,
    Brown = 8,
    Grey = 9,
    White = 10,
}

/// Translates `TERRAIN_*` constants.
#[wasm_bindgen]
#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Hash,
    FromPrimitive,
    Serialize_repr,
    Deserialize_repr,
    Sequence,
)]
#[repr(u8)]
pub enum Terrain {
    // There's no constant for plains, but the absense of a terrain value indicates a plain
    Plain = 0,
    // TERRAIN_MASK_WALL
    Wall = 1,
    // TERRAIN_MASK_SWAMP
    Swamp = 2,
    /* TERRAIN_MASK_LAVA, unimplemented in game
     * Lava = 4, */
}

impl Terrain {
    // the strings here do not match the terrain mask constants, appearing nowhere
    // but look results. assuming it's a plain if it's anything invalid is probably
    // not the best approach but for now it's something
    pub fn from_look_constant_str(terrain_look_str: &str) -> Self {
        match terrain_look_str {
            "wall" => Terrain::Wall,
            "swamp" => Terrain::Swamp,
            "plain" => Terrain::Plain,
            _ => Terrain::Plain,
        }
    }

    pub fn from_look_constant_jsvalue(terrain_look_jsvalue: JsValue) -> Self {
        let terrain_look_string: String = JsString::from(terrain_look_jsvalue).into();
        Self::from_look_constant_str(&terrain_look_string)
    }
}

/// Translates body part type and `BODYPARTS_ALL` constants
#[wasm_bindgen]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Sequence)]
pub enum Part {
    Move = "move",
    Work = "work",
    Carry = "carry",
    Attack = "attack",
    RangedAttack = "ranged_attack",
    Tough = "tough",
    Heal = "heal",
    Claim = "claim",
}

named_enum_serialize_deserialize!(Part);

impl Part {
    /// Translates the `BODYPART_COST` constant.
    #[inline]
    pub const fn cost(self) -> u32 {
        match self {
            Part::Move => 50,
            Part::Work => 100,
            Part::Carry => 50,
            Part::Attack => 80,
            Part::RangedAttack => 150,
            Part::Tough => 10,
            Part::Heal => 250,
            Part::Claim => 600,
            // I guess bindgen is adding a `#[non_exhaustive]` onto the enum and forcing us to do
            // this:
            _ => 0,
        }
    }
}

/// Translates the `DENSITY_*` constants.
#[wasm_bindgen]
#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    FromPrimitive,
    Hash,
    Serialize_repr,
    Deserialize_repr,
    Sequence,
)]
#[repr(u8)]
pub enum Density {
    Low = 1,
    Moderate = 2,
    High = 3,
    Ultra = 4,
}

impl Density {
    /// Translates the `MINERAL_DENSITY` constant, the amount of mineral
    /// generated for each density level
    #[inline]
    pub const fn amount(self) -> u32 {
        match self {
            Density::Low => 15_000,
            Density::Moderate => 35_000,
            Density::High => 70_000,
            Density::Ultra => 100_000,
        }
    }

    /// Translates the `MINERAL_DENSITY_PROBABILITY` constant.
    ///
    /// These are values intended for subsequent percentage checks
    /// in the order `Low` -> `Medium` -> `High` -> `Ultra`. Use the
    /// [`enum_iterator::all`] iterator to iterate in this order.
    ///
    /// If low or ultra on previous regeneration, or random number rolled at
    /// probability [`MINERAL_DENSITY_CHANGE`], the mineral will determine a
    /// random new value ([source]):
    ///
    ///  - Low: 10% chance
    ///  - Moderate: 40% chance
    ///  - High: 40% chance
    ///  - Ultra: 10% chance
    ///
    /// [source]: https://github.com/screeps/engine/blob/c0cfac8f746f26c660501686f16a1fcdb0396d8d/src/processor/intents/minerals/tick.js#L19
    /// [`MINERAL_DENSITY_CHANGE`]: crate::constants::MINERAL_DENSITY_CHANGE
    #[inline]
    pub const fn probability(self) -> f32 {
        match self {
            Density::Low => 0.1,
            Density::Moderate => 0.5,
            Density::High => 0.9,
            Density::Ultra => 1.0,
        }
    }
}

/// Translates `ORDER_*` constants.
#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Sequence)]
pub enum OrderType {
    Sell = "sell",
    Buy = "buy",
}
