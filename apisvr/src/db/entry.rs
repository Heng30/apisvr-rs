use super::{pool, ComEntry};
use anyhow::Result;

async fn _new(table_name: &str, is_unique_data: bool) -> Result<()> {
    let sql = format!(
        "CREATE TABLE IF NOT EXISTS {} (
             id INTEGER PRIMARY KEY,
             uuid TEXT NOT NULL UNIQUE,
             data TEXT NOT NULL {}
             )",
        table_name,
        if is_unique_data { "UNIQUE" } else { "" }
    );

    sqlx::query(&sql).execute(&pool()).await?;

    Ok(())
}

pub async fn new(table_name: &str) -> Result<()> {
    _new(table_name, false).await
}

pub async fn new_with_unique(table_name: &str) -> Result<()> {
    _new(table_name, true).await
}

pub async fn delete(table_name: &str, uuid: &str) -> Result<()> {
    sqlx::query(&format!("DELETE FROM {} WHERE uuid=?", table_name))
        .bind(uuid)
        .execute(&pool())
        .await?;
    Ok(())
}

#[allow(dead_code)]
pub async fn delete_all(table_name: &str) -> Result<()> {
    sqlx::query(&format!("DELETE FROM {}", table_name))
        .execute(&pool())
        .await?;
    Ok(())
}

pub async fn insert(table_name: &str, uuid: &str, data: &str) -> Result<()> {
    sqlx::query(&format!(
        "INSERT INTO {} (uuid, data) VALUES (?, ?)",
        table_name
    ))
    .bind(uuid)
    .bind(data)
    .execute(&pool())
    .await?;
    Ok(())
}

#[allow(dead_code)]
pub async fn insert_all(table_name: &str, entrys: Vec<ComEntry>) -> (usize, Result<()>) {
    if entrys.is_empty() {
        return (0, Ok(()));
    }

    let entrys_len = entrys.len();
    let p = pool();

    for (index, ComEntry { uuid, data }) in entrys.into_iter().enumerate() {
        if let Err(e) = sqlx::query(&format!(
            "INSERT INTO {} (uuid, data) VALUES (?, ?)",
            table_name
        ))
        .bind(uuid)
        .bind(data)
        .execute(&p)
        .await
        {
            return (index, Err(anyhow::anyhow!("{e:?}")));
        }
    }

    (entrys_len, Ok(()))
}

#[allow(dead_code)]
pub async fn update(table_name: &str, uuid: &str, data: &str) -> Result<()> {
    sqlx::query(&format!("UPDATE {} SET data=? WHERE uuid=?", table_name))
        .bind(data)
        .bind(uuid)
        .execute(&pool())
        .await?;

    Ok(())
}

#[allow(dead_code)]
pub async fn select(table_name: &str, uuid: &str) -> Result<ComEntry> {
    Ok(
        sqlx::query_as::<_, ComEntry>(&format!("SELECT * FROM {} WHERE uuid=?", table_name))
            .bind(uuid)
            .fetch_one(&pool())
            .await?,
    )
}

pub async fn select_all(table_name: &str) -> Result<Vec<ComEntry>> {
    Ok(
        sqlx::query_as::<_, ComEntry>(&format!("SELECT * FROM {}", table_name))
            .fetch_all(&pool())
            .await?,
    )
}

#[allow(dead_code)]
pub async fn is_exist(table_name: &str, uuid: &str) -> bool {
    select(table_name, uuid).await.is_ok()
}

#[allow(dead_code)]
pub async fn drop_table(table_name: &str) -> Result<()> {
    super::drop_table(&table_name).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use rocket::tokio;
    use std::sync::Mutex;

    static MTX: Mutex<()> = Mutex::new(());
    const DB_PATH: &str = "/tmp/entry-test.db";

    #[tokio::test]
    async fn test_table_new() -> Result<()> {
        let _mtx = MTX.lock().unwrap();
        db::init(DB_PATH).await;
        new("suuid_1").await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_delete_all() -> Result<()> {
        let _mtx = MTX.lock().unwrap();
        db::init(DB_PATH).await;
        new("suuid_1").await?;
        delete_all("suuid_1").await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_delete_one() -> Result<()> {
        let _mtx = MTX.lock().unwrap();
        db::init(DB_PATH).await;
        new("suuid_1").await?;

        delete_all("suuid_1").await?;
        insert("suuid_1", "uuid-1", "data-1").await?;
        delete("suuid_1", "uuid-1").await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_insert() -> Result<()> {
        let _mtx = MTX.lock().unwrap();
        db::init(DB_PATH).await;
        new("suuid_1").await?;
        delete_all("suuid_1").await?;

        insert("suuid_1", "uuid-1", "data-1").await?;
        insert("suuid_1", "uuid-2", "data-2").await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_insert_all() -> Result<()> {
        let _mtx = MTX.lock().unwrap();
        db::init(DB_PATH).await;
        new("suuid_1").await?;
        delete_all("suuid_1").await?;

        let entrys = (0..100)
            .into_iter()
            .map(|index| ComEntry {
                uuid: format!("uuid-{index}"),
                data: format!("data-{index}"),
            })
            .collect();

        let (counts, _) = insert_all("suuid_1", entrys).await;
        assert_eq!(counts, 100);

        Ok(())
    }

    #[tokio::test]
    async fn test_update() -> Result<()> {
        let _mtx = MTX.lock().unwrap();
        db::init(DB_PATH).await;
        new("suuid_1").await?;
        delete_all("suuid_1").await?;

        insert("suuid_1", "uuid-1", "data-1").await?;
        update("suuid_1", "uuid-1", "data-1-1").await?;

        assert_eq!(
            select("suuid_1", "uuid-1").await?.data,
            "data-1-1".to_string()
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_select_one() -> Result<()> {
        let _mtx = MTX.lock().unwrap();
        db::init(DB_PATH).await;
        new("suuid_1").await?;
        delete_all("suuid_1").await?;

        assert!(select("suuid_1", "uuid-1").await.is_err());

        insert("suuid_1", "uuid-1", "data-1").await?;
        let item = select("suuid_1", "uuid-1").await?;
        assert_eq!(item.uuid, "uuid-1");
        assert_eq!(item.data, "data-1");
        Ok(())
    }

    #[tokio::test]
    async fn test_select_all() -> Result<()> {
        let _mtx = MTX.lock().unwrap();
        db::init(DB_PATH).await;
        new("suuid_1").await?;
        delete_all("suuid_1").await?;

        insert("suuid_1", "uuid-1", "data-1").await?;
        insert("suuid_1", "uuid-2", "data-2").await?;

        let v = select_all("suuid_1").await?;
        assert_eq!(v[0].uuid, "uuid-1");
        assert_eq!(v[0].data, "data-1");
        assert_eq!(v[1].uuid, "uuid-2");
        assert_eq!(v[1].data, "data-2");
        Ok(())
    }

    #[tokio::test]
    async fn test_drop_table() -> Result<()> {
        let _mtx = MTX.lock().unwrap();
        db::init(DB_PATH).await;
        new("suuid_1").await?;
        delete_all("suuid_1").await?;
        insert("suuid_1", "uuid-1", "data-1").await?;

        assert!(drop_table("suuid_0").await.is_err());
        assert!(drop_table("suuid_1").await.is_ok());
        Ok(())
    }
}
