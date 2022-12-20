mod mock_error;

use pact_broker_api::client::{self, BrokerClient};
use pact_broker_models::{contract::Contract, pacts::Pacts};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

fn setup_client(uri: &str) -> BrokerClient {
    client::Builder::new()
        .base_url(uri)
        .unwrap()
        .build()
        .unwrap()
}

#[tokio::test]
async fn should_return_latest_pact() {
    let mock_server = MockServer::start().await;
    let latest: Pacts = serde_json::from_str(
        &include_str!("resources/pacts-latest.json").replace("{{host}}", &mock_server.uri()),
    )
    .unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&latest);
    Mock::given(method("GET"))
        .and(path("/pacts/latest"))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    mock_error::setup_error_handler(&mock_server, "GET on /pacts/latest was not received").await;
    // let mock_server = setup_api(template).await;

    let broker_client = setup_client(&mock_server.uri());
    let got = broker_client.pacts().latest().await;

    assert!(
        got.is_ok(),
        "expected successful result, got error: {:#?}",
        got
    );

    let Pacts { pacts, .. } = got.unwrap();
    assert_eq!(pacts.len(), 2);

    let first = pacts.first().unwrap();
    assert_eq!(first.embedded.consumer.name, "ms.idx1");
    assert_eq!(first.embedded.provider.name, "ms.idx2");

    let second = pacts.get(1).unwrap();
    assert_eq!(second.embedded.consumer.name, "ms.idx2");
    assert_eq!(second.embedded.provider.name, "ms.idx3");
}

#[tokio::test]
async fn should_return_latest_interaction() {
    let mock_server = MockServer::start().await;
    let latest: Contract = serde_json::from_str(
        &include_str!("resources/pacts-interactions.json").replace("{{host}}", &mock_server.uri()),
    )
    .unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&latest);
    Mock::given(method("GET"))
        .and(path(
            "/pacts/provider/provider_name/consumer/consumer_name/latest",
        ))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    mock_error::setup_error_handler(
        &mock_server,
        "GET on /pacts/provider/provider_name/consumer/consumer_name/latest was not received",
    )
    .await;

    let broker_client = setup_client(&mock_server.uri());
    let got = broker_client
        .pacts()
        .contract("provider_name", "consumer_name", None)
        .await;

    assert!(
        got.is_ok(),
        "expected successful result, got error: {:#?}",
        got
    );

    let Contract {
        consumer,
        provider,
        interactions,
        ..
    } = got.unwrap();

    assert_eq!(consumer.name, "consumer_name");
    assert_eq!(provider.name, "provider_name");
    let interactions = interactions.unwrap();
    assert_eq!(interactions.len(), 1);
    let interation = interactions.first().unwrap();
    assert_eq!(interation.request.method, "GET");
    assert_eq!(interation.request.path, "/api/profiles");
}
