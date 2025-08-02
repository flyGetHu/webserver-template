//! Web服务器主入口点

use dotenvy::dotenv;
use tracing::info;

mod app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 加载.env文件中的环境变量
    dotenv().ok();
    
    info!("Starting web server...");
    
    // 运行应用
    app::run().await?;
    
    info!("Web server stopped");
    Ok(())
}
