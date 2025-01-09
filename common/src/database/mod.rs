use anyhow::Result;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;
use tokio::sync::OnceCell;

mod migrate;

use configs::CFG;
pub use migrate::migrate;

//  异步初始化数据库
pub static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();
pub async fn get_db_conn() -> Result<DatabaseConnection> {
    let url = CFG.database.link.clone();
    DB.get_or_try_init(|| async {
        tracing::info!("正在连接数据库...");

        let mut opt = ConnectOptions::new(url).to_owned();
        opt.max_connections(1000)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .sqlx_logging(false);
        let db = Database::connect(opt)
            .await
            .map_err(|e| anyhow::anyhow!("数据库连接失败: {}", e))?;
        tracing::info!("数据库连接成功");
        // 数据库链接成功后，执行一次数据库迁移
        migrate(&db).await?;
        Ok(db)
    })
    .await
    .cloned()
}
