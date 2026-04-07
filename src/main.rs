use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    // Personal Details
    let name = "Leanne J Dong";
    let location = "Brisbane, QLD";
    let email = "jdleanne@gmail.com";
    let phone = "0412908292";
    let github = "https://github.com/leannejdong";
    let linkedin = "https://www.linkedin.com/in/leanne-j-dong/";

    // Summary
    let summary = "Highly analytical IT professional and C++ Developer with a PhD in Mathematics \
and over 10 years of experience in technical problem-solving, high-performance computing, and \
systems engineering. Currently in final month completing Advanced Diploma in IT (Networking and \
Cloud Engineering) to bridge deep academic research with enterprise infrastructure. Expertise in \
Linux environments, audio software development, and real-time systems, with a proven track record \
of delivering complex software solutions across fintech, robotics, and music technology sectors.";

    // Education
    let education = vec![
        "Diploma of IT (Advanced Networking & Cloud Engineering) — Expected 2026",
        "Ph.D. in Mathematics (Theoretical Probability) — University of Sydney",
        "Bachelor of Mathematics (First Class Honours with Medal) — UTS",
        "CCNA 200-301 — In Progress",
        "Master of Finance in Investment Banking",
        "Bachelor of Commerce (double degree: Economics and Accounting)",
        "Google IT Support Professional Certificate (5 certificates issued by Google)",
    ];

    // Work Experience
    let work = vec![
        (
            "IT Lead & Freelance C++ Developer",
            "2024 – Present",
            "Independent / Non-Profit Sector",
            vec![
                "Infrastructure Strategy: Oversee general IT infrastructure, implementing policies \
and procedures to ensure secure and scalable ICT services for non-profit operations.",
                "Full-Stack Development: Developed custom software solutions including an Online \
Beauty App for beauticians and Brisbane Job Helper app using TypeScript / React Native.",
                "Audio Engineering (JUCE): Specialised in JUCE framework development for \
VST3/AU plugins, including music information retrieval and Linux compatibility for Scaler2.",
                "Systems Administration: Manage a sophisticated home lab environment utilising \
Synology NAS, Docker, Tailscale, and Cloudflare Tunnels to simulate enterprise networking scenarios.",
                "Cloud & Networking: Currently deploying cloud-based services and managing \
virtualised server environments (Windows/Linux) as part of advanced networking certification.",
                "Key Achievement: Successfully migrated domain and DNS management to Cloudflare, \
optimising web presence and security.",
                "Key Achievement: Developed and deployed multiple audio-centric C++ applications \
for real-time signal processing.",
            ],
        ),
        (
            "Postdoctoral Research Fellow / C++ Developer",
            "2020 – 2023",
            "Concordia University / Various Industries (Robotics, Fintech, Music Tech)",
            vec![
                "Software Engineering: Engineered backend sparse decision tree C++ solutions for \
logistics and fintech, focusing on low-latency, high-performance algorithms and cross-platform deployment.",
                "Scientific Computing: Developed a nonlinear equation system solver (SimSolve-Liu) \
using Modern C++ (17/20) and Eigen3, applying advanced mathematical modelling to circuit network automation.",
                "Key Achievement: Developed a minimal sparse tensor library and various neural \
network implementations in C++.",
                "Key Achievement: Architected a circuit network simulation tool using undirected \
graph approaches.",
            ],
        ),
        (
            "Lead Robotic Coding Instructor",
            "2023 – 2025",
            "Junior Engineers",
            vec![
                "Delivered hands-on robotics and coding curriculum to K–12 students.",
            ],
        ),
        (
            "IT Manager & IoT Coordinator / Data Engineer",
            "2024",
            "Energy Skills Queensland",
            vec![
                "Managed IT operations and coordinated IoT infrastructure for Queensland's \
energy skills training organisation.",
            ],
        ),
        (
            "Sessional Lecturer & Tutor",
            "2011 – 2019",
            "University of Sydney / UTS / ACU",
            vec![
                "Advanced Instruction: Delivered curriculum in Applied Mathematics, Real Analysis, \
and Machine Learning to high-performing STEM students.",
                "Research Associate: Conducted data-driven research in Computer Social Science at \
FEIT (UTS), leveraging statistical modelling and large-scale data analysis.",
            ],
        ),
    ];

    // Technical Skills
    let skills = vec![
        ("Languages", "C++ (98 through 20), Rust, Python, TypeScript (React Native), R, MATLAB"),
        ("Systems & Tools", "Linux (all variants), Windows Server, Docker, Git, CMake, JUCE, Ninja, MSVC"),
        ("CCNA level Networking", "Server Virtualisation, Cloudflare, Private VPN, Tailscale"),
        ("Cloud & DevOps", "AWS, GCP, Docker, Agile / Jira, CI/CD"),
        ("Specialties", "Audio plugin programming, real-time signal processing, cross-platform software development, networking, Cloud networking, operating systems, system administration, cyber security"),
    ];

    // ── Build Markdown ────────────────────────────────────────────────────────

    let mut md = String::new();

    // Header
    md.push_str(&format!("# {}\n\n", name));
    md.push_str(&format!(
        "📧 [{}](mailto:{}) | 📞 {} | 📍 {} | 🇦🇺 Australian Citizen | 🔗 [{}]({})\n\n",
        email, email, phone, location, github, github
    ));

    // Summary
    md.push_str("## Summary\n\n");
    md.push_str(summary);
    md.push_str("\n\n");

    // Work Experience
    md.push_str("## Work Experience\n\n");
    for (title, period, org, bullets) in &work {
        md.push_str(&format!("### {} | {}\n", title, period));
        md.push_str(&format!("**{}**\n\n", org));
        for b in bullets {
            md.push_str(&format!("- {}\n", b));
        }
        md.push('\n');
    }

    // Education
    md.push_str("## Education & Certifications\n\n");
    for item in &education {
        md.push_str(&format!("- {}\n", item));
    }
    md.push('\n');

    // Skills
    md.push_str("## Technical Skills\n\n");
    for (category, detail) in &skills {
        md.push_str(&format!("- **{}:** {}\n", category, detail));
    }
    md.push('\n');

    // ── Write README.md ───────────────────────────────────────────────────────

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("README.md")
        .unwrap();

    println!("Writing to README.md...");
    file.write_all(md.as_bytes()).unwrap();
    println!("README.md updated successfully.");

    // ── (Optional) Convert to PDF via Pandoc ─────────────────────────────────
    // Uncomment when pandoc is available in the build environment:
    //
    // use std::process::Command;
    // let output = Command::new("pandoc")
    //     .args(&["README.md", "-o", "CV.pdf"])
    //     .output()
    //     .expect("Failed to execute Pandoc command");
    //
    // if output.status.success() {
    //     println!("PDF generated: CV.pdf");
    // } else {
    //     eprintln!("Pandoc error: {}", String::from_utf8_lossy(&output.stderr));
    // }
}
