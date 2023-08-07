use colored::Colorize;
use simpleargs::SimpleArgs;
use std::{collections::HashMap, env, fs, path::PathBuf, process::exit};

use crate::patches::patch_index_js;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

mod asar;
mod files;
mod folders;
mod patches;
mod updates;
mod versions;

fn patch(extracted_resource_dir: PathBuf, opts: &HashMap<String, String>) -> std::io::Result<()> {
    println!("Enabling pro...");

    patches::patch_pro_mode(extracted_resource_dir.clone(), &opts);

    println!("Done.");

    println!("Enabling creator mode...");

    patches::patch_creator_mode(extracted_resource_dir.clone());

    println!("Done.");

    println!("Patching vendor bundle...");

    patches::patch_vendor_bundle(extracted_resource_dir.clone());

    println!("Done.");

    println!("Patching index.js...");

    patch_index_js(extracted_resource_dir.clone());

    println!("Done.");

    Ok(())
}

pub fn err(msg: String) {
    println!("{}", msg.red());
    exit(1);
}

fn main() -> std::io::Result<()> {
    if env::consts::OS != "windows" {
        err(format!("Your OS ({}) is not supported.", env::consts::OS))
    }

    let (_cmds, flags, opts) = SimpleArgs::new(env::args().collect()).parse();

    if flags.contains(&"v".to_string()) {
        println!("{}", VERSION);
        exit(0);
    }

    println!("WeMod Pro Unlocker v{}", VERSION);

    updates::check(&flags);

    let wemod_folder = if opts.contains_key("wemod-dir") {
        PathBuf::from(opts.get("wemod-dir").unwrap())
    } else {
        folders::get_wemod_folder()
    };

    if !wemod_folder.exists() {
        err(
            "WeMod is not installed/not found (specify custom install location with --wemod-dir)."
                .to_string(),
        );
    }

    let wemod_version_folder = if opts.contains_key("wemod-version") {
        let folder = wemod_folder.join(PathBuf::from(
            "app-".to_string()
                + PathBuf::from(opts.get("wemod-version").unwrap())
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap(),
        ));

        if !folder.exists() {
            err("WeMod version specified does not exist.".to_string())
        }

        wemod_folder.join(folder)
    } else {
        folders::get_latest_app_dir(wemod_folder).expect(
            "failed to find latest WeMod version. you can manually specify it with --wemod-version",
        )
    };

    let resource_dir = wemod_version_folder.join("resources");

    println!(
        "Attempting to patch WeMod v{}...",
        versions::get_version_from_path(wemod_version_folder)
    );

    println!("Extracting resources...");

    if resource_dir.join("app.asar.old").exists() {
        fs::copy(
            resource_dir.join("app.asar.old"),
            resource_dir.join("app.asar"),
        )?;
    } else {
        fs::copy(
            resource_dir.join("app.asar"),
            resource_dir.join("app.asar.old"),
        )?;
    }

    asar::run(
        resource_dir.clone(),
        vec![
            "extract".to_string(),
            "app.asar".to_string(),
            "app".to_string(),
        ],
    );

    println!("Done.");

    let extracted_resource_dir = resource_dir.join("app");

    patch(extracted_resource_dir.clone(), &opts)?;

    println!("Repacking resources...");

    asar::run(
        resource_dir,
        vec![
            "pack".to_string(),
            "app".to_string(),
            "app.asar".to_string(),
        ],
    );

    println!("Done.");
    println!("Cleaning up...");

    fs::remove_dir_all(extracted_resource_dir).expect("failed to remove extracted resources.");

    println!("Done.");

    Ok(())
}
