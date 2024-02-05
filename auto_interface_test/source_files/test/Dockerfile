FROM registry.scontain.com/scone.cloud/crosscompilers-ubuntu:5.8.0-rc.24

COPY operation /operation
COPY client /work/client
COPY server /work/server
COPY file_sys /work/file_sys
COPY data-original /R_ue

ENV SCONE_MODE=hw
ENV SCONE_NO_MMAP_ACCESS=1
ENV SCONE_FSGSBASE_MODE=enable
ENV SCONE_NETWORK_SHIELD=disabled
ENV SCONE_CAS_ADDR=scone-cas.cf
ENV SCONE_LAS_ADDR=0.0.0.0

RUN cd /work \
    && scone cargo build --manifest-path file_sys/Cargo.toml --target=x86_64-scone-linux-musl \
    && cp file_sys/target/x86_64-scone-linux-musl/debug/file_sys /operation/file_sys \
    && scone cargo build --manifest-path server/Cargo.toml --target=x86_64-scone-linux-musl \
    && cp server/target/x86_64-scone-linux-musl/debug/server /operation/server \
    && scone cargo build --manifest-path client/Cargo.toml --target=x86_64-scone-linux-musl \
    && cp client/target/x86_64-scone-linux-musl/debug/client /operation/client \
    && cd /operation && ./fspf.sh 

# CMD ["sh", "/operation/run_encrypted.sh"]