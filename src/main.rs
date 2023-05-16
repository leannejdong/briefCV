

use std::fs::OpenOptions;
use std::io::Write;


fn main() {
    // Set up my data for the CV
    let name = "Dr Leanne Dong";
    let email = "leanne@spacetimenoise.dev";
        let education = vec![
            "PhD in Mathematics, Sydney University (2018)\n",
            "Bachelor of Science in Mathematics and Statistic (Hon 1, Medal), UTS\n",
            "Master of Science in Finance, UNSW\n",
            "Bachelor of Art and Business, Macquarie University\n",
        ]
    .join("\n");

    let work = vec![
            "Independent Software Engineer at SpaceTimeNoise, 2021-Current (Multiple clients)\n",
            "C++ Linux developer at tonicbits, 2022\n",
            "Postdoctoral researcher in Computer Science and Engineering at UTS and Concordia Uni, 2019-2022\n",
        "Casual Lecturer and Tutor (UTS, USYD, ACU), 2010-2019\n",
    ]
    .join("\n");

    let skills = vec![
        "C++",
        "Linux",
        "JUCE",
        "Boost",
        "Qt",
        "C",
        "Rust",
        "Python",
        "R",
        "JavaScript, HTML, CSS",
        "Git",
        "Linux (Arch, Ubuntu)",
        "Docker",
        "AWS",
        "Social Media",
        "Mathematics",
        "Statistics",
        "Machine Learning",
        "Data Analytics",
    ];

    let skills_str = skills
        .iter()
        .map(|s| format!("- {}\n", s))
        .collect::<String>();

    // Format the data as a Markdown string
    let markdown = format!(
        "# {}\n\nEmail: {}\n\n**Education:**\n\n{}\n\n**Work:**\n\n{}\n\n**Skills:**\n\n{}",
        name, email, education, work, skills_str
    );

    // Write the Markdown string to a file
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("CV.txt")
        .unwrap();
    file.write_all(markdown.as_bytes()).unwrap();

}
