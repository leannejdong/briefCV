use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    // Set up my data for the CV
    let name = "Leanne Dong";
    let email = "jdleanne@levynoise.com";
    let education = format!(
        "{:#?}",
    vec!["PhD in Mathematics, Sydney University",
    "Bachelor of Science in Mathematics and Statistic (Hon 1, Medal), UTS",
    "Master of Science in Finance, UNSW",
    "Bachelor of Art and Business, Macquarie University "]);

    let work= format!(
        "{:#?}",
        vec!["Independent Software Engineer at SpaceTimeNoise, 2022-Current",
    "C++ Linux developer at tonicbits, 2022",
    "Postdoctoral researcher in Computer Science and Engineering at UTS and Concordia Uni, 2019-2022",
    "Casual Lecturer and Tutor (UTS, USYD, ACU), 2010-2019"
    ]);
    let skills = vec![
        "C++",
        "C",
        "Rust",
        "Python",
        "R",
        "JavaScript",
        "HTML",
        "CSS",
        "Git",
        "Linux",
        "Docker",
        "AWS",
        "Social Media",
        "Mathematics",
        "Statistics",
        "Machine Learning",
        "Data Analytics"
    ];

    // Format the data as a Markdown string
    let markdown = format!(
        "# {}\n\nEmail: {}\n\nEducation: {}\n\nWork: {}\n\nSkills:\n\n{}",
        name,
        email,
        education,
        work,
        skills
            .iter()
            .map(|s| format!("- {}\n", s))
            .collect::<String>()
    );

    // Write the Markdown string to a file
    // let mut file = File::create("../README.md").unwrap();
    // file.write_all(markdown.as_bytes()).unwrap();

    let mut file = OpenOptions::new()
    .write(true)
    .truncate(true)
    .create(true)
    .open("../README.md")
    .unwrap();
file.write_all(markdown.as_bytes()).unwrap();
}
