# 图书馆管理系统技术架构文档

## 1. 系统架构概览

### 1.1 架构模式
- **模式**: 前后端分离架构
- **部署**: 本地开发环境
- **通信**: RESTful API + JSON

### 1.2 技术栈
```mermaid
graph TD
    A[React前端] -->|HTTP/JSON| B[Actix-web后端]
    B -->|Diesel ORM| C[MySQL 8.0]
    C -->|本地文件| D[数据存储]
```

## 2. 数据库设计

### 2.1 实体关系图
```mermaid
erDiagram
    BOOKS ||--o{ BORROW_RECORDS : has
    USERS ||--o{ BORROW_RECORDS : creates
    
    BOOKS {
        int id PK
        string isbn UK
        string title
        string author
        string category
        string publisher
        int total_copies
        int available_copies
        datetime created_at
        datetime updated_at
    }
    
    USERS {
        int id PK
        string username UK
        string password_hash
        string role
        string phone
        datetime created_at
        datetime updated_at
    }
    
    BORROW_RECORDS {
        int id PK
        int user_id FK
        int book_id FK
        datetime borrow_date
        datetime due_date
        datetime return_date
        string status
    }
```

### 2.2 表结构定义

#### 图书表 (books)
```sql
CREATE TABLE books (
    id INT AUTO_INCREMENT PRIMARY KEY,
    isbn VARCHAR(13) UNIQUE NOT NULL,
    title VARCHAR(255) NOT NULL,
    author VARCHAR(255) NOT NULL,
    category VARCHAR(100),
    publisher VARCHAR(255),
    total_copies INT DEFAULT 1,
    available_copies INT DEFAULT 1,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_isbn (isbn),
    INDEX idx_category (category)
);
```

#### 用户表 (users)
```sql
CREATE TABLE users (
    id INT AUTO_INCREMENT PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    role ENUM('reader', 'admin') DEFAULT 'reader',
    phone VARCHAR(20),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);
```

#### 借阅记录表 (borrow_records)
```sql
CREATE TABLE borrow_records (
    id INT AUTO_INCREMENT PRIMARY KEY,
    user_id INT NOT NULL,
    book_id INT NOT NULL,
    borrow_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    due_date TIMESTAMP NOT NULL,
    return_date TIMESTAMP NULL,
    status ENUM('borrowed', 'returned', 'overdue') DEFAULT 'borrowed',
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (book_id) REFERENCES books(id) ON DELETE CASCADE,
    INDEX idx_user_id (user_id),
    INDEX idx_book_id (book_id),
    INDEX idx_status (status)
);
```

## 3. API设计

### 3.1 基础规范
- **Base URL**: `http://localhost:8080/api/v1`
- **认证**: 本地密码验证
- **格式**: JSON
- **状态码**: 标准HTTP状态码

### 3.2 端点设计

#### 图书管理
| 方法 | 路径 | 描述 | 权限 |
|---|---|---|---|
| GET | `/books` | 获取图书列表 | 所有用户 |
| GET | `/books/:id` | 获取图书详情 | 所有用户 |
| POST | `/books` | 添加图书 | 管理员 |
| PUT | `/books/:id` | 更新图书信息 | 管理员 |
| DELETE | `/books/:id` | 删除图书 | 管理员 |

#### 用户管理
| 方法 | 路径 | 描述 | 权限 |
|---|---|---|---|
| POST | `/auth/register` | 用户注册 | 所有用户 |
| POST | `/auth/login` | 用户登录 | 所有用户 |
| GET | `/users/profile` | 获取用户信息 | 已登录用户 |

#### 借阅管理
| 方法 | 路径 | 描述 | 权限 |
|---|---|---|---|
| POST | `/borrow` | 借书 | 读者 |
| POST | `/return` | 还书 | 读者 |
| GET | `/borrow/history` | 借阅历史 | 读者 |

### 3.3 请求/响应示例

#### 借书请求
```json
POST /api/v1/borrow
{
    "book_id": 1,
    "user_id": 1
}
```

#### 借书响应
```json
{
    "success": true,
    "data": {
        "record_id": 1,
        "book_title": "Rust权威指南",
        "due_date": "2025-08-22T10:00:00Z"
    }
}
```

## 4. 项目结构

### 4.1 目录结构
```
library-management/
├── backend/
│   ├── src/
│   │   ├── main.rs
│   │   ├── models/
│   │   ├── handlers/
│   │   ├── services/
│   │   └── utils/
│   ├── migrations/
│   └── Cargo.toml
├── frontend/
│   ├── src/
│   │   ├── components/
│   │   ├── pages/
│   │   └── services/
│   └── package.json
├── docs/
│   ├── brief.md
│   ├── prd.md
│   └── architecture.md
└── README.md
```

### 4.2 启动脚本
```bash
# 一键启动脚本
#!/bin/bash
echo "启动图书馆管理系统..."
cd backend && cargo run &
cd frontend && npm run dev &
```

## 5. 开发环境配置

### 5.1 后端环境
```bash
# 安装依赖
cargo install diesel_cli --no-default-features --features mysql

# 数据库初始化
diesel setup
diesel migration run
```

### 5.2 前端环境
```bash
# 安装依赖
npm install

# 启动开发服务器
npm run dev
```

## 6. 部署说明

### 6.1 本地部署步骤
1. 安装MySQL 8.0
2. 创建数据库 `library_db`
3. 运行数据库迁移
4. 启动后端服务
5. 启动前端服务

### 6.2 配置示例
```bash
# .env文件
DATABASE_URL=mysql://user:password@localhost/library_db
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
```

## 7. 测试策略

### 7.1 单元测试
- 模型验证测试
- 业务逻辑测试
- API接口测试

### 7.2 集成测试
- 数据库操作测试
- 端到端流程测试
- 并发操作测试

## 8. 下一步计划
1. 创建项目骨架
2. 实现数据库迁移
3. 开发基础API
4. 前端界面开发
5. 集成测试