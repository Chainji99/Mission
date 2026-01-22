use anyhow::Result;

impl MisssionOperationPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
    async fn set_status(
          &self,
        mission_id: i32,
        chief_id: i32,
        status: MissionStatuses,
    ) -> Result<i32> {
        let db_pool = Arc::clone(&self.db_pool);
        let status_str = status.to_string();

        let id = tokio::task::spawn_blocking(move || -> Result<i32> {
            ...
        })
        .await??;

        Ok(id)
    }
     let id = tokio::task::spawn_blocking(move || -> Result<i32> {
            let mut conn = db_pool
                .get()
                .context("Failed to get database connection from pool")?;

            diesel::update(missions::table)
                .filter(missions::id.eq(mission_id))
                .filter(missions::chief_id.eq(chief_id))
                .filter(missions::deleted_at.is_null())
                .set((missions::status.eq(status_str),))
                .returning(missions::id)
                .get_result::<i32>(&mut conn)
                .context("Failed to execute mission update query")
            })

    
}
#[async_trait]
impl MissionOperationRepository for MisssionOperationPostgres {
    async fn in_progress(&self, mission_id: i32, chief_id: i32) -> Result<i32> {

        let result = self.set_status(mission_id, chief_id, MissionStatuses::InProgress).await?;
        Ok(result)

    }
    async fn to_completed(&self, mission_id: i32, chief_id: i32) -> Result<i32> {

         let result = self.set_status(mission_id, chief_id, MissionStatuses::Completed).await?;
        Ok(result)
    }
    async fn to_failed(&self, mission_id: i32, chief_id: i32) -> Result<i32> {

        async fn to_failed(&self, mission_id: i32, chief_id: i32) -> Result<i32> {
        let result = self.set_status(mission_id, chief_id, MissionStatuses::Failed).await?;
        Ok(result)
     }
    }
}
