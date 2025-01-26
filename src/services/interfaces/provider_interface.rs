use crate::models::course::Course;
use crate::models::grade::UserGrades;
use crate::models::user::User;
use async_trait::async_trait;

#[async_trait]
pub trait ProviderInterface: Send + Sync  {
    async fn get_user(&self, token: &str) -> Result<User, reqwest::Error>;
    async fn valid_token(&self, token: &str) -> Result<(), reqwest::Error>;
    async fn get_courses(&self, token: &str, user_id: i64) -> Result<Vec<Course>, reqwest::Error>;
    async fn get_grades_by_course_id(&self, token: &str, user_id: i64, course_id: i64) -> Result<UserGrades, reqwest::Error>;
}