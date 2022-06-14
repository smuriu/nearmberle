#!/usr/bin/env bash

USERNAME=${1:-"vscode"}
USER_UID=${2:-"1000"}
USER_GID=${3:-"1000"}
# export CARGO_HOME=${4:-"/usr/local/cargo"}
# export RUSTUP_HOME=${5:-"/usr/local/rustup"}
SCRIPT_DIR="$(cd $(dirname "${BASH_SOURCE[0]}") && pwd)"
MARKER_FILE="/usr/local/etc/vscode-dev-containers/common"

set -e

# Ensure that login shells get the correct path if the user updated the PATH using ENV.
rm -f /etc/profile.d/00-restore-env.sh
echo "export PATH=${PATH//$(sh -lc 'echo $PATH')/\$PATH}" >/etc/profile.d/00-restore-env.sh
chmod +x /etc/profile.d/00-restore-env.sh

# Load markers to see which steps have already run
if [ -f "${MARKER_FILE}" ]; then
  echo "Marker file found:"
  cat "${MARKER_FILE}"
  source "${MARKER_FILE}"
fi

# Ensure at least the en_US.UTF-8 UTF-8 locale is available.
# Common need for both applications and things like the agnoster ZSH theme.
if [ "${LOCALE_ALREADY_SET}" != "true" ] && ! grep -o -E '^\s*en_US.UTF-8\s+UTF-8' /etc/locale.gen >/dev/null; then
  echo "en_US.UTF-8 UTF-8" >>/etc/locale.gen
  locale-gen
  LOCALE_ALREADY_SET="true"
fi

# Create or update a non-root user to match UID/GID.
group_name="${USERNAME}"
if id -u ${USERNAME} >/dev/null 2>&1; then
  # User exists, update if needed
  if [ "$USER_GID" != "$(id -g $USERNAME)" ]; then
    group_name="$(id -gn $USERNAME)"
    groupmod --gid $USER_GID ${group_name}
    usermod --gid $USER_GID $USERNAME
  fi
  if [ "$USER_UID" != "$(id -u $USERNAME)" ]; then
    usermod --uid $USER_UID $USERNAME
  fi
else
  # Create user
  groupadd --gid $USER_GID $USERNAME
  useradd -s /bin/bash --uid $USER_UID --gid $USERNAME -m $USERNAME
fi

# Add add sudo support for non-root user
if [ "${USERNAME}" != "root" ] && [ "${EXISTING_NON_ROOT_USER}" != "${USERNAME}" ]; then
  echo $USERNAME ALL=\(root\) NOPASSWD:ALL >/etc/sudoers.d/$USERNAME
  chmod 0440 /etc/sudoers.d/$USERNAME
  EXISTING_NON_ROOT_USER="${USERNAME}"
fi

# set permissions for future files and folders
setfacl -dR -m u:"root":rwX -m u:"$USERNAME":rwX /usr/local/cargo
setfacl -dR -m u:"root":rwX -m u:"$USERNAME":rwX /usr/local/rustup
# set permissions on existing files and folders
setfacl -R -m u:"root":rwX -m u:"$USERNAME":rwX /usr/local/cargo
setfacl -R -m u:"root":rwX -m u:"$USERNAME":rwX /usr/local/rustup
# if ! cat /etc/group | grep -e "^rustlang:" > /dev/null 2>&1; then
#     groupadd -r rustlang
# fi
# usermod -a -G rustlang "${USERNAME}"
# chown -R :rustlang "${RUSTUP_HOME}" "${CARGO_HOME}"
# chmod -R g+r+w+s "${RUSTUP_HOME}" "${CARGO_HOME}"

# ** Shell customization section **
if [ "${USERNAME}" = "root" ]; then
  user_rc_path="/root"
else
  user_rc_path="/home/${USERNAME}"
fi

# Restore user .bashrc defaults from skeleton file if it doesn't exist or is empty
if [ ! -f "${user_rc_path}/.bashrc" ] || [ ! -s "${user_rc_path}/.bashrc" ]; then
  cp /etc/skel/.bashrc "${user_rc_path}/.bashrc"
fi

# Restore user .profile defaults from skeleton file if it doesn't exist or is empty
if [ ! -f "${user_rc_path}/.profile" ] || [ ! -s "${user_rc_path}/.profile" ]; then
  cp /etc/skel/.profile "${user_rc_path}/.profile"
fi

# .bashrc/.zshrc snippet
rc_snippet="$(
  cat <<'EOF'
if [ -z "${USER}" ]; then export USER=$(whoami); fi
if [[ "${PATH}" != *"$HOME/.local/bin"* ]]; then export PATH="${PATH}:$HOME/.local/bin"; fi
# Display optional first run image specific notice if configured and terminal is interactive
if [ -t 1 ] && [[ "${TERM_PROGRAM}" = "vscode" || "${TERM_PROGRAM}" = "codespaces" ]] && [ ! -f "$HOME/.config/vscode-dev-containers/first-run-notice-already-displayed" ]; then
    if [ -f "/usr/local/etc/vscode-dev-containers/first-run-notice.txt" ]; then
        cat "/usr/local/etc/vscode-dev-containers/first-run-notice.txt"
    elif [ -f "/workspaces/.codespaces/shared/first-run-notice.txt" ]; then
        cat "/workspaces/.codespaces/shared/first-run-notice.txt"
    fi
    mkdir -p "$HOME/.config/vscode-dev-containers"
    # Mark first run notice as displayed after 10s to avoid problems with fast terminal refreshes hiding it
    ((sleep 10s; touch "$HOME/.config/vscode-dev-containers/first-run-notice-already-displayed") &)
fi
# Set the default git editor if not already set
if [ -z "$(git config --get core.editor)" ] && [ -z "${GIT_EDITOR}" ]; then
    if  [ "${TERM_PROGRAM}" = "vscode" ]; then
        if [[ -n $(command -v code-insiders) &&  -z $(command -v code) ]]; then 
            export GIT_EDITOR="code-insiders --wait"
        else 
            export GIT_EDITOR="code --wait"
        fi
    fi
fi
EOF
)"

# code shim, it fallbacks to code-insiders if code is not available
cat <<'EOF' >/usr/local/bin/code
#!/bin/sh
get_in_path_except_current() {
    which -a "$1" | grep -A1 "$0" | grep -v "$0"
}
code="$(get_in_path_except_current code)"
if [ -n "$code" ]; then
    exec "$code" "$@"
elif [ "$(command -v code-insiders)" ]; then
    exec code-insiders "$@"
else
    echo "code or code-insiders is not installed" >&2
    exit 127
fi
EOF
chmod +x /usr/local/bin/code

# systemctl shim - tells people to use 'service' if systemd is not running
cat <<'EOF' >/usr/local/bin/systemctl
#!/bin/sh
set -e
if [ -d "/run/systemd/system" ]; then
    exec /bin/systemctl/systemctl "$@"
else
    echo '\n"systemd" is not running in this container due to its overhead.\nUse the "service" command to start services instead. e.g.: \n\nservice --status-all'
fi
EOF
chmod +x /usr/local/bin/systemctl

# Codespaces bash and OMZ themes - partly inspired by https://github.com/ohmyzsh/ohmyzsh/blob/master/themes/robbyrussell.zsh-theme
codespaces_bash="$(
  cat \
    <<'EOF'
# Codespaces bash prompt theme
__bash_prompt() {
    local userpart='`export XIT=$? \
        && [ ! -z "${GITHUB_USER}" ] && echo -n "\[\033[0;32m\]@${GITHUB_USER} " || echo -n "\[\033[0;32m\]\u " \
        && [ "$XIT" -ne "0" ] && echo -n "\[\033[1;31m\]➜" || echo -n "\[\033[0m\]➜"`'
    local gitbranch='`\
        if [ "$(git config --get codespaces-theme.hide-status 2>/dev/null)" != 1 ]; then \
            export BRANCH=$(git symbolic-ref --short HEAD 2>/dev/null || git rev-parse --short HEAD 2>/dev/null); \
            if [ "${BRANCH}" != "" ]; then \
                echo -n "\[\033[0;36m\](\[\033[1;31m\]${BRANCH}" \
                && if git ls-files --error-unmatch -m --directory --no-empty-directory -o --exclude-standard ":/*" > /dev/null 2>&1; then \
                        echo -n " \[\033[1;33m\]✗"; \
                fi \
                && echo -n "\[\033[0;36m\]) "; \
            fi; \
        fi`'
    local lightblue='\[\033[1;34m\]'
    local removecolor='\[\033[0m\]'
    PS1="${userpart} ${lightblue}\w ${gitbranch}${removecolor}\$ "
    unset -f __bash_prompt
}
__bash_prompt
EOF
)"

# Add RC snippet and custom bash prompt
if [ "${RC_SNIPPET_ALREADY_ADDED}" != "true" ]; then
  echo "${rc_snippet}" >>/etc/bash.bashrc
  echo "${codespaces_bash}" >>"${user_rc_path}/.bashrc"
  echo 'export PROMPT_DIRTRIM=4' >>"${user_rc_path}/.bashrc"
  if [ "${USERNAME}" != "root" ]; then
    echo "${codespaces_bash}" >>"/root/.bashrc"
    echo 'export PROMPT_DIRTRIM=4' >>"/root/.bashrc"
  fi
  chown ${USERNAME}:${group_name} "${user_rc_path}/.bashrc"
  RC_SNIPPET_ALREADY_ADDED="true"
fi

# Persist image metadata info, script if meta.env found in same directory
meta_info_script="$(
  cat <<'EOF'
#!/bin/sh
. /usr/local/etc/vscode-dev-containers/meta.env
# Minimal output
if [ "$1" = "version" ] || [ "$1" = "image-version" ]; then
    echo "${VERSION}"
    exit 0
elif [ "$1" = "release" ]; then
    echo "${GIT_REPOSITORY_RELEASE}"
    exit 0
elif [ "$1" = "content" ] || [ "$1" = "content-url" ] || [ "$1" = "contents" ] || [ "$1" = "contents-url" ]; then
    echo "${CONTENTS_URL}"
    exit 0
fi
#Full output
echo
echo "Development container image information"
echo
if [ ! -z "${VERSION}" ]; then echo "- Image version: ${VERSION}"; fi
if [ ! -z "${DEFINITION_ID}" ]; then echo "- Definition ID: ${DEFINITION_ID}"; fi
if [ ! -z "${VARIANT}" ]; then echo "- Variant: ${VARIANT}"; fi
if [ ! -z "${GIT_REPOSITORY}" ]; then echo "- Source code repository: ${GIT_REPOSITORY}"; fi
if [ ! -z "${GIT_REPOSITORY_RELEASE}" ]; then echo "- Source code release/branch: ${GIT_REPOSITORY_RELEASE}"; fi
if [ ! -z "${BUILD_TIMESTAMP}" ]; then echo "- Timestamp: ${BUILD_TIMESTAMP}"; fi
if [ ! -z "${CONTENTS_URL}" ]; then echo && echo "More info: ${CONTENTS_URL}"; fi
echo
EOF
)"
if [ -f "${SCRIPT_DIR}/meta.env" ]; then
  mkdir -p /usr/local/etc/vscode-dev-containers/
  cp -f "${SCRIPT_DIR}/meta.env" /usr/local/etc/vscode-dev-containers/meta.env
  echo "${meta_info_script}" >/usr/local/bin/devcontainer-info
  chmod +x /usr/local/bin/devcontainer-info
fi

# Write marker file
mkdir -p "$(dirname "${MARKER_FILE}")"
echo -e "\
    LOCALE_ALREADY_SET=${LOCALE_ALREADY_SET}\n\
    EXISTING_NON_ROOT_USER=${EXISTING_NON_ROOT_USER}\n\
    RC_SNIPPET_ALREADY_ADDED=${RC_SNIPPET_ALREADY_ADDED}" >"${MARKER_FILE}"

# Get central common setting
get_common_setting() {
  if [ "${common_settings_file_loaded}" != "true" ]; then
    curl -sfL "https://aka.ms/vscode-dev-containers/script-library/settings.env" -o /tmp/vsdc-settings.env 2>/dev/null || echo "Could not download settings file. Skipping."
    common_settings_file_loaded=true
  fi
  if [ -f "/tmp/vsdc-settings.env" ]; then
    local multi_line=""
    if [ "$2" = "true" ]; then multi_line="-z"; fi
    local result="$(grep ${multi_line} -oP "$1=\"?\K[^\"]+" /tmp/vsdc-settings.env | tr -d '\0')"
    if [ ! -z "${result}" ]; then declare -g $1="${result}"; fi
  fi
  echo "$1=${!1}"
}

updaterc() {
  echo "Updating /etc/bash.bashrc and /etc/zsh/zshrc..."
  if [[ "$(cat /etc/bash.bashrc)" != *"$1"* ]]; then
    echo -e "$1" >>/etc/bash.bashrc
  fi
}

echo "Done!"
