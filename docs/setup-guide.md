# 图书馆管理系统 - 开发环境配置指南

## 1. 数据库配置（MySQL 8.4 LTS）

### 1.1 安装MySQL 8.4 LTS
- **Windows**: 下载 [MySQL Installer](https://dev.mysql.com/downloads/installer/)
- **Mac**: `brew install mysql@8.4`
- **Linux**: `sudo apt install mysql-server-8.4`

### 1.2 数据库配置
```sql
-- 登录MySQL
mysql -u root -p

-- 创建数据库
CREATE DATABASE library_db CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- 创建用户
CREATE USER 'library_user'@'localhost' IDENTIFIED BY '123456';
GRANT ALL PRIVILEGES ON library_db.* TO 'library_user'@'localhost';
FLUSH PRIVILEGES;
```

### 1.3 环境变量配置
在项目根目录创建 `.env` 文件：

```bash
# 数据库配置
DATABASE_URL=mysql://library_user:123456@localhost/library_db
SERVER_HOST=127.0.0.1
SERVER_PORT=8080

# 前端配置
VITE_API_URL=http://localhost:8080/api/v1
```

## 2. 后端环境配置

### 2.1 安装Rust工具链
```bash
# 安装Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装Diesel CLI
cargo install diesel_cli --no-default-features --features mysql
```

### 2.2 项目初始化
```bash
cd library-backend

# 初始化Diesel
diesel setup
diesel migration generate create_tables

# 创建迁移文件
# 编辑 migrations/2025-08-15-000000_create_tables/up.sql
# 编辑 migrations/2025-08-15-000000_create_tables/down.sql
```

### 2.3 依赖安装
编辑 `Cargo.toml`：
```toml
[dependencies]
actix-web = "4.0"
diesel = { version = "2.0", features = ["mysql", "chrono"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
dotenv = "0.15"
```

## 3. 前端环境配置

### 3.1 安装依赖
```bash
cd library-frontend
npm install
```

### 3.2 安装额外依赖
```bash
npm install axios react-router-dom @types/react-router-dom
npm install -D tailwindcss postcss autoprefixer
npx tailwindcss init -p
```

### 3.3 配置Tailwind
编辑 `tailwind.config.js`：
```javascript
module.exports = {
  content: ["./src/**/*.{js,jsx,ts,tsx}"],
  theme: {
    extend: {},
  },
  plugins: [],
}
```

## 4. 一键启动脚本

### 4.1 创建启动脚本
创建 `start-dev.sh` (Linux/Mac) 或 `start-dev.bat` (Windows):

```bash
#!/bin/bash
# start-dev.sh

echo "🚀 启动图书馆管理系统..."

# 启动后端
echo "📦 启动后端服务..."
cd library-backend
cargo run &

# 等待后端启动
sleep 5

# 启动前端
echo "🎨 启动前端服务..."
cd ../library-frontend
npm run dev &

echo "✅ 系统已启动！"
echo "📱 前端: http://localhost:5173"
echo "🔧 后端: http://localhost:8080"
```

### 4.2 Windows批处理
```batch
@echo off
echo 🚀 启动图书馆管理系统...

start cmd /k "cd library-backend && cargo run"
timeout /t 5 /nobreak >nul
start cmd /k "cd library-frontend && npm run dev"

echo ✅ 系统已启动！
pause
```

## 5. 验证步骤

### 5.1 数据库连接测试
```bash
# 测试数据库连接
mysql -u root -p -e "USE library_db; SHOW TABLES;"
```

### 5.2 后端启动测试
```bash
cd library-backend
cargo check
cargo run
# 访问: http://localhost:8080/health
```

### 5.3 前端启动测试
```bash
cd library-frontend
npm run dev
# 访问: http://localhost:5173
```

## 6. 常见问题

### 6.1 MySQL连接失败
- 检查MySQL服务是否启动
- 确认用户名密码正确
- 检查防火墙设置

### 6.2 Diesel CLI安装失败
```bash
# 安装MySQL开发库
# Ubuntu/Debian:
sudo apt install libmysqlclient-dev

# macOS:
brew install mysql
```

### 6.3 端口冲突
- 后端端口：8080
- 前端端口：5173
- 如冲突，修改.env文件中的端口配置

## 7. 下一步行动
1. ✅ 完成环境配置
2. 🔄 创建数据库迁移
3. 🔄 实现第一个用户故事
4. 🔄 开发基础API