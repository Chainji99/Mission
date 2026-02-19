use std::sync::Arc;
use anyhow::{Result, anyhow};
use rand::Rng;

use crate::domain::entities::cards::{CardEntity, UserCardDetail, NewUserCardEntity};
use crate::domain::entities::battles::{BattleEntity, NewBattleEntity};
use crate::domain::repositories::cards::CardRepository;

pub struct CardUseCase<T>
where
    T: CardRepository + Send + Sync,
{
    card_repository: Arc<T>,
}

impl<T> CardUseCase<T>
where
    T: CardRepository + Send + Sync,
{
    pub fn new(card_repository: Arc<T>) -> Self {
        Self { card_repository }
    }

    pub async fn get_all_cards(&self) -> Result<Vec<CardEntity>> {
        self.card_repository.get_all_cards().await
    }

    pub async fn get_user_inventory(&self, user_id: i32) -> Result<Vec<UserCardDetail>> {
        self.card_repository.get_user_cards(user_id).await
    }

    pub async fn draw_gacha(&self, user_id: i32) -> Result<UserCardDetail> {
        let all_cards = self.card_repository.get_all_cards().await?;
        if all_cards.is_empty() {
            return Err(anyhow!("No cards available in the system"));
        }

        // Weighted Random Logic
        // Legendary: 2%, Epic: 8%, Rare: 20%, Common: 70%
        let (target_rarity, _random_index_in_filtered) = {
            let mut rng = rand::rng();
            let chance: f64 = rng.random_range(0.0..100.0);
            
            let target = if chance < 2.0 {
                "Legendary"
            } else if chance < 10.0 {
                "Epic"
            } else if chance < 30.0 {
                "Rare"
            } else {
                "Common"
            };

            // We need to know how many cards there are for this rarity to pick one
            // But we don't have the cards yet. Wait, we have all_cards.
            (target, chance) // Just return target and chance for now
        };

        // Filter cards by selected rarity
        let mut filtered_cards: Vec<&CardEntity> = all_cards.iter()
            .filter(|c| c.rarity.eq_ignore_ascii_case(target_rarity))
            .collect();

        // Fallback logic if no cards of target rarity exist
        if filtered_cards.is_empty() {
            // Try Common as fallback, if still empty, use all cards
            filtered_cards = all_cards.iter()
                .filter(|c| c.rarity.eq_ignore_ascii_case("Common"))
                .collect();
            
            if filtered_cards.is_empty() {
                filtered_cards = all_cards.iter().collect();
            }
        }

        let selected_card = {
            let mut rng = rand::rng();
            let random_index = rng.random_range(0..filtered_cards.len());
            filtered_cards[random_index]
        };

        let new_user_card = NewUserCardEntity {
            user_id,
            card_id: selected_card.id,
        };

        let created_card = self.card_repository.add_user_card(new_user_card).await?;
        
        Ok(UserCardDetail {
            user_card: created_card,
            card: selected_card.clone(),
        })
    }

    pub async fn upgrade_card(&self, user_card_id: i32, amount: i32) -> Result<String> {
        let user_card_detail = self.card_repository.get_user_card_by_id(user_card_id).await?;
        let mut current_exp = user_card_detail.user_card.experience;
        let mut current_level = user_card_detail.user_card.level;

        current_exp += amount;

        // Simple Level Up Logic: 100 EXP per level
        while current_exp >= 100 {
            current_exp -= 100;
            current_level += 1;
        }

        self.card_repository.update_card_exp(user_card_id, current_exp, current_level).await?;

        Ok(format!("Card upgraded to Level {} (EXP: {})", current_level, current_exp))
    }

    pub async fn battle(&self, attacker_id: i32, defender_id: i32) -> Result<BattleEntity> {
        // Simplified Battle Logic: Compare total stats of all cards
        let attacker_deck = self.card_repository.get_user_cards(attacker_id).await?;
        let defender_deck = self.card_repository.get_user_cards(defender_id).await?;

        let attacker_power: i32 = attacker_deck.iter().map(|c| c.card.attack * c.user_card.level).sum();
        let defender_power: i32 = defender_deck.iter().map(|c| c.card.defense * c.user_card.level).sum();

        let mut log = String::new();
        log.push_str(&format!("Attacker Power: {}\n", attacker_power));
        log.push_str(&format!("Defender Power: {}\n", defender_power));

        let winner_id = if attacker_power > defender_power {
            log.push_str("Result: Attacker Wins!");
            Some(attacker_id)
        } else if defender_power > attacker_power {
            log.push_str("Result: Defender Wins!");
            Some(defender_id)
        } else {
            log.push_str("Result: Draw!");
            None
        };

        let new_battle = NewBattleEntity {
            attacker_id,
            defender_id,
            winner_id,
            log: Some(log),
        };

        self.card_repository.create_battle(new_battle).await
    }
}
