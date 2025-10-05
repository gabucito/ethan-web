use axum::{
    extract::{State, ConnectInfo},
    response::Html,
    http::StatusCode,
};
use crate::models::{AppState, HomeTemplate, AchievementsTemplate, ResumeTemplate, Achievement, Education, Skill};
use std::net::SocketAddr;

pub async fn home_handler(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Html<String> {
    let key = state.cache_manager.make_key("/");
    if let Some(cached) = state.cache_manager.get(&key).await {
        log_visit(&state, "/", &addr.ip().to_string()).await;
        return Html(cached);
    }

    let template = HomeTemplate {
        name: "Ethan".to_string(),
        title: "College Application Portfolio".to_string(),
    };
    let rendered = template.render().unwrap();
    state.cache_manager.set(key, rendered.clone()).await;
    log_visit(&state, "/", &addr.ip().to_string()).await;
    Html(rendered)
}

pub async fn achievements_handler(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Html<String> {
    let key = state.cache_manager.make_key("/achievements");
    if let Some(cached) = state.cache_manager.get(&key).await {
        log_visit(&state, "/achievements", &addr.ip().to_string()).await;
        return Html(cached);
    }

    // Sample data
    let achievements = vec![
        Achievement {
            title: "Interview Video".to_string(),
            description: "An interview video showcasing interviewees, edited and processed into multiple formats.".to_string(),
            date: "2024".to_string(),
        },
        // Add more as needed
    ];

    let template = AchievementsTemplate { achievements };
    let rendered = template.render().unwrap();
    state.cache_manager.set(key, rendered.clone()).await;
    log_visit(&state, "/achievements", &addr.ip().to_string()).await;
    Html(rendered)
}

pub async fn resume_handler(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Html<String> {
    let key = state.cache_manager.make_key("/resume");
    if let Some(cached) = state.cache_manager.get(&key).await {
        log_visit(&state, "/resume", &addr.ip().to_string()).await;
        return Html(cached);
    }

    // Sample data
    let resume = crate::models::ResumeItem {
        education: vec![
            Education {
                school: "High School".to_string(),
                degree: "High School Diploma".to_string(),
                gpa: "3.8".to_string(),
                graduation_year: "2025".to_string(),
            },
        ],
        skills: vec![
            Skill {
                category: "Programming".to_string(),
                skills: vec!["Rust".to_string(), "Python".to_string()],
            },
        ],
        experience: vec![
            Achievement {
                title: "Software Development Intern".to_string(),
                description: "Built web applications using various technologies.".to_string(),
                date: "2024".to_string(),
            },
        ],
    };

    let template = ResumeTemplate { resume };
    let rendered = template.render().unwrap();
    state.cache_manager.set(key, rendered.clone()).await;
    log_visit(&state, "/resume", &addr.ip().to_string()).await;
    Html(rendered)
}

async fn log_visit(state: &Arc<AppState>, page: &str, ip: &str) {
    use crate::models::VisitRecord;
    use chrono::Utc;

    let visit = VisitRecord {
        ip: ip.to_string(),
        page: page.to_string(),
        timestamp: Utc::now(),
    };
    let mut visits = state.visits.lock().await;
    visits.push(visit);
    tracing::info!("Logged visit to {} from {}", page, ip);
}