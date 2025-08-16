#!/usr/bin/env python3
"""
借书功能测试脚本
测试故事4.1的借书功能
"""

import requests
import json
import sys
from datetime import datetime

# 配置
BASE_URL = "http://localhost:8080/api/v1"
HEADERS = {"Content-Type": "application/json"}

def register_user(username, password, phone=None):
    """注册用户"""
    data = {
        "username": username,
        "password": password,
        "phone": phone
    }
    response = requests.post(f"{BASE_URL}/auth/register", json=data, headers=HEADERS)
    return response.json()

def login_user(username, password):
    """用户登录"""
    data = {
        "username": username,
        "password": password
    }
    response = requests.post(f"{BASE_URL}/auth/login", json=data, headers=HEADERS)
    return response.json()

def create_book(token, book_data):
    """创建图书"""
    headers = {**HEADERS, "Authorization": f"Bearer {token}"}
    response = requests.post(f"{BASE_URL}/books", json=book_data, headers=headers)
    return response.json()

def borrow_book(token, book_id):
    """借书"""
    headers = {**HEADERS, "Authorization": f"Bearer {token}"}
    data = {"book_id": book_id}
    response = requests.post(f"{BASE_URL}/borrow", json=data, headers=headers)
    return response.json()

def get_borrow_history(token):
    """获取借阅历史"""
    headers = {"Authorization": f"Bearer {token}"}
    response = requests.get(f"{BASE_URL}/borrow/history", headers=headers)
    return response.json()

def get_books(token):
    """获取图书列表"""
    headers = {"Authorization": f"Bearer {token}"}
    response = requests.get(f"{BASE_URL}/books", headers=headers)
    return response.json()

def test_borrow_function():
    """测试借书功能"""
    print("🧪 开始测试借书功能...")
    
    # 1. 注册用户
    print("\n1. 注册用户...")
    username = f"test_user_{datetime.now().strftime('%Y%m%d%H%M%S')}"
    register_result = register_user(username, "test123456", "13800138000")
    if not register_result.get("success"):
        print(f"❌ 注册失败: {register_result.get('message')}")
        return False
    print("✅ 注册成功")
    
    # 2. 用户登录
    print("\n2. 用户登录...")
    login_result = login_user(username, "test123456")
    if not login_result.get("success"):
        print(f"❌ 登录失败: {login_result.get('message')}")
        return False
    
    token = login_result["data"]["token"]
    user_id = login_result["data"]["user"]["id"]
    print(f"✅ 登录成功，用户ID: {user_id}")
    
    # 3. 创建测试图书
    print("\n3. 创建测试图书...")
    book_data = {
        "isbn": "9787115546081",
        "title": "Rust权威指南",
        "author": "Steve Klabnik",
        "category": "编程",
        "publisher": "人民邮电出版社",
        "total_copies": 5,
        "available_copies": 5
    }
    create_result = create_book(token, book_data)
    if not create_result.get("success"):
        print(f"❌ 创建图书失败: {create_result.get('message')}")
        return False
    
    book_id = create_result["data"]["id"]
    print(f"✅ 创建图书成功，图书ID: {book_id}")
    
    # 4. 获取图书列表
    print("\n4. 获取图书列表...")
    books_result = get_books(token)
    if not books_result.get("success"):
        print(f"❌ 获取图书列表失败: {books_result.get('message')}")
        return False
    
    books = books_result["data"]["data"]
    print(f"✅ 获取图书列表成功，共{len(books)}本图书")
    
    # 5. 借书测试
    print("\n5. 测试借书...")
    borrow_result = borrow_book(token, book_id)
    if not borrow_result.get("success"):
        print(f"❌ 借书失败: {borrow_result.get('message')}")
        return False
    
    print("✅ 借书成功")
    print(f"📖 借阅信息: {json.dumps(borrow_result['data'], indent=2, ensure_ascii=False)}")
    
    # 6. 验证库存减少
    print("\n6. 验证库存减少...")
    books_after = get_books(token)
    if not books_after.get("success"):
        print(f"❌ 获取图书列表失败: {books_after.get('message')}")
        return False
    
    book_after = next(book for book in books_after["data"]["data"] if book["id"] == book_id)
    if book_after["available_copies"] != 4:
        print(f"❌ 库存未正确减少，当前库存: {book_after['available_copies']}")
        return False
    
    print("✅ 库存正确减少")
    
    # 7. 获取借阅历史
    print("\n7. 获取借阅历史...")
    history_result = get_borrow_history(token)
    if not history_result.get("success"):
        print(f"❌ 获取借阅历史失败: {history_result.get('message')}")
        return False
    
    history = history_result["data"]
    print(f"✅ 获取借阅历史成功，共{len(history)}条记录")
    
    if len(history) > 0:
        print(f"📋 最新借阅: {json.dumps(history[0], indent=2, ensure_ascii=False)}")
    
    # 8. 重复借书测试
    print("\n8. 测试重复借书...")
    duplicate_result = borrow_book(token, book_id)
    if duplicate_result.get("success"):
        print("❌ 重复借书应该失败")
        return False
    
    print("✅ 重复借书正确被拒绝")
    
    print("\n🎉 所有测试通过！借书功能正常")
    return True

if __name__ == "__main__":
    try:
        success = test_borrow_function()
        sys.exit(0 if success else 1)
    except Exception as e:
        print(f"❌ 测试过程中发生错误: {e}")
        sys.exit(1)