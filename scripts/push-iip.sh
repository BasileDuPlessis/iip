#!/usr/bin/env bash
set -euo pipefail

TARGET_REMOTE_URL="https://github.com/BasileDuPlessis/iip.git"
RETURN_GH_USER="basile-du-plessis_accent"

if ! git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  echo "Error: this command must be run inside a git repository." >&2
  exit 1
fi

current_branch="$(git rev-parse --abbrev-ref HEAD)"
if [[ -z "${current_branch}" || "${current_branch}" == "HEAD" ]]; then
  echo "Error: unable to determine current branch." >&2
  exit 1
fi

git remote set-url origin "${TARGET_REMOTE_URL}" 2>/dev/null || git remote add origin "${TARGET_REMOTE_URL}"
git push origin "${current_branch}"

if ! gh auth switch -u "${RETURN_GH_USER}"; then
  echo "Error: pushed successfully, but failed to switch gh account back to ${RETURN_GH_USER}." >&2
  exit 1
fi

echo "Push completed and gh account switched back to ${RETURN_GH_USER}."
