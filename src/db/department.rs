use crate::models::{Disease as Model, NewDisease as NewModel};
use anyhow::Result;
use sqlx::postgres::{PgPool, PgRow};
use sqlx::Row;

impl Model {
    pub async fn all(pool: &PgPool) -> Result<Vec<Self>> {
        let mut items = Vec::new();

        let recs = sqlx::query!(
            r#"
                SELECT id, name, symptoms
                    FROM diseases
                ORDER BY id
            "#
        )
        .fetch_all(pool)
        .await?;

        for rec in recs {
            items.push(Self {
                id: rec.id,
                name: rec.name,
                symptoms: rec.symptoms
            });
        }

        Ok(items)
    }

    pub async fn by_id(id: i32, pool: &PgPool) -> Result<Self> {
        let rec = sqlx::query!(
            r#"
                SELECT * FROM diseases WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(Self {
            id: rec.id,
            name: rec.name,
            symptoms: rec.symptoms
        })
    }

    pub async fn by_name(name: String, pool: &PgPool) -> Result<Self> {
        let rec = sqlx::query!(
            r#"
                SELECT * FROM diseases WHERE name = $1
            "#,
            name
        )
        .fetch_one(pool)
        .await?;

        Ok(Self {
            id: rec.id,
            name: rec.name,
            symptoms: rec.symptoms
        })
    }

    pub async fn by_symptoms(symptoms: &[i32], pool: &PgPool) -> Result<Vec<Self>> {
        let mut items = Vec::new();

        let recs = sqlx::query!(
            r#"
                SELECT * FROM diseases WHERE symptoms && $1
            "#,
            symptoms
        )
            .fetch_all(pool)
            .await?;

        for rec in recs {
            items.push(Self {
                id: rec.id,
                name: rec.name,
                symptoms: rec.symptoms
            });
        }

        Ok(items)
    }

    pub async fn create(item: NewModel, pool: &PgPool) -> Result<Self> {
        let mut tx = pool.begin().await?;
        let created = sqlx::query(
            r#"
                INSERT INTO diseases (name, symptoms) VALUES ($1)
                RETURNING id, name, symptoms
            "#,
        )
        .bind(&item.name)
        .bind(&item.symptoms)
        .map(|row: PgRow| Self {
            id: row.get(0),
            name: row.get(1),
            symptoms: row.get(2),
        })
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await?;
        Ok(created)
    }

    pub async fn update(id: i32, item: NewModel, pool: &PgPool) -> Result<Self> {
        let mut tx = pool.begin().await?;
        let updated = sqlx::query(
            r#"
                UPDATE diseases SET name = $1, symptoms = $2
                WHERE id = $3
                RETURNING id, name
            "#,
        )
        .bind(&item.name)
        .bind(&item.symptoms)
        .bind(id)
        .map(|row: PgRow| Self {
            id: row.get(0),
            name: row.get(1),
            symptoms: row.get(2),
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
                DELETE FROM diseases
                WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&mut tx)
        .await?;

        tx.commit().await?;
        Ok(true)
    }
}
