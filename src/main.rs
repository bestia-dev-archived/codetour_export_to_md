// region: lmake_readme include "readme.md" //! A

// endregion: lmake_readme include "readme.md" //! A

use ansi_term::Colour::{Green, Red, Yellow};
use clap::App;
use glob::glob;
use serde_derive::{Deserialize, Serialize};
use std::env;
use std::fs;
use unwrap::unwrap;

#[derive(Serialize, Deserialize, Clone)]
pub struct Position {
    line: usize,
    character: usize,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Selection {
    start: Position,
    end: Position,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Step {
    file: String,
    line: usize,
    description: String,
    selection: Option<Selection>,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Tour {
    tour: String,
    title: String,
    steps: Vec<Step>,
    description: String,
    // additional fields that I suggest for github
    github_url: String,
    github_user: String,
    github_repo: String,
    github_branch: String,
}

/// the default is the subfolder .tour inside the current folder
/// for other cases add the argument "folder"
fn main() {
    // this function is different for Windows and for Linux.
    // Look at the code of this function (2 variations).
    enable_ansi_support();
    println!("Export all .tour/*.tour to md");
    // define the CLI input line parameters using the clap library
    let arguments = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            clap::Arg::with_name("folder")
                .value_name("folder")
                .long("folder")
                .help("--folder=example/.tours"),
        )
        .get_matches();

    let folder = match arguments.value_of("folder") {
        Some(folder) => folder,
        None => ".tours",
    };
    println!("folder: {}", &folder);
    //find all files in tour/*.tour
    for filename_result in unwrap!(glob(&format!("{}/*.tour", folder))) {
        let filename_pathbuff = unwrap!(filename_result);
        let filename_tour = unwrap!(filename_pathbuff.to_str());
        println!("file tour: {}", Green.paint(filename_tour));
        // read tour file
        let tour = unwrap!(fs::read_to_string(filename_tour));
        let text_len = tour.len();
        let tour: Tour = unwrap!(serde_json::from_str(&tour));
        let mut md_text = String::with_capacity(text_len * 4);

        md_text.push_str(&format!("# {}\n", &tour.title));
        md_text.push_str(&format!("{}\n", &tour.description));
        for (step_number, step) in tour.steps.iter().enumerate() {
            //enumerator is zero-based.
            // I need one-based.
            let step_number = step_number + 1;
            // the step description is not really markdown
            // temporary I have to escape the <> symbols
            let description = step.description.replace("<", "\\<").replace(">", "\\>");
            // inside the description is also the step title with ###
            md_text.push_str(&format!("{}\n\n", &description));

            md_text.push_str(&format!(
                "##### step {} of {} ",
                step_number,
                tour.steps.len()
            ));
            md_text.push_str(&format!(
                "[View code in GitHub]({}/{}/{}/blob/{}/{}#L{})\n",
                tour.github_url,
                tour.github_user,
                tour.github_repo,
                tour.github_branch,
                step.file,
                step.line,
            ));

            //open the file and take 10 lines before line
            let filename_code = format!("{}/{}", folder.replace("/.tours", ""), &step.file);
            println!("file code: {}", &filename_code);
            let step_code = unwrap!(fs::read_to_string(&filename_code));
            md_text.push_str(&delimiter_for_code_start(&filename_code));
            for (i, line) in step_code.lines().enumerate() {
                // the enumerator is zero-based.
                // I would like here one-based.
                let i = i + 1;
                // selection of code is optional
                if let Some(selection) = &step.selection {
                    if i < selection.start.line && i < selection.end.line - 10 {
                        //nothing
                    } else if i < selection.start.line {
                        md_text.push_str(line);
                        md_text.push_str("\n");
                    } else if i <= selection.end.line {
                        // I need a way to show the user selection
                        if i == selection.start.line {
                            md_text.push_str(
                            "#//---------------------- selection start ----------------------\n",
                        );
                        }
                        md_text.push_str(line);
                        md_text.push_str("\n");
                        if i == selection.end.line {
                            md_text.push_str(
                            "#//----------------------- selection end -----------------------\n",
                        );
                        }
                    } else {
                        md_text.push_str("```\n");
                        break;
                    }
                } else {
                    // selection=none; write 10 lines prior to step line
                    if i < step.line - 10 {
                        //nothing
                    } else if i <= step.line {
                        md_text.push_str(line);
                        md_text.push_str("\n");
                    } else {
                        md_text.push_str("```\n");
                        break;
                    }
                }
            }
        }
        //save the file
        let filename_md = format!("{}.md", &tour.tour);
        println!("saved md: {}", Green.paint(&filename_md));
        let _x = fs::write(&format!("{}.md", filename_md), md_text);
    }
    println!("Export ended");
}

/// return md code definition from file extension
pub fn delimiter_for_code_start(filename_code: &str) -> String {
    let pos = filename_code.rfind('.');
    let lang = match pos {
        Some(pos) => {
            let file_extension = &filename_code[pos + 1..];
            if file_extension == "rs" {
                "```rust".to_string()
            } else {
                format!("```{}", file_extension)
            }
        }
        None => format!("```{}", ""),
    };
    //return
    format!("{}\n", &lang)
}

pub fn export_all_tours() {}

// region: different function code for Linux and Windows
#[cfg(target_family = "windows")]
/// only on windows "enable ansi support" must be called
pub fn enable_ansi_support() {
    let _enabled = ansi_term::enable_ansi_support();
}

#[cfg(target_family = "unix")]
//on Linux "enable ansi support" must not be called
pub fn enable_ansi_support() {
    // do nothing
}
// endregion
