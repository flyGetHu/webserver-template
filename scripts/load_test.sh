#!/bin/bash

# WebServer Template 负载测试脚本
# 使用 wrk 工具进行 HTTP 负载测试

set -e

# 配置参数
SERVER_URL="http://localhost:7878"
DURATION="30s"
CONNECTIONS=100
THREADS=10
TEST_RESULTS_DIR="./test_results"

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 创建结果目录
mkdir -p "$TEST_RESULTS_DIR"

echo -e "${BLUE}=== WebServer Template 负载测试 ===${NC}"
echo -e "${YELLOW}测试配置:${NC}"
echo "  服务器地址: $SERVER_URL"
echo "  测试时长: $DURATION"
echo "  并发连接: $CONNECTIONS"
echo "  线程数: $THREADS"
echo ""

# 检查服务器是否运行
echo -e "${YELLOW}检查服务器状态...${NC}"
if ! curl -s "$SERVER_URL/api/v1/health" > /dev/null; then
    echo -e "${RED}错误: 服务器未运行或无法访问 $SERVER_URL${NC}"
    echo "请先启动服务器: cargo run"
    exit 1
fi
echo -e "${GREEN}✓ 服务器运行正常${NC}"
echo ""

# 测试函数
run_load_test() {
    local test_name="$1"
    local endpoint="$2"
    local method="$3"
    local headers="$4"
    local body="$5"
    
    echo -e "${BLUE}测试: $test_name${NC}"
    echo "  端点: $endpoint"
    echo "  方法: $method"
    
    local output_file="$TEST_RESULTS_DIR/${test_name}_$(date +%Y%m%d_%H%M%S).txt"
    
    if [ "$method" = "GET" ]; then
        wrk -t$THREADS -c$CONNECTIONS -d$DURATION \
            --latency \
            "$SERVER_URL$endpoint" \
            | tee "$output_file"
    elif [ "$method" = "POST" ]; then
        # 创建临时脚本文件用于 POST 请求
        local script_file="/tmp/wrk_post_script.lua"
        cat > "$script_file" << EOF
wrk.method = "POST"
wrk.body = '$body'
wrk.headers["Content-Type"] = "application/json"
$headers
EOF
        
        wrk -t$THREADS -c$CONNECTIONS -d$DURATION \
            --latency \
            -s "$script_file" \
            "$SERVER_URL$endpoint" \
            | tee "$output_file"
        
        rm -f "$script_file"
    fi
    
    echo ""
}

# 1. 健康检查端点测试
run_load_test "health_check" "/api/v1/health" "GET"

# 2. 用户注册端点测试
register_body='{"username":"test_user","email":"test@example.com","password":"test123456"}'
run_load_test "user_register" "/api/v1/auth/register" "POST" "" "$register_body"

# 3. 用户登录端点测试
login_body='{"username":"test_user","password":"test123456"}'
run_load_test "user_login" "/api/v1/auth/login" "POST" "" "$login_body"

# 4. 获取用户信息端点测试（需要认证）
# 首先获取 JWT token
echo -e "${YELLOW}获取认证令牌...${NC}"
TOKEN=$(curl -s -X POST "$SERVER_URL/api/v1/auth/login" \
    -H "Content-Type: application/json" \
    -d "$login_body" | \
    grep -o '"token":"[^"]*"' | \
    cut -d'"' -f4)

if [ -n "$TOKEN" ]; then
    echo -e "${GREEN}✓ 获取到认证令牌${NC}"
    
    # 创建带认证的测试脚本
    cat > "/tmp/wrk_auth_script.lua" << EOF
wrk.headers["Authorization"] = "Bearer $TOKEN"
EOF
    
    echo -e "${BLUE}测试: 认证用户信息查询${NC}"
    echo "  端点: /api/v1/users/me"
    echo "  方法: GET (带认证)"
    
    wrk -t$THREADS -c$CONNECTIONS -d$DURATION \
        --latency \
        -s "/tmp/wrk_auth_script.lua" \
        "$SERVER_URL/api/v1/users/me" \
        | tee "$TEST_RESULTS_DIR/user_info_$(date +%Y%m%d_%H%M%S).txt"
    
    rm -f "/tmp/wrk_auth_script.lua"
else
    echo -e "${RED}✗ 无法获取认证令牌，跳过认证测试${NC}"
fi

echo ""

# 生成测试报告
echo -e "${BLUE}=== 测试完成 ===${NC}"
echo -e "${GREEN}测试结果已保存到: $TEST_RESULTS_DIR${NC}"
echo ""

# 显示最新的测试结果摘要
echo -e "${YELLOW}最新测试结果摘要:${NC}"
for file in "$TEST_RESULTS_DIR"/*.txt; do
    if [ -f "$file" ]; then
        echo "文件: $(basename "$file")"
        echo "  请求总数: $(grep "requests in" "$file" | awk '{print $1}')"
        echo "  平均延迟: $(grep "Latency" "$file" | awk '{print $2}')"
        echo "  QPS: $(grep "Requests/sec:" "$file" | awk '{print $2}')"
        echo ""
    fi
done

echo -e "${BLUE}使用以下命令查看详细结果:${NC}"
echo "  ls -la $TEST_RESULTS_DIR"
echo "  cat $TEST_RESULTS_DIR/<test_file>.txt"
