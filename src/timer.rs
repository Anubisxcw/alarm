use delay_timer::prelude::*;
use anyhow::Result;
use smol::Timer;
use std::sync::Arc;
use std::time::Duration;
use crate::{config, update_access_token};
use crate::config::AppConfig;
use crate::qywx::get_wechat_token_with_proxy;

pub(crate) async fn init_timer(app_config: &AppConfig) -> std::result::Result<(), TaskError> {
    // 使用 Arc 包裹 app_config
    let app_config = Arc::new(app_config.clone()); // 假设 AppConfig 实现了 Clone
    let delay_timer = DelayTimerBuilder::default().build();

    // 创建任务
    let task = get_token(Arc::clone(&app_config))?;
    delay_timer.insert_task(task)?;

    // **立即触发一次任务逻辑**
    trigger_task_immediately(Arc::clone(&app_config)).await;

    // 保持延时定时器运行
    tracing::info!("Delay timer started and will run indefinitely...");
    loop {
        smol::Timer::after(Duration::from_secs(60)).await; // 保持循环，避免程序退出
    }
}

// 定义任务逻辑
fn get_token(app_config: Arc<AppConfig>) -> Result<Task, TaskError> {
    let mut task_builder = TaskBuilder::default();

    let app_config = Arc::clone(&app_config);

    let body = move || {
        let app_config = Arc::clone(&app_config);

        async move {
            tracing::info!("Ready to get access token");
            match get_wechat_token_with_proxy(&app_config).await {
                Ok(token) => {
                    update_access_token(&token);
                    tracing::info!("Update access token success");
                }
                Err(e) => {
                    tracing::error!("Failed to get access token: {:?}", e);
                }
            }
        }
    };

    task_builder
        .set_task_id(1)
        .set_frequency_repeated_by_seconds(60) // 任务每 60 秒执行一次
        .set_maximum_parallel_runnable_num(1)
        .spawn_async_routine(body)
}

// 手动触发任务逻辑
async fn trigger_task_immediately(app_config: Arc<AppConfig>) {
    tracing::info!("Triggering task immediately on startup");
    match get_wechat_token_with_proxy(&app_config).await {
        Ok(token) => {
            update_access_token(&token);
            tracing::info!("Initial access token update success");
        }
        Err(e) => {
            tracing::error!("Failed to get access token on startup: {:?}", e);
        }
    }
}

