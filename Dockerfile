# Image for building Dispute AppImage which includes all the needed
# dependencies and tools

# Bullseye has a relatively old GLIBC version while having QT 5.15 available
# which is the minimum required QT version for Slint

# The command to properly build this container:
# `docker build -t dispute_builder .
# The command to propertly run this container:
# `docker run -it --device /dev/fuse --cap-add SYS_ADMIN dispute_builder`
# After that, inside the container, clone the Vinegret43/dispute repo, cd into it and run
# `cargo appimage`
# Last step: copy the appimage from container to your host OS using
# `docker cp <container_id>:/root/dispute/target/appimage/dispute.AppImage .`
# You may see the container ID using `docker ps`

FROM debian:bullseye

WORKDIR /root

RUN apt-get update
RUN apt-get install -y git make cmake python3 pkg-config\
 fontconfig libfontconfig1-dev libxcb-render0-dev libxcb-shape0-dev\
 libxcb-xfixes0-dev curl vim libdbus-1-dev libasound2-dev\
 wget libfuse2 qtbase5-dev gcc g++ file

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > install_rust.sh
RUN chmod +x ./install_rust.sh
RUN ./install_rust.sh -y --profile minimal --default-host x86_64-unknown-linux-gnu\
 --default-toolchain stable

RUN wget 'https://github.com/AppImage/AppImageKit/releases/download/13/appimagetool-x86_64.AppImage'
RUN chmod +x appimagetool-x86_64.AppImage
RUN mv appimagetool-x86_64.AppImage .cargo/bin/appimagetool
RUN groupadd fuse
RUN usermod root -aG fuse
RUN git clone 'https://github.com/Vinegret43/cargo-appimage'
RUN ./.cargo/bin/cargo install --path cargo-appimage

ENV PATH="$PATH:/root/.cargo/bin"
