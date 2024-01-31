#bookworm refers to the debian version
FROM ghcr.io/void-linux/void-glibc:latest

RUN xbps-install -Syu xbps
RUN xbps-install -Syu &&  \
    xbps-install -y base-devel && \
    xbps-install -y clang && \
    xbps-install -y pkg-config && \
    xbps-install -y gtk4-devel && \
    xbps-install -y libadwaita-devel && \
    xbps-install -y rust && \
    xbps-install -y cargo && \
    xbps-install -y git

WORKDIR /root
RUN git clone https://github.com/lemmygtk/lemoa
RUN cd lemoa && \
    cargo build --release

#COPY --chown=lemoa:lemoa . /home/lemoa/lemoa
#RUN rustup component add rustfmt
#RUN rustfmt --check /home/lemoa/lemoa
#These might be useful for wasm deployments in the future.
#RUN rustup target add wasm32-unknown-unknown

#CMD cd /home/lemoa/lemoa && cargo build
