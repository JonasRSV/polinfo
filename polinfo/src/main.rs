use dotenv::dotenv;
use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse, Error};
use actix_cors::Cors;
use deadpool_postgres::{Client, Pool};
use tokio_postgres::NoTls;
use actix_web::middleware::Logger;
use std::sync::{Arc, RwLock};
use log::info;

mod core;
mod db;
mod textprocessing;
mod prefixsum;
mod cache;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn get_anforanden(
    req: web::Json<core::AnforandeRequest>, 
    prefixsum_cache: web::Data<Arc<RwLock<cache::KVCache<String, core::PrefixSum>>>>) 
    -> Result<HttpResponse, Error> {

        let anforende_request: core::AnforandeRequest = req.into_inner();

        let cache_key: String;
        let cache_result: Option<core::PrefixSum>;

        // Create scope for read lock
        {
            let cache = prefixsum_cache.read().unwrap();

            cache_key = String::from(anforende_request.affiliation.clone());
            cache_result = cache.get(&cache_key)
                .map(|v| v.clone());
            }

        match cache_result {
            Some(prefixsum) => Ok(HttpResponse::Ok().json2(&prefixsum)),
            None => {
                println!("Cache Miss - {}", cache_key);


                Ok(HttpResponse::Ok().json(
                        serde_json::json!({
                                "error": format!("Invalid affiliation {}", cache_key)
                            })
                        )
                    )
            }
        }
}

// Populate cache befoe launching webserver instead of on request
async fn prefetch_prefixsum_cache(
    db_pool: Pool,
    prefixsum_factory: Arc<prefixsum::PrefixSumFactory>,
    cache: Arc<RwLock<cache::KVCache<String, core::PrefixSum>>>) {
    let mut c = cache.write().unwrap();

    let db_client: Client = db_pool.get().await.unwrap();

    for affiliation in ["SD", "V", "S", "MP", "L", "KD", "M", "C"].iter() {
        let affiliation_string = String::from(affiliation.to_owned());
        info!("Calculating prefixsum for {}", affiliation_string);

        let mut tts = db::get_anforande_texttimes(&db_client, 
                 Some(affiliation_string.clone().into())).await.unwrap();

        let p = prefixsum_factory.from_texttimes(&mut tts);

        c.add(affiliation_string.to_owned(), p);
    }

        let mut tts = db::get_anforande_texttimes(&db_client, 
            None).await.unwrap();
        let p = prefixsum_factory.from_texttimes(&mut tts);

        c.add(core::Affiliation::ALL.into(), p);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config = core::Config::from_env().unwrap();
    let pool = config.pg.create_pool(NoTls).unwrap();
    let prefixsum_factory = Arc::new(prefixsum::PrefixSumFactory::new().unwrap());

    let prefixsum_cache: Arc<RwLock<cache::KVCache<String, core::PrefixSum>>> = Arc::new(
        RwLock::new(
            cache::KVCache::new(
                Some(cache::PersistConfig::new("anforande_cache.json".to_owned())))));

    if prefixsum_cache.read().unwrap().is_empty() {
        info!("filling anforande prefixsums cache");
        prefetch_prefixsum_cache(
            pool.clone(), 
            prefixsum_factory.clone(), 
            prefixsum_cache.clone()).await;
    } else {
        info!("anforande prefixsums cache was loaded from disk");
    }
    // populate cache on server start

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

