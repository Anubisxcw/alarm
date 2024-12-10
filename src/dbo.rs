use crate::config::AppConfig;
use duckdb::Connection;

pub fn init_db(conn: &Connection) {
    tracing::info!("ready to init db");
    conn.execute_batch(
        "CREATE SEQUENCE seq; create table if not exists alarm_cluster_info (id integer primary key default nextval('seq'),\
        cluster text not null,\
        chat_id text);"
    ).expect("create table if not exists alarm_cluster_info failed");
    tracing::info!("init db success");
}

pub fn get_all_data(conn: &Connection) -> Result<Vec<AlarmClusterInfo>, duckdb::Error> {
    let mut stmt = conn.prepare("SELECT id, cluster, chat_id FROM alarm_cluster_info")?;
    let alarm_cluster_infos = stmt.query_map([], |row| {
        Ok(AlarmClusterInfo {
            id: row.get(0)?,
            cluster: row.get(1)?,
            chat_id: row.get(2)?,
        })
    })?;

    let mut results = Vec::new();
    for info in alarm_cluster_infos {
        results.push(info?);
    }

    Ok(results)
}
pub fn get_by_cluster(conn: &Connection, cluster_name: &str) -> Result<Vec<AlarmClusterInfo>, duckdb::Error> {
    // 准备查询，查找指定 cluster 的记录
    let mut stmt = conn.prepare("SELECT id, cluster, chat_id FROM alarm_cluster_info WHERE cluster = ?")?;

    // 执行查询并将结果映射到 `AlarmClusterInfo` 结构体
    let alarm_cluster_infos = stmt.query_map([cluster_name], |row| {
        Ok(AlarmClusterInfo {
            id: row.get(0)?,
            cluster: row.get(1)?,
            chat_id: row.get(2)?,
        })
    })?;

    // 收集查询结果
    let mut results = Vec::new();
    for info in alarm_cluster_infos {
        results.push(info?);
    }

    Ok(results)
}
#[derive(Debug)]
pub struct AlarmClusterInfo {
    id: i32,
    cluster: String,
    chat_id: String,
}