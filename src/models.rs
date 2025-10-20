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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MediaSource {
    pub src: String,
    pub mime: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MediaVideo {
    pub title: String,
    pub description: String,
    pub sources: Vec<MediaSource>,
    pub poster: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MediaImage {
    pub title: String,
    pub description: String,
    pub src: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MediaTimelineEntry {
    pub period: String,
    pub title: String,
    pub description: String,
    pub media_kind: String,
    pub media_src: String,
    pub poster: Option<String>,
    pub accent: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MediaExperienceHighlight {
    pub title: String,
    pub description: String,
    pub media_kind: String,
    pub media_src: String,
    pub poster: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MediaExperience {
    pub id: String,
    pub name: String,
    pub tagline: String,
    pub description: String,
    pub hero_image: String,
    pub color: String,
    pub highlights: Vec<MediaExperienceHighlight>,
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
#[template(path = "resume.html")]
pub struct ResumeTemplate {
    pub resume: ResumeItem,
}

#[derive(Debug, Template)]
#[template(path = "resume2.html")]
pub struct Resume2Template {
    pub resume: ResumeItem,
}

#[derive(Debug, Template)]
#[template(path = "media.html")]
pub struct MediaTemplate {
    pub videos: Vec<MediaVideo>,
    pub images: Vec<MediaImage>,
}

#[derive(Debug, Template)]
#[template(path = "media_gallery.html")]
pub struct MediaGalleryTemplate {
    pub images: Vec<MediaImage>,
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
#[template(path = "media_storyline.html")]
pub struct MediaStorylineTemplate {
    pub timeline: Vec<MediaTimelineEntry>,
}

#[derive(Debug, Template)]
#[template(path = "media_experiences.html")]
pub struct MediaExperiencesTemplate {
    pub experiences: Vec<MediaExperience>,
}

#[derive(Debug, Template)]
#[template(path = "resume3.html")]
pub struct Resume3Template {
    pub resume: ResumeItem,
}

#[derive(Debug, Template)]
#[template(path = "media3.html")]
pub struct Media3Template {
    pub videos: Vec<MediaVideo>,
    pub images: Vec<MediaImage>,
}
