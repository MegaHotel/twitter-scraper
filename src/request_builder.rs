use reqwest::header::{HeaderMap, HeaderValue};

pub struct RequestConfig {
    pub path_query: Vec<(String, String)>,
    pub headers: HeaderMap,
}

pub fn build_request_config(
    headers_tuples: &[(&'static str, &'static str); 2],
    query: String,
    cursor: Option<String>,
) -> RequestConfig {
    RequestConfig {
        path_query: build_path_query(query, cursor),
        headers: build_headers(headers_tuples),
    }
}

fn build_path_query(query: String, cursor: Option<String>) -> Vec<(String, String)> {
    let mut default_query: Vec<(String, String)> = vec![
        ("include_profile_interstitial_type".to_string(), "1".to_string()),
        ("include_blocking".to_string(), "1".to_string()),
        ("include_blocked_by".to_string(), "1".to_string()),
        ("include_followed_by".to_string(), "1".to_string()),
        ("include_want_retweets".to_string(), "1".to_string()),
        ("include_mute_edge".to_string(), "1".to_string()),
        ("include_can_dm".to_string(), "1".to_string()),
        ("include_can_media_tag".to_string(), "1".to_string()),
        ("include_ext_has_nft_avatar".to_string(), "1".to_string()),
        ("skip_status".to_string(), "1".to_string()),
        ("cards_platform".to_string(), "Web-12".to_string()),
        ("include_cards".to_string(), "1".to_string()),
        ("include_ext_alt_text".to_string(), "true".to_string()),
        ("include_quote_count".to_string(), "true".to_string()),
        ("include_reply_count".to_string(), "1".to_string()),
        ("tweet_mode".to_string(), "extended".to_string()),
        ("include_ext_collab_control".to_string(), "true".to_string()),
        ("include_entities".to_string(), "true".to_string()),
        ("include_user_entities".to_string(), "true".to_string()),
        ("include_ext_media_color".to_string(), "true".to_string()),
        ("include_ext_media_availability".to_string(), "true".to_string()),
        ("include_ext_sensitive_media_warning".to_string(), "true".to_string()),
        ("include_ext_trusted_friends_metadata".to_string(), "true".to_string()),
        ("send_error_codes".to_string(), "true".to_string()),
        ("simple_quoted_tweet".to_string(), "true".to_string()),
        ("count".to_string(), "20".to_string()),
        ("query_source".to_string(), "".to_string()),
        ("pc".to_string(), "1".to_string()),
        ("spelling_corrections".to_string(), "1".to_string()),
        ("include_ext_edit_control".to_string(), "false".to_string()),
        ("ext".to_string(), "mediaStats,highlightedLabel,hasNftAvatar,voiceInfo,enrichments,superFollowMetadata,unmentionInfo,collab_control,vibe".to_string()),

    ];
    default_query.push(("q".to_string(), query));
    if let Some(cursor_id) = cursor {
        default_query.push(("cursor".to_string(), cursor_id));
    }
    default_query
}

fn build_headers(headers_tuples: &[(&'static str, &'static str); 2]) -> HeaderMap {
    let mut headers_map: HeaderMap = HeaderMap::new();
    headers_tuples.iter().for_each(|header_tuple| {
        headers_map.insert(
            header_tuple.0,
            HeaderValue::from_str(header_tuple.1).unwrap(),
        );
    });
    headers_map
}
