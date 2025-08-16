#!/bin/bash

# 借书功能测试脚本
echo "🧪 开始测试借书功能..."

# 启动后端服务
echo "启动后端服务..."
cd library-backend
cargo build --release
./target/release/library-backend &
BACKEND_PID=$!
sleep 5

# 等待服务启动
echo "等待服务启动..."
sleep 3

# 测试借书功能
echo "🧪 运行Python测试脚本..."
cd ..
python3 test_borrow_function.py

# 测试结果
TEST_RESULT=$?

# 清理
echo "清理测试环境..."
kill $BACKEND_PID 2>/dev/null || true

if [ $TEST_RESULT -eq 0 ]; then
    echo "✅ 借书功能测试通过！"
else
    echo "❌ 借书功能测试失败！"
fi

exit $TEST_RESULT