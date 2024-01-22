FROM ubuntu:latest

RUN apt-get update && \
    apt-get install -y &&\
    apt install cargo -y
    
COPY . /app

WORKDIR /app

#CMD uname -m
CMD cargo test --release

#cmd to build : sudo docker build --platform=x86_64 -t moseiik:latest -f Dockerfile .
# 		sudo docker build --platform=arm64 -t moseiik:latest -f Dockerfile .
#cmd to run : sudo docker run moseiik:latest
