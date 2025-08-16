#!/usr/bin/env python3
"""
续借功能测试脚本
测试故事4.3：续借功能
"""

import requests
import json
import time
from datetime import datetime, timedelta

# 配置
BASE_URL = "http://localhost:8080/api/v1"
HEADERS = {"Content-Type": "application/json"}

def test_renewal_function():
    """测试续借功能的完整流程"""
    print("🧪 开始测试续借功能...")
    
    # 1. 注册用户
    print("\n1. 注册用户...")
    register_data = {
        "username": "test_renewal_user",
        "password": "test123456",
        "phone": "13800138000"
    }
    
    response = requests.post(f"{BASE_URL}/auth/register", json=register_data, headers=HEADERS)
    if response.status_code == 409:
        print("用户已存在，继续测试...")
    elif response.status_code != 201:
        print(f"注册失败: {response.text}")
        return False
    
    # 2. 登录获取token
    print("\n2. 登录获取token...")
    login_data = {
        "username": "test_renewal_user",
        "password": "test123456"
    }
    
    response = requests.post(f"{BASE_URL}/auth/login", json=login_data, headers=HEADERS)
    if response.status_code != 200:
        print(f"登录失败: {response.text}")
        return False
    
    token = response.json()["data"]["token"]
    auth_headers = {"Authorization": f"Bearer {token}", "Content-Type": "application/json"}
    
    # 3. 创建测试图书
    print("\n3. 创建测试图书...")
    book_data = {
        "isbn": "9787121399999",
        "title": "续借测试图书",
        "author": "测试作者",
        "category": "测试",
        "total_copies": 5,
        "available_copies": 5
    }
    
    response = requests.post(f"{BASE_URL}/books", json=book_data, headers=auth_headers)
    if response.status_code != 201:
        print(f"创建图书失败: {response.text}")
        return False
    
    book_id = response.json()["data"]["id"]
    print(f"创建图书成功，ID: {book_id}")
    
    # 4. 借阅图书
    print("\n4. 借阅图书...")
    borrow_data = {"book_id": book_id}
    response = requests.post(f"{BASE_URL}/borrow", json=borrow_data, headers=auth_headers)
    if response.status_code != 201:
        print(f"借阅失败: {response.text}")
        return False
    
    borrow_record = response.json()["data"]
    borrow_id = borrow_record["id"]
    print(f"借阅成功，借阅记录ID: {borrow_id}")
    
    # 5. 获取借阅历史
    print("\n5. 获取借阅历史...")
    response = requests.get(f"{BASE_URL}/borrow/history", headers=auth_headers)
    if response.status_code != 200:
        print(f"获取借阅历史失败: {response.text}")
        return False
    
    history = response.json()["data"]
    print(f"当前借阅记录数: {len(history)}")
    
    # 6. 测试续借功能
    print("\n6. 测试续借功能...")
    response = requests.post(f"{BASE_URL}/renew/{borrow_id}", headers=auth_headers)
    if response.status_code == 200:
        print("✅ 续借成功！")
        renewed_record = response.json()["data"]
        print(f"新的到期日期: {renewed_record['due_date']}")
        print(f"续借次数: {renewed_record.get('renewal_count', 0)}")
    else:
        print(f"❌ 续借失败: {response.text}")
        return False
    
    # 7. 测试重复续借（应该失败）
    print("\n7. 测试重复续借...")
    response = requests.post(f"{BASE_URL}/renew/{borrow_id}", headers=auth_headers)
    if response.status_code == 400:
        print("✅ 重复续借被拒绝，符合预期")
    else:
        print(f"❌ 重复续借测试失败: {response.text}")
        return False
    
    # 8. 归还图书
    print("\n8. 归还图书...")
    response = requests.post(f"{BASE_URL}/return/{borrow_id}", headers=auth_headers)
    if response.status_code == 200:
        print("✅ 归还成功！")
    else:
        print(f"❌ 归还失败: {response.text}")
        return False
    
    print("\n🎉 续借功能测试完成！")
    return True

if __name__ == "__main__":
    # 等待服务启动
    print("等待服务启动...")
    time.sleep(2)
    
    try:
        success = test_renewal_function()
        if success:
            print("\n✅ 所有测试通过！")
        else:
            print("\n❌ 测试失败！")
    except Exception as e:
        print(f"\n❌ 测试异常: {e}")