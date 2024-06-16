default:
  just --list

alias c := check
check:
  cargo fmt 
  cargo clippy -- -Dwarnings -Dclippy::unwrap_used
  cargo doc
  cargo c 
  cargo t 

alias r := run
run: check
    cargo run

push MESSAGE: check 
  git add -A 
  git commit -m "{{MESSAGE}}"
  git push -u origin
  git push -u github
