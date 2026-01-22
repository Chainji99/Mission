use anyhow::Result;
use diesel::{
    PgConnection,
    r2d2::{ConnectionManager, PooledConnection},
};

#[async_trait]
#[automock]
pub trait CrewPaticipationRepository {
    async fn join(&self, crew_memberships: CrewMemberShips) -> Result<()>;
    async fn leave(&self, crew_memberships: CrewMemberShips) -> Result<()>;

     //testing method (example)
    fn for_insert_transaction_test(
        &self,
        conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
        crew_memberships: CrewMemberShips,
    ) -> Result<()>;
    fn for_delete_transaction_test(
        &self,
        conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
        crew_memberships: CrewMemberShips,
    ) -> Result<()>;

}
