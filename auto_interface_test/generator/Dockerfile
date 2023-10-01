FROM registry.scontain.com/scone.cloud/crosscompilers-ubuntu:5.8.0-rc.24
# FROM registry.scontain.com/sconecuratedimages/crosscompilers:ubuntu

COPY operation /operation
COPY threshold /work/threshold
COPY data-original /data-original
COPY volume /data

ENV SCONE_MODE=hw
ENV SCONE_NO_MMAP_ACCESS=1
ENV SCONE_FSGSBASE_MODE=enable
ENV SCONE_NETWORK_SHIELD=disabled
ENV SCONE_CAS_ADDR=scone-cas.cf
ENV SCONE_LAS_ADDR=0.0.0.0

RUN cd /work \
    && scone cargo build --manifest-path threshold/Cargo.toml --target=x86_64-scone-linux-musl \
    && cp threshold/target/x86_64-scone-linux-musl/debug/threshold /operation/threshold \
    && cd /operation && ./fspf.sh 

CMD ["sh", "/operation/threshold.sh"]