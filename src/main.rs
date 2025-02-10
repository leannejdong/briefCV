use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;

fn main() {
    // Set up my data for the CV
    let name = "Dr Leanne J Dong";
    let location = "Brisbane, Australia";
    let github = "https://github.com/leannejdong";
    let linkedin = "https://www.linkedin.com/in/leanne-j-dong/";
    let email = "leanne@spacetimeit.com";
        let education= vec![
            "Diploma of IT in advanced networking and cloud engineering (2024-Current, part time)\n",
            "PhD in Mathematics, Sydney University (2018)\n",
            "Bachelor of Science in Mathematics (Hon 1, Medal), UTS\n",
            "Master of Science in Finance, UNSW\n",
            "Bachelor of Art and Business, Macquarie University\n",
            "Google technical support technician certificate\n",
            "Conputer networking, Operating system power user, system admin and infrastructure service management (issued by Google)"
        ]
    .join("\n");

    let work = vec![
        "- Tech support/Assistant instructor at Junior Engineers, 2023-2024\n",
        "- IT manager and IoT Coordinator at Energy Skills Queensland, 2024\n",
        "- Independent IT Manager and Software Engineer at SpaceTimeIT, 2021-Current \n",
        "- C++ Linux developer at tonicbits, 2022\n",
        "- Postdoctoral researcher in Computer Science and Engineering at UTS and Concordia Uni, 2019-2022\n",
        "- Casual Lecturer and Tutor (UTS, USYD, ACU), 2010-2019\n",
    ]
    .join("\n");


    let skills: Vec<&str> = vec![
        "Specialties: Trouble shooting (microsoft 365, software deployment techniques), customer service, windows/linux servers, virtualization, cloud, networking, operating system, system admin,cyber security, Cross platform custom software development and design\n",
        "Language: C++, Python, Rust, C, Java, C#, R, JS/HTML/CSS/React\n",
        "Build tools: CMake, Ninja, MSVC, vscode, neovim\n",
        "Git, Jira, Agile, AWS, Docker\n",
        "Linux (Arch, Ubuntu), system programming\n",
        "Code refactoring, debugging, testing\n",
        "Mathematics, Satistics, Data Analytics\n",
    ];


    let skills_str = skills
        .iter()
        .map(|s| format!("- {}\n", s))
        .collect::<String>();

    // Format the data as a Markdown string
    let markdown = format!(
        "# {}\n\n**Location:** {}\n\n**Github:** {}\n\n**Linkedin:** {}\n\n**Email:** {}\n\n**Education:** {}\n\n**Work:** {}\n\n**Skills:** {}",
        name, location, github, linkedin, email, education, work, skills_str
    );

    // Write the Markdown string to a file
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("README.md")
        .unwrap();
    println!("Writing to README.md...");
    file.write_all(markdown.as_bytes()).unwrap();
    println!("README.md updated successfully.");

     // Convert Markdown to Word document
    let input_file = "README.md";
    let output_file = "CV.pdf";

/*    
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
*/
}
