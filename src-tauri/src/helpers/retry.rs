use std::future::Future;

pub async fn retry<F, Fu, V, E>(mut attempts: u8, delay: u64, f: F) -> Result<V, E>
where F: Fn() -> Fu,
      Fu: Future<Output=Result<V, E>> 
{
    loop 
    {
        match f().await 
        {
            Ok(v) => return Ok(v),
            Err(e) if attempts == 1 => return Err(e),
            _ => 
            {
                attempts -= 1;
                tokio::time::sleep(tokio::time::Duration::from_secs(delay)).await;
            }
        };
    }
}