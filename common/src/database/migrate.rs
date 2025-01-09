use anyhow::Result;
use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;
use tracing::{error, info};

pub async fn migrate(db: &DatabaseConnection) -> Result<()> {
    if let Ok(migrations) = Migrator::get_pending_migrations(db).await {
        if !migrations.is_empty() {
            info!("数据库有 {} 个待迁移的表", migrations.len());
            Migrator::up(db, Some(migrations.len() as u32))
                .await
                .expect("数据库迁移失败");
        }
    } else {
        error!("获取待迁移的数量失败");
    }
    info!("数据迁移完成");
    Ok(())
}
