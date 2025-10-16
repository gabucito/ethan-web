use crate::models::{
    Achievement, AppState, Award, Certification, Education, HomeTemplate, MediaGalleryTemplate,
    MediaImage, MediaSource, MediaTemplate, MediaVideo, PersonalInfo, Project, Resume2Template,
    ResumeTemplate, Skill,
};
use askama_axum::Template;
use axum::{extract::State, response::Html};
use std::sync::Arc;

fn featured_media_videos() -> Vec<MediaVideo> {
    vec![MediaVideo {
        title: "Portfolio Highlight Reel".to_string(),
        description: "Quick overview showcasing recent projects and accomplishments.".to_string(),
        sources: vec![
            MediaSource {
                src: "/static/output.mp4".to_string(),
                mime: "video/mp4".to_string(),
            },
            MediaSource {
                src: "/static/output_aac.mp4".to_string(),
                mime: "video/mp4".to_string(),
            },
        ],
    }]
}

fn featured_media_images() -> Vec<MediaImage> {
    vec![
        MediaImage {
            title: "Robotics Showcase".to_string(),
            description: "Demonstration from the regional robotics invitational.".to_string(),
            src: "/static/media/robotics.svg".to_string(),
        },
        MediaImage {
            title: "STEM Expo Booth".to_string(),
            description: "Poster session highlighting LiDAR research project.".to_string(),
            src: "/static/media/stem-expo.svg".to_string(),
        },
    ]
}

pub async fn home_handler(State(state): State<Arc<AppState>>) -> Html<String> {
    let key = state.cache_manager.make_key("/");
    let ip = "127.0.0.1".to_string(); // TODO: get from request
    if let Some(cached) = state.cache_manager.get(&key).await {
        log_visit(&state, "/", &ip).await;
        return Html(cached);
    }

    let template = HomeTemplate {
        name: "Ethan".to_string(),
        title: "Aspiring Computer Science Student Portfolio".to_string(),
        video_mp4: "/static/output.mp4".to_string(),
        video_aac: "/static/output_aac.mp4".to_string(),
    };
    let rendered = template.render().unwrap();
    state.cache_manager.set(key, rendered.clone()).await;
    log_visit(&state, "/", &ip).await;
    Html(rendered)
}

pub async fn media_handler(State(state): State<Arc<AppState>>) -> Html<String> {
    let key = state.cache_manager.make_key("/media");
    let ip = "127.0.0.1".to_string(); // TODO: extract from request metadata
    if let Some(cached) = state.cache_manager.get(&key).await {
        log_visit(&state, "/media", &ip).await;
        return Html(cached);
    }

    let videos = featured_media_videos();
    let images = featured_media_images();

    let template = MediaTemplate { videos, images };
    let rendered = template.render().unwrap();
    state.cache_manager.set(key, rendered.clone()).await;
    log_visit(&state, "/media", &ip).await;
    Html(rendered)
}

pub async fn media_gallery_handler(State(state): State<Arc<AppState>>) -> Html<String> {
    let key = state.cache_manager.make_key("/media/gallery");
    let ip = "127.0.0.1".to_string(); // TODO: extract from request metadata
    if let Some(cached) = state.cache_manager.get(&key).await {
        log_visit(&state, "/media/gallery", &ip).await;
        return Html(cached);
    }

    let images = featured_media_images();

    let template = MediaGalleryTemplate { images };
    let rendered = template.render().unwrap();
    state.cache_manager.set(key, rendered.clone()).await;
    log_visit(&state, "/media/gallery", &ip).await;
    Html(rendered)
}

pub async fn resume_handler(State(state): State<Arc<AppState>>) -> Html<String> {
    let key = state.cache_manager.make_key("/resume");
    let ip = "127.0.0.1".to_string(); // TODO
    if let Some(cached) = state.cache_manager.get(&key).await {
        log_visit(&state, "/resume", &ip).await;
        return Html(cached);
    }

    let personal_info = PersonalInfo {
        name: "Ethan Cha".to_string(),
        phone: "(551) 223-9739".to_string(),
        email: "ethan.j.cha@gmail.com".to_string(),
        linkedin: "linkedin.com/in/ethan-cha-67639730b".to_string(),
    };

    let resume = crate::models::ResumeItem {
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
                title: "Volunteer Service".to_string(),
                description: "LIBERTY SCIENCE CENTER: Presented hands-on demonstrations and guided guests through STEM exhibits as part of the museum's volunteer team. MUSEUM OF MATH (MOMATH): Led interactive math activities and supported educational programming for families and youth visitors. YOUTH COUNCIL FORT LEE (YCFL): Organized large-scale community events, including the Fort Lee Color Run, and participated in civic outreach initiatives. CHODAE CHURCH: Drummer for the youth praise team, contributing to weekly worship services and church events.".to_string(),
                date: "700+ Hours".to_string(),
                link: "".to_string(),
                attachment: "".to_string(),
                tags: vec![],
            },
            Project {
                title: "Capture The Flag (CTF) Club - Leader".to_string(),
                description: "Spearheaded a student cybersecurity team to compete in CTF tournaments, solving challenges involving network intrusion, cryptography, and binary exploitation. Coached peers in ethical hacking tools and techniques; placed top 3 in school-wide competitions.".to_string(),
                date: "".to_string(),
                link: "".to_string(),
                attachment: "".to_string(),
                tags: vec!["Cybersecurity".to_string(), "Cryptography".to_string(), "Network Security".to_string()],
            },
            Project {
                title: "Tennis - Junior Varsity Captain".to_string(),
                description: "Served as captain for the JV team, coordinating practices, leading strategy discussions, and mentoring new players. Competed in regional league matches and trained alongside varsity athletes.".to_string(),
                date: "".to_string(),
                link: "".to_string(),
                attachment: "".to_string(),
                tags: vec![],
            },
            Project {
                title: "Rock Climbing - Competitive Youth Team Member".to_string(),
                description: "Competed in bouldering and sport climbing events at local and regional levels as part of a certified youth climbing team. Trained 3x weekly to improve strength, endurance, and technique under professional coaching.".to_string(),
                date: "".to_string(),
                link: "".to_string(),
                attachment: "".to_string(),
                tags: vec![],
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
            Project {
                title: "Columbia Competitive Programming Camp - Participant".to_string(),
                description: "Selected for a two-week intensive training program hosted by Columbia University, focused on advanced algorithms, graph theory, and dynamic programming. Collaborated with college students on timed problem sets and algorithmic challenges under real competition conditions.".to_string(),
                date: "".to_string(),
                link: "".to_string(),
                attachment: "".to_string(),
                tags: vec!["Algorithms".to_string(), "Competitive Programming".to_string(), "C++".to_string()],
            },
        ],
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
    };

    let template = ResumeTemplate { resume };
    let rendered = template.render().unwrap();
    state.cache_manager.set(key, rendered.clone()).await;
    log_visit(&state, "/resume", &ip).await;
    Html(rendered)
}

pub async fn resume2_handler(State(state): State<Arc<AppState>>) -> Html<String> {
    let key = state.cache_manager.make_key("/resume2");
    let ip = "127.0.0.1".to_string();
    if let Some(cached) = state.cache_manager.get(&key).await {
        log_visit(&state, "/resume2", &ip).await;
        return Html(cached);
    }

    let personal_info = PersonalInfo {
        name: "Ethan Cha".to_string(),
        phone: "(551) 223-9739".to_string(),
        email: "ethan.j.cha@gmail.com".to_string(),
        linkedin: "linkedin.com/in/ethan-cha-67639730b".to_string(),
    };

    let resume = crate::models::ResumeItem {
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
                skills: vec!["Java".to_string(), "Python".to_string()],
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
                description: "Published a research paper on Assessing Local Sources for PM2.5 in the NY Metro area using Conditional Probability Function Analysis. https://www.journalresearchhs.org/_files/ugd/ebf144_b1857e9968904942baabed016e618e89.pdf".to_string(),
                date: "2024".to_string(),
                link: "".to_string(),
            },
            Achievement {
                title: "Published Article in JSR Journal".to_string(),
                description: "Researched about use of KZ Filter technique to evaluate PM2.5 Air Quality long term trends in NJ. Submitted the journal to publish the article with the topic Use of Kolmogorov-Zurbenko Filter Technique to Evaluate PM 2.5 Air Quality Long-Term Trends in New Jersey. https://www.jsr.org/hs/index.php/path/article/view/4393".to_string(),
                date: "2023".to_string(),
                link: "".to_string(),
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
                title: "Volunteer Service".to_string(),
                description: "LIBERTY SCIENCE CENTER: Presented hands-on demonstrations and guided guests through STEM exhibits as part of the museum's volunteer team. MUSEUM OF MATH (MOMATH): Led interactive math activities and supported educational programming for families and youth visitors. YOUTH COUNCIL FORT LEE (YCFL): Organized large-scale community events, including the Fort Lee Color Run, and participated in civic outreach initiatives. CHODAE CHURCH: Drummer for the youth praise team, contributing to weekly worship services and church events.".to_string(),
                date: "700+ Hours".to_string(),
                link: "".to_string(),
                attachment: "".to_string(),
                tags: vec![],
            },
            Project {
                title: "Capture The Flag (CTF) Club - Leader".to_string(),
                description: "Spearheaded a student cybersecurity team to compete in CTF tournaments, solving challenges involving network intrusion, cryptography, and binary exploitation. Coached peers in ethical hacking tools and techniques; placed top 3 in school-wide competitions.".to_string(),
                date: "".to_string(),
                link: "".to_string(),
                attachment: "".to_string(),
                tags: vec!["Cybersecurity".to_string(), "Cryptography".to_string(), "Network Security".to_string()],
            },
            Project {
                title: "Tennis - Junior Varsity Captain".to_string(),
                description: "Served as captain for the JV team, coordinating practices, leading strategy discussions, and mentoring new players. Competed in regional league matches and trained alongside varsity athletes.".to_string(),
                date: "".to_string(),
                link: "".to_string(),
                attachment: "".to_string(),
                tags: vec![],
            },
            Project {
                title: "Rock Climbing - Competitive Youth Team Member".to_string(),
                description: "Competed in bouldering and sport climbing events at local and regional levels as part of a certified youth climbing team. Trained 3x weekly to improve strength, endurance, and technique under professional coaching.".to_string(),
                date: "".to_string(),
                link: "".to_string(),
                attachment: "".to_string(),
                tags: vec![],
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
            Project {
                title: "Columbia Competitive Programming Camp - Participant".to_string(),
                description: "Selected for a two-week intensive training program hosted by Columbia University, focused on advanced algorithms, graph theory, and dynamic programming. Collaborated with college students on timed problem sets and algorithmic challenges under real competition conditions.".to_string(),
                date: "".to_string(),
                link: "".to_string(),
                attachment: "".to_string(),
                tags: vec!["Algorithms".to_string(), "Competitive Programming".to_string(), "C++".to_string()],
            },
        ],
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
    };

    let template = Resume2Template { resume };
    let rendered = template.render().unwrap();
    state.cache_manager.set(key, rendered.clone()).await;
    log_visit(&state, "/canvas", &ip).await;
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
