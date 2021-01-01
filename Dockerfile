FROM ubuntu:20.04
LABEL maintainer="Aditya Kresna <kresna@tapalogi.com>"

RUN mkdir /app
WORKDIR /app
COPY build/release/tapa-micro-mailer tapa-micro-mailer

ENTRYPOINT [ "./tapa-micro-mailer" ]
