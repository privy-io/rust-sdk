//! Subclients
//!
//! This module houses all the sub-clients that are used by the main client.
//! You can usually attain an instance of a sub-client by calling the relevant
//! function on the main client. See `PrivyClient` for more information.

include!(concat!(env!("OUT_DIR"), "/subclients.rs"));

mod key_quorums;
mod policies;
mod wallets;

/// An authorization signature and the request expiry it covers.
///
/// Pass `request_expiry` as the generated method's `privy_request_expiry`
/// argument so the header matches the signed value.
pub(crate) struct SignedRequest {
    pub(crate) signature: String,
    pub(crate) request_expiry: Option<String>,
}

/// Signs a request, covering the idempotency key and request expiry when set.
pub(crate) async fn sign_request<S: serde::Serialize>(
    ctx: &crate::AuthorizationContext,
    app_id: &str,
    method: crate::Method,
    url: String,
    body: S,
    idempotency_key: Option<&str>,
    request_expiry: Option<u64>,
) -> Result<SignedRequest, crate::SignatureGenerationError> {
    let headers = crate::AuthorizationSignatureHeaders {
        idempotency_key: idempotency_key.map(str::to_owned),
        request_expiry,
    };
    let signature = crate::generate_authorization_signatures_with_headers(
        ctx, app_id, method, url, body, &headers,
    )
    .await?;

    Ok(SignedRequest {
        signature,
        request_expiry: request_expiry.map(|expiry| expiry.to_string()),
    })
}
