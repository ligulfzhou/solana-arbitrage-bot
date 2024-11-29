use reqwest::Client;
use serde::{Deserialize, Serialize};

pub async fn get_json<T: for<'de> Deserialize<'de>, Q: Serialize>(url: &str) -> anyhow::Result<T> {
    let client = Client::new();

    let res = client
        .get(url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::ACCEPT, "application/json")
        .send()
        .await?
        .json::<T>()
        .await?;

    Ok(res)
}

pub async fn get_json_with_params<T: for<'de> Deserialize<'de>>(
    url: &str,
    query: &[(String, String)],
) -> anyhow::Result<T> {
    let client = Client::new();

    let res = client
        .get(url)
        .query(&query)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::ACCEPT, "application/json")
        .send()
        .await?
        .json::<T>()
        .await?;

    Ok(res)
}


pub async fn post_json<Res: for<'de> Deserialize<'de>, Body: Serialize>(
    url: &str,
    body: Body,
) -> anyhow::Result<Res> {
    let client = Client::new();

    let res = client
        .post(url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::ACCEPT, "application/json")
        .json(&body)
        .send()
        .await?
        .json::<Res>()
        .await?;

    Ok(res)
}


pub async fn post_json_raw<Body: Serialize>(
    url: &str,
    body: Body,
) -> anyhow::Result<String> {
    let client = Client::new();

    let res = client
        .post(url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::ACCEPT, "application/json")
        .json(&body)
        .send()
        .await?
        .text()
        .await?;

    Ok(res)
}
