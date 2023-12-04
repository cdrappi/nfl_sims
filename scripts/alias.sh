
alias nfl_sims="cd ${NFL_SIMS_PATH}; source venv/bin/activate; source scripts/source.sh";
alias ip="ipython --ipython-dir=${NFL_DFS_PATH}/ipython";

alias gdi="git add -A :/ && git commit -am 'do it'";
alias gpob='git push origin $(git rev-parse --abbrev-ref HEAD)';
alias gdip="gdi && gpob";
alias prepush="nfl_sims; mypy src/; flake8 src/ --ignore=E203,W503; nfl_sims";
