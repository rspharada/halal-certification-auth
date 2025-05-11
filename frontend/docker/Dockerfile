FROM rust:1.86.0-slim

# 必要なツールを先にインストール
RUN apt-get update && apt-get install -y \
    curl \
    ca-certificates \
    gnupg \
    lsb-release \
    build-essential \
    && rustup target add wasm32-unknown-unknown \
    && cargo install trunk

# Node.js インストール（必要な場合のみ）
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash - \
    && apt-get install -y nodejs

WORKDIR /app