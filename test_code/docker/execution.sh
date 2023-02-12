export SCONE_NO_MMAP_ACCESS=1
export SCONE_FSGSBASE_MODE=enable
export SCONE_MODE=hw

cd /data
scone fspf create fspf.pb
scone fspf addr fspf.pb / --kernel / --not-protected
scone fspf addr fspf.pb /data --encrypted --kernel /data
scone fspf addf fspf.pb /data /data-original /data
scone fspf encrypt fspf.pb > /data-original/keytag

# cat > example.c << EOF
# #include <stdio.h>
# #include <stdlib.h>

# void printfile(const char* fn) {
#     FILE *fp = fopen(fn, "r");
#     char c;
#     while((c=fgetc(fp))!=EOF){
#         printf("%c",c);
#     }
#     fclose(fp);
# }

# int main() {
#     printfile("/data/hello.txt");
#     printfile("/data/world.py");
# }
# EOF

# scone gcc example.c -o example

# export SCONE_FSPF_KEY=$(cat /data-original/keytag | awk '{print $11}')
# export SCONE_FSPF_TAG=$(cat /data-original/keytag | awk '{print $9}')
# export SCONE_FSPF=/data/fspf.pb

# export
# ./example

cd /generator
scone cargo build --target=x86_64-scone-linux-musl
cd target/debug
./generator