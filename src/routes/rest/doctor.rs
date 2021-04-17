use crate::models::{Doctor as Model, Doctor as NewModel};
use anyhow::Result;
use sqlx::postgres::{PgPool, PgRow};
use sqlx::Row;

impl Model {
    pub async fn all(pool: &PgPool) -> Result<Vec<Self>> {
        let mut items = Vec::new();

        let recs = sqlx::query!(
            r#"
                SELECT id, name, occupied, department
                    FROM doctors
                ORDER BY id
            "#
        )
            .fetch_all(pool)
            .await?;

        for rec in recs {
            items.push(Self {
                id: rec.id,
                name: rec.name,
                occupied: rec.occupied,
                department: rec.department
            });
        }

        Ok(items)
    }

    pub async fn by_id(id: i32, pool: &PgPool) -> Result<Self> {
        let rec = sqlx::query!(
            r#"
                SELECT * FROM doctors WHERE id = $1
            "#,
            id
        )
            .fetch_one(pool)
            .await?;

        Ok(Self {
            id: rec.id,
            name: rec.name,
            occupied: rec.occupied,
            department: rec.department
        })
    }

    pub async fn by_name(name: String, pool: &PgPool) -> Result<Self> {
        let rec = sqlx::query!(
            r#"
                SELECT * FROM doctors WHERE name = $1
            "#,
            name
        )
            .fetch_one(pool)
            .await?;

        Ok(Self {
            id: rec.id,
            name: rec.name,
            occupied: rec.occupied,
            department: rec.department
        })
    }

    pub async fn by_department(department: i32, pool: &PgPool) -> Result<Self> {
        let rec = sqlx::query!(
            r#"
                SELECT * FROM doctors WHERE department = $1
            "#,
            department
        )
            .fetch_one(pool)
            .await?;

        Ok(Self {
            id: rec.id,
            name: rec.name,
            occupied: rec.occupied,
            department: rec.department
        })
    }

    pub async fn available_by_department(department: i32, pool: &PgPool) -> Result<Self> {
        let rec = sqlx::query!(
            r#"
                SELECT * FROM doctors WHERE department = $1 AND occupied = false
            "#,
            department
        )
            .fetch_one(pool)
            .await?;

        Ok(Self {
            id: rec.id,
            name: rec.name,
            occupied: rec.occupied,
            department: rec.department
        })
    }

    pub async fn create(item: NewModel, pool: &PgPool) -> Result<Self> {
        let mut tx = pool.begin().await?;
        let created = sqlx::query(
            r#"
                INSERT INTO doctors (name, occupied, department) VALUES ($1, $2, $3)
                RETURNING id, name, occupied, department
            "#,
        )
            .bind(&item.name)
            .bind(&item.occupied)
            .bind(&item.department)
            .map(|row: PgRow| Self {
                id: row.get(0),
                name: row.get(1),
                occupied: row.get(2),
                department: row.get(3)
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
                UPDATE doctors SET name = $1, occupied = $2, department = $3
                WHERE id = $4
                RETURNING id, name, occupied, department
            "#,
        )
            .bind(&item.name)
            .bind(id)
            .map(|row: PgRow| Self {
                id: row.get(0),
                name: row.get(1),
                occupied: row.get(2),
                department: row.get(3)
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
                DELETE FROM doctors
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
