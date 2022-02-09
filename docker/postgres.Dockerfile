FROM postgres:alpine
COPY ./database /sql
WORKDIR /sql
RUN chmod 777 ./start.sh