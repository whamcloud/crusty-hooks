#!/bin/sh
# crusty-hooks
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
  echo "[crusty-hooks] Upgrading crusty-hook cli..."
  echo "[crusty-hooks] This may take a few seconds..."
  cargo install --force crusty-hook >/dev/null 2>&1
}

installRustyHooksCli() {
  echo "[crusty-hooks] Finalizing crusty-hooks configuration..."
  echo "[crusty-hooks] This may take a few seconds..."
  cargo install crusty-hooks >/dev/null 2>&1
}

ensureMinimumRustyHooksCliVersion() {
  currentVersion=$(crusty-hooks -v)
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
      echo "[crusty-hooks] crusty-hooks git hooks are configured, but no config file was found"
      echo "[crusty-hooks] In order to use crusty-hooks, your project must have a config file"
      echo "[crusty-hooks] See https://github.com/whamcloud/crusty-hooks#configure for more information about configuring crusty-hooks"
      echo
      echo "[crusty-hooks] If you were trying to remove crusty-hooks, then you should also delete the git hook files to remove this warning"
      echo "[crusty-hooks] See https://github.com/whamcloud/crusty-hooks#removing-crusty-hooks for more information about removing rusty-hook from your project"
      echo
    fi
    exit 0
  else
    echo "[crusty-hooks] Configured hook command failed"
    echo "[crusty-hooks] ${hookName} hook rejected"
    # shellcheck disable=SC2086
    exit ${rustyHooksExitCode}
  fi
}

# shellcheck source=src/hook_files/semver.sh
. "$(dirname "$0")"/semver.sh
