use actix_service::Transform;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse};
use actix_web::{Error, http, HttpResponse};
use futures::future::{ok, Ready, Either};

pub struct RService<S> {
    service: S,
}

pub struct Redirect {}

impl Redirect {
    pub fn new() -> Self {
        Self { /* fields */ }
    }
}


impl<S> Transform<S, ServiceRequest> for Redirect
    where
        S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error>,
        S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Transform = RService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RService {service})
    }
}

impl<S> Service<ServiceRequest> for RService<S>
    where
        S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
        S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future =  Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

    actix_service::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        if req.connection_info().scheme() == "https" {
            Either::Left(self.service.call(req))
        } else {
            let host = req.connection_info().host().to_owned();
            let uri = req.uri().to_owned();
            let url = format!("https://{}{}", host, uri);

            Either::Right(ok(req.into_response(
                HttpResponse::TemporaryRedirect()
                    .insert_header((http::header::LOCATION, url))
                    .finish()
            )))

        }
    }
}

