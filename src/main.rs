use std::fs::File;
use std::io::Write;

fn main() {
    // Set up my data for the CV
    let name = "Leanne Dong";
    let email = "jdleanne@levynoise.com";
    let education = "Bachelor of Science, Commerce and Art in Computer Science, Macquarie University";
    let skills = vec![
        "Rust",
        "C++",
        "Python",
        "R",
        "JavaScript",
        "HTML",
        "CSS",
        "Git",
        "Linux",
        "Docker",
    ];

    // Format the data as a Markdown string
    let markdown = format!(
        "# {}\n\nEmail: {}\n\nEducation: {}\n\nSkills:\n\n{}",
        name,
        email,
        education,
        skills
            .iter()
            .map(|s| format!("- {}\n", s))
            .collect::<String>()
    );

    // Write the Markdown string to a file
    let mut file = File::create("cv.md").unwrap();
    file.write_all(markdown.as_bytes()).unwrap();
}
