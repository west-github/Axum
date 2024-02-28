use sqlx::postgres::PgPoolOptions;
use std::path::{Path, PathBuf};

pub type Result<T> = core::result::Result<T, sqlx::Error>;

pub const RECREATE_FILE: &'static str = "00-recreate-db.sql";

pub async fn db_setup(admin_str: &str, dev_str: &str, sql_dir: &PathBuf) -> Result<()> {
    // Scope to drop and recreate the db
    {
        pexec(admin_str, &sql_dir.join(RECREATE_FILE)).await?;
    }

    let mut paths = std::fs::read_dir(sql_dir)?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect::<Vec<PathBuf>>();

    paths.sort();

    for path in paths {
        let path_str = path.to_string_lossy();

        if !path_str.ends_with(".sql") || path_str.contains(RECREATE_FILE) {
            continue;
        }

        pexec(dev_str, &path).await?;
    }

    Ok(())
}

pub async fn pexec(url: &str, file: &Path) -> Result<()> {
    let db = PgPoolOptions::new().connect(url).await?;

    let content = std::fs::read_to_string(file)?;
    let sqls: Vec<&str> = content.split(';').collect();

    for sql in sqls {
        sqlx::query(sql).execute(&db).await?;
    }

    Ok(())
}
