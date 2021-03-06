use deadpool_postgres::Client;
use deadpool_postgres::PoolError;
use tokio_postgres::error::Error as PGError;
use actix_web::{ResponseError, HttpResponse};
use crate::core;
use derive_more::{Display, From};

#[derive(Debug, Display, From)]
pub enum DBError {
    NotFound,
    PGError(PGError),
    PoolError(PoolError)
}

impl std::error::Error for DBError {}

impl ResponseError for DBError { 
    fn error_response(&self) -> HttpResponse {
        match *self {
            DBError::NotFound => HttpResponse::NotFound().finish(),
            DBError::PoolError(ref err) => {
                HttpResponse::InternalServerError().body(err.to_string())
            }
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}

pub async fn get_anforande_texttimes(client: &Client, affiliation: Option<String>) 
    -> Result<Vec<core::TextTime>, DBError> { 

        let statement = match affiliation {
            None => client.prepare("SELECT content, time FROM anforande;").await.unwrap(),
            Some(a) => {
                let mut _statement = include_str!(
                    "../sql/anforande_by_affiliation.psql").to_string();
                _statement = _statement.replace("$1", 
                    format!("'{}'", a).as_str());

                client.prepare(&_statement).await.unwrap()
            }
        };

        // postgresql query cannot accept strings as enums and I cannot declare the psql enum in
        // rust so the query formatting is unusable in my case. 
        let db_response = client
            .query(
                &statement,
                &[ ],
            )
            .await;


        let response = match db_response {
                Ok(res) => Ok(res
                    .iter()
                    .map(|row| core::TextTime {
                        content: row.get("content"),
                        time: row.get("time")                    
                    })
                    .collect::<Vec<core::TextTime>>()),
                Err(err) => Err(DBError::from(err))

        };

        response

}
