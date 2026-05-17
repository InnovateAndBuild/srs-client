use srs_client::{SrsClient, SrsClientError, SrsClientResp, SrsClientRespData};
use std::env;
use tokio;

// #[tokio::test]
// async fn test_kickoff_client() -> Result<(), Box<dyn std::error::Error>> {
//     let srs_http_api_url =
//         env::var("SRS_HTTP_API_URL").expect("SRS_HTTP_API_URL not set");
//     let client = SrsClient::build(&srs_http_api_url)?;
//     let result: Result<SrsClientResp, SrsClientError> =
//         client.kickoff_client("21233").await;
//     assert!(result.is_ok());
//     Ok(())
// }

#[tokio::test]
async fn test_get_version() -> Result<(), Box<dyn std::error::Error>> {
    let srs_http_api_url = env::var("SRS_HTTP_API_URL").expect("SRS_HTTP_API_URL not set");
    let client = SrsClient::build(&srs_http_api_url)?;
    let result: Result<SrsClientResp, SrsClientError> = client.get_version().await;
    assert!(result.is_ok());
    Ok(())
}

#[tokio::test]
async fn test_get_vhosts() -> Result<(), Box<dyn std::error::Error>> {
    let srs_http_api_url = env::var("SRS_HTTP_API_URL").expect("SRS_HTTP_API_URL not set");
    let client = SrsClient::build(&srs_http_api_url)?;
    let result: Result<SrsClientResp, SrsClientError> = client.get_vhosts().await;
    assert!(result.is_ok());
    Ok(())
}

#[tokio::test]
async fn test_get_streams() -> Result<(), Box<dyn std::error::Error>> {
    let srs_http_api_url = env::var("SRS_HTTP_API_URL").expect("SRS_HTTP_API_URL not set");
    let client = SrsClient::build(&srs_http_api_url)?;
    let result: Result<SrsClientResp, SrsClientError> = client.get_streams().await;
    assert!(result.is_ok());
    Ok(())
}

#[tokio::test]
async fn test_get_clients() -> Result<(), Box<dyn std::error::Error>> {
    let srs_http_api_url = env::var("SRS_HTTP_API_URL").expect("SRS_HTTP_API_URL not set");
    let client = SrsClient::build(&srs_http_api_url)?;
    let result: Result<SrsClientResp, SrsClientError> = client.get_clients().await;
    assert!(result.is_ok());
    Ok(())
}

#[tokio::test]
async fn test_get_features() -> Result<(), Box<dyn std::error::Error>> {
    let srs_http_api_url = env::var("SRS_HTTP_API_URL").expect("SRS_HTTP_API_URL not set");
    let client = SrsClient::build(&srs_http_api_url)?;
    let result: Result<SrsClientResp, SrsClientError> = client.get_features().await;
    assert!(result.is_ok());
    Ok(())
}

#[tokio::test]
async fn test_get_rusages() -> Result<(), Box<dyn std::error::Error>> {
    let srs_http_api_url = env::var("SRS_HTTP_API_URL").expect("SRS_HTTP_API_URL not set");
    let client = SrsClient::build(&srs_http_api_url)?;
    let result: Result<SrsClientResp, SrsClientError> = client.get_rusages().await;
    assert!(result.is_ok());
    Ok(())
}
#[tokio::test]
async fn test_get_self_proc_stats() -> Result<(), Box<dyn std::error::Error>> {
    let srs_http_api_url = env::var("SRS_HTTP_API_URL").expect("SRS_HTTP_API_URL not set");
    let client = SrsClient::build(&srs_http_api_url)?;
    let result: Result<SrsClientResp, SrsClientError> = client.get_self_proc_stats().await;
    assert!(result.is_ok());
    Ok(())
}
#[tokio::test]
async fn test_get_system_proc_stats() -> Result<(), Box<dyn std::error::Error>> {
    let srs_http_api_url = env::var("SRS_HTTP_API_URL").expect("SRS_HTTP_API_URL not set");
    let client = SrsClient::build(&srs_http_api_url)?;
    let result: Result<SrsClientResp, SrsClientError> = client.get_system_proc_stats().await;
    assert!(result.is_ok());
    Ok(())
}
#[tokio::test]
async fn test_get_meminfos() -> Result<(), Box<dyn std::error::Error>> {
    let srs_http_api_url = env::var("SRS_HTTP_API_URL").expect("SRS_HTTP_API_URL not set");
    let client = SrsClient::build(&srs_http_api_url)?;
    let result: Result<SrsClientResp, SrsClientError> = client.get_meminfos().await;
    assert!(result.is_ok());
    Ok(())
}

#[test]
fn test_active_stream_response_with_media_metadata() -> Result<(), Box<dyn std::error::Error>> {
    let response: SrsClientResp = serde_json::from_str(include_str!("fixtures/srs-streams.json"))?;

    match response.data {
        SrsClientRespData::Streams { streams } => {
            assert_eq!(streams.len(), 2);

            let video = streams[0].video.as_ref().expect("video metadata");
            assert_eq!(video.codec, "H264");
            assert_eq!(video.profile, "High");
            assert_eq!(video.level, "3.2");
            assert_eq!(video.width, 1280);
            assert_eq!(video.height, 720);

            let audio = streams[0].audio.as_ref().expect("audio metadata");
            assert_eq!(audio.codec, "AAC");
            assert_eq!(audio.sample_rate, 44100);
            assert_eq!(audio.channel, 2);
            assert_eq!(audio.profile, "LC");
        }
        _ => panic!("expected streams response"),
    }

    Ok(())
}

#[test]
fn test_single_resource_responses() -> Result<(), Box<dyn std::error::Error>> {
    let response: SrsClientResp = serde_json::from_str(include_str!("fixtures/srs-stream.json"))?;
    match response.data {
        SrsClientRespData::Stream { stream } => {
            assert_eq!(stream.id, "vid-c99a4wx");
            assert_eq!(stream.publish.cid.as_deref(), Some("nf14l8c0"));
        }
        _ => panic!("expected stream response"),
    }

    let response: SrsClientResp = serde_json::from_str(include_str!("fixtures/srs-client.json"))?;
    match response.data {
        SrsClientRespData::Client { client } => {
            assert_eq!(client.id, "206sj057");
            assert_eq!(client.r#type, "fmle-publish");
        }
        _ => panic!("expected client response"),
    }

    let response: SrsClientResp = serde_json::from_str(include_str!("fixtures/srs-vhost.json"))?;
    match response.data {
        SrsClientRespData::Vhost { vhost } => {
            assert_eq!(vhost.id, "vid-ibe77pd");
            assert_eq!(vhost.hls.fragment, Some(1.0));
        }
        _ => panic!("expected vhost response"),
    }

    Ok(())
}

#[test]
fn test_vhost_list_response() -> Result<(), Box<dyn std::error::Error>> {
    let response: SrsClientResp = serde_json::from_str(include_str!("fixtures/srs-vhosts.json"))?;

    match response.data {
        SrsClientRespData::Vhosts { vhosts } => {
            assert_eq!(vhosts.len(), 1);
            assert_eq!(vhosts[0].hls.fragment, Some(1.0));
        }
        _ => panic!("expected vhosts response"),
    }

    Ok(())
}
