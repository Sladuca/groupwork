const shell = require("shelljs");

shell.fatal = true; // same as "set -e"

shell.cd("contract");
shell.exec("cargo build --target wasm32-unknown-unknown --release");
shell.cp("./target/wasm32-unknown-unknown/release/groupwork.wasm", "./res");