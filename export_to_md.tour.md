# export_to_md
## start arguments

The default is to search in the .tours folder.

##### step 1 of 10 [View code in GitHub](https://github.com/LucianoBestia/codetour_export_to_md/blob/master/src/main.rs#L104)
```rust
            clap::Arg::with_name("folder")
                .value_name("folder")
                .long("folder")
                .help("--folder=example/.tours"),
        )
        .get_matches();

#//---------------------- selection start ----------------------
    let folder = match arguments.value_of("folder") {
        Some(folder) => folder,
        None => ".tours",
    };
#//----------------------- selection end -----------------------
```
## export all tours files

The function will export all the tours files in one go.

##### step 2 of 10 [View code in GitHub](https://github.com/LucianoBestia/codetour_export_to_md/blob/master/src/main.rs#L106)
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
#//---------------------- selection start ----------------------
    lib_internal::export_all_tours(folder);
#//----------------------- selection end -----------------------
```
## tour_extended.json

Some extended data is in a separate file. Mostly to make links to github.

##### step 3 of 10 [View code in GitHub](https://github.com/LucianoBestia/codetour_export_to_md/blob/master/src/lib_internal.rs#L68)
```rust
        }
        None => format!("```{}", ""),
    };
    //return
    format!("{}\n", &lang)
}

pub fn export_all_tours(folder: &str) {
    //read the tour_extended.json
#//---------------------- selection start ----------------------
    if let Ok(tour_extended) = fs::read_to_string("tour_extended.json"){
        let tour_extended: TourExtended = unwrap!(serde_json::from_str(&tour_extended));
#//----------------------- selection end -----------------------
```
## read file
Read and deserialize the tour file. It is json.

##### step 4 of 10 [View code in GitHub](https://github.com/LucianoBestia/codetour_export_to_md/blob/master/src/lib_internal.rs#L79)
```rust
    if let Ok(tour_extended) = fs::read_to_string("tour_extended.json"){
        let tour_extended: TourExtended = unwrap!(serde_json::from_str(&tour_extended));

        //find all files in tour/*.tour
        for filename_result in unwrap!(glob(&format!("{}/*.tour", folder))) {
            let filename_pathbuff = unwrap!(filename_result);
            let filename_tour = unwrap!(filename_pathbuff.to_str());
            println!("file tour: {}", Green.paint(filename_tour));
            // read tour file
            // todo: this is expect!
#//---------------------- selection start ----------------------
            let tour = unwrap!(fs::read_to_string(filename_tour),"Glob just gave me this filename, it cannot panic.");
#//----------------------- selection end -----------------------
```
## header
Data of the header and description.

##### step 5 of 10 [View code in GitHub](https://github.com/LucianoBestia/codetour_export_to_md/blob/master/src/lib_internal.rs#L86)
```rust
            let filename_tour = unwrap!(filename_pathbuff.to_str());
            println!("file tour: {}", Green.paint(filename_tour));
            // read tour file
            // todo: this is expect!
            let tour = unwrap!(fs::read_to_string(filename_tour),"Glob just gave me this filename, it cannot panic.");
            let text_len = tour.len();
            let tour: Tour = unwrap!(serde_json::from_str(&tour));
            let mut md_text = String::with_capacity(text_len * 4);
#//---------------------- selection start ----------------------

            md_text.push_str(&format!("# {}\n", &tour.title));
            if let Some(description) = &tour.description {
#//----------------------- selection end -----------------------
```
## step header
Every step has a number and a link to github.

##### step 6 of 10 [View code in GitHub](https://github.com/LucianoBestia/codetour_export_to_md/blob/master/src/lib_internal.rs#L102)
```rust
                let description = step.description.replace("<", "\\<").replace(">", "\\>");
                // inside the description is also the step title with ###
                md_text.push_str(&format!("{}\n\n", &description));

#//---------------------- selection start ----------------------
                md_text.push_str(&format!(
                    "##### step {} of {} ",
                    step_number,
                    tour.steps.len()
                ));
                md_text.push_str(&format!(
                    "[View code in GitHub]({}/{}/{}/blob/{}/{}#L{})\n",
#//----------------------- selection end -----------------------
```
## source code file
Read the source code.

##### step 7 of 10 [View code in GitHub](https://github.com/LucianoBestia/codetour_export_to_md/blob/master/src/lib_internal.rs#L119)
```rust
                    tour_extended.github_url,
                    tour_extended.github_user,
                    tour_extended.github_repo,
                    tour_extended.github_branch,
                    step.file,
                    step.line,
                ));

#//---------------------- selection start ----------------------
                //open the file and take 10 lines before line
                let filename_code = format!("{}{}", folder.replace(".tours", ""), &step.file);
                if let Ok(step_code) = fs::read_to_string(&filename_code){
#//----------------------- selection end -----------------------
```
## selection fragment
Take lines defined in selection. At least 10 lines. Added delimiters to make selection visible.

##### step 8 of 10 [View code in GitHub](https://github.com/LucianoBestia/codetour_export_to_md/blob/master/src/lib_internal.rs#L145)
```rust
                                // I need a way to show the user selection
                                if i == selection.start.line {
#//---------------------- selection start ----------------------
                                    md_text.push_str(
                                    "#//---------------------- selection start ----------------------\n",
                                );
                                }
                                md_text.push_str(line);
                                md_text.push_str("\n");
                                if i == selection.end.line {
                                    md_text.push_str(
                                    "#//----------------------- selection end -----------------------\n",
#//----------------------- selection end -----------------------
```
## without selection
If there is no selection, use the line number.

##### step 9 of 10 [View code in GitHub](https://github.com/LucianoBestia/codetour_export_to_md/blob/master/src/lib_internal.rs#L153)
```rust
                                md_text.push_str("```\n");
                                break;
                            }
                        } else {
#//---------------------- selection start ----------------------
                            // selection=none; write 10 lines prior to step line
                            if i < step.line - 10 {
                                //nothing
                            } else if i <= step.line {
                                md_text.push_str(line);
                                md_text.push_str("\n");
                            } else {
#//----------------------- selection end -----------------------
```
## Save
Save the md file.

##### step 10 of 10 [View code in GitHub](https://github.com/LucianoBestia/codetour_export_to_md/blob/master/src/lib_internal.rs#L168)
```rust
                 } else {
                    println!("File does not exist: {}", Red.paint(&filename_code));
                 }
            }
            //save the md file with same name
            let spl = filename_tour.split("/");
            let name = unwrap!(spl.last());
            let name = name.to_string();
            let filename_md = format!("{}.md", name);
            println!("saved md: {}", Green.paint(&filename_md));
#//---------------------- selection start ----------------------
            let _x = fs::write(&format!("{}", filename_md), md_text);
#//----------------------- selection end -----------------------
```
