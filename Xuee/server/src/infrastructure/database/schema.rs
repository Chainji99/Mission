// @generated automatically by Diesel CLI.

diesel::table! {
    brawlers (id) {
        id -> Int4,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        #[max_length = 255]
        display_name -> Varchar,
        #[max_length = 255]
        avatar_url -> Nullable<Varchar>,
        #[max_length = 255]
        avatar_public_id -> Nullable<Varchar>,
    }
}

diesel::table! {
    cards (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 50]
        language -> Varchar,
        #[max_length = 20]
        rarity -> Varchar,
        attack -> Int4,
        defense -> Int4,
        #[max_length = 255]
        image_url -> Nullable<Varchar>,
    }
}

diesel::table! {
    user_cards (id) {
        id -> Int4,
        user_id -> Int4,
        card_id -> Int4,
        level -> Int4,
        experience -> Int4,
        obtained_at -> Timestamp,
    }
}

diesel::table! {
    battles (id) {
        id -> Int4,
        attacker_id -> Int4,
        defender_id -> Int4,
        winner_id -> Nullable<Int4>,
        log -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    crew_memberships (mission_id, brawler_id) {
        mission_id -> Int4,
        brawler_id -> Int4,
        joined_at -> Timestamp,
    }
}

diesel::table! {
    missions (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 255]
        status -> Varchar,
        chief_id -> Int4,
        mission_date -> Nullable<Timestamp>,
        #[max_length = 255]
        time -> Nullable<Varchar>,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        #[max_length = 255]
        phone -> Nullable<Varchar>,
        location -> Nullable<Text>,
        rewards -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}





diesel::joinable!(crew_memberships -> brawlers (brawler_id));
diesel::joinable!(crew_memberships -> missions (mission_id));
diesel::joinable!(missions -> brawlers (chief_id));
diesel::joinable!(user_cards -> brawlers (user_id));
diesel::joinable!(user_cards -> cards (card_id));
diesel::joinable!(battles -> brawlers (attacker_id));


diesel::allow_tables_to_appear_in_same_query!(
    brawlers,
    cards,
    user_cards,
    battles,
    crew_memberships,
    missions,

);
