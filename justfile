commit-input:
    cd input && [[ -n $(git status -s) ]] && git add . && git commit -m "$(git status --porcelain | sed 's/A /Add/')" && git push
author-to-commit-date:
    git filter-branch --env-filter 'export GIT_COMMITTER_DATE="$GIT_AUTHOR_DATE"'
