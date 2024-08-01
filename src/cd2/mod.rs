pub(crate) mod hazard_6;

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct CustomDifficulty {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caps: Option<Caps>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resupply: Option<Resupply>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub difficulty_setting: Option<DifficultySetting>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pools: Option<Pools>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enemies: Option<BTreeMap<String, EnemyDescriptor>>,
}

/// A [`Value`] can be either `Simple` (array based on player count) or `Mutator`, which is the new
/// CD2 feature that allows dynamic adjustments based on various conditions.
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value<T = f64> {
    Simple(Vec<T>),
    Mutator(Mutator),
}

/// A [`Mutator`] allows dynamically adjusting Custom Difficulty field values based on various
/// conditions.
#[derive(Debug, Serialize, Deserialize)]
pub struct Mutator;

#[derive(Debug, Serialize, Deserialize)]
pub struct WeightedRandInterval {
    pub weight: f64,
    pub min: f64,
    pub max: f64,
}

/// There are soft caps on enemy/swarmer/critter counts. Above these caps, enemies will despawn when
/// the game spawns new enemies.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Caps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_active_swarmers: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_active_enemies: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_active_critters: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Resupply {
    pub cost: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BaseHazard {
    #[serde(rename = "Hazard 1")]
    Hazard1,
    #[serde(rename = "Hazard 2")]
    Hazard2,
    #[serde(rename = "Hazard 3")]
    Hazard3,
    #[serde(rename = "Hazard 4")]
    Hazard4,
    #[serde(rename = "Hazard 5")]
    Hazard5,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct DifficultySetting {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_large_enemy_damage_resistance: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_large_enemy_damage_resistance_b: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_large_enemy_damage_resistance_c: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_large_enemy_damage_resistance_d: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enemy_damage_resistance: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_enemy_damage_resistance: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enemy_count_modifier: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enemy_damage_modifier: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encounter_difficulty: Option<Vec<WeightedRandInterval>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stationary_difficulty: Option<Vec<WeightedRandInterval>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enemy_wave_interval: Option<Vec<WeightedRandInterval>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enemy_normal_wave_interval: Option<Vec<WeightedRandInterval>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enemy_normal_wave_difficulty: Option<Vec<WeightedRandInterval>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enemy_diversity: Option<Vec<WeightedRandInterval>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stationary_enemy_diversity: Option<Vec<WeightedRandInterval>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub veteran_normal: Option<Vec<WeightedRandInterval>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub veteran_large: Option<Vec<WeightedRandInterval>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environmental_damage_modifier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_extraction_scalar: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friendly_fire_modifier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wave_start_delay_scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_modifier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_cooldown_modifier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projectile_speed_modifier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_regeneration_max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revive_health_ratio: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uses_veteran_large: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_hazard: Option<BaseHazard>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Range {
    pub min: f64,
    pub max: f64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Pools {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_pool_size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disruptive_enemy_pool_count: Option<Range>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stationary_enemy_count: Option<Range>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_enemies: Option<EnemyPool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_enemies: Option<EnemyPool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disruptive_enemies: Option<EnemyPool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enemy_pool: Option<EnemyPool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stationary_pool: Option<EnemyPool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct EnemyPool {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct EnemyDescriptor {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
}
