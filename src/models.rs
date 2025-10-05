use askama::Template;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VisitRecord {
    pub ip: String,
    pub page: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Achievement {
    pub title: String,
    pub description: String,
    pub date: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Education {
    pub school: String,
    pub degree: String,
    pub gpa: String,
    pub graduation_year: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Skill {
    pub category: String,
    pub skills: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResumeItem {
    pub education: Vec<Education>,
    pub skills: Vec<Skill>,
    pub experience: Vec<Achievement>, // reusing for job experience
}

#[derive(Clone)]
pub struct AppState {
    pub cache_manager: crate::cache::CacheManager,
    pub visits: std::sync::Arc<tokio::sync::Mutex<Vec<VisitRecord>>>,
    pub version: std::sync::Arc<std::sync::Mutex<u64>>,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct HomeTemplate {
    pub name: String,
    pub title: String,
}

#[derive(Template)]
#[template(path = "achievements.html")]
pub struct AchievementsTemplate {
    pub achievements: Vec<Achievement>,
}

#[derive(Template)]
#[template(path = "resume.html")]
pub struct ResumeTemplate {
    pub resume: ResumeItem,
}