#!/bin/sh
# rusty-hooks
# version {{VERSION}}

hookName=$(basename "$0")
gitParams="$*"

# shellcheck source=src/hook_files/cli.sh
. "$(dirname "$0")"/cli.sh

if ! command -v rusty-hooks >/dev/null 2>&1; then
  if [ -z "${RUSTY_HOOKS_SKIP_AUTO_INSTALL}" ]; then
    installRustyHookCli
  else
    echo "[rusty-hooks] rusty-hook is not installed, and auto install is disabled"
    echo "[rusty-hooks] skipping ${hookName} hook"
    echo "[rusty-hooks] You can reinstall it using 'cargo install rusty-hooks' or delete this hook"
    exit 0
  fi
else
  ensureMinimumRustyHooksCliVersion || true
fi

# shellcheck disable=SC2046
rusty-hooks run --hook "${hookName}" $([ -z "$gitParams" ] && echo "" || echo "-- $gitParams")
handleRustyHooksCliResult $? "${hookName}"
