use std::process::Command;
use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let topic = "29 - File Size Formatter";

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
    
    // Now update the Cargo.toml file
    let toml_path = Path::new("/app/Cargo.toml");
    
    // Read the current toml file
    let mut toml_content = String::new();
    let mut file = fs::File::open(&toml_path)?;
    file.read_to_string(&mut toml_content)?;
    
    // Check if the project is already in the members list
    if !toml_content.contains(&format!("\"{}\"", topic)) {
        // Find the last member in the list
        if let Some(last_bracket_pos) = toml_content.rfind(']') {
            // Check if there are existing members
            let insert_position = if toml_content[..last_bracket_pos].trim().ends_with(',') || 
                                   !toml_content[..last_bracket_pos].contains('"') {
                // No comma needed, just add the line
                last_bracket_pos
            } else {
                // Add a comma before adding the new member
                let mut new_content = toml_content.clone();
                new_content.insert(last_bracket_pos, ',');
                toml_content = new_content;
                last_bracket_pos + 1
            };
            
            // Insert the new project into the members list
            let mut new_content = toml_content.clone();
            new_content.insert_str(insert_position, &format!("\n    \"{}\"", topic));
            
            // Write the updated content back to the file
            let mut file = fs::File::create(&toml_path)?;
            file.write_all(new_content.as_bytes())?;
            
            println!("Successfully updated Cargo.toml with the new project");
        } else {
            eprintln!("Failed to find closing bracket in Cargo.toml");
            return Err(Box::new(io::Error::new(io::ErrorKind::Other, "Invalid Cargo.toml format")));
        }
    } else {
        println!("Project already exists in Cargo.toml");
    }
    
    Ok(())
}