FROM registry.fedoraproject.org/fedora-toolbox:44

ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=${USER_UID}

# System packages needed for Rust/Python development and the devcontainer agent.
RUN dnf -y install \
		sudo passwd shadow-utils \
		git make gcc gcc-c++ clang lld sccache \
		pkgconf-pkg-config openssl-devel \
		python3 python3-pip python3-devel python3-virtualenv \
		libffi-devel zlib-devel xz-devel \
		libseccomp-devel \
		ca-certificates curl which findutils \
		procps-ng tar gzip xz \
		ripgrep jq \
		\
	&& dnf clean all

# Create the non-root user up-front so VS Code can attach as ${USERNAME}.
RUN if ! getent group ${USER_GID} >/dev/null; then groupadd -g ${USER_GID} ${USERNAME}; fi \
	&& if ! id -u ${USERNAME} >/dev/null 2>&1; then \
		useradd -m -u ${USER_UID} -g ${USER_GID} -s /bin/bash ${USERNAME}; \
	   fi \
	&& echo "${USERNAME} ALL=(ALL) NOPASSWD:ALL" >/etc/sudoers.d/${USERNAME} \
	&& chmod 0440 /etc/sudoers.d/${USERNAME} \
	&& mkdir -p /workspaces && chown ${USER_UID}:${USER_GID} /workspaces

USER ${USERNAME}
ENV CARGO_HOME=/home/${USERNAME}/.cargo \
	RUSTUP_HOME=/home/${USERNAME}/.rustup \
	PATH=/home/${USERNAME}/.cargo/bin:/home/${USERNAME}/.local/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin

# Rust toolchain + cargo-insta for snapshot tests.
RUN curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs \
		| sh -s -- -y --default-toolchain stable --profile minimal \
			--component rustfmt --component clippy \
	&& cargo install cargo-insta --locked

# Python tooling for PyO3/maturin interop and tests.
RUN python3 -m pip install --user --upgrade pip \
	&& python3 -m pip install --user maturin pytest

WORKDIR /workspaces/dsport
