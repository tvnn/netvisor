FROM node:20-slim AS builder
WORKDIR /app

# Install dependencies first (better caching)
COPY package*.json ./
RUN npm ci --frozen-lockfile

# Copy source code
COPY . .

# Build static site
RUN npm run build

# Production stage - output to /app/build for volume sharing
FROM alpine:latest AS runtime
WORKDIR /app

# Copy built static files
COPY --from=builder /app/build ./build

# Keep container running to share volume
CMD ["tail", "-f", "/dev/null"]