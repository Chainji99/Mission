pub struct MissionEntity { ... }

impl MissionEntity {
    pub fn to_model(&self, crew_count: i64) -> MissionModel {
        MissionModel {
            id: self.id,
            name: self.name.clone(),
            description: self.description.clone(),
            status: self.status.clone(),
            chief_id: self.chief_id,
            crew_count,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
