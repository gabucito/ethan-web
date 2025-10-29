use crate::models::{
    Achievement, AchievementDetailTemplate, AchievementListItem, AchievementsTemplate, Activity,
    ActivityDetailTemplate, ActivityListItem, ActivitiesTemplate, AppState, Award, Certification,
    Education, HomeTemplate, PersonalInfo, Project, ProjectDetailTemplate, ProjectListItem,
    ProjectsTemplate, Resume3Template, ResumeItem, Skill,
};
use axum::{
    extract::{ConnectInfo, Path, State},
    response::Html,
};
use std::net::SocketAddr;
use std::sync::Arc;

const SITE_BASE: &str = "";

fn slugify(input: &str) -> String {
    let mut slug = String::new();
    let mut prev_dash = false;

    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
            prev_dash = false;
        } else if matches!(ch, ' ' | '-' | '_' | '/' | ':' | '&' | ',') {
            if !prev_dash && !slug.is_empty() {
                slug.push('-');
                prev_dash = true;
            }
        }
    }

    if slug.ends_with('-') {
        slug.pop();
    }

    if slug.is_empty() {
        "entry".to_string()
    } else {
        slug
    }
}

fn achievement_hero(title: &str) -> (Option<&'static str>, Option<&'static str>) {
    match title {
        "Codificar Inc. - President & Founder" => (
            Some("/static/media/images/codificar.jpg"),
            Some("Ethan Cha mentoring Codificar students during a weekend programming workshop"),
        ),
        "Data Science Club - President" => (
            Some("/static/media/images/codificar.jpg"),
            Some("Ethan Cha guiding classmates through a student-led data science session"),
        ),
        "Counterspell Bergen - Lead Organizer" => (
            Some("/static/media/images/counterspell.jpg"),
            Some("Counterspell robotics team collaborating during a build session"),
        ),
        "Published Research Paper in Journal of International Research" => (
            Some("/static/media/images/momath museum.jpg"),
            Some("Ethan Cha presenting research at the MoMath museum exhibit space"),
        ),
        "Published Article in JSR Journal" => (
            Some("/static/media/images/momath museum.jpg"),
            Some("Ethan Cha engaging visitors with an interactive research demonstration"),
        ),
        "Columbia Science Honors Program - Scholar" => (
            Some("/static/media/images/orchestra.jpg"),
            Some("Ethan Cha collaborating with peers during an intensive STEM enrichment program"),
        ),
        "Cooper Union STEM - Linear Algebra Scholar" => (
            Some("/static/media/images/orchestra.jpg"),
            Some("Ethan Cha studying advanced mathematics within a collegiate cohort"),
        ),
        "Technology Intern at Chibitek" => (
            Some("/static/media/images/counterspell.jpg"),
            Some(
                "Counterspell robotics workbench showcasing Ethan Cha's hands-on hardware experience",
            ),
        ),
        _ => (None, None),
    }
}

fn project_hero(title: &str) -> (Option<&'static str>, Option<&'static str>) {
    match title {
        "Generating Realistic Cities with Perlin Noise" => (
            Some("/static/media/images/momath museum.jpg"),
            Some("Procedural city generation visuals presented during a STEM showcase"),
        ),
        "Developing an Automated Spectrophotometer Using Arduino Microcontroller" => (
            Some("/static/media/images/counterspell.jpg"),
            Some("Prototype hardware assembled for the Arduino-based spectrophotometer project"),
        ),
        "Liquiboard: Research and Development of Dynamic Error-Correcting Keyboard App" => (
            Some("/static/media/images/codificar.jpg"),
            Some("User testing the adaptive typing experience of the Liquiboard prototype"),
        ),
        "Congressional App Challenge" => (
            Some("/static/media/images/vfw.jpg"),
            Some("Ethan Cha presenting a civic engagement app during competition judging"),
        ),
        _ => (None, None),
    }
}

fn achievements_list() -> Vec<AchievementListItem> {
    create_resume_data()
        .experience
        .into_iter()
        .map(|achievement| {
            let slug = slugify(&achievement.title);
            AchievementListItem { achievement, slug }
        })
        .collect()
}

fn projects_list() -> Vec<ProjectListItem> {
    create_resume_data()
        .projects
        .into_iter()
        .map(|project| {
            let slug = slugify(&project.title);
            ProjectListItem { project, slug }
        })
        .collect()
}

fn activities_data() -> Vec<Activity> {
    vec![
        Activity {
            title: "Volunteer Service & Community Outreach".to_string(),
            description: "Liberty Science Center: Facilitated hands-on STEM demos and guided visitors across galleries. MoMath Museum: Led interactive math activities and supported educational programming for families. Youth Council Fort Lee: Coordinated large-scale community events, including the Fort Lee Color Run. Chodae Church: Drummer for the youth praise team supporting weekly services and special events.".to_string(),
            date: "2019 - Present".to_string(),
            link: "".to_string(),
            tags: vec![
                "Community Impact".to_string(),
                "STEM Outreach".to_string(),
                "Leadership".to_string(),
            ],
        },
        Activity {
            title: "Capture The Flag (CTF) Club - Leader".to_string(),
            description: "Founded a cybersecurity club that practices ethical hacking, cryptography, and binary exploitation. Organized scrimmages, prepared write-ups, and mentored peers ahead of competitive events.".to_string(),
            date: "2023 - Present".to_string(),
            link: "".to_string(),
            tags: vec![
                "Cybersecurity".to_string(),
                "Cryptography".to_string(),
                "Technical Leadership".to_string(),
            ],
        },
        Activity {
            title: "Tennis - Junior Varsity Captain".to_string(),
            description: "Coordinated practice plans, led match strategy, and mentored underclassmen for the Bergen County Academies tennis program.".to_string(),
            date: "2023 - Present".to_string(),
            link: "".to_string(),
            tags: vec!["Athletics".to_string(), "Team Leadership".to_string()],
        },
        Activity {
            title: "Rock Climbing - Competitive Youth Team Member".to_string(),
            description: "Competed in bouldering and sport climbing competitions while training three times a week under professional coaching to refine strength and technique.".to_string(),
            date: "2021 - Present".to_string(),
            link: "".to_string(),
            tags: vec!["Athletics".to_string(), "Discipline".to_string()],
        },
        Activity {
            title: "Columbia Competitive Programming Camp - Participant".to_string(),
            description: "Selected for an intensive Columbia University program focused on advanced algorithms, graph theory, and dynamic programming with collegiate-level problem sets.".to_string(),
            date: "Summer 2024".to_string(),
            link: "".to_string(),
            tags: vec![
                "Algorithms".to_string(),
                "Competitive Programming".to_string(),
                "C++".to_string(),
            ],
        },
    ]
}

fn activities_list() -> Vec<ActivityListItem> {
    activities_data()
        .into_iter()
        .map(|activity| {
            let slug = slugify(&activity.title);
            ActivityListItem { activity, slug }
        })
        .collect()
}

fn activity_hero(title: &str) -> (Option<&'static str>, Option<&'static str>) {
    match title {
        "Volunteer Service & Community Outreach" => (
            Some("/static/media/images/nhs2.jpg"),
            Some("Ethan Cha organizing community service volunteers during an NHS initiative"),
        ),
        "Capture The Flag (CTF) Club - Leader" => (
            Some("/static/media/images/counterspell.jpg"),
            Some("Ethan Cha coaching teammates through a competitive programming puzzle"),
        ),
        "Tennis - Junior Varsity Captain" => (
            Some("/static/media/images/counterspell.jpg"),
            Some("BCA athletes strategizing courtside with team captain Ethan Cha"),
        ),
        "Rock Climbing - Competitive Youth Team Member" => (
            Some("/static/media/images/nhs.jpg"),
            Some("Ethan Cha preparing for a climbing session with his youth team"),
        ),
        "Columbia Competitive Programming Camp - Participant" => (
            Some("/static/media/images/counterspell.jpg"),
            Some("Students collaborating during an advanced algorithms workshop at Columbia"),
        ),
        _ => (None, None),
    }
}

fn create_resume_data() -> ResumeItem {
    let personal_info = PersonalInfo {
        name: "Ethan Cha".to_string(),
        phone: "(551) 223-9739".to_string(),
        email: "ethan.j.cha@gmail.com".to_string(),
        linkedin: "linkedin.com/in/ethan-cha-67639730b".to_string(),
    };

    ResumeItem {
        personal_info,
        education: vec![
            Education {
                school: "Bergen County Academies".to_string(),
                location: "Hackensack, NJ".to_string(),
                period: "Sep. 2022 - Present".to_string(),
                degree: "High School Diploma".to_string(),
                gpa: "3.83".to_string(),
                sat: "1550".to_string(),
                psat: "1460".to_string(),
                ap_scores: "AP Computer Science A (5), AP Physics 1 (5), AP Calc BC (5), AP Microeconomics (5), AP Physics C: Electricity and Magnetism (4), AP Physics C: Mechanics (5)".to_string(),
                college_credits: "Intro to Programming (BCC:INF 103), US History Since 1877 (FDU:HIST 1131) - 6 credits".to_string(),
                graduation_year: "2026".to_string(),
            },
        ],
        skills: vec![
            Skill {
                category: "Programming Languages".to_string(),
                skills: vec!["Java".to_string(), "Python".to_string(), "Dart".to_string()],
            },
            Skill {
                category: "Technologies & Tools".to_string(),
                skills: vec!["Arduino".to_string(), "Machine Learning".to_string(), "AWS".to_string(), "Competitive Programming".to_string(), "Data Visualization Tools".to_string()],
            },
        ],
        experience: vec![
            Achievement {
                title: "Codificar Inc. - President & Founder".to_string(),
                description: "Established and currently lead a 501(c)(3) nonprofit offering free coding instruction in Java, Python, and competitive programming. Developed a multi-tier curriculum and managed a team of peer tutors; instructed over 30+ students across middle and high school levels. Mentored competitive programming team that placed 1st in NJ at the ACSL All-Star Senior Division.".to_string(),
                date: "2023 - Present".to_string(),
                link: "https://www.codificar.me/".to_string(),
            },
            Achievement {
                title: "Data Science Club - President".to_string(),
                description: "Created a schoolwide initiative introducing students to real-world data science applications through workshops and hands-on challenges. Partnered with university researchers and national data science nonprofits to provide mentorship and industry exposure. Led peer-teaching seminars on Pandas, NumPy, and data visualization; organized internal competitions and case study analyses.".to_string(),
                date: "2023 - Present".to_string(),
                link: "".to_string(),
            },
            Achievement {
                title: "Technology Intern at Chibitek".to_string(),
                description: "Helped monitor and maintain clients' systems to ensure consistent performance and minimal downtime. Collaborated on diagnosing and resolving a range of IT issues, from network connectivity challenges to software errors. Assisted in managing cloud services, including data migrations and secure storage solutions. Learned to use industry-standard remote monitoring and management (RMM) tools to oversee IT environments. Contributed to documentation efforts, creating guides and process manuals for both internal use and client reference. Participated in team discussions to develop new strategies for optimizing IT operations.".to_string(),
                date: "Jul. 2024 - Present".to_string(),
                link: "https://www.chibitek.com/".to_string(),
            },
            Achievement {
                title: "Published Research Paper in Journal of International Research".to_string(),
                description: "Published a research paper on Assessing Local Sources for PM2.5 in the NY Metro area using Conditional Probability Function Analysis.".to_string(),
                date: "2024".to_string(),
                link: "https://www.journalresearchhs.org/_files/ugd/ebf144_b1857e9968904942baabed016e618e89.pdf".to_string(),
            },
            Achievement {
                title: "Published Article in JSR Journal".to_string(),
                description: "Researched about use of KZ Filter technique to evaluate PM2.5 Air Quality long term trends in NJ. Submitted the journal to publish the article with the topic Use of Kolmogorov-Zurbenko Filter Technique to Evaluate PM 2.5 Air Quality Long-Term Trends in New Jersey.".to_string(),
                date: "2023".to_string(),
                link: "https://www.jsr.org/hs/index.php/path/article/view/4393".to_string(),
            },
            Achievement {
                title: "Counterspell Bergen - Lead Organizer".to_string(),
                description: "Founded and led a regional high school game jam with 50+ attendees as part of Hack Club's global hackathon initiative. Designed and launched the official event website, later spotlighted by Columbia Engineering Outreach for innovation in STEM youth engagement. Secured over $1,000 in sponsorships from local businesses and educational organizations to fund venue, prizes, and technical resources.".to_string(),
                date: "Nov. 2024".to_string(),
                link: "https://counterspell.hackclub.com/bergen".to_string(),
            },
            Achievement {
                title: "Columbia Science Honors Program - Scholar".to_string(),
                description: "Selected through a highly competitive application process (acceptance rate <10%) to participate in Columbia University's weekend STEM enrichment program. Completed advanced coursework in theoretical computer science, neuroscience, and applied mathematics under Columbia faculty.".to_string(),
                date: "2022 - Present".to_string(),
                link: "".to_string(),
            },
            Achievement {
                title: "Cooper Union STEM - Linear Algebra Scholar".to_string(),
                description: "Studied matrix theory, linear transformations, and eigenvector applications as part of an engineering-focused linear algebra cohort. Applied mathematical modeling to solve real-world engineering problems in collaborative team settings.".to_string(),
                date: "Summer 2024".to_string(),
                link: "".to_string(),
            },
        ],
        projects: vec![
            Project {
                title: "Generating Realistic Cities with Perlin Noise".to_string(),
                description: "Conducted research on procedural generation using Perlin noise to model realistic city layouts, applying computational methods to urban design and simulation.".to_string(),
                date: "Wolfram Summer Research Program".to_string(),
                link: "https://community.wolfram.com/groups/-/m/t/3501352".to_string(),
                attachment: "".to_string(),
                tags: vec!["Wolfram Mathematica".to_string(), "Procedural Generation".to_string(), "Simulation".to_string()],
            },
            Project {
                title: "Developing an Automated Spectrophotometer Using Arduino Microcontroller".to_string(),
                description: "Developed spectrophotometer using RGB LEDs, LDR sensors, 3D-printed cartridge, and Arduino Mega; implemented automated wavelength selection and calibration software.".to_string(),
                date: "BCA Expo".to_string(),
                link: "".to_string(),
                attachment: "Spectrophotometer.zip".to_string(),
                tags: vec!["Arduino".to_string(), "Electronics".to_string(), "3D Printing".to_string(), "Sensors".to_string()],
            },
            Project {
                title: "Liquiboard: Research and Development of Dynamic Error-Correcting Keyboard App".to_string(),
                description: "Developed a custom Android keyboard that utilizes machine learning to analyze and adapt to user's unique typing pattern. The keyboard dynamically adjusts the shape and sensitivity of key borders based on frequently tapped areas, reducing typing errors and improving overall accuracy and user experience.".to_string(),
                date: "".to_string(),
                link: "".to_string(),
                attachment: "".to_string(),
                tags: vec!["Android".to_string(), "Machine Learning".to_string(), "Mobile Development".to_string()],
            },
            Project {
                title: "Congressional App Challenge".to_string(),
                description: "Designed and developed a mobile app for intermittent fasting, featuring customizable scheduling tools, reminders, and evidence-based nutrition tips. Built an AI-driven news summarization app that converts headlines from major outlets into accessible content for children, promoting early civic awareness.".to_string(),
                date: "".to_string(),
                link: "".to_string(),
                attachment: "".to_string(),
                tags: vec!["Mobile Apps".to_string(), "AI".to_string(), "UI/UX Design".to_string()],
            },
        ],
        activities: activities_data(),
        awards: vec![
            Award { name: "AP Scholar with Distinction".to_string(), description: "".to_string(), date: "2025".to_string() },
            Award { name: "STEM Light House Challenge".to_string(), description: "2nd place".to_string(), date: "2023".to_string() },
            Award { name: "National Honor Society (NHS) Member".to_string(), description: "".to_string(), date: "2024 - 2026".to_string() },
            Award { name: "Congressional Award".to_string(), description: "Gold certificate".to_string(), date: "2024".to_string() },
            Award { name: "Congressional Certificate of Appreciation".to_string(), description: "YCFL".to_string(), date: "2023 - 2025".to_string() },
            Award { name: "President's Volunteer Service Award".to_string(), description: "".to_string(), date: "2023".to_string() },
            Award { name: "American Computer Science League (ACSL) All Star Senior".to_string(), description: "Gold Medal".to_string(), date: "2023".to_string() },
            Award { name: "American Computer Science League (ACSL) All Star Intermediate".to_string(), description: "Gold Medal".to_string(), date: "2022".to_string() },
            Award { name: "Bergen County Academies Science Olympiad".to_string(), description: "2nd place".to_string(), date: "2022".to_string() },
            Award { name: "USA Computing Olympiad (USACO)".to_string(), description: "Platinum level".to_string(), date: "2022 - 2023".to_string() },
            Award { name: "Veterans of Foreign Wars (VFW) Voice of Democracy County winner".to_string(), description: "".to_string(), date: "2022 - 2023".to_string() },
            Award { name: "Bergen Science Fair".to_string(), description: "3rd place & Broadcom Coding with Commitment Award".to_string(), date: "2022".to_string() },
            Award { name: "Scholastic Art & Writing Award".to_string(), description: "Silver Key".to_string(), date: "".to_string() },
        ],
        certifications: vec![
            Certification { name: "Java SE 8 Programmer I".to_string(), description: "Oracle certification for Java SE 8 - 1Z0-808".to_string(), date: "2024".to_string() },
            Certification { name: "The Coding School - Artificial Intelligence".to_string(), description: "".to_string(), date: "2023 - 2024".to_string() },
            Certification { name: "The Coding School - Machine Learning".to_string(), description: "".to_string(), date: "2024".to_string() },
            Certification { name: "PCEP Certified Entry-Level Python Programmer".to_string(), description: "".to_string(), date: "2023".to_string() },
            Certification { name: "AWS Practitioner".to_string(), description: "".to_string(), date: "2022".to_string() },
            Certification { name: "Diplomas de Español como Lengua Extranjera (DELE)".to_string(), description: "A1 & A2".to_string(), date: "2021".to_string() },
            Certification { name: "Microsoft Office Certification (MOS)".to_string(), description: "Excel, Powerpoint, Word".to_string(), date: "2020".to_string() },
            Certification { name: "Taekwondo Black Belt".to_string(), description: "2nd dan".to_string(), date: "2020".to_string() },
        ],
    }
}

pub async fn home_handler(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Html<String> {
    let ip = addr.ip().to_string();
    let template = HomeTemplate {
        name: "Ethan".to_string(),
        title: "Aspiring Computer Science Student Portfolio".to_string(),
        video_mp4: "/static/media/videos/output.mp4".to_string(),
        video_aac: "/static/media/videos/output_aac.mp4".to_string(),
    };
    render_cached_page(&state, "/", &ip, &template).await
}

pub async fn activities_handler(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Html<String> {
    let ip = addr.ip().to_string();
    let activities = activities_list();
    let template = ActivitiesTemplate { activities };
    render_cached_page(&state, "/activities", &ip, &template).await
}

pub async fn activity_detail_handler(
    Path(slug): Path<String>,
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Html<String> {
    let ip = addr.ip().to_string();
    if let Some(item) = activities_list().into_iter().find(|entry| entry.slug == slug) {
        let (hero_image_raw, hero_alt_raw) = activity_hero(&item.activity.title);
        let hero_image_abs = hero_image_raw.map(|path| format!("{}{}", SITE_BASE, path));
        let hero_image_clone = hero_image_abs.clone();
        let has_hero_image = hero_image_abs.is_some();
        let hero_image = hero_image_abs.unwrap_or_default();
        let hero_alt = hero_alt_raw
            .map(|alt| alt.to_string())
            .unwrap_or_else(|| format!("{} spotlight", item.activity.title));
        let og_image = hero_image_clone
            .unwrap_or_else(|| format!("{}{}", SITE_BASE, "/static/media/images/nhs2.jpg"));
        let og_image_alt = hero_alt.clone();
        let mut keywords = vec![
            item.activity.title.clone(),
            "Ethan Cha".to_string(),
            "Activity".to_string(),
        ];
        keywords.extend(item.activity.tags.clone());
        let page_url = format!("{}/activities/{}", SITE_BASE, slug);
        let template = ActivityDetailTemplate {
            activity: item.activity,
            slug: slug.clone(),
            page_url,
            hero_image,
            hero_alt,
            has_hero_image,
            og_image,
            og_image_alt,
            keywords,
        };
        let cache_path = format!("/activities/{}", slug);
        return render_cached_page(&state, &cache_path, &ip, &template).await;
    }

    Html("<h1>Activity not found</h1>".to_string())
}

pub async fn achievements_handler(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Html<String> {
    let ip = addr.ip().to_string();
    let achievements = achievements_list();
    let template = AchievementsTemplate { achievements };
    render_cached_page(&state, "/achievements", &ip, &template).await
}

pub async fn achievement_detail_handler(
    Path(slug): Path<String>,
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Html<String> {
    let ip = addr.ip().to_string();

    if let Some(item) = achievements_list()
        .into_iter()
        .find(|entry| entry.slug == slug)
    {
        let (hero_image_raw, hero_alt_raw) = achievement_hero(&item.achievement.title);
        let hero_image_abs = hero_image_raw.map(|path| format!("{}{}", SITE_BASE, path));
        let hero_image_clone = hero_image_abs.clone();
        let has_hero_image = hero_image_abs.is_some();
        let hero_image = hero_image_abs.unwrap_or_default();
        let hero_alt = hero_alt_raw
            .map(|alt| alt.to_string())
            .unwrap_or_else(|| format!("{} highlight", item.achievement.title));
        let og_image = hero_image_clone
            .unwrap_or_else(|| format!("{}{}", SITE_BASE, "/static/media/images/codificar.jpg"));
        let og_image_alt = hero_alt.clone();
        let mut keywords = vec![
            item.achievement.title.clone(),
            "Ethan Cha".to_string(),
            "Achievement".to_string(),
        ];
        keywords.push(item.achievement.date.clone());
        let page_url = format!("{}/achievements/{}", SITE_BASE, slug);
        let template = AchievementDetailTemplate {
            achievement: item.achievement,
            slug: slug.clone(),
            page_url,
            hero_image,
            hero_alt,
            has_hero_image,
            og_image,
            og_image_alt,
            keywords,
        };
        let cache_path = format!("/achievements/{}", slug);
        return render_cached_page(&state, &cache_path, &ip, &template).await;
    }

    Html("<h1>Achievement not found</h1>".to_string())
}

pub async fn projects_handler(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Html<String> {
    let ip = addr.ip().to_string();
    let projects = projects_list();
    let template = ProjectsTemplate { projects };
    render_cached_page(&state, "/projects", &ip, &template).await
}

pub async fn project_detail_handler(
    Path(slug): Path<String>,
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Html<String> {
    let ip = addr.ip().to_string();

    if let Some(item) = projects_list().into_iter().find(|entry| entry.slug == slug) {
        let (hero_image_raw, hero_alt_raw) = project_hero(&item.project.title);
        let hero_image_abs = hero_image_raw.map(|path| format!("{}{}", SITE_BASE, path));
        let hero_image_clone = hero_image_abs.clone();
        let has_hero_image = hero_image_abs.is_some();
        let hero_image = hero_image_abs.unwrap_or_default();
        let hero_alt = hero_alt_raw
            .map(|alt| alt.to_string())
            .unwrap_or_else(|| format!("{} showcase", item.project.title));
        let og_image = hero_image_clone
            .unwrap_or_else(|| format!("{}{}", SITE_BASE, "/static/media/images/counterspell.jpg"));
        let og_image_alt = hero_alt.clone();
        let mut keywords = vec![
            item.project.title.clone(),
            "Ethan Cha".to_string(),
            "Project".to_string(),
        ];
        keywords.extend(item.project.tags.clone());
        let page_url = format!("{}/projects/{}", SITE_BASE, slug);
        let template = ProjectDetailTemplate {
            project: item.project,
            slug: slug.clone(),
            page_url,
            hero_image,
            hero_alt,
            has_hero_image,
            og_image,
            og_image_alt,
            keywords,
        };
        let cache_path = format!("/projects/{}", slug);
        return render_cached_page(&state, &cache_path, &ip, &template).await;
    }

    Html("<h1>Project not found</h1>".to_string())
}

pub async fn resume_handler(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Html<String> {
    let ip = addr.ip().to_string();
    let mut resume = create_resume_data();

    // Keep the published entries concise for the interactive view
    for exp in &mut resume.experience {
        if exp.title.contains("Published") && exp.description.chars().count() > 150 {
            let truncated = exp.description.chars().take(150).collect::<String>();
            exp.description = format!("{}...", truncated);
        }
    }

    let template = Resume3Template { resume };
    render_cached_page(&state, "/resume", &ip, &template).await
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

// Generic function to handle cached page rendering
async fn render_cached_page<T: askama::Template + std::fmt::Debug>(
    state: &Arc<AppState>,
    path: &str,
    ip: &str,
    template: &T,
) -> Html<String> {
    let key = state.cache_manager.make_key(path);

    if let Some(cached) = state.cache_manager.get(&key).await {
        log_visit(state, path, ip).await;
        return Html(cached);
    }

    let rendered = template.render().expect("Failed to render template");
    state.cache_manager.set(key, rendered.clone()).await;
    log_visit(state, path, ip).await;
    Html(rendered)
}
