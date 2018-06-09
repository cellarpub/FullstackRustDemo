use schema::answers;
use user::User;
use question::Question;
//use error::JoeResult;
use uuid::Uuid;



#[derive(Debug, Clone, Identifiable, Queryable, Associations, CrdUuid, ErrorHandler)]
#[primary_key(uuid)]
#[table_name = "answers"]
#[insertable = "NewAnswer"]
#[belongs_to(User, foreign_key = "author_uuid")]
#[belongs_to(Question, foreign_key = "question_uuid")]
pub struct Answer {
    /// Primary Key.
    pub uuid: Uuid,
    pub question_uuid: Uuid,
    pub author_uuid: Uuid,
    pub answer_text: Option<String>,
}

#[derive(Insertable, Debug)]
#[table_name = "answers"]
pub struct NewAnswer {
    pub author_uuid: Uuid,
    pub question_uuid: Uuid,
    pub answer_text: Option<String>,
}

pub struct AnswerData {
    pub answer: Answer,
    pub user: User,
}