use crate::models::device::{Device, DeviceUpload};
use sqlx::SqlitePool;

#[allow(dead_code)]
pub async fn get_devices(pool: &SqlitePool) -> Result<Vec<Device>, sqlx::Error> {
    let devices = sqlx::query_as::<_, Device>(
        r#"
            SELECT 
            id,
            manufacturer,
            serial,
            capacity,
            assigned_number,
            registered,
            secret,
            special,
            secclass,
            max_secclass,
            owner,
            register_number,
            conclusion_number,
            prescription,
            zones,
            destroyed
        FROM devices
        "#,
    )
    .fetch_all(pool)
    .await?;
    Ok(devices)
}

#[allow(dead_code)]
pub async fn get_device_by_id(pool: &SqlitePool, id: i64) -> Result<Option<Device>, sqlx::Error> {
    let device = sqlx::query_as(
        r#"
            SELECT 
            manufacturer,
            serial,
            capacity,
            assigned_number,
            registered,
            secret,
            special,
            secclass,
            max_secclass,
            owner,
            register_number,
            conclusion_number,
            prescription,
            zones,
            destroyed
        FROM devices
        WHERE id = ?
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(device)
}

#[allow(dead_code)]
pub async fn get_device_by_serial(
    pool: &SqlitePool,
    serial: String,
) -> Result<Option<Device>, sqlx::Error> {
    let device = sqlx::query_as::<_, Device>(
        r#"
            SELECT 
            manufacturer,
            serial,
            capacity,
            assigned_number,
            registered,
            secret,
            special,
            secclass,
            max_secclass,
            owner,
            register_number,
            conclusion_number,
            prescription,
            zones,
            destroyed
        FROM devices
        WHERE serial = ?
        "#,
    )
    .bind(serial)
    .fetch_optional(pool)
    .await?;
    Ok(device)
}

#[allow(dead_code)]
pub async fn insert_device(pool: &SqlitePool, device: &DeviceUpload) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        r#"
        INSERT INTO devices (
            manufacturer,
            serial,
            capacity,
            assigned_number,
            registered,
            secret,
            special,
            secclass,
            max_secclass,
            owner,
            register_number,
            conclusion_number,
            prescription,
            zones,
            destroyed
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&device.manufacturer)
    .bind(&device.serial)
    .bind(&device.capacity)
    .bind(&device.assigned_number)
    .bind(&device.registered)
    .bind(&device.secret)
    .bind(&device.special)
    .bind(&device.secclass)
    .bind(&device.max_secclass)
    .bind(&device.owner)
    .bind(&device.register_number)
    .bind(&device.conclusion_number)
    .bind(&device.prescription)
    .bind(&device.zones)
    .bind(&device.destroyed)
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}

#[allow(dead_code)]
pub async fn insert_devices(
    pool: &SqlitePool,
    devices: &[DeviceUpload],
) -> Result<u64, sqlx::Error> {
    let mut tx = pool.begin().await?;
    let mut affected = 0;

    for device in devices {
        let result = sqlx::query(
            r#"
            INSERT INTO devices (
                manufacturer,
                serial,
                capacity,
                assigned_number,
                registered,
                secret,
                special,
                secclass,
                max_secclass,
                owner,
                register_number,
                conclusion_number,
                prescription,
                zones,
                destroyed
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&device.manufacturer)
        .bind(&device.serial)
        .bind(&device.capacity)
        .bind(&device.assigned_number)
        .bind(device.registered)
        .bind(device.secret)
        .bind(device.special)
        .bind(&device.secclass)
        .bind(&device.max_secclass)
        .bind(&device.owner)
        .bind(&device.register_number)
        .bind(&device.conclusion_number)
        .bind(&device.prescription)
        .bind(&device.zones)
        .bind(device.destroyed)
        .execute(&mut *tx)
        .await?;

        affected += result.rows_affected();
    }
    tx.commit().await?;
    Ok(affected)
}

#[allow(dead_code)]
pub async fn update_device(
    pool: &SqlitePool,
    id: i64,
    device: &DeviceUpload,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        r#"
        UPDATE devices
        SET
            manufacturer = ?,
            serial = ?,
            capacity = ?,
            assigned_number = ?,
            registered = ?,
            secret = ?,
            special = ?,
            secclass = ?,
            max_secclass = ?,
            owner = ?,
            register_number = ?,
            conclusion_number = ?,
            prescription = ?,
            zones = ?,
            destroyed = ?
        WHERE id = ?
        "#,
    )
    .bind(&device.manufacturer)
    .bind(&device.serial)
    .bind(&device.capacity)
    .bind(&device.assigned_number)
    .bind(&device.registered)
    .bind(&device.secret)
    .bind(&device.special)
    .bind(&device.secclass)
    .bind(&device.max_secclass)
    .bind(&device.owner)
    .bind(&device.register_number)
    .bind(&device.conclusion_number)
    .bind(&device.prescription)
    .bind(&device.zones)
    .bind(&device.destroyed)
    .bind(id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}

#[allow(dead_code)]
pub async fn delete_device(pool: &SqlitePool, id: i64) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM devices WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}

#[allow(dead_code)]
pub async fn delete_devices(pool: &SqlitePool, ids: Vec<i64>) -> Result<u64, sqlx::Error> {
    let mut affected = 0;
    let mut tx = pool.begin().await?;

    for id in ids {
        let r = sqlx::query("DELETE FROM devices WHERE id = ?")
            .bind(id)
            .execute(&mut *tx)
            .await?;
        affected += r.rows_affected();
    }
    tx.commit().await?; // транзакция
    Ok(affected)
}
