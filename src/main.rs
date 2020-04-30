// region: lmake_readme include "readme.md" //! A
//! # codetour_export_to_md
//! 
//! version: 0.1.27  date: 2020-04-30 authors: Luciano  
//! **Export CodeTour files to md**
//! 
//! 
//! CodeTour is a fantastic extension for VSCode. Kudos to the authors.  
//! <https://marketplace.visualstudio.com/items?itemName=vsls-contrib.codetour>  
//! Every CodeTour contains steps.  
//! Each step has a description and a link to the file and line number in the source code.  
//! No more out of sync examples in the documentation.  
//! This type of documentation is meant to be used for code flow explanation.  
//! To show other programmers the important information step by step in a logical order for humans.  
//! This extension for now works only inside VSCode. That is a problem for coders with other editors.  
//! 
//! ## markdown
//! 
//! In the present version (2020-04-27) the extension has no functionality to export to a markdown file.  
//! I don't have enough knowledge in vs code extensions and Typescript to make a PR contribution.  
//! So I make a rust tiny small CLI program.  
//! I prepared this project as a proof of concept how the "export to md" could look like.  
//! The resulting md is very nice. It is a file and therefore it can be committed to Github.  
//! In the md there are links to the source code on Github.  
//! This way all coders can follow the code flow on the actual code.  
//! 
//! ## example
//! 
//! I copied to the folder example/ a few files from my other project where I use CodeTour.  
//! There are 2 similar *.tour files. The CLI will export all tours files in that folder.  
//! Without any arguments the CLI will look at the standard `.tour/` folder.  
//! If the files are in another folder, like for my example, the argument is like this:  
//! `codetour_export_to_md folder=example/.tours`  
//! 
//! ## GitHub and working example
//! 
//! In my other project I tried to write some documentation about the code flow.  
//! It was horrific.  
//! I avoided copy/paste the source code because in no time it is obsolete and misleading.  
//! <https://github.com/LucianoBestia/mem6_game/blob/master/CodeFlow.md>  
//! Now I exported the md from CodeTour and it is amazing:  
//! <https://github.com/LucianoBestia/mem6_game/blob/master/codetour_start_route_template_render.md>  
//! The step by step approach jumping from module to module is great.  
//! It just hides all the other non-important code for basic human understanding of the code flow.  
//! And the links are "alive", they go to the actual code in Github.  
//! 
// endregion: lmake_readme include "readme.md" //! A

mod lib_internal;

use clap::App;
use std::env;

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
    println!("folder: {}", folder);
    lib_for_main::export_all_tours(folder);
    println!("Export ended");
}
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
