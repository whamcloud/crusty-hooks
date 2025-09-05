#!/bin/sh
# rusty-hooks
# version {{VERSION}}

hookName=$(basename "$0")
gitParams="$*"

# shellcheck source=src/hook_files/cli.sh
. "$(dirname "$0")"/cli.sh

if ! command -v crusty-hooks >/dev/null 2>&1; then
  if [ -z "${RUSTY_HOOKS_SKIP_AUTO_INSTALL}" ]; then
    installRustyHookCli
  else
    echo "[crusty-hooks] rusty-hook is not installed, and auto install is disabled"
    echo "[crusty-hooks] skipping ${hookName} hook"
    echo "[crusty-hooks] You can reinstall it using 'cargo install crusty-hooks' or delete this hook"
    exit 0
  fi
else
  ensureMinimumRustyHooksCliVersion || true
fi

# shellcheck disable=SC2046
crusty-hooks run --hook "${hookName}"
handleRustyHooksCliResult $? "${hookName}"
