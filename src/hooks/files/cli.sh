#!/bin/sh
# rusty-hooks
# version {{VERSION}}

# shellcheck disable=SC2170,SC1083
minimumMajorCliVersion={{MINIMUM_MAJOR}}
# shellcheck disable=SC2170,SC1083
minimumMinorCliVersion={{MINIMUM_MINOR}}
# shellcheck disable=SC2170,SC1083
minimumPatchCliVersion={{MINIMUM_PATCH}}
# shellcheck disable=SC2170,SC1083
allowPrereleaseCliVersion={{MINIMUM_ALLOW_PRE}}
# shellcheck disable=SC2170,SC1083
noConfigFileExitCode={{NO_CONFIG_FILE_EXIT_CODE}}

upgradeRustyHooksCli() {
  echo "[rusty-hooks] Upgrading rusty-hook cli..."
  echo "[rusty-hooks] This may take a few seconds..."
  cargo install --force rusty-hook >/dev/null 2>&1
}

installRustyHooksCli() {
  echo "[rusty-hooks] Finalizing rusty-hooks configuration..."
  echo "[rusty-hooks] This may take a few seconds..."
  cargo install rusty-hooks >/dev/null 2>&1
}

ensureMinimumRustyHooksCliVersion() {
  currentVersion=$(rusty-hooks -v)
  isGreaterThanEqualToMinimumVersion "${currentVersion}" ${minimumMajorCliVersion} ${minimumMinorCliVersion} ${minimumPatchCliVersion} ${allowPrereleaseCliVersion} >/dev/null 2>&1
  versionCompliance=$?
  if [ ${versionCompliance} -gt 0 ]; then
    upgradeRustyHooksCli || true
  fi
}

handleRustyHooksCliResult() {
  rustyHooksExitCode=${1}
  hookName=${2}

  # shellcheck disable=SC2086
  if [ ${rustyHooksExitCode} -eq 0 ]; then
    exit 0
  fi

  # shellcheck disable=SC2086
  if [ ${rustyHooksExitCode} -eq ${noConfigFileExitCode} ]; then
    if [ "${hookName}" = "pre-commit" ]; then
      echo "[rusty-hooks] rusty-hooks git hooks are configured, but no config file was found"
      echo "[rusty-hooks] In order to use rusty-hooks, your project must have a config file"
      echo "[rusty-hooks] See https://github.com/kaimast/rusty-hooks#configure for more information about configuring rusty-hooks"
      echo
      echo "[rusty-hooks] If you were trying to remove rusty-hooks, then you should also delete the git hook files to remove this warning"
      echo "[rusty-hooks] See https://github.com/kaimast/rusty-hooks#removing-rusty-hooks for more information about removing rusty-hook from your project"
      echo
    fi
    exit 0
  else
    echo "[rusty-hooks] Configured hook command failed"
    echo "[rusty-hooks] ${hookName} hook rejected"
    # shellcheck disable=SC2086
    exit ${rustyHooksExitCode}
  fi
}

# shellcheck source=src/hook_files/semver.sh
. "$(dirname "$0")"/semver.sh
