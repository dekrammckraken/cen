
***Cen***

Protect your privacy from prying eyes while browsing files in the terminal.
Sometimes even a filename alone can spark unwanted curiosity.

Highly customizable to hide specific words, including file extensions.
You can choose the censor character, mask the exact word length, or completely hide lines.
All by simply adding an alias or function, as shown below.

Add this function to your .bashrc to manage private and guest ls sessions.
Protect your files from prying eyes.

gls (guest ls) removes lines containing sensitive words,
while ls masks them with a censor symbol.

```bash

alias ls='cenls'
alias gls='cenls --maniac'

cenls() {
  local blacklist=("password" "vault" "secret" "secure")
  local joined_blacklist
  joined_blacklist=$(IFS=, ; echo "${blacklist[*]}")

  local maniac_flag=0
  local args=()

  for arg in "$@"; do
    if [[ $arg == "--maniac" ]]; then
      maniac_flag=1
    else
      args+=("$arg")
    fi
  done

  if (( maniac_flag )); then
    eza -l --icons --all --color=always "${args[@]}" | cen \
      --blacklist "$joined_blacklist" \
      --maniac
  else
    eza -l --icons --all --color=always "${args[@]}" | cen \
      --censor-symbol '-' \
      --blacklist "$joined_blacklist" \
      --match-len
  fi
}
