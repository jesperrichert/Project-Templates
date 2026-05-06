# projectler

> [!WARNING]
> Please note that projectler is currently in beta.

CLI tool to manage project templates from Git.  
To create a template, define a `projects.json` with the required structure.

Then you can set the template URL with the CLI:

`projectler config -t <url>`

After this, you can see your templates with:

`projectler list`

To create a project from a template, use:

`projectler create -i <id> -n <name> -p <path>`

Use `projectler --help` to show the help message.


### // TODO

- Add Rust Tauri Template
- Add Rust clap/ratatui Template
- Add Rust Web Template (leptos, yew, sycamore, ...)
- Add Rust App Template (egui, dioxus, ...)
- Add C# App Template (...)
