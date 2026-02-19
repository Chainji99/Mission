use diesel::prelude::*;
use crate::domain::entities::brawlers::RegisterBrawlerEntity;
use crate::infrastructure::database::postgresql_connection::PgPoolSquad;
use crate::infrastructure::database::schema::brawlers;
use anyhow::Result;
use argon2::{
    password_hash::SaltString,
    Argon2,
};
use rand_core::OsRng;

pub async fn seed_test_users(pool: &PgPoolSquad) -> Result<()> {
    let mut connection = pool.get()?;
    let argon2 = Argon2::default();
    
    let test_users = vec![
        ("test_user_1", "Test User 1"),
        ("test_user_2", "Test User 2"),
        ("test_user_3", "Test User 3"),
        ("test_user_4", "Test User 4"),
        ("test_user_5", "Test User 5"),
        ("test_user_6", "Test User 6"),
        ("test_user_7", "Test User 7"),
        ("test_user_8", "Test User 8"),
        ("test_user_9", "Test User 9"),
        ("test_user_10", "Test User 10"),
    ];

    for (username, display_name) in test_users {
        // Hash password: Test@123
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = argon2
            .hash_password(b"Test@123", &salt)?
            .to_string();

        let user = RegisterBrawlerEntity {
            username: username.to_string(),
            password: password_hash,
            display_name: display_name.to_string(),
        };

        // Insert if not exists
        let result: Result<(), _> = diesel::insert_into(brawlers::table)
            .values(&user)
            .on_conflict_do_nothing()
            .execute(&mut connection)
            .map(|_| ());

        if result.is_ok() {
            println!("✓ Created user: {}", username);
        } else {
            println!("- User already exists: {}", username);
        }
    }

    println!("\n✓ Seeding completed!");
    Ok(())
}
