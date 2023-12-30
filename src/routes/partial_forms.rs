use crate::domain::parse_json;
use crate::domain::NewPartialForm;
use crate::domain::PartialFormComplete;
use crate::domain::PartialFormSection;
use actix_web::http::StatusCode;
use actix_web::web;
use actix_web::HttpResponse;
use actix_web::ResponseError;
use anyhow::Context;
use serde_json::to_value;
use sqlx::PgPool;
use sqlx::Postgres;
use sqlx::Transaction;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    field_json: String,
    section: i32,
    complete: bool,
}

impl TryFrom<FormData> for NewPartialForm {
    type Error = String;

    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let field = parse_json(&value.field_json).map_err(|e| e.to_string())?;
        let section = PartialFormSection::from_int(value.section).map_err(|e| e.to_string())?;
        let complete = PartialFormComplete::new(value.complete);

        Ok(NewPartialForm {
            field,
            section,
            complete,
        })
    }
}

#[tracing::instrument(
    name = "Adding a new partial form",
    skip(form, pool),
    fields(
        field_json = %form.field_json,
        section = %form.section,
        complete = %form.complete,
    )
)]
// TODO
pub async fn partial_form(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, PartialFormError> {
    // TODO
    let new_partial_form = form
        .0
        .try_into()
        .map_err(PartialFormError::ValidationError)?;

    let mut transaction = pool
        .begin()
        .await
        .context("Failed to acquire a Postgres connection from the Pool.")?;

    let _partial_form_id = insert_partial_form(&mut transaction, &new_partial_form)
        .await
        .context("Failed to insert new subscriber in the database.")?;

    transaction
        .commit()
        .await
        .context("Failed to commit SQL transaction to store a new subscriber.")?;

    Ok(HttpResponse::Ok().finish())
}

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by: {}", cause)?;
        current = cause.source();
    }
    Ok(())
}

#[derive(thiserror::Error)]
pub enum PartialFormError {
    #[error("{0}")]
    ValidationError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for PartialFormError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for PartialFormError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::ValidationError(_) => StatusCode::BAD_REQUEST,
            Self::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[tracing::instrument(
    name = "Saving new partial form in the database",
    skip(new_partial_form, transaction)
)]
pub async fn insert_partial_form(
    transaction: &mut Transaction<'_, Postgres>,
    new_partial_form: &NewPartialForm,
) -> Result<Uuid, sqlx::Error> {
    let partial_form_id = Uuid::new_v4();
    let field_as_value = match to_value(&new_partial_form.field) {
        Ok(value) => value,
        Err(e) => {
            let error = std::io::Error::new(std::io::ErrorKind::Other, e.to_string());
            return Err(sqlx::Error::from(error));
        }
    };

    sqlx::query!(
        r#"INSERT INTO partial_form (id, section, field, complete, created_at)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        partial_form_id,
        new_partial_form.section.to_int(),
        field_as_value,
        new_partial_form.complete.as_ref(),
        chrono::Utc::now(),
    )
    .execute(transaction)
    .await?;
    Ok(partial_form_id)
}
