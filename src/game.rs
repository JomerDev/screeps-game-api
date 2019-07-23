use crate::{
    macros::*,
    objects::{HasId, RoomObject, SizedRoomObject},
    traits::TryInto,
    ConversionError,
};

pub mod cpu;
pub mod gcl;
pub mod map;
pub mod market;
pub mod shards;

game_map_access! {
    /// See [http://docs.screeps.com/api/#Game.constructionSites]
    ///
    /// [http://docs.screeps.com/api/#Game.constructionSites]: http://docs.screeps.com/api/#Game.constructionSites
    (construction_sites, objects::ConstructionSite, Game.constructionSites),
    /// See [http://docs.screeps.com/api/#Game.creeps]
    ///
    /// [http://docs.screeps.com/api/#Game.creeps]: http://docs.screeps.com/api/#Game.creeps
    (creeps, objects::Creep, Game.creeps),
    /// See [http://docs.screeps.com/api/#Game.flags]
    ///
    /// [http://docs.screeps.com/api/#Game.flags]: http://docs.screeps.com/api/#Game.flags
    (flags, objects::Flag, Game.flags),
    // TODO: See [http://docs.screeps.com/api/#Game.resources]
    ///
    /// [http://docs.screeps.com/api/#Game.resources]: http://docs.screeps.com/api/#Game.resources
    /// See [http://docs.screeps.com/api/#Game.rooms]
    ///
    /// [http://docs.screeps.com/api/#Game.rooms]: http://docs.screeps.com/api/#Game.rooms
    (rooms, objects::Room, Game.rooms),
    /// See [http://docs.screeps.com/api/#Game.spawns]
    ///
    /// [http://docs.screeps.com/api/#Game.spawns]: http://docs.screeps.com/api/#Game.spawns
    (spawns, objects::StructureSpawn, Game.spawns),
    /// See [http://docs.screeps.com/api/#Game.structures]
    ///
    /// [http://docs.screeps.com/api/#Game.structures]: http://docs.screeps.com/api/#Game.structures
    (structures, objects::Structure, Game.structures)
}

/// See [http://docs.screeps.com/api/#Game.time]
///
/// [http://docs.screeps.com/api/#Game.time]: http://docs.screeps.com/api/#Game.time
pub fn time() -> u32 {
    js_unwrap!(Game.time)
}

/// See [http://docs.screeps.com/api/#Game.getObjectById]
///
/// This gets an object expecting a specific type and will return a
/// `ConversionError` if the type does not match.
///
/// If all you want to assume is that something has an ID, use
/// [`get_object_erased`].
///
/// [http://docs.screeps.com/api/#Game.getObjectById]: http://docs.screeps.com/api/#Game.getObjectById
pub fn get_object_typed<T>(id: &str) -> Result<Option<T>, ConversionError>
where
    T: HasId + SizedRoomObject,
{
    js!(return Game.getObjectById(@{id});).try_into()
}

/// See [http://docs.screeps.com/api/#Game.getObjectById]
///
/// This gets the object in 'erased' form - all that is known about it is that
/// it's a RoomObject.
///
/// If a more specific type is expected, [`get_object_typed`] can be used.
///
/// [http://docs.screeps.com/api/#Game.getObjectById]: http://docs.screeps.com/api/#Game.getObjectById
pub fn get_object_erased(id: &str) -> Option<RoomObject> {
    js_unwrap_ref!(Game.getObjectById(@{id}))
}

pub fn notify(message: &str, group_interval: Option<u32>) {
    js! { @(no_return)
        Game.notify(@{message}, @{group_interval.unwrap_or(0)});
    }
}
