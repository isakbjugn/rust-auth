use sqlx::PgPool;

#[tracing::instrument(name = "Activating user", skip(db))]
pub async fn activate_user(db: &PgPool, user_id: uuid::Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE users
        SET is_active = true
        WHERE id = $1 AND is_active = false",
        user_id
    ).execute(db).await?;

    Ok(())
}