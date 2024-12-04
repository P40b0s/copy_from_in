use nosleep::NoSleep;
pub async fn prevent_sleep()
{
    let _ = tokio::task::spawn_blocking(||
    {
        let mut nosleep = NoSleep::new().unwrap();
        nosleep
        .start(nosleep::NoSleepType::PreventUserIdleSystemSleep)
        .unwrap();
    }).await;
    
}

