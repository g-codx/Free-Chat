use crate::db::entity::room::Room;
use crate::db::error;
use crate::db::util::{split_as_i64_set, split_as_i64_vec};
use crate::Rooms;
use anyhow::Error;
use sqlx::SqlitePool;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn create_room(name: String, user_id: i64, pool: &SqlitePool) -> anyhow::Result<i64> {
    Room::insert(name, user_id.to_string(), pool).await
}

pub async fn get_all_rooms_map(pool: Arc<SqlitePool>) -> anyhow::Result<Rooms> {
    let rooms = Room::find_all(&pool).await?;
    let mut rooms_map = HashMap::with_capacity(rooms.len());

    for room in rooms {
        let user_ids = if room.user_ids.is_empty() {
            HashSet::with_capacity(0)
        } else {
            split_as_i64_set(room.user_ids, ',')?
        };

        rooms_map.insert(room.id, user_ids);
    }

    Ok(Rooms::new(RwLock::new(rooms_map)))
}

pub async fn add_user_to_room(room_id: i64, user_id: i64, pool: &SqlitePool) -> anyhow::Result<()> {
    match Room::find_by_id(room_id, pool).await? {
        None => Err(Error::from(error::Error::RoomNotFound { room_id })),
        Some(room) => {
            let mut user_ids = if room.user_ids.is_empty() {
                Vec::with_capacity(1)
            } else {
                split_as_i64_vec(room.user_ids, ',')?
            };

            if user_ids.contains(&user_id) {
                return Err(Error::from(error::Error::AlreadyInRoom {
                    user_id,
                    room_id,
                }));
            }

            user_ids.push(user_id);

            let user_ids: Vec<String> = user_ids.iter().map(|id| id.to_string()).collect();
            let user_ids_str = user_ids.join(",");

            Room::update_user_ids_by_id(room_id, user_ids_str, pool).await?;

            Ok(())
        }
    }
}

pub async fn remove_user_from_room(
    room_id: i64,
    user_id: i64,
    pool: &SqlitePool,
) -> anyhow::Result<()> {
    match Room::find_by_id(room_id, pool).await? {
        None => return Err(Error::from(error::Error::RoomNotFound { room_id })),
        Some(room) => {
            let mut user_ids = if room.user_ids.is_empty() {
                return Ok(());
            } else {
                split_as_i64_vec(room.user_ids, ',')?
            };

            user_ids.retain(|id| *id != user_id);

            let user_ids: Vec<String> = user_ids.iter().map(|id| id.to_string()).collect();
            let user_ids_str = user_ids.join(",");

            Room::update_user_ids_by_id(room_id, user_ids_str, pool).await?;

            Ok(())
        }
    }
}
