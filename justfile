default:
  just --list

alias c := check
check:
  cargo fmt 
  cargo clippy -- -Dwarnings -Dclippy::unwrap_used
  cargo c 
  cargo t 

alias r := run
run: check
    RUST_LOG=debug cargo run

push MESSAGE: check 
  git add -A 
  git commit -m "{{MESSAGE}}"
  git push
