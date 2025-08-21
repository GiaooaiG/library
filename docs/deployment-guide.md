# 图书馆管理系统部署指南

## 系统要求

### 开发环境要求

#### 后端要求
- **Rust**: 1.75+ (推荐最新稳定版)
- **MySQL**: 8.0+
- **Diesel CLI**: 最新版本
- **操作系统**: Windows 10/11, macOS 11+, 或 Linux (Ubuntu 20.04+)

#### 前端要求
- **Node.js**: 18.x LTS 或 20.x LTS
- **npm**: 9.x+ (或 yarn 1.22+)
- **现代浏览器**: Chrome 90+, Firefox 88+, Safari 14+, Edge 90+

### 检查系统环境
```bash
# 检查Rust版本
rustc --version

# 检查MySQL版本
mysql --version

# 检查Node.js版本
node --version

# 检查npm版本
npm --version
```

## 环境准备

### 1. 安装MySQL 8.0

#### Windows
1. 访问 [MySQL官网](https://dev.mysql.com/downloads/installer/)
2. 下载 MySQL Installer for Windows
3. 运行安装程序，选择 "Custom" 安装
4. 选择 MySQL Server 8.4 和 MySQL Workbench
5. 设置 root 密码（请记住此密码）

#### macOS
```bash
# 使用Homebrew安装
brew install mysql@8.4
brew services start mysql@8.4

# 设置root密码
mysql_secure_installation
```

#### Ubuntu/Debian
```bash
sudo apt update
sudo apt install mysql-server mysql-client
sudo mysql_secure_installation
```

### 2. 安装Rust
```bash
# 所有平台通用安装命令
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env  # 或重新打开终端

# Windows用户也可以使用 rustup-init.exe
```

### 3. 安装Node.js

#### Windows/macOS
访问 [Node.js官网](https://nodejs.org/) 下载 LTS 版本安装包

#### Ubuntu/Debian
```bash
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt-get install -y nodejs
```

## 数据库配置

### 1. 创建数据库和用户

#### 登录MySQL
```bash
# 使用root用户登录
mysql -u root -p

# 如果root密码为空，使用：
mysql -u root
```

#### 执行SQL命令
```sql
-- 创建数据库
CREATE DATABASE library_db CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- 创建用户并授权
CREATE USER 'library_user'@'localhost' IDENTIFIED BY '123456';
GRANT ALL PRIVILEGES ON library_db.* TO 'library_user'@'localhost';
FLUSH PRIVILEGES;

-- 验证数据库创建
SHOW DATABASES;

-- 退出MySQL
EXIT;
```

### 2. 安装Diesel CLI
```bash
# 安装Diesel CLI（仅MySQL支持）
cargo install diesel_cli --no-default-features --features mysql

# 验证安装
diesel --version
```

## 后端部署步骤

### 1. 代码获取和准备
```bash
# 克隆项目代码
git clone <your-repository-url>
cd library-system
```

### 2. 配置后端环境变量
创建 `library-backend/.env` 文件：
```bash
cd library-backend
# 创建.env文件
```

编辑 `.env` 文件内容：
```env
# 数据库配置
DATABASE_URL=mysql://library_user:123456@localhost/library_db

# 服务器配置
SERVER_HOST=127.0.0.1
SERVER_PORT=8080

# JWT配置
JWT_SECRET=your-secret-key-change-this-in-production
JWT_EXPIRATION=24h

# 日志配置
RUST_LOG=debug
```

### 3. 运行数据库迁移
```bash
# 确保在library-backend目录下
cd library-backend

# 设置数据库URL（仅第一次需要）
export DATABASE_URL=mysql://library_user:123456@localhost/library_db

# 运行迁移
diesel migration run

# 验证迁移
diesel migration list
```

### 4. 启动后端服务
```bash
# 开发模式启动
cargo run

# 或使用发布模式（更快）
cargo run --release

# 后台启动（开发时）
cargo run &
### 1. 安装前端依赖
```bash
# 进入前端目录
cd library-frontend

# 安装依赖
npm install

# 如果遇到权限问题，使用：
npm install --legacy-peer-deps
```

### 2. 配置前端环境变量
创建 `library-frontend/.env` 文件：
```bash
cd library-frontend
```

编辑 `.env` 文件内容：
```env
# API配置
VITE_API_URL=http://localhost:8080/api/v1

# 应用信息
VITE_APP_NAME=图书馆管理系统
VITE_APP_VERSION=1.0.0
```

### 3. 启动前端开发服务器
```bash
# 开发模式启动
npm run dev

# 指定端口启动（如果需要）
npm run dev -- --port 5173

# 服务将在 http://localhost:5173 启动
```

### 4. 构建生产版本（可选）
```bash
# 构建生产版本
npm run build

# 预览构建结果
npm run preview
```

## 环境变量详细配置

### 后端环境变量 (.env)
```env
# 数据库配置
DATABASE_URL=mysql://username:password@localhost/database_name

# 服务器配置
SERVER_HOST=127.0.0.1          # 服务器监听地址
SERVER_PORT=8080               # 服务器端口

# JWT配置
JWT_SECRET=your-secret-key     # JWT签名密钥（必须修改）
JWT_EXPIRATION=24h             # Token过期时间

# 日志配置
RUST_LOG=debug                 # 日志级别: error, warn, info, debug, trace
RUST_BACKTRACE=1               # 错误回溯

# CORS配置（可选）
ALLOWED_ORIGINS=http://localhost:5173,http://127.0.0.1:5173
```

### 前端环境变量 (.env)
```env
# API配置
VITE_API_URL=http://localhost:8080/api/v1

# 应用信息
VITE_APP_NAME=图书馆管理系统
VITE_APP_VERSION=1.0.0

# 功能开关
VITE_ENABLE_MOCK=false
VITE_DEBUG_MODE=true
```

## 数据库初始化和迁移

### 1. 数据库迁移步骤
```bash
# 1. 确保数据库已创建
mysql -u library_user -p -e "CREATE DATABASE IF NOT EXISTS library_db;"

# 2. 设置数据库URL环境变量
export DATABASE_URL=mysql://library_user:123456@localhost/library_db

# 3. 运行迁移
cd library-backend
diesel migration run

# 4. 验证迁移
diesel migration list
```

### 2. 数据库结构验证
```bash
# 登录MySQL检查表结构
mysql -u library_user -p library_db -e "SHOW TABLES;"

# 查看表结构
mysql -u library_user -p library_db -e "DESCRIBE users;"
```

### 3. 重置数据库（开发用）
```bash
# 回滚所有迁移
diesel migration redo

# 重置数据库（谨慎使用）
diesel database reset
```

## 快速启动脚本

### Windows (start.bat)
```batch
@echo off
echo 启动图书馆管理系统...

echo 启动后端服务...
start cmd /k "cd library-backend && cargo run"

timeout /t 5 /nobreak

echo 启动前端服务...
start cmd /k "cd library-frontend && npm run dev"

echo 系统启动完成！
echo 前端地址: http://localhost:5173
echo 后端地址: http://localhost:8080
pause
```

### macOS/Linux (start.sh)
```bash
#!/bin/bash
echo "启动图书馆管理系统..."

# 启动后端
echo "启动后端服务..."
cd library-backend
cargo run &

sleep 5

# 启动前端
echo "启动前端服务..."
cd ../library-frontend
npm run dev &
echo "系统启动完成！"
echo "前端地址: http://localhost:5173"
echo "后端地址: http://localhost:8080"
```

## 管理员账号和测试数据

### 1. 预设管理员账号
系统已预设管理员账号，部署完成后可直接使用：

- **用户名**: `admin`
- **密码**: `123456`
- **权限**: 完整管理员权限

### 2. 创建测试数据
```bash
# 使用curl测试管理员登录
curl -X POST http://localhost:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"123456"}'

# 添加测试图书
curl -X POST http://localhost

# 服务将在 http://localhost:8080 启动
```

## 前端部署步骤

## 故障排除指南

### 1. 数据库连接问题

#### 错误：Can't connect to MySQL server
```bash
# 检查MySQL服务状态
sudo systemctl status mysql  # Linux
net start mysql              # Windows

# 启动MySQL服务
sudo systemctl start mysql   # Linux
net start mysql              # Windows

# 检查端口
netstat -an | grep 3306      # Linux/macOS
netstat -an | findstr 3306   # Windows
```

#### 错误：Access denied for user
```sql
-- 重置用户密码
ALTER USER 'library_user'@'localhost' IDENTIFIED BY '123456';
FLUSH PRIVILEGES;

-- 检查用户权限
SHOW GRANTS FOR 'library_user'@'localhost';
```

### 2. Rust编译错误

#### 错误：diesel: command not found
```bash
# 重新安装Diesel CLI
cargo install diesel_cli --no-default-features --features mysql

# 检查PATH
echo $PATH | grep cargo
```

#### 错误：mysqlclient not found
```bash
# Ubuntu/Debian
sudo apt install libmysqlclient-dev

# CentOS/RHEL
sudo yum install mysql-devel

# macOS
brew install mysql
```

### 3. Node.js相关问题

#### 错误：npm install 失败
```bash
# 清除npm缓存
npm cache clean --force

# 删除node_modules和package-lock.json
rm -rf node_modules package-lock.json
npm install

# 使用国内镜像
npm config set registry https://registry.npmmirror.com
```

#### 错误：端口被占用
```bash
# 检查端口占用
lsof -i :8080    # 检查后端口
lsof -i :5173    # 检查前端口

# Windows
netstat -ano | findstr 8080
```

### 4. 常见错误解决方案

#### 后端启动失败
1. 检查 `.env` 文件是否存在且配置正确
2. 确认数据库已创建且用户有权限
3. 检查端口是否被占用
4. 查看详细错误日志：`RUST_LOG=debug cargo run`

#### 前端无法连接后端
1. 确认后端服务已启动
2. 检查 `VITE_API_URL` 配置是否正确
3. 检查CORS配置
4. 查看浏览器控制台错误信息

#### 数据库迁移失败
1. 确认数据库已创建
2. 检查数据库连接字符串
3. 确认用户权限足够
4. 手动执行迁移文件检查错误

## 验证部署成功

### 1. 基础功能验证清单
- [ ] MySQL服务运行正常
- [ ] 数据库 `library_db` 已创建
- [ ] 数据库迁移成功完成
- [ ] 后端服务在 http://localhost:8080 启动
- [ ] 前端服务在 http://localhost:5173 启动
- [ ] 管理员账号 `admin/123456` 可正常登录
- [ ] 可以正常添加图书
- [ ] 可以正常查询图书列表

### 2. 测试命令
```bash
# 测试后端健康检查
curl http://localhost:8080/api/v1/health

# 测试图书列表
curl http://localhost:8080/api/v1/books

# 测试管理员登录
curl -X POST http://localhost:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"123456"}'
```

## 开发环境优化建议

### 1. 开发工具推荐
- **IDE**: VS Code + Rust插件 + Volar插件
- **数据库工具**: MySQL Workbench 或 DBeaver
- **API测试**: Postman 或 curl
- **版本控制**: Git

### 2. 开发效率提升
```bash
# 安装cargo-watch实现热重载
cargo install cargo-watch

# 使用热重载启动后端
cargo watch -x run

# 前端热重载已内置
npm run dev
```

### 3. 日志查看技巧
```bash
# 查看后端日志
tail -f library-backend/logs/app.log

# 查看MySQL日志
tail -f /var/log/mysql/error.log
```

## 项目结构说明

```
library-system/
├── library-backend/          # 后端代码
│   ├── src/
│   │   ├── main.rs          # 主程序入口
│   │   ├── models/          # 数据模型
│   │   ├── services/        # 业务逻辑
│   │   └── handlers.rs        # 路由处理
│   ├── migrations/            # 数据库迁移文件
│   ├── Cargo.toml            # Rust依赖配置
│   └── .env                  # 后端环境变量
├── library-frontend/         # 前端代码
│   ├── src/
│   │   ├── main.tsx          # 前端入口
│   │   ├── components/       # React组件
│   │   └── services/         # API服务
│   ├── package.json          # Node.js依赖配置
│   ├── vite.config.ts        # Vite配置
│   └── .env                  # 前端环境变量
└── docs/                     # 项目文档
    ├── deployment-guide.md   # 部署指南
    └── setup-guide.md        # 设置指南
```

## 总结

完成以上步骤后，您将拥有一个完整的图书馆管理系统开发环境：

- **后端API**: http://localhost:8080
- **前端应用**: http://localhost:5173
- **管理员账号**: admin/123456
- **数据库**: MySQL 8.0 with library_db

系统支持图书管理、用户管理、借阅管理、权限控制等完整功能。

## 获取帮助

如果在部署过程中遇到问题：

1. 检查本指南的故障排除部分
2. 查看项目README.md文件
3. 检查GitHub Issues
4. 联系项目维护者

**快速重启命令**：
```bash
# 停止所有服务
pkill -f "cargo run"
pkill -f "npm run dev"

# 重新启动
./start.sh  # 或 start.bat
```