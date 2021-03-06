use wire::post::PostResponse;
use datatypes::user::UserData;
use chrono::NaiveDateTime;
use identifiers::post::PostUuid;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct MinimalNewPostData {
    pub author_id: i32, // TODO what is this?
    pub content: String,
}


#[derive(Debug, Clone, PartialEq)]
pub struct PostData {
    pub uuid: PostUuid,
    pub author: UserData,
    pub created_date: NaiveDateTime,
    pub modified_date: Option<NaiveDateTime>,
    pub content: String,
    pub censored: bool,
    pub children: Vec<PostData>,
}

impl Default for PostData {
    fn default() -> Self {
        PostData {
            uuid: PostUuid::default(),
            author: UserData::default(),
            created_date: NaiveDateTime::from_timestamp(0,0),
            modified_date: Option::default(),
            content: String::default(),
            censored: bool::default(),
            children: Vec::default()
        }
    }
}

impl From<PostResponse> for PostData {
    fn from(response: PostResponse) -> Self{
        PostData {
            uuid: response.uuid,
            author: UserData::from(response.author),
            created_date: response.created_date,
            modified_date: response.modified_date,
            content: response.content,
            censored: response.censored,
            children: response.children.into_iter().map(PostData::from).collect()
        }
    }
}

impl PostData {
    pub fn merge_childless(&mut self, other: PostData) {
        self.content = other.content;
        self.modified_date = other.modified_date;
        self.author = other.author; // There is a very remote chance that user related data changed between updates
    }
}