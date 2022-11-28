use pxollyrs::database::conn::DatabaseConn;
use pxollyrs::WebhookResult;

#[tokio::test]
async fn test_database() -> WebhookResult<()> {
    let conn = DatabaseConn::new("test_chat.json").await?;
    let mut guard = conn.lock().await;

    let bytes = guard.read().await?;
    assert_eq!(bytes, b"[]");

    guard.rewrite(b"1").await?;
    let bytes = guard.read().await?;
    assert_eq!(bytes, b"1");

    guard.rewrite(b"kek").await?;
    let bytes = guard.read().await?;
    assert_eq!(bytes, b"kek");

    tokio::fs::remove_file("test_chat.json").await?;
    Ok(())
}
