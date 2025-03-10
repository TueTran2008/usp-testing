#!/usr/bin/env bash
#Chay tren Ubuntu 22
set -x
set -eo pipefail

# Kiểm tra và cài đặt các gói cần thiết nếu chưa có
if ! command -v gpg &>/dev/null || ! command -v curl &>/dev/null; then
    echo >&2 "Error: gpg or curl is not installed. Installing..."
    sudo apt-get update && sudo apt-get -y install gnupg curl
fi

# Thêm key GPG cho MongoDB và kiểm tra lỗi
if curl -fsSL https://www.mongodb.org/static/pgp/server-8.0.asc | sudo gpg --dearmor -o /usr/share/keyrings/mongodb-server-8.0.gpg; then
    echo "GPG key added successfully."
else
    echo >&2 "Error: Failed to download or process MongoDB GPG key."
    exit 1
fi

# Thêm repository của MongoDB
echo "deb [ arch=amd64,arm64 signed-by=/usr/share/keyrings/mongodb-server-8.0.gpg ] https://repo.mongodb.org/apt/ubuntu jammy/mongodb-org/8.0 multiverse" | sudo tee /etc/apt/sources.list.d/mongodb-org-8.0.list

# Cập nhật danh sách gói
sudo apt-get update

# Cài đặt MongoDB
sudo apt-get install -y mongodb-org

# Khởi động và kích hoạt MongoDB
sudo systemctl start mongod
sudo systemctl enable mongod

# Kiểm tra trạng thái MongoDB
sudo systemctl status mongod --no-pager
