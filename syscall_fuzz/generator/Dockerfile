FROM registry.scontain.com/sconecuratedimages/crosscompilers:ubuntu

COPY operation /operation
COPY client /work/client
COPY server /work/server
COPY file_sys /work/file_sys
COPY data-original /data-original
COPY volume /data

# ENV SCONE_ALLOW_DLOPEN=0
ENV SCONE_MODE=hw
# ENV SCONE_SYSLIBS=1
# ENV SCONE_VERSION=1
# ENV SCONE_FSPF_MUTABLE=0
ENV SCONE_LOG=debug
# ENV SCONE_MIN_HEAP=20M
# ENV SCONE_TCS=8
# ENV SCONE_MPROTECT=0
# ENV SCONE_XFRM=3
# ENV SCONE_ISVSVN=0
# ENV SCONE_ISVPRODID=0
# ENV SCONE_EXTENSIONS_PATH=""
ENV SCONE_NO_MMAP_ACCESS=1
ENV SCONE_FSGSBASE_MODE=enable
ENV SCONE_NETWORK_SHIELD=disabled

RUN cd /work \
    && scone cargo build --manifest-path file_sys/Cargo.toml --target=x86_64-scone-linux-musl \
    && cp file_sys/target/x86_64-scone-linux-musl/debug/file_sys /operation/file_sys \
    && scone cargo build --manifest-path server/Cargo.toml --target=x86_64-scone-linux-musl \
    && cp server/target/x86_64-scone-linux-musl/debug/server /operation/server \
    && scone cargo build --manifest-path client/Cargo.toml --target=x86_64-scone-linux-musl \
    && cp client/target/x86_64-scone-linux-musl/debug/client /operation/client \
    && cd /operation && ./fspf.sh

CMD ["sh", "/operation/run_program.sh"]