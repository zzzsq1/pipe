use crate::data::Pool;
use crate::github::GitHubClient;
use crate::models::{Tenant, UserTenant};

use crate::RequestId;
use actix_http::body::Body;
use actix_session::Session;
use actix_web::error::Error as AWError;
use actix_web::{get, post, put, web, HttpResponse};
use base58::ToBase58;
use log::info;
use rand::{thread_rng, Rng};
use reqwest::Client;
use serde::Deserialize;
use std::env;

pub const TENANT_ID_KEY: &str = "tenant_id";
pub const STATE_KEY: &str = "state";

#[get("/user")]
pub async fn user(
    session: Session,
    client: web::Data<GitHubClient>,
    pool: web::Data<Pool>,
    request_id: RequestId,
) -> std::result::Result<HttpResponse, AWError> {
    info!("{} [Get User]", request_id);
    if let Some(tenant_id) = session.get::<i64>(TENANT_ID_KEY)? {
        if let Some(tenant) = pool.find_tenant_by_id(tenant_id).await? {
            info!("{} [Get User] {}", request_id, tenant.id);
            return Ok(HttpResponse::Ok().json(UserTenant::from(tenant)));
        };
    }

    let state = new_csrf_token();
    let url = client.authorize_url(&state);
    session.set(STATE_KEY, state)?;
    Ok(HttpResponse::Unauthorized()
        .header("Location", url.to_string())
        .body(Body::Empty))
}

fn new_csrf_token() -> String {
    let random_bytes: [u8; 16] = thread_rng().gen::<[u8; 16]>();
    random_bytes.to_base58()
}

#[derive(Deserialize)]
pub struct Callback {
    code: String,
    state: String,
}

#[derive(Deserialize)]
pub struct LoginCallback {
    access_token: String,
}

// It's for testing purpose.
#[post("/login")]
pub async fn login(
    session: Session,
    http_client: web::Data<Client>,
    github_client: web::Data<GitHubClient>,
    pool: web::Data<Pool>,
    web::Query(login): web::Query<LoginCallback>,
) -> std::result::Result<HttpResponse, AWError> {
    let access_token = login.access_token;
    let github_user = github_client.get_user(&http_client, &access_token).await?;
    match pool.find_tenant_by_github_id(github_user.id).await? {
        Some(tenant) => session.set(TENANT_ID_KEY, tenant.id)?,
        None => {
            let app_id: i64 = thread_rng().gen();
            let tenant = Tenant::new(app_id, github_user.login, github_user.id);
            let tenant = pool.insert_tenant(tenant).await?;
            session.set(TENANT_ID_KEY, tenant.id)?
        }
    }
    Ok(HttpResponse::Found()
        .header("Location", "/#/user")
        .body(Body::Empty))
}

#[get("/callback")]
pub async fn callback(
    session: Session,
    github_client: web::Data<GitHubClient>,
    http_client: web::Data<Client>,
    pool: web::Data<Pool>,
    web::Query(callback): web::Query<Callback>,
    request_id: RequestId,
) -> std::result::Result<HttpResponse, AWError> {
    match session.get::<String>(STATE_KEY)? {
        Some(state) if state == callback.state => {
            let access_token = github_client
                .exchange_code(&http_client, &callback.code)
                .await?;
            let github_user = github_client.get_user(&http_client, &access_token).await?;

            match pool.find_tenant_by_github_id(github_user.id).await? {
                Some(tenant) => {
                    info!("{} [Login] {}", request_id, github_user.login);
                    session.set(TENANT_ID_KEY, tenant.id)?
                }
                None => {
                    let app_id: i64 = thread_rng().gen();
                    info!("{} [Register] {}", request_id, github_user.login);
                    let tenant = Tenant::new(app_id, github_user.login, github_user.id);
                    let tenant = pool.insert_tenant(tenant).await?;
                    session.set(TENANT_ID_KEY, tenant.id)?
                }
            }
            Ok(HttpResponse::Found()
//                 .header(
//                     "Location",
//                     format!("{}/#/user", env::var("pipehub_domain_web").unwrap()),
//                 )
                .header("Location", "/#/user")
                .body(Body::Empty))
        }
        _ => Ok(HttpResponse::Found()
//             .header(
//                 "Location",
//                 format!("{}/", env::var("pipehub_domain_web").unwrap()),
//             )
            .header("Location", "/#/user")
            .body(Body::Empty)),
    }
}

#[post("/user/reset_key")]
pub async fn reset_key(
    session: Session,
    pool: web::Data<Pool>,
) -> std::result::Result<HttpResponse, AWError> {
    if let Some(tenant_id) = session.get::<i64>(TENANT_ID_KEY)? {
        if let Some(tenant) = pool.find_tenant_by_id(tenant_id).await? {
            let new_tenant = Tenant {
                id: tenant.id,
                app_id: thread_rng().gen(),
                github_login: tenant.github_login,
                github_id: tenant.github_id,
                block_list: tenant.block_list,
                captcha: tenant.captcha,
            };
            pool.update_tenant(new_tenant.clone()).await?;

            return Ok(HttpResponse::Ok().json(UserTenant::from(new_tenant)));
        };
    }

    Ok(HttpResponse::Unauthorized().body(Body::Empty))
}

#[put("/user")]
pub async fn update(
    session: Session,
    pool: web::Data<Pool>,
    web::Json(new_tenant): web::Json<Tenant>,
    request_id: RequestId,
) -> std::result::Result<HttpResponse, AWError> {
    info!("{} [Update User]", request_id);
    if let Some(tenant_id) = session.get::<i64>(TENANT_ID_KEY)? {
        if let Some(tenant) = pool.find_tenant_by_id(tenant_id).await? {
            info!("{} [Update User] {}", request_id, tenant.id);
            let new_tenant = Tenant {
                id: tenant.id,
                app_id: tenant.app_id,
                github_login: tenant.github_login,
                github_id: tenant.github_id,
                block_list: new_tenant.block_list,
                captcha: new_tenant.captcha,
            };
            pool.update_tenant(new_tenant.clone()).await?;

            return Ok(HttpResponse::Ok().json(UserTenant::from(new_tenant)));
        };
    }

    Ok(HttpResponse::Unauthorized().body(Body::Empty))
}
