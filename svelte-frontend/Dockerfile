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


# Production stage
FROM node:20.8-alpine AS deploy
ENV NODE_ENV production
WORKDIR /app

COPY . .
COPY --from=build /app/build ./build

EXPOSE 3000
CMD ["node", "./build"]

