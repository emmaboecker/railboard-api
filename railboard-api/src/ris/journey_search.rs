use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::{NaiveDate, TimeZone};
use chrono_tz::Europe::Berlin;
use serde::Deserialize;

use ris_client::journey_search::RisJourneySearchElement;

use crate::{cache::{CachableObject, Cache}, error::RailboardResult, SharedState};

#[derive(Deserialize)]
pub struct JounreySearchPath {
    pub number: String,
    pub category: String,
}

#[derive(Deserialize)]
pub struct JounreySearchQuery {
    pub date: Option<NaiveDate>,
}

#[utoipa::path(
    get,
    path = "/ris/v1/journey_search/{category}/{number}",
    params(
        ("category" = String, Path, description = "The category of this Train (e.g. ICE, IC, RE, VIA, ...)"),
        ("number" = String, Path, description = "The number of this Train (e.g. for ICE 2929 it would be 2929 and for RE 1 it could be 4570)"), 
        ("date" = Option<String>, Query, description = "The date this train is running and should be searched for (e.g. 2023-01-25)")
    ),
    tag = "Ris",
    responses(
        (status = 200, description = "The requested Journey Details", body = [RisJourneySearchElement]),
        (status = 400, description = "The Error returned by Ris, will be the Ris Domain with UnderlyingApiError Variant 3 or 4", body = RailboardApiError),
        (status = 500, description = "The Error returned if the request or deserialization fails, will be domain Request", body = RailboardApiError)
    )
)]
pub async fn journey_search(
    Path(path): Path<JounreySearchPath>,
    Query(query): Query<JounreySearchQuery>,
    State(state): State<Arc<SharedState>>,
) -> RailboardResult<Json<Vec<RisJourneySearchElement>>> {
    let category = path.category;
    let number = path.number;
    let date = query.date;

    if let Some(cached) = state
        .cache
        .get_from_id(&format!(
            "ris.journey-search.{}.{}.{}",
            &category,
            &number,
            &date
                .map(|date| date.format("%Y-%m-%d").to_string())
                .unwrap_or_else(|| Berlin
                    .from_utc_datetime(&chrono::Utc::now().naive_utc())
                    .format("%Y-%m-%d")
                    .to_string())
        ))
        .await
    {
        return Ok(Json(cached));
    }

    let response = state
        .ris_client
        .journey_search(&category, &number, date)
        .await?;

    {
        let response = response.clone();
        tokio::spawn(async move {
            let cache = state.cache.clone();
            let category = category.clone();
            let number = number.clone();
            (category, number, response)
                .insert_to_cache(&cache, None)
                .await
        });
    }

    Ok(Json(response.journeys))
}
