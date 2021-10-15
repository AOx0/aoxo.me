// use actix_service::Service;

use futures::future::ok;

use actix_web::dev::{Service, Transform};
use actix_web::{
    dev::ServiceRequest,
    dev::ServiceResponse,
    http::header::{HeaderValue},
    Error,
};
// use actix_http::http::header::Expires;
use futures::{
    future::{ Ready},
    Future,
};

use std::pin::Pin;
use std::task::{Context, Poll};

pub struct MyCacheInterceptor;

impl<S, B> Transform<S> for MyCacheInterceptor
    where
        S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = MyCacheInterceptorMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(MyCacheInterceptorMiddleware { service })
    }
}

pub struct MyCacheInterceptorMiddleware<S> {
    service: S,
}

impl<S, B> Service for MyCacheInterceptorMiddleware<S>
    where
        S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);

        Box::pin(async move {
            let mut res = fut.await?;
            let headers = res.headers_mut();
            headers.append(
                actix_web::http::header::CACHE_CONTROL,
                HeaderValue::from_str("no-cache").unwrap()
            );
            headers.append(
                actix_web::http::header::CACHE_CONTROL,
                HeaderValue::from_str("no-store").unwrap()
            );
            return Ok(res);
        })
    }
}