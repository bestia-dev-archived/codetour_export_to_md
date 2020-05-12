# export_to_md
## start arguments

The default is to search in the .tours folder.

##### step 1 of 10 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/main.rs#L76)
```rust
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
```
## export all tours files

The function will export all the tours files in one go.

##### step 2 of 10 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/main.rs#L79)
```rust
                .long("folder")
                .help("--folder=example/.tours"),
        )
        .get_matches();

    let folder = match arguments.value_of("folder") {
        Some(folder) => folder,
        None => ".tours",
    };
    println!("folder: {}", folder);
    lib_internal::export_all_tours(folder);
```
## tour_extended.json

Some extended data is in a separate file. Mostly to make links to github.

##### step 3 of 10 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/lib_internal.rs#L68)
```rust
        }
        None => format!("```{}", ""),
    };
    //return
    format!("{}\n", &lang)
}

pub fn export_all_tours(folder: &str) {
    //read the tour_extended.json
    let tour_extended = unwrap!(fs::read_to_string(&format!(
        "{}/tour_extended.json",
```
## read file
Read and deserialize the tour file. It is json.

##### step 4 of 10 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/lib_internal.rs#L76)
```rust
    //read the tour_extended.json
    let tour_extended = unwrap!(fs::read_to_string(&format!(
        "{}/tour_extended.json",
        folder
    )));
    let tour_extended: TourExtended = unwrap!(serde_json::from_str(&tour_extended));

    //find all files in tour/*.tour
    for filename_result in unwrap!(glob(&format!("{}/*.tour", folder))) {
        let filename_pathbuff = unwrap!(filename_result);
        let filename_tour = unwrap!(filename_pathbuff.to_str());
```
## header
Data of the header and description.

##### step 5 of 10 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/lib_internal.rs#L82)
```rust

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
```
## step header
Every step has a number and a link to github.

##### step 6 of 10 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/lib_internal.rs#L97)
```rust
        }
        for (step_number, step) in tour.steps.iter().enumerate() {
            //enumerator is zero-based.
            // I need one-based.
            let step_number = step_number + 1;
            // the step description is not really markdown
            // temporary I have to escape the <> symbols
            let description = step.description.replace("<", "\\<").replace(">", "\\>");
            // inside the description is also the step title with ###
            md_text.push_str(&format!("{}\n\n", &description));

```
## source code file
Read the source code.

##### step 7 of 10 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/lib_internal.rs#L112)
```rust
                "##### step {} of {} ",
                step_number,
                tour.steps.len()
            ));
            md_text.push_str(&format!(
                "[View code in GitHub]({}/{}/{}/blob/{}/{}#L{})\n",
                tour_extended.github_url,
                tour_extended.github_user,
                tour_extended.github_repo,
                tour_extended.github_branch,
#//---------------------- selection start ----------------------
                step.file,
#//----------------------- selection end -----------------------
```
## selection fragment
Take lines defined in selection. At least 10 lines. Added delimiters to make selection visible.

##### step 8 of 10 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/lib_internal.rs#L135)
```rust
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
#//---------------------- selection start ----------------------
                        if i == selection.end.line {
#//----------------------- selection end -----------------------
```
## without selection
If there is no selection, use the line number.

##### step 9 of 10 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/lib_internal.rs#L148)
```rust
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
```
## Save
Save the md file.

##### step 10 of 10 [View code in GitHub](https://github.com/LucianoBestia/cargo_crev_web/blob/master/src/lib_internal.rs#L163)
```rust
                        md_text.push_str("\n");
                    } else {
                        md_text.push_str("```\n");
                        break;
                    }
                }
            }
        }
        //save the md file with same name
        let spl = filename_tour.split("/");
        let name = unwrap!(spl.last());
```
