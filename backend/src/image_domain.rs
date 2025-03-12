/* pub async fn extract_image_domain(
    State(domain_service): State<DomainService>,
    request: Request,
    next: Next,
) -> SysRes<impl IntoResponse> {
    let image_domain = domain_service
        .get_image_domain()
        .await
        .unwrap_or(Some("".to_string()))
        .unwrap_or("".to_string());

    let mut response = request;
    response.extensions_mut().insert(ImageDomain(image_domain));

    Ok(next.run(response).await)
}

#[derive(Debug, Clone)]
pub struct ImageDomain(pub String);

#[async_trait]
impl<S> FromRequestParts<S> for ImageDomain {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> std::result::Result<Self, Self::Rejection> {
        if let Some(domain) = parts.extensions.get::<ImageDomain>() {
            Ok(domain.clone())
        } else {
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
*/

use axum::{
    RequestPartsExt,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};

use crate::{Error, app_state::AppState};

#[derive(Debug, Clone)]
pub struct ImageDomain(pub String);

impl<S> FromRequestParts<S> for ImageDomain
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = crate::Error;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> std::result::Result<Self, Self::Rejection> {
        let state = parts
            .extract_with_state::<AppState, _>(state)
            .await
            .map_err(|_| Error::AppStateMissing)?;

        let domain = state
            .domain_service
            .get_image_domain()
            .await?
            .unwrap_or(String::new());

        Ok(ImageDomain(domain))

        // let image_domain = domain_service
        //     .get_image_domain()
        //     .await
        //     .unwrap_or(Some("".to_string()))
        //     .unwrap_or("".to_string());
        // if let Some(domain) = parts.extensions.get::<ImageDomain>() {
        //     Ok(domain.clone())
        // } else {
        //     Err(StatusCode::INTERNAL_SERVER_ERROR)
        // }
    }
}
