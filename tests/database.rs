use pxollyrs::database::conn::DatabaseConn;
use pxollyrs::WebhookResult;

#[tokio::test]
async fn test_database() -> WebhookResult<()> {
    let conn = DatabaseConn::new("chat.json").await?;
    conn.lock().await.rewrite(b"[1]").await?;
    let bytes = conn.lock().await.read().await?;
    assert_eq!(bytes, b"[1]");

    conn.lock().await.truncate().await?;
    Ok(())
}
