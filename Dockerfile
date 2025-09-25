FROM node:20-slim as builder
WORKDIR /app

# Install dependencies first (better caching)
COPY package*.json ./
RUN npm ci --only=production --frozen-lockfile

# Copy source code
COPY . .

# Build for production (optimized, minified)
RUN npm run build

# Production stage - minimal runtime
FROM node:20-alpine as runtime
WORKDIR /app

# Copy only the built artifacts
COPY --from=builder /app/build ./build
COPY --from=builder /app/package*.json ./

# Install only production dependencies for serving
RUN npm ci --only=production --frozen-lockfile && npm cache clean --force

# Create non-root user for security
RUN addgroup -g 1001 -S nodejs && adduser -S sveltekit -u 1001

# Switch to non-root user
USER sveltekit

# Expose port (if you ever want to serve directly from container)
EXPOSE 3000

# Keep container running to share volume
CMD ["tail", "-f", "/dev/null"]