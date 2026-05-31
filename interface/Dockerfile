# --- Build the React frontend ---
FROM node:20 AS build

WORKDIR /app

COPY package.json package-lock.json* ./
RUN npm install

COPY . .

RUN npm run build

# --- Run the Node.js backend that serves the frontend + the API ---
FROM node:20-slim

WORKDIR /app

# Install only the backend dependencies (express + pg).
COPY server/package.json ./
RUN npm install

# Backend source and the compiled frontend.
COPY server/index.js ./
COPY --from=build /app/build ./build

EXPOSE 80

CMD ["node", "index.js"]
