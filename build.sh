#!/bin/bash

# 图书馆系统构建脚本
# 支持Windows (Git Bash) 和 Linux/macOS

echo "🚀 开始构建图书馆系统..."

# 设置颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 检查命令是否存在
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# 检查必要工具
check_requirements() {
    echo "📋 检查构建环境..."
    
    if ! command_exists cargo; then
        echo -e "${RED}❌ Rust/Cargo 未安装${NC}"
        echo "请访问: https://rustup.rs/"
        exit 1
    fi
    
    if ! command_exists npm; then
        echo -e "${RED}❌ Node.js/npm 未安装${NC}"
        echo "请访问: https://nodejs.org/"
        exit 1
    fi
    
    echo -e "${GREEN}✅ 环境检查通过${NC}"
}

# 构建后端
build_backend() {
    echo "🔧 构建后端 (Rust)..."
    cd library-backend
    
    # 检查并安装依赖
    echo "  📦 检查Rust依赖..."
    cargo check --quiet
    
    # 构建release版本
    echo "  🏗️  构建Release版本..."
    cargo build --release
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}  ✅ 后端构建成功${NC}"
        echo "  📁 二进制文件: target/release/library-backend(.exe)"
    else
        echo -e "${RED}  ❌ 后端构建失败${NC}"
        exit 1
    fi
    
    cd ..
}

# 构建前端
build_frontend() {
    echo "🎨 构建前端 (React + Vite)..."
    cd library-frontend
    
    # 安装依赖
    echo "  📦 安装Node.js依赖..."
    npm install --silent
    
    # 构建生产版本
    echo "  🏗️  构建生产版本..."
    npm run build
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}  ✅ 前端构建成功${NC}"
        echo "  📁 静态文件: dist/"
    else
        echo -e "${RED}  ❌ 前端构建失败${NC}"
        exit 1
    fi
    
    cd ..
}

# 创建发布包
create_package() {
    echo "📦 创建发布包..."
    
    # 创建发布目录
    mkdir -p release/library-system
    
    # 复制后端二进制文件
    if [ -f "library-backend/target/release/library-backend" ]; then
        cp library-backend/target/release/library-backend release/library-system/
    elif [ -f "library-backend/target/release/library-backend.exe" ]; then
        cp library-backend/target/release/library-backend.exe release/library-system/
    fi
    
    # 复制前端静态文件
    cp -r library-frontend/dist release/library-system/
    
    # 复制配置文件
    cp library-backend/.env release/library-system/
    cp -r library-backend/migrations release/library-system/
    
    # 创建启动脚本
    cat > release/library-system/start.sh << 'EOF'
#!/bin/bash
echo "🚀 启动图书馆系统..."
export DATABASE_URL="mysql://library_user:123456@localhost/library_db"
export SERVER_HOST="127.0.0.1"
export SERVER_PORT="8080"
export JWT_SECRET="your-secret-key-change-this"

# 启动后端服务
./library-backend
EOF

    cat > release/library-system/start.bat << 'EOF'
@echo off
echo 🚀 启动图书馆系统...
set DATABASE_URL=mysql://library_user:123456@localhost/library_db
set SERVER_HOST=127.0.0.1
set SERVER_PORT=8080
set JWT_SECRET=your-secret-key-change-this

:: 启动后端服务
library-backend.exe
pause
EOF

    chmod +x release/library-system/start.sh
    chmod +x release/library-system/library-backend*
    
    echo -e "${GREEN}✅ 发布包创建完成${NC}"
    echo "📁 发布目录: release/library-system/"
}

# 主流程
main() {
    check_requirements
    build_backend
    build_frontend
    create_package
    
    echo ""
    echo -e "${GREEN}🎉 构建完成！${NC}"
    echo ""
    echo "📋 使用说明:"
    echo "1. 确保MySQL数据库已配置"
    echo "2. 运行数据库迁移: diesel migration run"
    echo