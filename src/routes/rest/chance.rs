use crate::models::{Chance as Model};
use anyhow::Result;
use sqlx::postgres::{PgPool, PgRow};
use sqlx::Row;

impl Model {
    pub async fn all(pool: &PgPool) -> Result<Vec<Self>> {
        let mut items = Vec::new();

        let recs = sqlx::query!(
            r#"
                SELECT disease, chance
                    FROM chances
                ORDER BY disease
            "#
        )
        .fetch_all(pool)
        .await?;

        for rec in recs {
            items.push(Self {
                disease: rec.disease,
                chance: rec.chance,
            });
        }

        Ok(items)
    }

    pub async fn by_disease(disease: i32, pool: &PgPool) -> Result<Self> {
        let rec = sqlx::query!(
            r#"
                SELECT * FROM chances WHERE disease = $1
            "#,
            disease
        )
        .fetch_one(pool)
        .await?;

        Ok(Self {
            disease: rec.disease,
            chance: rec.chance,
        })
    }

    pub async fn by_chance(chance: i32, pool: &PgPool) -> Result<Self> {
        let rec = sqlx::query!(
            r#"
                SELECT * FROM chances WHERE chance = $1
            "#,
            chance
        )
        .fetch_one(pool)
        .await?;

        Ok(Self {
            disease: rec.disease,
            chance: rec.chance,
        })
    }

    pub async fn create(item: Model, pool: &PgPool) -> Result<Self> {
        let mut tx = pool.begin().await?;
        let created = sqlx::query(
            r#"
                INSERT INTO chances (disease, chance) VALUES ($1, $2)
                RETURNING disease, chance
            "#,
        )
        .bind(&item.disease)
        .bind(&item.chance)
        .map(|row: PgRow| Self {
            disease: row.get(0),
            chance: row.get(1)
        })
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await?;
        Ok(created)
    }

    pub async fn update(disease: i32, chance: i32, pool: &PgPool) -> Result<Self> {
        let mut tx = pool.begin().await?;
        let updated = sqlx::query(
            r#"
                UPDATE chances SET chance = $1
                WHERE disease = $2
                RETURNING disease, chance
            "#,
        )
        .bind(chance)
        .bind(disease)
        .map(|row: PgRow| Self {
            disease: row.get(0),
            chance: row.get(1)
        })
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await?;
        Ok(updated)
    }

    pub async fn delete(id: i32, pool: &PgPool) -> Result<bool> {
        let mut tx = pool.begin().await?;
        sqlx::query(
            r#"
                DELETE FROM chances
                WHERE disease = $1
            "#,
        )
        .bind(id)
        .execute(&mut tx)
        .await?;

        tx.commit().await?;
        Ok(true)
    }
}
