FROM node:lts-alpine as builder
RUN npm config set registry https://registry.npm.taobao.org \
    && npm install -g pnpm
COPY ./web /app/mini-oss
WORKDIR /app/mini-oss
RUN pnpm install \
    && pnpm run build --filter ./packages/admin
FROM nginx:stable-alpine as prod
COPY ./docker/web/nginx.conf /etc/nginx/nginx.conf
COPY --from=builder ./app/mini-oss/packages/admin/dist /app
EXPOSE 80