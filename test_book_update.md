# 图书编辑功能测试指南

## 后端API测试

### 1. 启动后端服务
```bash
cd library-backend
cargo run
```

### 2. 测试更新图书API

#### API端点
```
PUT /api/v1/books/{id}
```

#### 请求头
```
Authorization: Bearer {your_jwt_token}
Content-Type: application/json
```

#### 请求体示例
```json
{
  "isbn": "9787115546081",
  "title": "Rust权威指南（更新版）",
  "author": "Steve Klabnik",
  "category": "编程",
  "publisher": "人民邮电出版社",
  "total_copies": 10,
  "available_copies": 8
}
```

#### 预期响应
```json
{
  "success": true,
  "message": "",
  "data": {
    "id": 1,
    "isbn": "9787115546081",
    "title": "Rust权威指南（更新版）",
    "author": "Steve Klabnik",
    "category": "编程",
    "publisher": "人民邮电出版社",
    "total_copies": 10,
    "available_copies": 8,
    "created_at": "2025-08-15T03:22:11",
    "updated_at": "2025-12-09T10:30:00"
  }
}
```

## 前端功能测试

### 1. 启动前端服务
```bash
cd library-frontend
npm run dev
```

### 2. 测试步骤

1. **登录管理员账户**
   - 用户名：admin
   - 密码：需要设置或从数据库获取

2. **进入图书列表页面**
   - 访问：`http://localhost:5173`
   - 应该能看到图书列表

3. **点击编辑按钮**
   - 在图书卡片上点击"编辑"按钮
   - 应该跳转到编辑页面：`/admin/books/edit/{id}`

4. **修改图书信息**
   - 修改书名、作者、数量等信息
   - 点击"更新图书"按钮

5. **验证更新结果**
   - 应该显示"图书更新成功！"
   - 自动返回图书列表页面
   - 图书信息应该已更新

## 常见问题排查

### 1. 权限问题
- 确保使用管理员账户登录
- 检查JWT令牌是否正确包含在请求头中

### 2. 数据验证失败
- 检查ISBN是否为13位数字
- 确保总数量和可用数量为有效值
- 可用数量不能超过总数量

### 3. 数据库连接问题
- 检查PostgreSQL是否运行
- 确认数据库连接配置正确
- 检查表结构是否正确

### 4. 路由问题
- 确认前端路由配置正确
- 检查后端API端点是否匹配
- 验证参数传递是否正确

## 功能验证清单

- [ ] 管理员可以访问编辑页面
- [ ] 普通用户无法访问编辑页面
- [ ] 可以正确加载现有图书数据
- [ ] 可以修改图书信息
- [ ] 数据验证正常工作
- [ ] 更新成功后显示正确提示
- [ ] 更新成功后自动返回列表页面
- [ ] 数据库中的数据确实被更新
- [ ] updated_at字段自动更新