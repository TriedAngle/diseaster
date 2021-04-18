use crate::models::{Department as Model, NewDepartment as NewModel};
use anyhow::Result;
use sqlx::postgres::{PgPool, PgRow};
use sqlx::Row;

impl Model {
    pub async fn all(pool: &PgPool) -> Result<Vec<Self>> {
        let mut items = Vec::new();

        let recs = sqlx::query!(
            r#"
                SELECT id, name, diseases
                    FROM departments
                ORDER BY id
            "#
        )
        .fetch_all(pool)
        .await?;

        for rec in recs {
            items.push(Self {
                id: rec.id,
                name: rec.name,
                diseases: rec.diseases,
            });
        }

        Ok(items)
    }

    pub async fn by_id(id: i32, pool: &PgPool) -> Result<Self> {
        let rec = sqlx::query!(
            r#"
                SELECT * FROM departments WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(Self {
            id: rec.id,
            name: rec.name,
            diseases: rec.diseases,
        })
    }

    pub async fn by_name(name: String, pool: &PgPool) -> Result<Self> {
        let rec = sqlx::query!(
            r#"
                SELECT * FROM departments WHERE name = $1
            "#,
            name
        )
        .fetch_one(pool)
        .await?;

        Ok(Self {
            id: rec.id,
            name: rec.name,
            diseases: rec.diseases,
        })
    }

    pub async fn by_disease(disease: &[i32], pool: &PgPool) -> Result<Self> {
        let rec = sqlx::query!(
            r#"
                SELECT * FROM departments WHERE diseases && $1
            "#,
            disease
        )
        .fetch_one(pool)
        .await?;

        Ok(Self {
            id: rec.id,
            name: rec.name,
            diseases: rec.diseases,
        })

    }

    pub async fn create(item: NewModel, pool: &PgPool) -> Result<Self> {
        let mut tx = pool.begin().await?;
        let created = sqlx::query(
            r#"
                INSERT INTO departments (name, diseases) VALUES ($1, $2)
                RETURNING id, name, diseases
            "#,
        )
        .bind(&item.name)
        .bind(&item.diseases)
        .map(|row: PgRow| Self {
            id: row.get(0),
            name: row.get(1),
            diseases: row.get(2),
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
                UPDATE departments SET name = $1, diseases = $2
                WHERE id = $3
                RETURNING id, name, diseases
            "#,
        )
        .bind(&item.name)
        .bind(&item.diseases)
        .bind(id)
        .map(|row: PgRow| Self {
            id: row.get(0),
            name: row.get(1),
            diseases: row.get(2),
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
                DELETE FROM departments
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
