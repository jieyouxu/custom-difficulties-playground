mod cd2;
mod logging;
mod vanilla_enemy_descriptors;

use std::collections::{BTreeMap, BTreeSet};

use serde::Serialize;

use cd2::{CustomDifficulty, EnemyDescriptor};
use vanilla_enemy_descriptors::VANILLA_ENEMY_DESCRIPTORS;

fn main() {
    logging::setup_logging();

    let enemy_descriptor_scale_map = BTreeMap::from_iter([
        ("ED_ShootingPlant", 0.2),
        ("ED_SpiderSpawner", 0.2),
        ("ED_InsectSwarm_Spawner", 0.35),
        ("ED_JellyBreeder", 0.3),
        ("ED_FacilityTurret_Sniper", 0.5),
        ("ED_FacilityTurret_Burst", 0.5),
        ("ED_FacilityTurret_Barrier", 0.5),
        ("ED_Prospector", 0.2),
        ("ED_PatrolBot_Caretaker", 0.5),
        ("ED_PatrolBot", 0.5),
        ("ED_Spider_Hoarder", 0.2),
        ("ED_PumpkinLootBug", 0.3),
        ("ED_Spider_ShieldTank", 0.3),
        ("ED_Spider_Spitter", 0.5),
        ("ED_Spider_Shooter", 0.5),
        ("ED_Spider_Grunt_Ice", 0.55),
        ("ED_Spider_Grunt", 0.55),
        ("ED_Spider_ExploderTank_King", 0.3),
        ("ED_Spider_Exploder", 0.55),
        ("ED_Spider_Buffer", 0.35),
        ("ED_Terminator", 0.3),
        ("ED_InfectedMule", 0.2),
        ("ED_HydraWeed", 0.4),
        ("ED_Bomber_Ice", 0.3),
        ("ED_Bomber", 0.3),
        ("ED_CaveLeech", 0.5),
        ("ED_Spider_Tank_Ice", 0.3),
        ("ED_Spider_Tank", 0.3),
        ("ED_Spider_RapidShooter", 0.35),
        ("ED_Spider_Grunt_Guard", 0.5),
        ("ED_Spider_Grunt_Attacker", 0.5),
        ("ED_Spider_ExploderTank", 0.25),
        ("ED_Mactera_TripleShooter", 0.35),
        ("ED_Mactera_Shooter_Normal", 0.5),
        ("ED_Mactera_Shooter_HeavyVeteran", 0.4),
        ("ED_Grabber", 0.5),
        ("ED_Shark", 0.45),
        ("ED_Spider_Boss_TwinB", 0.4),
        ("ED_Spider_Boss_TwinA", 0.4),
        ("ED_Spider_Tank_Boss", 0.3),
        ("ED_Spider_Boss_Heavy", 0.3),
        ("ED_Woodlouse_Youngling", 0.6),
        ("ED_Woodlouse", 0.4),
        ("ED_Spider_Charger", 0.35),
        ("ED_Spider_Exploder_Warning", 0.55),
        ("ED_Spider_ExploderTankGhost", 0.25),
        ("ED_Spider_Grunt_Mutated", 0.55),
        ("ED_Spider_Grunt_Rock", 0.55),
        ("ED_Spider_Amber_Shooter", 0.3),
        ("ED_Spider_Tank_Mutated", 0.35),
        ("ED_Spider_Tank_Amber", 0.3),
        ("ED_Spider_Tank_Rock", 0.35),
        ("ED_Spider_Tank_HeavySpawn", 0.35),
        ("ED_FlyingSmartRock", 0.4),
        ("ED_Spider_Stalker", 0.3),
        ("ED_BarrageInfector", 0.1),
        ("ED_TentaclePlant", 0.3),
        ("ED_Crawler", 0.3),
        ("ED_Bomber_Explosive", 0.3),
        ("ED_Bomber_Rockpox_Plague", 0.3),
        ("ED_EggSpider", 0.4),
        ("ED_Flea", 0.3),
        ("ED_GreatEggHunt_SpringBunny", 0.3),
        ("ED_Halloween_Skull", 0.3),
        ("ED_InfestationLarva", 0.3),
        ("ED_JellyBreeder_RockpoxPlague", 0.2),
        ("ED_Jelly_Spawn", 0.3),
        ("ED_Jelly_Swarmer", 0.3),
        ("ED_Mactera_Shooter_Amber", 0.2),
        ("ED_Mactera_Shooter_heavyVeteran", 0.2),
        ("ED_Nisse", 0.2),
        ("ED_PlagueLarva", 0.3),
        ("ED_PlagueShark", 0.1),
        ("ED_Shredder", 0.3),
        ("ED_Spider_Drone", 0.3),
        ("ED_Spider_Exploder_Rockpox_Plague", 0.2),
        ("ED_Spider_GruntTutorial", 0.2),
        ("ED_Spider_Grunt_Rock_Warning", 0.2),
        ("ED_Spider_Grunt_RockpoxPlague", 0.2),
        ("ED_Spider_Lobber", 0.1),
        ("ED_Spider_ShooterQueen", 0.1),
        ("ED_Spider_Shooter_Ground", 0.1),
        ("ED_Spider_Shooter_Rockpox_Plague", 0.1),
        ("ED_Spider_Spawn", 0.2),
        ("ED_Spider_Stinger", 0.2),
        ("ED_Spider_Swarmer", 0.3),
        ("ED_Spider_Swarmer_Ice", 0.3),
        ("ED_Spider_Swarmer_Mutated", 0.3),
        ("ED_Spider_Swarmer_Pheromone_NOFX", 0.3),
        ("ED_Spider_Tank_Rock_Warning", 0.3),
        ("ED_Spider_Tank_RockpoxPlague", 0.3),
        ("ED_TunnelSwarmer", 0.3),
        ("ED_WalkingPlagueheart", 0.3),
    ]);

    let specified_enemies = BTreeSet::from_iter(enemy_descriptor_scale_map.keys().copied());
    let all_enemies = BTreeSet::from_iter(VANILLA_ENEMY_DESCRIPTORS.iter().copied());
    let missing_enemies = all_enemies.difference(&specified_enemies);
    assert_eq!(missing_enemies.count(), 0);

    let enemies = enemy_descriptor_scale_map
        .iter()
        .map(|(name, scale)| (name.to_string(), EnemyDescriptor { scale: Some(*scale) }))
        .collect::<BTreeMap<_, _>>();

    let cd = CustomDifficulty {
        name: "Hazard 6x2 (60 nitra) Mini Edition".to_string(),
        description: Some("Hazard 6x2 60 nitra but smol".to_string()),
        enemies: Some(enemies),
        ..CustomDifficulty::hazard_6()
    };

    let mut buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
    cd.serialize(&mut ser).unwrap();

    std::fs::write("hazard-6x2-mini-edition.json", String::from_utf8(buf).unwrap()).unwrap();
}
