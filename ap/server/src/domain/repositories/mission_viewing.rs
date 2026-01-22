use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;

#[async_trait]
#[automock]
pub trait MissionViewingRepository {
    async fn crew_counting(&self, mission_id: i32) -> Result<u32>;
    async fn get_one(&self, mission_id: i32) -> Result<MissionEntity>;
    async fn get_all(&self, filter: &MissionFilter) -> Result<Vec<MissionEntity>>;
    async fn get_crew(&self, filter: &MissionFilter) -> Result<Vec<MissionEntity>>;

}

pub async fn get_crew(&self, mission_id: i32) -> Result<Vec<UploadAvatarModel>> {
    let result: Vec<UploadAvatarModel> = self.mission_viewing_repository.get_crew(mission_id).await?;
    Ok(result)
}


pub async fn get_crew(&self, filter: &MissionFilter) -> Result<Vec<MissionEntity>> {
    let sql: r#"
SELECT 
    b.display_name,
    COALESCE(b.avatar_url, '') AS avatar_url,
    COALESCE(s.success_count, 0) AS mission_success_count,
    COALESCE(j.joined_count, 0) AS mission_joined_count
FROM crew_memberships cm
INNER JOIN brawlers b ON b.id = cm.brawler_id
LEFT JOIN (
    SELECT cm2.brawler_id, COUNT(*) AS success_count
    FROM crew_memberships cm2
    INNER JOIN missions m2 ON m2.id = cm2.mission_id
    WHERE m2.status = '???'
    GROUP BY cm2.brawler_id
) s ON s.brawler_id = b.id
LEFT JOIN (
    SELECT cm3.brawler_id, COUNT(*) AS joined_count
    FROM crew_memberships cm3
    GROUP BY cm3.brawler_id
) j ON j.brawler_id = b.id
WHERE cm.mission_id = $1
    "#;
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = self.db_pool.get()?;
    let brawler_list = diesel::sql_query(sql)
        .bind::<diesel::sql_types::Integer, _>(mission_id)
        .load::<UploadAvatarModel>(&mut conn) ?;
    Ok(brawler_list)
}