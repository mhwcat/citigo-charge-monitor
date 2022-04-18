FROM node:17 AS builder
WORKDIR /app
COPY . .
RUN npm install && npm run build

FROM nginx:alpine
WORKDIR /app
COPY --from=builder /app/dist .
COPY nginx.conf /etc/nginx/nginx.conf