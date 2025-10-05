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
    pub link: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PersonalInfo {
    pub name: String,
    pub phone: String,
    pub email: String,
    pub linkedin: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Education {
    pub school: String,
    pub location: String,
    pub period: String,
    pub degree: String,
    pub gpa: String,
    pub sat: String,
    pub psat: String,
    pub ap_scores: String,
    pub college_credits: String,
    pub graduation_year: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Skill {
    pub category: String,
    pub skills: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Project {
    pub title: String,
    pub description: String,
    pub date: String,
    pub link: String,
    pub attachment: String,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Award {
    pub name: String,
    pub description: String,
    pub date: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Certification {
    pub name: String,
    pub description: String,
    pub date: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResumeItem {
    pub personal_info: PersonalInfo,
    pub education: Vec<Education>,
    pub skills: Vec<Skill>,
    pub experience: Vec<Achievement>,
    pub projects: Vec<Project>,
    pub awards: Vec<Award>,
    pub certifications: Vec<Certification>,
}

#[derive(Clone)]
pub struct AppState {
    pub cache_manager: std::sync::Arc<crate::cache::CacheManager>,
    pub visits: std::sync::Arc<tokio::sync::Mutex<Vec<VisitRecord>>>,
    pub version: std::sync::Arc<std::sync::Mutex<u64>>,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct HomeTemplate {
    pub name: String,
    pub title: String,
    pub video_mp4: String,
    pub video_aac: String,
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

#[derive(Template)]
#[template(path = "resume2.html")]
pub struct Resume2Template {
    pub resume: ResumeItem,
}