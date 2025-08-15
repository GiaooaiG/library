# 图书馆管理系统部署指南

## 系统要求

### 后端要求
- Rust 1.75+
- MySQL 8.0+
- Diesel CLI

### 前端要求
- Node.js 18+
- npm 或 yarn

## 部署步骤

### 1. 数据库设置

#### 1.1 安装MySQL
确保MySQL已安装并运行：
```bash
# Ubuntu/Debian
sudo apt-get install mysql-server

# macOS
brew install mysql
brew services start mysql

# Windows
# 下载并安装MySQL Installer
```

#### 1.2 创建数据库和用户
```sql
CREATE DATABASE library_db;
CREATE USER 'library_user'@'localhost' IDENTIFIED BY '123456';
GRANT ALL PRIVILEGES ON library_db.* TO 'library_user'@'localhost';
FLUSH PRIVILEGES;
```

#### 1.3 运行数据库迁移
```bash
cd library-backend
# 安装Diesel CLI（如果未安装）
cargo install diesel_cli --no-default-features --features mysql

# 运行迁移
diesel migration run
```

### 2. 后端部署

#### 2.1 配置环境变量
编辑 `library-backend/.env` 文件：
```env
DATABASE_URL=mysql://library_user:123456@localhost/library_db
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
```

#### 2.2 启动后端服务
```bash
cd library-backend
export LIBRARY_PATH="/opt/homebrew/lib:/opt/homebrew/Cellar/openssl@3/3.5.
2/lib" && export DYLD_LIBRARY_PATH="/opt/homebrew/lib:/opt/homebrew/Cellar/openssl@3/3.5.2/lib" && cargo run
```

后端服务将在 http://localhost:8080 启动

### 3. 前端部署

#### 3.1 安装依赖
```bash
cd library-frontend
npm install
```

#### 3.2 配置环境变量
确保 `library-frontend/.env` 文件存在：
```env
VITE_API_URL=http://localhost:8080/api/v1
```

#### 3.3 启动开发服务器
```bash
npm run dev
```

前端应用将在 http://localhost:5173 启动

#### 3.4 构建生产版本
```bash
npm run build
```

构建后的文件在 `dist/` 目录

## 功能测试

### 1. 后端API测试

#### 1.1 测试健康检查
```bash
curl http://localhost:8080/api/v1/books
```

#### 1.2 测试添加图书
```bash
curl -X POST http://localhost:8080/api/v1/books \
  -H "Content-Type: application/json" \
  -d '{
    "isbn": "9787123456789",
    "title": "测试图书",
    "author": "测试作者",
    "category": "测试分类",
    "publisher": "测试出版社",
    "total_copies": 5,
    "available_copies": 5
  }'
```

### 2. 前端功能测试

#### 2.1 手动测试
1. 打开 http://localhost:5173
2. 填写图书添加表单
3. 提交表单并查看结果
4. 检查图书列表更新

#### 2.2 集成测试
使用提供的测试组件：
```typescript
// 在App.tsx中添加测试组件
import BookAddTest from './test/BookAddTest';

// 在组件中添加
<BookAddTest />
```

## 故障排除

### 常见问题

#### 1. 数据库连接失败
- 检查MySQL服务是否运行
- 确认用户名密码正确
- 检查数据库权限

#### 2. 端口冲突
- 后端端口：8080
- 前端端口：5173
- 使用 `lsof -i :8080` 检查端口占用

#### 3. CORS问题
- 确保后端已配置CORS
- 检查前端API_URL配置

#### 4. 依赖问题
```bash
# 清除缓存重新安装
rm -rf node_modules package-lock.json
npm install

# 或清除Cargo缓存
cargo clean
cargo build
```

## 性能优化

### 后端优化
- 启用连接池
- 添加数据库索引
- 使用缓存

### 前端优化
- 启用代码分割
- 使用懒加载
- 优化图片资源

## 监控和日志

### 后端日志
- 日志文件：`library-backend/logs/`
- 日志级别：info, warn, error

### 前端监控
- 控制台错误监控
- 性能指标收集

## 安全建议

### 后端安全
- 使用HTTPS
- 输入验证和清理
- 数据库连接加密
- 限制请求频率

### 前端安全
- 输入验证
- XSS防护
- HTTPS强制

## 扩展功能

### 计划中的功能
- 用户认证系统
- 图书借阅管理
- 搜索和筛选
- 批量导入
- 数据导出

## 支持

如有问题，请检查：
1. 所有服务是否正常运行
2. 网络连接是否正常
3. 配置文件是否正确
4. 查看日志文件获取详细信息