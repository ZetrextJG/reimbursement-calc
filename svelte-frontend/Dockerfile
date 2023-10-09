# Build stage
FROM node:20.8-alpine AS build
ENV NODE_ENV development
WORKDIR /app

COPY package.json ./
COPY package-lock.json ./
RUN npm ci

COPY . ./

ENV NODE_ENV production
RUN npm run build

EXPOSE 3000
CMD ["node", "./build"]

