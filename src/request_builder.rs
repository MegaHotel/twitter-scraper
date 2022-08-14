use reqwest::header::{HeaderMap, HeaderValue};

pub struct RequestConfig<'a> {
    pub path_query: Vec<(&'a str, &'a str)>,
    pub headers: HeaderMap,
}

pub fn build_request_config<'a>(
    headers_tuples: &[(&'static str, &'static str)],
    query: &'a str,
    cursor: Option<&'a str>,
) -> RequestConfig<'a> {
    RequestConfig {
        path_query: build_path_query(query, cursor),
        headers: build_headers(headers_tuples),
    }
}

fn build_path_query<'a>(query: &'a str, cursor: Option<&'a str>) -> Vec<(&'a str, &'a str)> {
    let mut default_query: Vec<(&str, &str)> = vec![
        ("include_profile_interstitial_type", "1"),
        ("include_blocking", "1"),
        ("include_blocked_by", "1"),
        ("include_followed_by", "1"),
        ("include_want_retweets", "1"),
        ("include_mute_edge", "1"),
        ("include_can_dm", "1"),
        ("include_can_media_tag", "1"),
        ("include_ext_has_nft_avatar", "1"),
        ("skip_status", "1"),
        ("cards_platform", "Web-12"),
        ("include_cards", "1"),
        ("include_ext_alt_text", "true"),
        ("include_quote_count", "true"),
        ("include_reply_count", "1"),
        ("tweet_mode", "extended"),
        ("include_ext_collab_control", "true"),
        ("include_entities", "true"),
        ("include_user_entities", "true"),
        ("include_ext_media_color", "true"),
        ("include_ext_media_availability", "true"),
        ("include_ext_sensitive_media_warning", "true"),
        ("include_ext_trusted_friends_metadata", "true"),
        ("send_error_codes", "true"),
        ("simple_quoted_tweet", "true"),
        ("count", "20"),
        ("query_source", ""),
        ("pc", "1"),
        ("spelling_corrections", "1"),
        ("include_ext_edit_control", "false"),
        ("ext", "mediaStats,highlightedLabel,hasNftAvatar,voiceInfo,enrichments,superFollowMetadata,unmentionInfo,collab_control,vibe"),

    ];
    default_query.push(("q", query));
    if let Some(cursor_id) = cursor {
        default_query.push(("cursor", cursor_id));
    }
    default_query
}

fn build_headers(headers_tuples: &[(&'static str, &'static str)]) -> HeaderMap {
    let mut headers_map: HeaderMap = HeaderMap::new();
    headers_tuples.iter().for_each(|header_tuple| {
        headers_map.insert(
            header_tuple.0,
            HeaderValue::from_str(header_tuple.1).unwrap(),
        );
    });
    headers_map
}
