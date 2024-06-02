use sqlx::PgPool;

#[tracing::instrument(name = "Change password", skip(db))]
pub async fn change_password(db: &PgPool, user_id: uuid::Uuid, new_password: String) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE users
        SET password = $1
        WHERE id = $2 AND is_active = true",
        new_password, user_id
    ).execute(db).await?;

    Ok(())
}