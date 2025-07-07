FROM rust:1.87
LABEL authors="cryptoracoons"

RUN apt update
RUN apt install -y curl git

RUN git clone https://github.com/iden3/circom.git
RUN cd circom && git checkout v2.1.9 && cargo install --path circom

ENV NVM_DIR=/root/.nvm
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash
RUN cd root && \. .nvm/nvm.sh && nvm install 22 && npm install -g snarkjs@latest

COPY aiken-zk/ /root/aiken-zk/
WORKDIR /root/aiken-zk
RUN bash -c "source $NVM_DIR/nvm.sh && npm install src/tests/sandbox/curve_compress"
RUN bash -c "source $NVM_DIR/nvm.sh && npm install milestone_2_example/curve_compress"

ENTRYPOINT ["bash"]