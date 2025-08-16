#!/usr/bin/env python3
"""
故事5.1：库存统计功能API测试脚本
测试GET /api/v1/statistics/inventory接口
"""

import requests
import json
import time
from typing import Dict, Any

# 配置
BASE_URL = "http://localhost:8080"
API_ENDPOINT = f"{BASE_URL}/api/v1/statistics/inventory"

def test_inventory_stats_api():
    """测试库存统计API"""
    print("🚀 开始测试库存统计API...")
    
    # 测试1：基本功能测试
    print("\n📋 测试1：基本功能测试")
    try:
        response = requests.get(API_ENDPOINT)
        response.raise_for_status()
        
        data = response.json()
        print(f"✅ 请求成功，状态码: {response.status_code}")
        
        # 验证响应结构
        assert data.get('success') is True, "success字段应为True"
        assert 'data' in data, "data字段应存在"
        
        stats = data['data']
        required_fields = ['total_books', 'total_copies', 'available_copies', 
                          'borrowed_copies', 'category_stats']
        
        for field in required_fields:
            assert field in stats, f"{field}字段应存在"
            print(f"✅ {field}字段存在")
        
        # 验证数据类型
        assert isinstance(stats['total_books'], int), "total_books应为整数"
        assert isinstance(stats['total_copies'], int), "total_copies应为整数"
        assert isinstance(stats['available_copies'], int), "available_copies应为整数"
        assert isinstance(stats['borrowed_copies'], int), "borrowed_copies应为整数"
        assert isinstance(stats['category_stats'], list), "category_stats应为列表"
        
        print("✅ 数据类型验证通过")
        
    except requests.exceptions.RequestException as e:
        print(f"❌ 请求失败: {e}")
        return False
    except AssertionError as e:
        print(f"❌ 验证失败: {e}")
        return False
    
    # 测试2：数据一致性验证
    print("\n📋 测试2：数据一致性验证")
    try:
        stats = data['data']
        
        # 验证数学关系
        expected_borrowed = stats['total_copies'] - stats['available_copies']
        actual_borrowed = stats['borrowed_copies']
        
        assert expected_borrowed == actual_borrowed, \
            f"借出数量计算错误: 期望{expected_borrowed}, 实际{actual_borrowed}"
        
        # 验证非负性
        assert stats['total_books'] >= 0, "图书种类不能为负"
        assert stats['total_copies'] >= 0, "总藏书量不能为负"
        assert stats['available_copies'] >= 0, "可借图书不能为负"
        assert stats['borrowed_copies'] >= 0, "已借出不能为负"
        
        print("✅ 数据一致性验证通过")
        
    except AssertionError as e:
        print(f"❌ 数据验证失败: {e}")
        return False
    
    # 测试3：分类统计验证
    print("\n📋 测试3：分类统计验证")
    try:
        category_stats = stats['category_stats']
        
        if len(category_stats) > 0:
            for category in category_stats:
                required_category_fields = ['category', 'book_count', 'total_copies', 'available_copies']
                
                for field in required_category_fields:
                    assert field in category, f"分类统计缺少{field}字段"
                
                # 验证分类数据类型
                assert isinstance(category['book_count'], int), "book_count应为整数"
                assert isinstance(category['total_copies'], int), "total_copies应为整数"
                assert isinstance(category['available_copies'], int), "available_copies应为整数"
                
                # 验证分类数据一致性
                assert category['book_count'] >= 0, "分类图书数量不能为负"
                assert category['total_copies'] >= 0, "分类总藏书量不能为负"
                assert category['available_copies'] >= 0, "分类可借图书不能为负"
                assert category['available_copies'] <= category['total_copies'], \
                    "可借图书不能超过总藏书量"
            
            print(f"✅ 分类统计验证通过，共{len(category_stats)}个分类")
        else:
            print("⚠️  暂无分类数据")
        
    except AssertionError as e:
        print(f"❌ 分类统计验证失败: {e}")
        return False
    
    # 测试4：性能测试
    print("\n📋 测试4：性能测试")
    try:
        start_time = time.time()
        response = requests.get(API_ENDPOINT)
        end_time = time.time()
        
        response_time = (end_time - start_time) * 1000  # 转换为毫秒
        print(f"✅ 响应时间: {response_time:.2f}ms")
        
        if response_time > 1000:
            print("⚠️  响应时间超过1秒，建议优化")
        
    except Exception as e:
        print(f"❌ 性能测试失败: {e}")
        return False
    
    # 测试5：错误处理测试
    print("\n📋 测试5：错误处理测试")
    try:
        # 测试无效方法
        response = requests.post(API_ENDPOINT)
        assert response.status_code == 405, "POST方法应返回405"
        print("✅ 无效方法处理正确")
        
    except Exception as e:
        print(f"❌ 错误处理测试失败: {e}")
        return False
    
    # 打印详细统计信息
    print("\n📊 库存统计详情:")
    print(f"  图书种类: {stats['total_books']}")
    print(f"  总藏书量: {stats['total_copies']}")
    print(f"  可借图书: {stats['available_copies']}")
    print(f"  已借出: {stats['borrowed_copies']}")
    print(f"  借阅率: {(stats['borrowed_copies']/stats['total_copies']*100):.1f}%" if stats['total_copies'] > 0 else "  借阅率: 0.0%")
    
    if len(category_stats) > 0:
        print("\n  分类统计:")
        for category in category_stats[:5]:  # 只显示前5个分类
            cat_name = category['category'] or '未分类'
            print(f"    {cat_name}: {category['book_count']}种, {category['total_copies']}册, {category['available_copies']}册可借")
        
        if len(category_stats) > 5:
            print(f"    ... 还有{len(category_stats)-5}个分类")
    
    return True

def test_with_auth():
    """测试需要认证的端点"""
    print("\n🔐 测试认证相关功能...")
    
    # 注册用户
    register_data = {
        "username": "test_user_stats",
        "password": "test123",
        "phone": "13800138000"
    }
    
    try:
        # 注册
        register_response = requests.post(f"{BASE_URL}/api/v1/auth/register", json=register_data)
        if register_response.status_code == 201:
            print("✅ 测试用户注册成功")
        else:
            print("⚠️  测试用户可能已存在")
        
        # 登录
        login_response = requests.post(f"{BASE_URL}/api/v1/auth/login", json={
            "username": "test_user_stats",
            "password": "test123"
        })
        
        if login_response.status_code == 200:
            token = login_response.json()['data']['token']
            print("✅ 测试用户登录成功")
            
            # 使用token访问库存统计
            headers = {"Authorization": f"Bearer {token}"}
            response = requests.get(API_ENDPOINT, headers=headers)
            
            if response.status_code == 200:
                print("✅ 认证用户可正常访问库存统计")
            else:
                print(f"❌ 认证用户访问失败: {response.status_code}")
        
    except Exception as e:
        print(f"⚠️  认证测试跳过: {e}")

if __name__ == "__main__":
    print("=" * 60)
    print("故事5.1：库存统计功能API测试")
    print("=" * 60)
    
    try:
        # 运行主要测试
        success = test_inventory_stats_api()
        
        if success:
            print("\n" + "=" * 60)
            print("🎉 所有测试通过！库存统计功能API工作正常")
            print("=" * 60)
        else:
            print("\n" + "=" * 60)
            print("❌ 测试失败，请检查问题")
            print("=" * 60)
            exit(1)
            
    except KeyboardInterrupt:
        print("\n\n测试被中断")
    except Exception as e:
        print(f"\n❌ 测试执行失败: {e}")
        exit(1)