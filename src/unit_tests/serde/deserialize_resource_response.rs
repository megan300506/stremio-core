use crate::types::addon::ResourceResponse::{Metas, MetasDetailed};
use crate::types::resource::{
    Link, MetaItem, MetaItemBehaviorHints, MetaItemPreview, SeriesInfo, Video,
};
use chrono::prelude::TimeZone;
use chrono::Utc;
use url::Url;

#[test]
fn deserialize_resource_response_metas() {
    let metas_vec = vec![MetaItemPreview {
        id: "id".to_owned(),
        type_name: "type_name".to_owned(),
        name: "name".to_owned(),
        poster: None,
        logo: None,
        description: None,
        release_info: None,
        runtime: None,
        released: None,
        poster_shape: Default::default(),
        trailer_streams: vec![],
        behavior_hints: MetaItemBehaviorHints {
            default_video_id: None,
            featured_video_id: None,
            has_scheduled_videos: false,
        },
    }];
    let metas_json = r#"{"metas":[{"id":"id","type":"type_name","name":"name","poster":null,"logo":null,"description":null,"releaseInfo":null,"runtime":null,"released":null,"posterShape":"poster","trailer_streams":[],"behaviorHints":{"defaultVideoId":null,"featuredVideoId":null}}]}"#;
    let metas_deserialize = serde_json::from_str(&metas_json).unwrap();
    match metas_deserialize {
        Metas { metas } => assert_eq!(metas, metas_vec, "metas deserialized successfully"),
        _ => panic!("failed getting metas"),
    };
}

#[test]
fn deserialize_resource_response_metas_detailed() {
    let metas_detailed_vec = vec![MetaItem {
        id: "id".to_owned(),
        type_name: "type_name".to_owned(),
        name: "name".to_owned(),
        poster: None,
        background: None,
        logo: None,
        popularity: None,
        description: None,
        release_info: None,
        runtime: None,
        released: None,
        poster_shape: Default::default(),
        videos: vec![Video {
            id: "id".to_owned(),
            title: "title".to_owned(),
            released: Some(Utc.ymd(2020, 1, 1).and_hms_milli(0, 0, 0, 0)),
            overview: None,
            thumbnail: None,
            streams: vec![],
            series_info: Some(SeriesInfo {
                season: 1,
                episode: 1,
            }),
            trailer_streams: vec![],
        }],
        links: vec![Link {
            name: "name".to_owned(),
            category: "category".to_owned(),
            url: Url::parse("https://url").unwrap(),
        }],
        trailer_streams: vec![],
        behavior_hints: Default::default(),
    }];
    let metas_detailed_json = r#"{"metasDetailed":[{"id":"id","type":"type_name","name":"name","poster":null,"background":null,"logo":null,"popularity":null,"description":null,"releaseInfo":null,"runtime":null,"released":null,"posterShape":"poster","videos":[{"id":"id","title":"title","released":"2020-01-01T00:00:00Z","overview":null,"thumbnail":null,"streams":[],"season":1,"episode":1,"trailer_streams":[]}],"links":[{"name":"name","category":"category","url":"https://url/"}],"trailer_streams":[],"behaviorHints":{"defaultVideoId":null,"featuredVideoId":null}}]}"#;
    let metas_detailed_deserialize = serde_json::from_str(&metas_detailed_json).unwrap();
    match metas_detailed_deserialize {
        MetasDetailed { metas_detailed } => assert_eq!(
            metas_detailed, metas_detailed_vec,
            "metas detailed deserialized successfully"
        ),
        _ => panic!("failed getting metas detailed"),
    };
}
