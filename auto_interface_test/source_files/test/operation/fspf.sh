# Create a fspf volume and encrypt the target file, prepare for the program
mkdir /encrypt
mkdir /R_en
mkdir -p /W_en/Re_We
mkdir -p /W_en/Ru_We
mkdir -p /W_ue/Re_Wu
mkdir -p /W_ue/Ru_Wu

cd /encrypt
scone fspf create fspf.pb
## Added region / to file system protection file fspf.pb
scone fspf addr fspf.pb / --kernel / --not-protected
## Add region /encrypetd, /R_en and /W_en to be encrypetd.
scone fspf addr fspf.pb /encrypt --encrypted --kernel /encrypt
scone fspf addr fspf.pb /R_en --encrypted --kernel /R_en
scone fspf addr fspf.pb /W_en --encrypted --kernel /W_en
## Encrypted the unencrypted files in the encrypted.
scone fspf addf fspf.pb /R_en /R_ue /R_en
scone fspf encrypt fspf.pb > /encrypt/keytag