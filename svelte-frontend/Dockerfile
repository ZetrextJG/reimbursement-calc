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
CMD ["npm", "run", "test", "--" ,"--run"]

FROM build AS integration
CMD ["npm", "run", "test", "--" ,"--run"]

FROM nginx:1.19-alpine AS deployment
COPY --from=build /app/build /usr/share/nginx/html

