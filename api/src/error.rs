use std::{error, fmt};

#[derive(thiserror::Error)]
pub enum DatabaseError {
  #[error("{0}")]
  ValidationError(String),
  #[error(transparent)]
  UnexpectedError(#[from] anyhow::Error),
}

pub fn db_error<E: Into<anyhow::Error>>(error: E) -> DatabaseError {
  DatabaseError::UnexpectedError(error.into())
}

impl fmt::Debug for DatabaseError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    error_chain_fmt_impl(self, f)
  }
}

pub fn error_chain_fmt_impl(e: &impl error::Error, f: &mut fmt::Formatter<'_>) -> fmt::Result {
  writeln!(f, "{}\n", e)?;

  let mut current = e.source();

  while let Some(cause) = current {
    writeln!(f, "Caused by:\n\t{}", cause)?;

    current = cause.source();
  }

  Ok(())
}

#[macro_export]
macro_rules! error_chain_fmt {
  ($error_enum:ty) => {
    impl std::fmt::Debug for $error_enum {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        $crate::error::error_chain_fmt_impl(self, f)
      }
    }
  };
}

/// This macro generates an `IntoResponse` implementation for an error type.
///
/// Usage:
/// ```ignore
/// into_response!(GetUserError {
///   NotFound => StatusCode::NOT_FOUND,
///   ValidationError(_) => StatusCode::BAD_REQUEST,
///   DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
/// });
/// ```
#[macro_export]
macro_rules! into_response {
  ($error_type:ident {
    $($variant:ident $(($($field:tt)*))? => $status:expr),* $(,)?
  }) => {
    impl axum::response::IntoResponse for $error_type {
      fn into_response(self) -> axum::http::Response<axum::body::Body> {
        #[allow(unused_imports)]
        use axum::http::StatusCode;
        let status = match self {$(
          $error_type::$variant $(($($field)*))? => $status,
        )*};

        axum::http::Response::builder()
          .status(status)
          .body(axum::body::Body::empty())
          .unwrap()
      }
    }
  };
}
