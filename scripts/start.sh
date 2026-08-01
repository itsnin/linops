#!/bin/sh -e

# prevent execution if this script was only partially downloaded
{
rc='\033'
red='\033'

check() {
    exit_code=$1
    message=$2

    if [ "$exit_code" -ne 0 ]; then
        printf '%sERROR: %s%s\n' "$red" "$message" "$rc"
        exit 1
    fi

    unset exit_code
    unset message
}

find_arch() {
    case "$(uname -m)" in
        x86_64|amd64) arch="x86_64" ;;
        aarch64|arm64) arch="aarch64" ;;
        *) check 1 "unsupported architecture" ;;
    esac
}

get_url() {
    case "${arch}" in
        x86_64) echo "https://github.com/itsnin/linops/releases/download/continuous/linops" ;;
        *) echo "https://github.com/itsnin/linops/releases/download/continuous/linops-${arch}" ;;
    esac
}

find_arch

temp_file=$(mktemp)
check $? "creating temporary file"

trap 'rm -f "$temp_file"' EXIT INT TERM

curl -fsL "$(get_url)" -o "$temp_file"
check $? "downloading linops"

chmod +x "$temp_file"
check $? "making linops executable"

"$temp_file" "$@"
check $? "executing linops"

rm -f "$temp_file"
check $? "cleaning up"
} # end of wrapping
