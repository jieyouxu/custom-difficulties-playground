use super::{
    BaseHazard, Caps, CustomDifficulty, DifficultySetting, Pools, Range, Resupply, Value,
    WeightedRandInterval,
};

impl CustomDifficulty {
    pub fn hazard_6() -> CustomDifficulty {
        CustomDifficulty {
            name: "Hazard 6x2".to_string(),
            description: Some("Hazard 6x2".to_string()),
            caps: Some(Caps {
                max_active_swarmers: Some(Value::Simple(vec![90.0, 120.0, 180.0, 180.0])),
                max_active_enemies: Some(Value::Simple(vec![90.0, 120.0, 180.0, 180.0])),
                max_active_critters: None,
            }),
            resupply: Some(Resupply { cost: Some(Value::Simple(vec![60.0])) }),
            difficulty_setting: Some(DifficultySetting {
                base_hazard: Some(BaseHazard::Hazard5),
                extra_large_enemy_damage_resistance: Some(Value::Simple(vec![1.0, 1.3, 1.7, 2.1])),
                extra_large_enemy_damage_resistance_b: Some(Value::Simple(vec![
                    1.0, 1.3, 1.5, 1.7,
                ])),
                extra_large_enemy_damage_resistance_c: Some(Value::Simple(vec![
                    1.0, 1.3, 1.7, 2.1,
                ])),
                extra_large_enemy_damage_resistance_d: Some(Value::Simple(vec![
                    1.5, 1.55, 1.75, 1.9,
                ])),
                enemy_damage_resistance: Some(Value::Simple(vec![1.4, 1.5, 1.6, 1.75])),
                enemy_damage_modifier: Some(Value::Simple(vec![3.3, 3.65, 3.9, 4.1])),
                enemy_count_modifier: Some(Value::Simple(vec![1.9, 2.1, 2.7, 3.3])),
                small_enemy_damage_resistance: Some(Value::Simple(vec![1.2])),
                encounter_difficulty: Some(vec![
                    WeightedRandInterval { weight: 1.0, min: 550.0, max: 650.0 },
                    WeightedRandInterval { weight: 6.0, min: 450.0, max: 750.0 },
                    WeightedRandInterval { weight: 1.0, min: 550.0, max: 650.0 },
                ]),
                stationary_difficulty: Some(vec![WeightedRandInterval {
                    weight: 1.0,
                    min: 350.0,
                    max: 450.0,
                }]),
                enemy_wave_interval: Some(vec![WeightedRandInterval {
                    weight: 7.5,
                    min: 125.0,
                    max: 165.0,
                }]),
                enemy_normal_wave_interval: Some(vec![
                    WeightedRandInterval { weight: 4.0, min: 170.0, max: 210.0 },
                    WeightedRandInterval { weight: 2.0, min: 180.0, max: 190.0 },
                ]),
                enemy_normal_wave_difficulty: Some(vec![WeightedRandInterval {
                    weight: 1.0,
                    min: 550.0,
                    max: 650.0,
                }]),
                enemy_diversity: Some(vec![
                    WeightedRandInterval { weight: 2.0, min: 4.0, max: 4.0 },
                    WeightedRandInterval { weight: 8.0, min: 6.0, max: 6.0 },
                ]),
                stationary_enemy_diversity: Some(vec![WeightedRandInterval {
                    weight: 1.0,
                    min: 4.0,
                    max: 4.0,
                }]),
                veteran_normal: Some(vec![
                    WeightedRandInterval { weight: 2.0, min: 0.0, max: 0.0 },
                    WeightedRandInterval { weight: 2.0, min: 0.5, max: 0.8 },
                ]),
                veteran_large: Some(vec![WeightedRandInterval { weight: 1.0, min: 0.2, max: 0.2 }]),
                environmental_damage_modifier: Some(1.4),
                point_extraction_scalar: Some(1.4),
                friendly_fire_modifier: Some(0.9),
                speed_modifier: Some(1.25),
                attack_cooldown_modifier: Some(1.75),
                projectile_speed_modifier: Some(1.9),
                ..Default::default()
            }), 
            pools: Some(Pools {
                min_pool_size: Some(14.0),
                disruptive_enemy_pool_count: Some(Range { min: 4.0, max: 8.0 }),
                stationary_enemy_count: Some(Range { min: 6.0, max: 6.0 }),
                ..Default::default()
            }),
            ..Default::default()
        }
    }
}
