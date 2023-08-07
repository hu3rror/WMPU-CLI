<div align="center">
  <h1>WeMod Pro Unlocker - CLI</h1>
  <h4>This program patches the WeMod app to think you're a pro subscriber.</h4>
  <img src="https://img.shields.io/github/v/release/bennett-sh/wemod-pro-unlocker.svg" alt="Latest version">
  <img src="https://img.shields.io/github/downloads/bennett-sh/wemod-pro-unlocker/total?label=GitHub%20Downloads" alt="Downloads on GitHub">
  <img src="https://img.shields.io/crates/l/wemod-pro-unlocker?color=green" alt="License">


  <img src="https://img.shields.io/badge/rust-2021-orange?logo=rust" alt="Rust 2021">
  <img src="https://img.shields.io/github/languages/code-size/bennett-sh/wemod-pro-unlocker?color=yellow" alt="Code Size"><br/><br/>
  <img width="256" src="https://user-images.githubusercontent.com/110846042/204567385-4df3007c-7a63-40fd-9feb-f9f36aa43030.png" alt="WeMod Pro Unlocker Logo">
</div>

#### [Back to the main page](../README.md)

<br/>

## ⬇️ Installation
For installation, you have three options.
1. Download the pre-built executable [here](https://github.com/bennett-sh/wemod-pro-unlocker/releases/latest/download/wemod-pro-unlocker.exe)
2. Install it using cargo: ```cargo install wemod-pro-unlocker```
3. (not recommended) Manually build it from source

<br/>

## ⚙️ Configuration
| Argument                  	| Description                                                                                                                           	| Example
|---------------------------	|---------------------------------------------------------------------------------------------------------------------------------------	|----------------------------------
| --wemod-dir <dir>         	 	 	 	 	 	 	          	  | Path to your WeMod dir. By default, this is "%localappdata%/WeMod".                                         	| C:\WeMod
| --wemod-version <version> 	 	 	 	 	 	 	          	  | The version to patch. By default, this will be the latest version installed. 	                                | 8.3.6
| --account <json>            	 	 	 	              	  | Overwrites the account data. You can find all available options by searching for /v3/account in the dev tools | username:'myaccount',email:'test'
| -no-update / -offline            	 	              	  | Doesn't check for/install updates                                                                             | ---
| -v                          	 	 	 	 	            	  | Prints out the version info. Will cancel everything else                                                      | ---

<br/>
