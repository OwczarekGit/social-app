use axum::Extension;
use axum::extract::State;
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::IntoResponse;
use tower_cookies::Cookies;
use tracing::debug;
use crate::{Result, Error, endpoint};
use crate::app_state::{ActiveUser, ActiveUserRole};
use crate::service::account::AccountService;

pub async fn authorize_admin(
    Extension(user): Extension<ActiveUser>,
    request: Request,
    next: Next
) -> Result<impl IntoResponse> {
    if user.role != ActiveUserRole::Admin {
        return Err(Error::UnauthorizedForAdminOperations(user.id));
    }

    debug!("User id: ({}), role: ({}) accessed the admin endpoints.", user.id, user.role.to_string());

    Ok(next.run(request).await)
}

pub async fn authorize_by_cookie(
    State(mut acs): State<AccountService>,
    cookies: Cookies,
    request: Request,
    next: Next
) -> Result<impl IntoResponse> {
    let cookie = cookies.get(endpoint::account::SESSION_COOKIE_NAME).ok_or(Error::UnauthorizedForUserOperations)?;

    let user = acs.verify_session(cookie.value()).await?;

    let mut response = request;
    response.extensions_mut().insert(user);

    Ok(next.run(response).await)
}
