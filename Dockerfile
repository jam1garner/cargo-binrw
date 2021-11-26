FROM kalilinux/kali-rolling

USER root
ENV USER root

SHELL ["/bin/bash", "-c"]

#ENV DEBIAN_FRONTEND=noninteractive
ENV SOURCE_REPO_CONDITION="/debian"
ENV SOURCE_REPO="deb http://ftp.us.debian.org/debian stable main contrib non-free"
# Opting for debian stable here since Kali is already a delicate house of cards
# and a light breeze can easily push it over
#
# Without knowing ahead of time how the default Kali installation is built, 
# I'm adding this check in case potentially duplicated sources are added to the APT repositories
# and make this Dockerfile a lot harder to debug. 
# 
# Ideally we only want it to apply a new repository source if nothing's already there.
# If things are present, an inplace `sed` replacement can potentially be added to apply
# the contrib and non-free sources.
RUN set -Eeo pipefail; shopt -s dotglob inherit_errexit nullglob; \
export DEBIAN_FRONTEND=noninteractive; \
    ( \
    for file in $(find /etc/apt/ -name "*.list"); do if [[ "$(grep $SOURCE_REPO_CONSITION $file)" != '' ]]; then \
        echo "$SOURCE_REPO" >> /etc/apt/sources.list && apt-get update break; fi; done ) \
    && ( \
        for i in $(seq 1 10); do [ $i -gt 1 ] && sleep 1; \
            apt-get update \
        && s=0 && break || s=$?; done; exit $s \
    ) && \
    ( \
    for i in $(seq 1 5); do [ $i -gt 1 ] && sleep 1; \
        apt-get install -y --no-install-recommends \
            apt-utils \
            vim \
            gcc \
            gnupg \
            dirmngr \
            curl \
            wget ca-certificates \
            apt-transport-https \
            jq \
            libssl-dev \
            cmake \
            pkg-config \
            git \
            build-essential \
            python3 \
            python3-dev \
            gcc && s=0 && break || s=$?; done; exit $s \
    )
# Sweet! Assuming all of that actually worked, let's get Rust installed
RUN curl https://sh.rustup.rs -sSf > /tmp/rustup-init.sh \
    && chmod +x /tmp/rustup-init.sh \
    && sh /tmp/rustup-init.sh -y \
    && rm -rf /tmp/rustup-init.sh
ENV PATH "$PATH:~/.cargo/bin"
RUN ~/.cargo/bin/rustup install stable

# At this point, let's build an editor from scratch
ENV IMHEX_VERSION_TAG="v1.10.1"
RUN mkdir /tmp/editor-integrations && \
    # Begin building ImHex from source
    git clone https://github.com/WerWolv/ImHex.git && \
    pushd ImHex && \
    git checkout "$IMHEX_VERSION_TAG" && \
    git submodule update --init --recursive && \
    mkdir -p ./build && \
    cd build && \
    cmake -DCMAKE_BUILD_TYPE=Release .. \
    make -j

# At this point, Running `ImHex` on Kali needs some configuration files
# moved out to the file system that are checked at runtime.
#
# If it doesn't find these, the whole thing uhh... will launch, but have a weird
# screen that shows only the background.
#
# To get this working, it was necessary to copy the `~/build/plugins/{lib,bin,share}` directories to the `/usr`
# directory on the local FS.
# 
# Finally, we just needed a non-null ~/.config/imhex/settings.json file
# {"hex.builtin.setting.general":{"hex.builtin.setting.general.auto_load_patterns":1,"hex.builtin.setting.general.show_tips":1},"hex.builtin.setting.hex_editor":{"hex.builtin.setting.hex_editor.advanced_decoding":0,"hex.builtin.setting.hex_editor.ascii":1,"hex.builtin.setting.hex_editor.column_count":16,"hex.builtin.setting.hex_editor.extra_info":1,"hex.builtin.setting.hex_editor.grey_zeros":1,"hex.builtin.setting.hex_editor.hexii":0,"hex.builtin.setting.hex_editor.uppercase_hex":1},"hex.builtin.setting.imhex":{"hex.builtin.setting.imhex.launched":1,"hex.builtin.setting.imhex.recent_files":["/home/david/devices.json.zip","/home/david/etest/backup"]},"hex.builtin.setting.interface":{"hex.builtin.setting.interface.color":0,"hex.builtin.setting.interface.fps":60,"hex.builtin.setting.interface.highlight_alpha":128,"hex.builtin.setting.interface.language":"en-US","hex.builtin.setting.interface.scaling":0}}                                                                                                                                                                                                                                            
#
RUN cp -ar plugins/bin/* /usr/bin/
RUN cp -ar plugins/lib/* /usr/lib/
RUN cp -ar plugins/share/* /usr/share/
RUN mkdir -p ~/.config/imhex
RUN echo '{"hex.builtin.setting.general":{"hex.builtin.setting.general.auto_load_patterns":1,"hex.builtin.setting.general.show_tips":1},"hex.builtin.setting.hex_editor":{"hex.builtin.setting.hex_editor.advanced_decoding":0,"hex.builtin.setting.hex_editor.ascii":1,"hex.builtin.setting.hex_editor.column_count":16,"hex.builtin.setting.hex_editor.extra_info":1,"hex.builtin.setting.hex_editor.grey_zeros":1,"hex.builtin.setting.hex_editor.hexii":0,"hex.builtin.setting.hex_editor.uppercase_hex":1},"hex.builtin.setting.imhex":{"hex.builtin.setting.imhex.launched":1,"hex.builtin.setting.imhex.recent_files":["/home/david/devices.json.zip","/home/david/etest/backup"]},"hex.builtin.setting.interface":{"hex.builtin.setting.interface.color":0,"hex.builtin.setting.interface.fps":60,"hex.builtin.setting.interface.highlight_alpha":128,"hex.builtin.setting.interface.language":"en-US","hex.builtin.setting.interface.scaling":0}}' > ~/.config/imhex/settings.json

RUN docker-testing-entrypoint.sh
