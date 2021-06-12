use dotenv::dotenv;
use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse, Error};
use actix_cors::Cors;
use deadpool_postgres::{Client, Pool};
use tokio_postgres::NoTls;
use actix_web::middleware::Logger;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

mod core;
mod db;
mod textprocessing;
mod prefixsum;

struct AnforandeCache {
    map: Mutex<HashMap<String, core::PrefixSum>>
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

/// API endpoint for getting the prefix sum of anforanden.
///
/// # Arguments
///
/// * `req` - actix web request
/// * `database_pool` - database client
///
/// # Errors
///
async fn get_anforanden(
    req: web::Json<core::AnforandeRequest>, 
    database_pool: web::Data<Pool>,
    prefixsum_factory: web::Data<Arc<prefixsum::PrefixSumFactory>>,
    prefixsum_cache: web::Data<Arc<AnforandeCache>>) 
    -> Result<HttpResponse, Error> {

        let mut cache = prefixsum_cache.map.lock().unwrap();
        let anforende_request: core::AnforandeRequest = req.into_inner();

        let cache_key = String::from(anforende_request.affiliation.clone());
        let cache_result = cache.get(&cache_key);

        match cache_result {
            Some(prefixsum) => Ok(HttpResponse::Ok().json2(&prefixsum)),
            None => {
                println!("Cache Miss - {}", cache_key);

                let client: Client = database_pool.get().await.map_err(db::DBError::PoolError)?;
                let mut anforanden = db::get_anforanden(&client, &anforende_request).await?;

                let prefixsum = prefixsum_factory.from_anforanden(&mut anforanden);

                cache.insert(cache_key, prefixsum.clone());
                Ok(HttpResponse::Ok().json2(&prefixsum))
            }
        }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config = core::Config::from_env().unwrap();
    let pool = config.pg.create_pool(NoTls).unwrap();
    let prefixsum_factory = Arc::new(prefixsum::PrefixSumFactory::new().unwrap());

    let prefixsum_cache = Arc::new(AnforandeCache {
        map: Mutex::new(HashMap::<String, core::PrefixSum>::new())
    });


    let server = HttpServer::new(move || {
        App::new()
            .wrap(Cors::new()
                .allowed_origin("http://localhost:5000")
                .allowed_origin("http://0.0.0.0:5000")
                .finish())
            .wrap(Logger::default())
            .data(pool.clone())
            .data(prefixsum_cache.clone())
            .data(prefixsum_factory.clone())
            .route("/", web::get().to(greet))
            .route("/anforanden", web::post().to(get_anforanden))
    })
    .bind(config.server_addr.clone())?
        .run();

    server.await
}

