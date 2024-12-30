use anyhow::{Context, Result};
use dialoguer::{MultiSelect, theme::ColorfulTheme, Confirm};
use git2::Repository;

fn get_local_branches(repo: &Repository) -> Result<Vec<String>> {
    let branches = repo.branches(Some(git2::BranchType::Local))?;
    let mut branch_names = Vec::new();
    
    for branch in branches {
        let (branch, _) = branch?;
        if let Some(name) = branch.name()? {
            branch_names.push(name.to_string());
        }
    }
    
    Ok(branch_names)
}

fn delete_branch(repo: &Repository, branch_name: &str) -> Result<()> {
    let mut branch = repo.find_branch(branch_name, git2::BranchType::Local)?;
    branch.delete().context(format!("Failed to delete branch '{}'", branch_name))?;
    Ok(())
}

fn main() -> Result<()> {
    // Open the repository
    let repo = Repository::open(".")?;
    
    // Get list of local branches
    let branches = get_local_branches(&repo)
        .context("Failed to get branch list")?;
    
    // Get current branch (to exclude it from deletion)
    let head = repo.head()?;
    let current_branch = head.shorthand().unwrap_or("HEAD");
    
    // Filter only branches available for deletion
    let available_branches: Vec<String> = branches
        .into_iter()
        .filter(|b| b != current_branch)
        .collect();
    
    if available_branches.is_empty() {
        println!("No branches available for deletion.");
        return Ok(());
    }

    // Interactive selection UI
    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select branches to delete (Space to select, Enter to confirm)")
        .items(&available_branches)
        .interact()?;
    
    if selections.is_empty() {
        println!("No branches selected.");
        return Ok(());
    }

    // Show confirmation for branch deletion
    println!("\nThe following branches will be deleted:");
    for &idx in selections.iter() {
        println!("- {}", available_branches[idx]);
    }
    
    let confirmed = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Are you sure you want to delete these branches?")
        .default(false)
        .interact()?;

    if !confirmed {
        println!("Operation cancelled.");
        return Ok(());
    }

    let mut success_count = 0;
    let mut error_branches = Vec::new();

    for &idx in selections.iter() {
        let branch_name = &available_branches[idx];
        match delete_branch(&repo, branch_name) {
            Ok(_) => {
                success_count += 1;
                println!("✓ Deleted '{}'", branch_name);
            }
            Err(e) => {
                error_branches.push((branch_name, e));
                eprintln!("✗ Failed to delete '{}'", branch_name);
            }
        }
    }

    println!("\nResults:");
    println!("Success: {} branch(es)", success_count);
    
    if !error_branches.is_empty() {
        println!("Failed: {} branch(es)", error_branches.len());
        println!("\nError details:");
        for (branch, error) in error_branches {
            eprintln!("- {}: {}", branch, error);
        }
    }

    Ok(())
}
