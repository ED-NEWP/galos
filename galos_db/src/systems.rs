use async_std::task;
use chrono::{DateTime, Utc};
use sqlx::types::Uuid;
use geozero::wkb;
use pathfinding::prelude::*;
use ordered_float::OrderedFloat;
use elite_journal::{prelude::*, system::System as JournalSystem, entry::incremental::travel::FsdJump};
use crate::{Error, Database};
use crate::factions::{Faction, Conflict};

#[derive(Debug, Clone)]
pub struct System {
    pub address: i64,
    // TODO: We need to support multiple names
    pub name: String,
    pub position: Coordinate,
    pub population: u64,
    pub security: Option<Security>,
    pub government: Option<Government>,
    pub allegiance: Option<Allegiance>,
    pub primary_economy: Option<Economy>,
    pub secondary_economy: Option<Economy>,

    // TODO: Find an elegent way to represent this.
    // & = foreign key = belongs_to
    // pub controlling_faction: &Faction,
    // pub factions: Vec<Faction>
}

impl System {
    pub async fn create(db: &Database,
        address: u64,
        name: &str,
        position: Coordinate,
        population: Option<u64>,
        security: Option<Security>,
        government: Option<Government>,
        allegiance: Option<Allegiance>,
        primary_economy: Option<Economy>,
        secondary_economy: Option<Economy>,
        updated_at: DateTime<Utc>)
        -> Result<(), Error>
    {
        sqlx::query!(
            r#"
            INSERT INTO systems
                (address,
                 name,
                 position,
                 population,
                 security,
                 government,
                 allegiance,
                 primary_economy,
                 secondary_economy,
                 updated_at)
            VALUES ($1, UPPER($2), $3::geometry, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (address)
            DO UPDATE SET
                population = $4,
                security = $5,
                government = $6,
                allegiance = $7,
                primary_economy = $8,
                secondary_economy = $9
            WHERE systems.updated_at < $10
            "#,
            address as i64,
            name,
            wkb::Encode(position) as _,
            population.map(|n| n as i64),
            security as _,
            government as _,
            allegiance as _,
            primary_economy as _,
            secondary_economy as _,
            updated_at.naive_utc())
            .execute(&db.pool)
            .await?;

        Ok(())
    }

    pub async fn from_journal(db: &Database, system: &JournalSystem, timestamp: DateTime<Utc>)
        -> Result<(), Error>
    {
        let position = Coordinate {
            x: system.pos.x,
            y: system.pos.y,
            z: system.pos.z,
        };
        // TODO: Conflicts on pos need to do something else.
        sqlx::query!(
            r#"
            INSERT INTO systems
                (address,
                 name,
                 position,
                 population,
                 security,
                 government,
                 allegiance,
                 primary_economy,
                 secondary_economy,
                 updated_at)
            VALUES ($1, UPPER($2), $3::geometry, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (address)
            DO UPDATE SET
                population = $4,
                security = $5,
                government = $6,
                allegiance = $7,
                primary_economy = $8,
                secondary_economy = $9
            "#, system.address as i64,
                system.name,
                wkb::Encode(position) as _,
                system.population.map(|n| n as i64),
                system.security as _,
                system.government as _,
                system.allegiance as _,
                system.economy as _,
                system.second_economy as _,
                timestamp.naive_utc())
            .execute(&db.pool)
            .await?;

        for faction in &system.factions {
            let id = Faction::create(db, &faction.name).await?.id;

            sqlx::query!(
                "
                INSERT INTO system_factions
                    (system_address,
                     faction_id,
                     updated_at,
                     state,
                     influence,
                     happiness,
                     government,
                     allegiance)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                ON CONFLICT (system_address, faction_id)
                DO UPDATE SET
                    updated_at = $3,
                    state = $4,
                    influence = $5,
                    happiness = $6,
                    government = $7,
                    allegiance = $8
                WHERE system_factions.updated_at < $3
                ",
                    system.address as i64,
                    id,
                    timestamp.naive_utc(),
                    faction.state as _,
                    faction.influence,
                    faction.happiness as _,
                    faction.government as _,
                    faction.allegiance as _)
                .execute(&db.pool)
                .await?;
        }

        for conflict in &system.conflicts {
            Conflict::from_journal(db, &conflict, system.address, timestamp).await?;
        }

        Ok(())
    }

    pub async fn fetch(db: &Database, address: i64) -> Result<Self, Error> {
        let row = sqlx::query!(
            r#"
            SELECT
                address,
                name,
                position AS "position!: wkb::Decode<Coordinate>",
                population,
                security as "security: Security",
                government as "government: Government",
                allegiance as "allegiance: Allegiance",
                primary_economy as "primary_economy: Economy",
                secondary_economy as "secondary_economy: Economy"
            FROM systems
            WHERE address = $1
            "#, address)
            .fetch_one(&db.pool)
            .await?;

        Ok(System {
            address: row.address,
            name: row.name,
            position: row.position.geometry.expect("not null or invalid"),
            population: row.population.map(|n| n as u64).unwrap_or(0),
            security: row.security,
            government: row.government,
            allegiance: row.allegiance,
            primary_economy: row.primary_economy,
            secondary_economy: row.secondary_economy,
        })
    }

    // NOTE: Assumes systems are unique by name, which is currently untrue.
    pub async fn fetch_by_name(db: &Database, name: &str) -> Result<Self, Error> {
        let row = sqlx::query!(
            r#"
            SELECT
                address,
                name,
                position AS "position!: wkb::Decode<Coordinate>",
                population,
                security as "security: Security",
                government as "government: Government",
                allegiance as "allegiance: Allegiance",
                primary_economy as "primary_economy: Economy",
                secondary_economy as "secondary_economy: Economy"
            FROM systems
            WHERE name = $1
            "#, name.to_uppercase())
            .fetch_one(&db.pool)
            .await?;

        Ok(System {
            address: row.address,
            name: row.name,
            position: row.position.geometry.expect("not null or invalid"),
            population: row.population.map(|n| n as u64).unwrap_or(0),
            security: row.security,
            government: row.government,
            allegiance: row.allegiance,
            primary_economy: row.primary_economy,
            secondary_economy: row.secondary_economy,
        })
    }

    // TODO: add migration for:
    // CREATE INDEX systems_position_idx ON systems USING GIST (position gist_geometry_ops_nd);
    pub fn neighbors(&self, db: &Database, range: f64) -> Vec<System> {
        let rows = task::block_on(async {
            sqlx::query!(
                r#"
                SELECT
                    address,
                    name,
                    position AS "position!: wkb::Decode<Coordinate>",
                    population,
                    security as "security: Security",
                    government as "government: Government",
                    allegiance as "allegiance: Allegiance",
                    primary_economy as "primary_economy: Economy",
                    secondary_economy as "secondary_economy: Economy"
                FROM systems
                WHERE ST_3DDWithin(position, $1, $2);
                "#, wkb::Encode(self.position) as _, range)
                .fetch_all(&db.pool)
                .await.unwrap()
        });

        println!("neighbors of {} ({})", self.name, rows.len());

        rows.into_iter().map(|row| {
            System {
                address: row.address,
                name: row.name,
                position: row.position.geometry.expect("not null or invalid"),
                population: row.population.map(|n| n as u64).unwrap_or(0),
                security: row.security,
                government: row.government,
                allegiance: row.allegiance,
                primary_economy: row.primary_economy,
                secondary_economy: row.secondary_economy,
            }
        }).collect()
    }

    pub fn distance(&self, other: &System) -> f64 {
        let p1 = self.position;
        let p2 = other.position;

        ((p2.x - p1.x).powi(2) +
            (p2.y - p1.y).powi(2) +
            (p2.z - p1.z).powi(2)).sqrt()
    }

    pub fn route_to(&self, db: &Database, end: &System, range: f64)
        -> Result<Option<(Vec<Self>, OrderedFloat<f64>)>, Error>
    {
        let successors = |s: &System| {
            s.neighbors(db, range).into_iter().map(|s| (s, OrderedFloat(1.)))
        };

        // Making the heuristic much larger than the successor's jump cost makes things run
        // faster, but is not optimal...
        let heuristic = |s: &System| {
            OrderedFloat((s.distance(end) / range).ceil())
        };

        let success = |s: &System| s == end;

        Ok(astar(self, successors, heuristic, success))
    }
}

// https://www.reddit.com/r/EliteDangerous/comments/30nx4u/the_hyperspace_fuel_equation_documented/
fn fuel_cost(distance: f64, mass: f64, optimal_mass: f64) -> f64 {
    // A: 12
    // B: 10
    // C: 8
    // D: 10
    // E: 11
    let l = 12.;
    // 2: 2.00
    // 3: 2.15
    // 4: 2.30
    // 5: 2.45
    // 6: 2.60
    // 7: 2.75
    // 8: 2.90
    let p = 2.45;

    l * 0.001 * (distance * mass / optimal_mass).powf(p)
}

impl Eq for System {}
impl PartialEq for System {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address
    }
}

use std::hash::{Hash, Hasher};
impl Hash for System {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.address.hash(state);
    }
}

/// These are just the game's names, they don't really make sense since tritium is an isotope
/// of hydrogen.
#[derive(sqlx::Type, Debug, Copy, Clone, PartialEq)]
// #[sqlx(type_name = "fuel")]
pub enum Fuel {
    /// When we enter for fleet carriers, not the event
    Tritium,
    /// Ship fuel from the [`elite_journal::entry::incremental::travel::FsdJump`]
    Hydrogen,
}

pub struct Cost {
    ty: Fuel,
    distance: f32,
    used: f32,
    level: f32,
}

pub struct Jump {
    id: Uuid,
    current_system_address: i64,
    /// `Some` for an [`elite_journal::entry::incremental::travel::FsdJump`]
    /// `None` for a `elite_journal::incremental::travel::CarrierJump`
    /// Also `None` when it's the start of a planned route
    cost: Option<Cost>,
    future: bool,
    timestamp: Option<DateTime<Utc>>,
    next_jump_id: Option<Uuid>,
}

impl Jump {
    pub async fn start(db: &Database,
        system: &System,
        at: Option<DateTime<Utc>>)
        -> Result<Jump, Error>
    {
        let row = sqlx::query!(
            r#"
            INSERT INTO jumps (
                id,
                current_system_address,
                future,
                timestamp)
            VALUES (uuid_generate_v4(), $1, true, $2)
            RETURNING *
            "#,
            system.address as i64,
            at.map(|t| t.naive_utc()))
            .fetch_one(&db.pool)
            .await?;

        Ok(Jump {
            id: row.id,
            current_system_address: row.current_system_address,
            cost: None,
            // TODO: Actually check if it's in the future if given...
            future: row.future,
            timestamp: row.timestamp.map(|t| DateTime::<Utc>::from_utc(t, Utc)),
            next_jump_id: None,
        })
    }

    pub async fn jump(&self, db: &Database,
        next: &Jump,
        cost: Option<Cost>,
        at: Option<DateTime<Utc>>)
        -> Result<Jump, Error>
    {
        unimplemented!()
    }

    pub async fn from_journal(db: &Database, jump: &FsdJump, timestamp: DateTime<Utc>)
        -> Result<Jump, Error>
    {
        System::from_journal(db, &jump.system, timestamp).await?;
        let row = sqlx::query!(
            r#"
            INSERT INTO jumps
                (id,
                 current_system_address,
                 distance,
                 fuel_used,
                 fuel_level,
                 fuel_type,
                 future,
                 timestamp)
            VALUES (uuid_generate_v4(), $1, $2, $3, $4, $5, false, $6)
            RETURNING *
            "#, jump.system.address as i64,
                jump.jump_dist.map(|d| d as f32),
                jump.fuel_used.map(|u| u as f32),
                jump.fuel_level.map(|l| l as f32),
                Fuel::Hydrogen as Fuel,
                timestamp.naive_utc())
            .fetch_one(&db.pool)
            .await?;

        Ok(Jump {
            id: row.id,
            current_system_address: row.current_system_address,
            // TODO
            cost: None,
            // TODO: Actually check if it's in the future if given...
            future: row.future,
            timestamp: row.timestamp.map(|t| DateTime::<Utc>::from_utc(t, Utc)),
            next_jump_id: None,
        })
    }
}

pub struct Route {
    id: i64,
    name: String,
    start_jump_id: Uuid,
}


// #[test]
// fn routes() {
//     let start_jump = Jump::()

//     assert_eq!(None, faction.state);
// }


