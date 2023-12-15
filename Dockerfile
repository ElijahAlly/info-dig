# Build Stage for Vue.js frontend
FROM node:18.18.2 as vue-build-stage
WORKDIR /web-client
COPY /web-client/package*.json ./
RUN yarn install
COPY ./web-client .
RUN yarn build

# Build Stage for Rust backend
FROM rust:1.65 as rust-build-stage
RUN USER=root cargo new --bin myapp
WORKDIR /server
COPY ./server .
RUN cargo build --release

# Production Stage
FROM nginx
# Install libpq
RUN apt-get update && apt-get install -y libpq5
COPY --from=vue-build-stage /web-client/dist /web-client
COPY --from=rust-build-stage /server/target/release/info_dig /usr/local/bin/
RUN chmod +x /usr/local/bin/info_dig
COPY nginx.conf /etc/nginx/nginx.conf

# Run Nginx and the Rust application
CMD ["sh", "-c", "nginx && /usr/local/bin/info_dig"]