#!/bin/bash

set -e

# do we have docker?
type -P docker >/dev/null || { 
    echo >&2 "docker not in path."; exit 1; 
}

# ... and the right version?
docker --version | grep -q "Docker version 17" && { 
    echo "Docker version too old. Please upgrade."; exit 1; }

REPO_NAME=immunant/c2rust
DATE_TAG=$(date +'%Y%m%d')
SCRIPT_DIR="$(dirname "$0")"

build_image() {
    BASE_IMAGE=${1}
    IMAGE_TAG=$(echo ${BASE_IMAGE} | tr -s :/ - ) # replace colons and slashes with hyphens
    PROVISION_SCRIPT=${2}

    # make sure provisioning script exists
    if [ ! -f "$PROVISION_SCRIPT" ]; then
	    echo >&2 "Provisioning script not found: $PROVISION_SCRIPT"; exit 1; 
    fi

    # pull the rust version out of ../rust-toolchain to keep things synched
    RUST_TOOLCHAIN_FILE="$SCRIPT_DIR/../rust-toolchain"
    RUST_VER=$(cat $RUST_TOOLCHAIN_FILE | tr -d '\n')

    docker build -f $SCRIPT_DIR/../docker/Dockerfile \
           --build-arg BASE_IMAGE=$BASE_IMAGE \
           --build-arg PROVISION_SCRIPT=$PROVISION_SCRIPT \
           --build-arg UID=$(id -u $(logname)) \
           --build-arg GID=$(id -g $(logname)) \
           --build-arg RUST_VER=$RUST_VER \
           --tag "$REPO_NAME:$IMAGE_TAG-$DATE_TAG" \
           --tag "$REPO_NAME:$IMAGE_TAG-latest" \
           $SCRIPT_DIR
}

case "$1" in
    ubuntu-bionic)
        build_image ubuntu:bionic provision_deb.sh
        ;;

    ubuntu-xenial)
        build_image ubuntu:xenial provision_deb.sh
        ;;

    fedora-29)
        build_image fedora:29 provision_yum.sh
        ;;

    debian-stretch)
        build_image debian:stretch provision_deb.sh
        ;;

    debian-jessie)
        build_image debian:jessie provision_deb.sh
        ;;

    archlinux-base)
        build_image archlinux/base provision_arch.sh
        ;;

    all)
        build_image ubuntu:bionic provision_deb.sh
        build_image ubuntu:xenial provision_deb.sh
        build_image fedora:29 provision_yum.sh
        build_image debian:jessie provision_deb.sh
        build_image debian:stretch provision_deb.sh
        build_image archlinux/base provision_arch.sh
        ;;

    *)
        echo $"Usage: $0 {ubuntu-bionic|ubuntu-xenial|fedora-29|debian-stretch|debian-jessie|archlinux-base|all}"
        exit 1

esac
