# Fuzzer of buffer's encryption of systemcalls (in processing)
This fuzzer is used to verify if the schielding layer of SCONE can work correctly to encrypt the buffer of the systemcalls which are made in the SCONE container.

## Fuzzing systemcall list program
In order to run the test, you need the quality of the SCONE to get the scone container image and the program need to be run at a device with SGX.

The code in the *executor.rs* will create the docker image with the execution commands which are saved in the *docker/executon.sh* and start the target container and the files/folder bellow will be linked to the container:
- generator: The rust create which is used to generate the target program in the SCONE container, the program will be compiled by scone cargo
- data-original: The example_file.txt is the file which will be encrypted by SCONE fspf.
- volume: Save the encryptred files and the target prorgam will read from it.

## Real SCONE applications run on the kubernetes
> TO DO
