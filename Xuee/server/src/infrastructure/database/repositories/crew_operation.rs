#[async_trait]
impl CrewPaticipationRepository for CrewPaticipationPostgres {
    async fn join(&self, crew_memberships: CrewMemberShips) -> Result<()> {

        let mut connection = Arc::clone(&self.db_pool).get()?;
        insert_into(crew_memberships::table) //diesel::dsl::insert_into,
            .values(crew_memberships)
            .execute(&mut connection)?;

        Ok(())

    }

    async fn leave(&self, crew_memberships: CrewMemberShips) -> Result<()> {

        let mut connection = Arc::clone(&self.db_pool).get()?;

        delete(crew_memberships::table)
            .filter(crew_memberships::brawler_id.eq(crew_memberships.brawler_id))
            .filter(crew_memberships::mission_id.eq(crew_memberships.mission_id))
            .execute(&mut connection)?;

        Ok(())

    }

}