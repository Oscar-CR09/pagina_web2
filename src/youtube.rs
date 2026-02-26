use serde::Deserialize;
use gloo_net::http::Request;
use crate::env::API_KEY;

pub  async fn search_youtube(text_to_search:String)->Result<VideoItem,gloo_net::Error>{
    //todo llamar api de yt
    let query_url:String=format!{
       " https://www.googleapis.com/youtube/v3/search?part=id%2Csnippet&q={}",
       text_to_search
    };
     
    let mut auth:=String::from("Bearer");
    auth.push_str(API_KEY);

    let response=Request::get(&query_url)
    .header(key:"Authorization",value:&auth)
    .send()
    .await?;

    let search_result:SearchResult=response.json::<SearchResult>().await?;
    let empty_video: = build_empty_video();
    let video:&VideoItem = match search_result.items.first() {
        Some(video)=>video,
        None=> &empty_video,        
    };

    web_sys::console::log_1(&text_to_search.into());
    
    Ok(video.clone())
}

fn build_empty_vide() -> VideoItem{
    VideoItem { 
        id: VideoItemId {
            kind:"".to_string(),
            video_id:"".to_string(),
         },
         snippet: VideoSnippet { 
            title: "".to_string(), 
            description:"".to_string(), 
        },
    }
}
#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
pub struct SearchResult{
   pub region_code: String,
   pub items:Vec<VideoItem>,
}

#[derive(Clone, Deserialize)]
pub struct VideoItem{
    pub id: VideoItemId,
    pub snippet:VideoSnippet,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct VideoItemId{
    pub kind:String,
    pub video_id:String,
}

#[derive(Clone, Deserialize)]
pub struct VideoSnippet{
    pub title:String,
    pub description:String,
}
