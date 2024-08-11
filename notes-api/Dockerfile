FROM oven/bun:latest
WORKDIR /app
COPY package*.json ./
RUN bun install --verbose
COPY . .
# RUN bun run generate
CMD ["bun", "run", "src/index.ts"]
