# 图书馆管理系统前端

基于React + TypeScript + Vite构建的现代化前端应用，用于图书馆图书管理。

## 功能特性

- 📚 图书添加功能
- 📖 图书列表展示
- 🔍 实时数据同步
- ✅ 表单验证
- 📱 响应式设计

## 技术栈

- **React 19** - 前端框架
- **TypeScript** - 类型安全
- **Vite** - 构建工具
- **React Hook Form** - 表单处理
- **Zod** - 数据验证
- **Axios** - HTTP客户端
- **CSS3** - 样式

## 快速开始

### 1. 安装依赖

```bash
npm install
```

### 2. 启动开发服务器

```bash
npm run dev
```

应用将在 http://localhost:5173 启动

### 3. 构建生产版本

```bash
npm run build
```

## 项目结构

```
src/
├── components/          # 可复用组件
│   ├── BookForm.tsx   # 图书添加表单
│   └── BookForm.css   # 表单样式
├── services/          # API服务
│   └── api.ts        # API接口封装
├── test/             # 测试文件
│   └── BookAddTest.tsx # 集成测试
├── App.tsx           # 主应用组件
├── App.css           # 主应用样式
└── main.tsx          # 应用入口
```

## 使用说明

### 添加新图书

1. 打开应用首页
2. 填写图书信息表单
3. 点击"添加图书"按钮
4. 查看添加结果和图书列表

### 表单验证规则

- **ISBN**: 必须为13位数字
- **书名**: 不能为空，最多255字符
- **作者**: 不能为空，最多255字符
- **分类**: 最多100字符（可选）
- **出版社**: 最多255字符（可选）
- **总数量**: 必须大于0
- **可用数量**: 不能为负数，不能超过总数量

## API接口

- `GET /api/v1/books` - 获取所有图书
- `POST /api/v1/books` - 添加新图书
- `GET /api/v1/books/:id` - 获取特定图书

## 环境变量

创建 `.env` 文件：

```env
VITE_API_URL=http://localhost:8080/api/v1
```

## 测试

运行集成测试：

1. 确保后端服务已启动
2. 访问测试页面（可在App.tsx中添加测试组件）
3. 运行测试用例

## 开发指南

### 添加新功能

1. 在 `components/` 目录创建新组件
2. 在 `services/api.ts` 添加API接口
3. 更新相关样式文件
4. 添加必要的测试用例

### 代码规范

- 使用TypeScript进行类型检查
- 遵循React Hooks最佳实践
- 使用CSS模块或CSS-in-JS
- 保持组件单一职责

## 故障排除

### 常见问题

1. **API连接失败**
   - 检查后端服务是否运行
   - 确认环境变量配置正确

2. **依赖安装失败**
   - 清除node_modules和package-lock.json
   - 重新运行 `npm install`

3. **类型错误**
   - 运行 `npm run build` 检查类型
   - 确保所有依赖已正确安装

## 贡献指南

1. Fork 项目
2. 创建功能分支
3. 提交更改
4. 发起Pull Request

## 许可证

MIT License
