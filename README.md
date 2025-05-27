
Add this function to your .bashrc to manage
private and guest ls sessions. Just protect your files from prying eyes.


gls (guest ls) removes lines containing privacy words,
while ls just masks them with a censor symbol.

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
