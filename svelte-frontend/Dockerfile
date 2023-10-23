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

FROM build AS test

CMD ["npm", "run", "test", "src", "--" ,"--run"]

FROM build AS integration

ARG BACKEND_URL
ENV BACKEND_URL=$BACKEND_URL

CMD ["npm", "run", "test", "--" ,"--run"]

FROM node:20.8-alpine AS deployment
WORKDIR /app

COPY --from=build /app/package.json ./
COPY --from=build /app/package-lock.json ./

ENV NODE_ENV production
RUN npm ci --omit dev

COPY --from=build /app/build ./build
CMD ["node", "./build"]

