use crate::models::device::Device;
use sqlx::SqlitePool;

pub async fn get_devices(pool: &SqlitePool) -> Result<Vec<Device>, sqlx::Error> {
    let devices = sqlx::query_as::<_, Device>(
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
        "#,
    )
    .fetch_all(pool)
    .await?;
    Ok(devices)
}

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

pub async fn insert_device(pool: &SqlitePool, device: &Device) -> Result<u64, sqlx::Error> {
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
pub async fn insert_devices() {}

#[allow(dead_code)]
pub async fn update_device() {}

#[allow(dead_code)]
pub async fn delete_device() {}

#[allow(dead_code)]
pub async fn delete_devices() {}
