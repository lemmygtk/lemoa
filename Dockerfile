#bookworm refers to the debian version
FROM rust:bookworm 

RUN apt update &&  \
    apt-get -y install clang && \
    apt-get -y install libclang-dev  && \
    apt-get install -y sudo && \
    apt-get install -y libgtk-4-dev && \
    apt-get install -y libadwaita-1-0 && \
    apt-get install  -y meson && \
    apt-get install -y ninja-build && \
    apt-get install -y git && \
    apt-get install -y valac && \
    apt-get install -y gettext

RUN echo '%sudo ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers

RUN useradd -ms /bin/bash -G sudo lemoa
USER lemoa
WORKDIR /home/lemoa
RUN git clone --recursive https://gitlab.gnome.org/GNOME/libadwaita.git
RUN cd /home/lemoa/libadwaita && git checkout 1.2.0
RUN cd /home/lemoa/libadwaita && sudo  meson . _build
RUN cd /home/lemoa/libadwaita && sudo  ninja -C _build
RUN cd /home/lemoa/libadwaita && sudo  ninja -C _build install

RUN mkdir lemoa_volume

#COPY --chown=lemoa:lemoa . /home/lemoa/lemoa
#RUN rustup component add rustfmt
#RUN rustfmt --check /home/lemoa/lemoa
#These might be useful for wasm deployments in the future.
#RUN rustup target add wasm32-unknown-unknown

#CMD cd /home/lemoa/lemoa && cargo build
