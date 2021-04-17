use crate::model::{Emoji, NewEmoji};
use anyhow::Result;
use sqlx::postgres::{PgPool, PgRow};
use sqlx::Row;

impl Emoji {
    pub async fn all(pool: &PgPool) -> Result<Vec<Self>> {
        let mut items = Vec::new();

        let recs = sqlx::query!(
            r#"
                SELECT id, name, utf8
                    FROM emojis
                ORDER BY id
            "#
        )
        .fetch_all(pool)
        .await?;

        for rec in recs {
            items.push(Self {
                id: rec.id,
                name: rec.name,
                utf8: rec.utf8,
            });
        }

        Ok(items)
    }

    pub async fn by_id(id: i32, pool: &PgPool) -> Result<Self> {
        let rec = sqlx::query!(
            r#"
                SELECT * FROM emojis WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(Self {
            id: rec.id,
            name: rec.name,
            utf8: rec.utf8,
        })
    }

    pub async fn by_name(name: String, pool: &PgPool) -> Result<Self> {
        let rec = sqlx::query!(
            r#"
                SELECT * FROM emojis WHERE name = $1
            "#,
            name
        )
        .fetch_one(pool)
        .await?;

        Ok(Self {
            id: rec.id,
            name: rec.name,
            utf8: rec.utf8,
        })
    }

    pub async fn create(item: NewEmoji, pool: &PgPool) -> Result<Self> {
        let mut tx = pool.begin().await?;
        let created = sqlx::query(
            r#"
                INSERT INTO emojis (name, utf8) VALUES ($1, $2)
                RETURNING id, name, utf8
            "#,
        )
        .bind(&item.name)
        .bind(&item.utf8)
        .map(|row: PgRow| Self {
            id: row.get(0),
            name: row.get(1),
            utf8: row.get(2),
        })
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await?;
        Ok(created)
    }

    pub async fn update(id: i32, item: NewEmoji, pool: &PgPool) -> Result<Self> {
        let mut tx = pool.begin().await?;
        let updated = sqlx::query(
            r#"
                UPDATE emojis SET name = $1, utf8 = $2
                WHERE id = $3
                RETURNING id, name, utf8
            "#,
        )
        .bind(&item.name)
        .bind(&item.utf8)
        .bind(id)
        .map(|row: PgRow| Self {
            id: row.get(0),
            name: row.get(1),
            utf8: row.get(2),
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
                DELETE FROM emojis
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
