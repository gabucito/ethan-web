use askama::Template;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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

#[derive(Clone, Debug)]
pub struct AchievementListItem {
    pub achievement: Achievement,
    pub slug: String,
}

#[derive(Clone, Debug)]
pub struct ProjectListItem {
    pub project: Project,
    pub slug: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Award {
    pub name: String,
    pub description: String,
    pub date: String,
    pub attachment: String,
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
    pub activities: Vec<Activity>,
    pub awards: Vec<Award>,
    pub certifications: Vec<Certification>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Activity {
    pub title: String,
    pub description: String,
    pub date: String,
    pub link: String,
    pub tags: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct ActivityListItem {
    pub activity: Activity,
    pub slug: String,
}

#[derive(Clone)]
pub struct AppState {
    pub cache_manager: std::sync::Arc<crate::cache::CacheManager>,
    pub visits: std::sync::Arc<tokio::sync::Mutex<Vec<VisitRecord>>>,
}

#[derive(Debug, Template)]
#[template(path = "index.html")]
pub struct HomeTemplate {
    pub name: String,
    pub title: String,
    pub video_mp4: String,
    pub video_aac: String,
}

#[derive(Debug, Template)]
#[template(path = "achievements.html")]
pub struct AchievementsTemplate {
    pub achievements: Vec<AchievementListItem>,
}

#[derive(Debug, Template)]
#[template(path = "projects.html")]
pub struct ProjectsTemplate {
    pub projects: Vec<ProjectListItem>,
}

#[derive(Debug, Template)]
#[template(path = "activities.html")]
pub struct ActivitiesTemplate {
    pub activities: Vec<ActivityListItem>,
}

#[derive(Debug, Template)]
#[template(path = "achievement_detail.html")]
pub struct AchievementDetailTemplate {
    pub achievement: Achievement,
    pub slug: String,
    pub page_url: String,
    pub hero_image: String,
    pub hero_alt: String,
    pub has_hero_image: bool,
    pub og_image: String,
    pub og_image_alt: String,
    pub keywords: Vec<String>,
}

#[derive(Debug, Template)]
#[template(path = "project_detail.html")]
pub struct ProjectDetailTemplate {
    pub project: Project,
    pub slug: String,
    pub page_url: String,
    pub hero_image: String,
    pub hero_alt: String,
    pub has_hero_image: bool,
    pub og_image: String,
    pub og_image_alt: String,
    pub keywords: Vec<String>,
}

#[derive(Debug, Template)]
#[template(path = "resume3.html")]
pub struct Resume3Template {
    pub resume: ResumeItem,
}

#[derive(Debug, Template)]
#[template(path = "activity_detail.html")]
pub struct ActivityDetailTemplate {
    pub activity: Activity,
    pub slug: String,
    pub page_url: String,
    pub hero_image: String,
    pub hero_alt: String,
    pub has_hero_image: bool,
    pub og_image: String,
    pub og_image_alt: String,
    pub keywords: Vec<String>,
}
