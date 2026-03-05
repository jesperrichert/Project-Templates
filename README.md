# Projectify

> **Warning:** Please note that Projectify is currently in beta.

CLI tool to manage project templates from Git.  
To create a template, define a `projects.json` with the required structure.

Then you can set the template URL with the CLI:

`projectify config -t <url>`

After this, you can see your templates with:

`projectify list`

To create a project from a template, use:

`projectify create -i <id> -n <name> -p <path>`

Use `projectify --help` to show the help message.