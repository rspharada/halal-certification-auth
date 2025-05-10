#!/bin/bash
set -e

# 同期対象の関数ディレクトリ一覧（必要に応じて追加）
TARGETS=(
  signin
  signup
  signup_confirm
  mfa_verify
  resend_code
  forgot_password
  forgot_password_confirm
  # ここに追加していける
)

# 共通モジュールの元パス
SHARED_DIR="./app/shared"

# 各関数ディレクトリへ shared をコピー
for target in "${TARGETS[@]}"; do
  echo "🔄 Copying shared/ into app/${target}/"
  cp -r "${SHARED_DIR}" "./app/${target}/shared"
done

# SAMビルド実行
echo "🚀 Running sam build"
sam build --no-cached

# 各関数ディレクトリから shared を削除（クリーンアップ）
for target in "${TARGETS[@]}"; do
  echo "🧹 Cleaning up shared/ from app/${target}/"
  rm -rf "./app/${target}/shared"
done

echo "✅ Build complete"