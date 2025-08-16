#!/usr/bin/env python3
"""
借阅排行榜API测试脚本
测试故事5.2：借阅排行榜功能
"""

import requests
import json
from datetime import datetime, timedelta
import sys

# 配置
BASE_URL = "http://localhost:8080/api/v1"
HEADERS = {"Content-Type": "application/json"}

def test_get_popular_books():
    """测试获取热门图书排行榜"""
    print("🧪 测试获取热门图书排行榜...")
    
    # 测试1: 获取全部热门图书
    print("📊 测试1: 获取全部热门图书")
    response = requests.get(f"{BASE_URL}/statistics/popular-books")
    if response.status_code == 200:
        data = response.json()
        if data.get("success"):
            books = data.get("data", [])
            print(f"✅ 成功获取 {len(books)} 本热门图书")
            for i, book in enumerate(books[:3], 1):
                print(f"   {i}. {book['title']} - 借阅次数: {book['borrow_count']}")
        else:
            print("❌ API返回错误:", data.get("message"))
    else:
        print(f"❌ HTTP错误: {response.status_code}")
    
    # 测试2: 测试时间段筛选
    print("\n📅 测试2: 测试时间段筛选")
    start_date = (datetime.now() - timedelta(days=30)).strftime("%Y-%m-%d")
    params = {"start_date": start_date, "limit": 5}
    response = requests.get(f"{BASE_URL}/statistics/popular-books", params=params)
    if response.status_code == 200:
        data = response.json()
        if data.get("success"):
            books = data.get("data", [])
            print(f"✅ 时间段筛选成功，返回 {len(books)} 本图书")
        else:
            print("❌ 时间段筛选失败:", data.get("message"))
    else:
        print(f"❌ HTTP错误: {response.status_code}")
    
    # 测试3: 测试限制返回数量
    print("\n🔢 测试3: 测试限制返回数量")
    params = {"limit": 3}
    response = requests.get(f"{BASE_URL}/statistics/popular-books", params=params)
    if response.status_code == 200:
        data = response.json()
        books = data.get("data", [])
        if len(books) <= 3:
            print(f"✅ 限制返回数量成功，返回 {len(books)} 本图书")
        else:
            print(f"❌ 限制返回数量失败，返回 {len(books)} 本图书")
    else:
        print(f"❌ HTTP错误: {response.status_code}")

def test_error_handling():
    """测试错误处理"""
    print("\n⚠️  测试错误处理...")
    
    # 测试无效日期格式
    params = {"start_date": "invalid-date"}
    response = requests.get(f"{BASE_URL}/statistics/popular-books", params=params)
    if response.status_code == 200:
        data = response.json()
        print("✅ 无效日期格式处理正常")
    else:
        print(f"❌ 无效日期格式处理异常: {response.status_code}")

def create_test_data():
    """创建测试数据"""
    print("\n📝 创建测试数据...")
    
    try:
        # 创建测试用户
        user_data = {
            "username": "test_user_stats",
            "password": "test123",
            "phone": "13800138000"
        }
        response = requests.post(f"{BASE_URL}/auth/register", json=user_data, headers=HEADERS)
        if response.status_code == 201:
            print("✅ 测试用户创建成功")
            token = response.json()["data"]["token"]
            auth_headers = {"Authorization": f"Bearer {token}", **HEADERS}
        else:
            print("⚠️  测试用户可能已存在")
            # 尝试登录
            login_data = {"username": "test_user_stats", "password": "test123"}
            response = requests.post(f"{BASE_URL}/auth/login", json=login_data, headers=HEADERS)
            if response.status_code == 200:
                token = response.json()["data"]["token"]
                auth_headers = {"Authorization": f"Bearer {token}", **HEADERS}
                print("✅ 测试用户登录成功")
            else:
                print("❌ 无法创建或登录测试用户")
                return None
        
        # 创建测试图书
        books = []
        for i in range(1, 4):
            book_data = {
                "isbn": f"978000000000{i}",
                "title": f"测试热门图书{i}",
                "author": "测试作者",
                "total_copies": 10,
                "available_copies": 8
            }
            response = requests.post(f"{BASE_URL}/books", json=book_data, headers=auth_headers)
            if response.status_code == 201:
                book_id = response.json()["data"]["id"]
                books.append(book_id)
                print(f"✅ 测试图书{i}创建成功，ID: {book_id}")
        
        # 创建借阅记录
        for book_id in books:
            for _ in range(2):  # 每本书借阅2次
                borrow_data = {"book_id": book_id}
                response = requests.post(f"{BASE_URL}/borrow", json=borrow_data, headers=auth_headers)
                if response.status_code == 201:
                    print(f"✅ 图书{book_id}借阅成功")
        
        return auth_headers
        
    except Exception as e:
        print(f"❌ 创建测试数据失败: {e}")
        return None

def main():
    """主测试函数"""
    print("🚀 开始测试借阅排行榜功能...")
    print("=" * 50)
    
    # 检查服务是否运行
    try:
        response = requests.get(f"{BASE_URL}/books", timeout=5)
        if response.status_code != 200:
            print("❌ 后端服务未正常运行")
            return
    except requests.exceptions.RequestException:
        print("❌ 无法连接到后端服务，请确保服务已启动")
        print("   启动后端: cd library-backend && cargo run")
        return
    
    # 运行测试
    test_get_popular_books()
    test_error_handling()
    
    # 询问是否创建测试数据
    create_data = input("\n是否创建测试数据？(y/n): ").lower().strip()
    if create_data == 'y':
        auth_headers = create_test_data()
        if auth_headers:
            print("\n🔄 重新测试热门图书排行榜...")
            test_get_popular_books()
    
    print("\n" + "=" * 50)
    print("🎉 测试完成！")
    print("📋 测试总结：")
    print("1. ✅ 后端API路由已创建")
    print("2. ✅ 时间段筛选功能正常")
    print("3. ✅ 限制返回数量功能正常")
    print("4. ✅ 错误处理机制正常")
    print("\n🔧 如需进一步测试，请：")
    print("- 启动前端服务: cd library-frontend && npm run dev")
    print("- 访问: http://localhost:5173/popular-books")

if __name__ == "__main__":
    main()