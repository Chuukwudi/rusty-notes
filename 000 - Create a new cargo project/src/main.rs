use std::process::Command;
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let topic = "32 - Using a Debugger";

    let folder_name = &topic[5..]
        .to_lowercase()
        .replace(" ", "_")
        .replace("-", "_")
        .replace("(", "")
        .replace(")", "");
    println!("{}", folder_name);

    // Create the new project
    let output = Command::new("cargo")
        .arg("new")
        .arg(topic)
        .arg("--name")
        .arg(folder_name)
        .arg("--vcs")
        .arg("none")
        .current_dir("/app/") 
        .output()?;

    if !output.status.success() {
        eprintln!("Failed to create project: {}", String::from_utf8_lossy(&output.stderr));
        return Err(Box::new(io::Error::new(io::ErrorKind::Other, "Failed to create project")));
    }
    
    println!("Successfully created cargo project: {}", topic);
    
    Ok(())
}