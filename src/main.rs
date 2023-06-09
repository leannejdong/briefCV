

use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;

fn main() {
    // Set up my data for the CV
    let name = "Dr Leanne Dong";
    let email = "leanne@spacetimenoise.dev";
        let education= vec![
            "PhD in Mathematics, Sydney University (2018)\n",
            "Bachelor of Science in Mathematics (Hon 1, Medal), UTS\n",
            "Master of Science in Finance, UNSW\n",
            "Bachelor of Art and Business, Macquarie University\n",
        ]
    .join("\n");

    let work = vec![
        "- Independent Software Engineer at SpaceTimeNoise, 2021-Current 
        (Multiple NDA clients focus on Audio, Data Science, Deep Learning, 
        Real Time Low latency application)",
        "- C++ Linux developer at tonicbits, 2022",
        "- Postdoctoral researcher in Computer Science and Engineering at UTS and Concordia Uni, 2019-2022",
        "- Casual Lecturer and Tutor (UTS, USYD, ACU), 2010-2019",
    ]
    .join("\n");


    let skills: Vec<&str> = vec![
        "Specialties: Cross platform custom software development",
        "Language: C++, Python, Rust, C, MATLAB, R, JS/HTML/CSS/React",
        "Build tools: CMake, Ninja, MSVC, vscode, neovim",
        "Frameworks: JUCE, Boost, Qt",
        "Git, Jira, Agile, AWS, Docker",
        "Linux (Arch, Ubuntu)",
        "Code refactoring, debugging, testing",
        "Mathematics",
        "Statistics",
        "System Programming",
        "Deep (Machine) Learning",
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
        .open("README.md")
        .unwrap();
    file.write_all(markdown.as_bytes()).unwrap();

     // Convert Markdown to Word document
    let input_file = "README.md";
    let output_file = "CV.docx";

    let output = Command::new("pandoc")
    .args(&[input_file, "-o", output_file])
    .output()
    .expect("Failed to execute Pandoc command");

if output.status.success() {
    println!("Conversion successful! Output file: {}", output_file);
} else {
    let error_message = String::from_utf8_lossy(&output.stderr);
    println!("Conversion failed. Error: {}", error_message);
}
}
