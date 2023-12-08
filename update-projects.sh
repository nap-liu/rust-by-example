#!bash

configFile=".vscode/settings.json"

find . -name "Cargo.toml" | grep "$1" | jq -Rs --slurpfile c $configFile 'split("\n")|[.[]|select(.!="")]|$c[0]+{"rust-analyzer.linkedProjects": .}' >$configFile
