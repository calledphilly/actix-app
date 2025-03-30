use actix_identity::{self, Identity};
use actix_service::{Service, Transform};
use actix_web::{
    self, body::{BoxBody, EitherBody}, dev::{ServiceRequest, ServiceResponse}, Error, FromRequest, HttpResponse
};
use futures_util::future::{ok, LocalBoxFuture, Ready};
use std::task::{Context, Poll};

pub struct AuthMiddleware;
impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareService { service })
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
}
impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<EitherBody<B>>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        Box::pin( async move {
            let (request, payload) = req.parts();
            let identity = Identity::from_request(&request, &mut payload);
            if let Ok(user_id) = identity.await.unwrap().id() {
                println!("Utilisateur authentifié : {}", user_id);
                let res = self.service.call(req).await?;
                Ok(res)
            } else {
                println!("Accès refusé : utilisateur non authentifié.");
                let res = HttpResponse::Unauthorized()
                    .body("Accès non autorisé. Veuillez vous authentifier.")
                    .map_into_boxed_body();
                let res = req.into_response(res);
                Ok(res)
            }
        })
    }
}
