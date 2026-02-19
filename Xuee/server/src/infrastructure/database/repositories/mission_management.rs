use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use diesel::{insert_into, ExpressionMethods};
use diesel_async::RunQueryDsl;

use crate::domain::{
    entities::missions::{AddMissionEntity, EditMissionEntity},
    repositories::mission_management::MissionManagementRepository,
    value_objects::mission_statuses::MissionStatuses,
};
use crate::infrastructure::database::{
    postgresql_connection::PgPoolSquad,
    schema::missions,
};

pub struct MisssionManagementPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl MisssionManagementPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl MissionManagementRepository for MisssionManagementPostgres {
    async fn add(&self, add_mission_entity: AddMissionEntity) -> Result<i32> {

        let mut connection = self.db_pool.get().await?;

        let result = insert_into(missions::table)
            .values(add_mission_entity)
            .returning(missions::id)
            .get_result::<i32>(&mut connection)
            .await?;

        Ok(result)
    }
    async fn edit(&self, mission_id: i32, edit_mission_entity: EditMissionEntity) -> Result<i32> {

         let mut connection = self.db_pool.get().await?;

        let result = diesel::update(missions::table)
            .filter(missions::id.eq(mission_id))
            .filter(missions::deleted_at.is_null())
            .filter(missions::status.eq(MissionStatuses::Open.to_string()))
            .set(edit_mission_entity)
            .returning(missions::id)
            .get_result::<i32>(&mut connection)
            .await?;

        Ok(result)

    }
    async fn remove(&self, mission_id: i32, chief_id: i32) -> Result<()> {

        let mut connection = self.db_pool.get().await?;

        diesel::update(missions::table)
            .filter(missions::id.eq(mission_id))
            .filter(missions::deleted_at.is_null())
            .filter(missions::status.eq(MissionStatuses::Open.to_string()))
            .set((
                missions::deleted_at.eq(diesel::dsl::now),
                missions::chief_id.eq(chief_id),
            ))
            .execute(&mut connection)
            .await?;

        Ok(())

    }
}
