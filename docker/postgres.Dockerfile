FROM postgres:alpine
COPY ./database /sql
WORKDIR /sql
ENV POSTGRES_PASSWORD sushao
ENV POSTGRES_USER sushao
RUN chmod 777 ./start.sh