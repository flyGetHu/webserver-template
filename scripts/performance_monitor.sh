#!/bin/bash

# WebServer Template 性能监控脚本
# 监控应用运行时的系统资源使用情况

set -e

# 配置参数
MONITOR_DURATION=60  # 监控时长（秒）
SAMPLE_INTERVAL=2    # 采样间隔（秒）
OUTPUT_DIR="./performance_data"
APP_NAME="webserver-template"

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 创建输出目录
mkdir -p "$OUTPUT_DIR"

echo -e "${BLUE}=== WebServer Template 性能监控 ===${NC}"
echo -e "${YELLOW}监控配置:${NC}"
echo "  应用名称: $APP_NAME"
echo "  监控时长: ${MONITOR_DURATION}秒"
echo "  采样间隔: ${SAMPLE_INTERVAL}秒"
echo "  输出目录: $OUTPUT_DIR"
echo ""

# 获取应用进程ID
get_app_pid() {
    pgrep -f "$APP_NAME" | head -1
}

# 检查应用是否运行
APP_PID=$(get_app_pid)
if [ -z "$APP_PID" ]; then
    echo -e "${RED}错误: 找不到运行中的 $APP_NAME 进程${NC}"
    echo "请先启动应用: cargo run"
    exit 1
fi

echo -e "${GREEN}✓ 找到应用进程 PID: $APP_PID${NC}"
echo ""

# 生成时间戳
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

# 输出文件
CPU_FILE="$OUTPUT_DIR/cpu_usage_$TIMESTAMP.csv"
MEMORY_FILE="$OUTPUT_DIR/memory_usage_$TIMESTAMP.csv"
NETWORK_FILE="$OUTPUT_DIR/network_stats_$TIMESTAMP.csv"
DISK_FILE="$OUTPUT_DIR/disk_io_$TIMESTAMP.csv"
SUMMARY_FILE="$OUTPUT_DIR/performance_summary_$TIMESTAMP.txt"

# 初始化CSV文件头
echo "timestamp,cpu_percent,cpu_user,cpu_system" > "$CPU_FILE"
echo "timestamp,memory_rss_mb,memory_vms_mb,memory_percent" > "$MEMORY_FILE"
echo "timestamp,bytes_sent,bytes_recv,packets_sent,packets_recv" > "$NETWORK_FILE"
echo "timestamp,read_bytes,write_bytes,read_ops,write_ops" > "$DISK_FILE"

echo -e "${YELLOW}开始监控...${NC}"
echo "按 Ctrl+C 停止监控"
echo ""

# 监控函数
monitor_performance() {
    local start_time=$(date +%s)
    local end_time=$((start_time + MONITOR_DURATION))
    local sample_count=0
    
    # 获取初始网络统计
    local initial_net_stats=$(cat /proc/net/dev | grep -E "(eth0|wlan0|enp|wlp)" | head -1 | awk '{print $2,$10,$3,$11}')
    
    while [ $(date +%s) -lt $end_time ]; do
        local current_time=$(date '+%Y-%m-%d %H:%M:%S')
        
        # 检查进程是否还在运行
        if ! kill -0 "$APP_PID" 2>/dev/null; then
            echo -e "${RED}警告: 应用进程已停止${NC}"
            break
        fi
        
        # CPU 使用率
        local cpu_stats=$(ps -p "$APP_PID" -o %cpu,time --no-headers)
        local cpu_percent=$(echo "$cpu_stats" | awk '{print $1}')
        
        # 更详细的CPU信息
        local proc_stat=$(cat /proc/"$APP_PID"/stat 2>/dev/null || echo "0 0 0 0")
        local cpu_user=$(echo "$proc_stat" | awk '{print $14}')
        local cpu_system=$(echo "$proc_stat" | awk '{print $15}')
        
        # 内存使用
        local memory_stats=$(ps -p "$APP_PID" -o rss,vsz,%mem --no-headers)
        local memory_rss_kb=$(echo "$memory_stats" | awk '{print $1}')
        local memory_vms_kb=$(echo "$memory_stats" | awk '{print $2}')
        local memory_percent=$(echo "$memory_stats" | awk '{print $3}')
        local memory_rss_mb=$((memory_rss_kb / 1024))
        local memory_vms_mb=$((memory_vms_kb / 1024))
        
        # 网络统计
        local current_net_stats=$(cat /proc/net/dev | grep -E "(eth0|wlan0|enp|wlp)" | head -1 | awk '{print $2,$10,$3,$11}')
        
        # 磁盘I/O统计
        local io_stats=""
        if [ -f "/proc/$APP_PID/io" ]; then
            io_stats=$(cat /proc/"$APP_PID"/io 2>/dev/null | grep -E "(read_bytes|write_bytes)" | awk '{print $2}' | tr '\n' ' ')
        else
            io_stats="0 0"
        fi
        local read_bytes=$(echo "$io_stats" | awk '{print $1}')
        local write_bytes=$(echo "$io_stats" | awk '{print $2}')
        
        # 写入数据
        echo "$current_time,$cpu_percent,$cpu_user,$cpu_system" >> "$CPU_FILE"
        echo "$current_time,$memory_rss_mb,$memory_vms_mb,$memory_percent" >> "$MEMORY_FILE"
        echo "$current_time,$current_net_stats" >> "$NETWORK_FILE"
        echo "$current_time,$read_bytes,$write_bytes,0,0" >> "$DISK_FILE"
        
        # 实时显示
        printf "\r${BLUE}样本 %d${NC} | CPU: %s%% | 内存: %dMB (RSS) | 时间: %s" \
               "$((++sample_count))" "$cpu_percent" "$memory_rss_mb" "$current_time"
        
        sleep "$SAMPLE_INTERVAL"
    done
    
    echo ""
}

# 信号处理
cleanup() {
    echo ""
    echo -e "${YELLOW}正在生成性能报告...${NC}"
    generate_summary
    echo -e "${GREEN}监控完成！${NC}"
    exit 0
}

trap cleanup SIGINT SIGTERM

# 开始监控
monitor_performance

# 生成性能摘要报告
generate_summary() {
    cat > "$SUMMARY_FILE" << EOF
WebServer Template 性能监控报告
=====================================

监控时间: $(date)
应用PID: $APP_PID
监控时长: ${MONITOR_DURATION}秒
采样间隔: ${SAMPLE_INTERVAL}秒

CPU 使用统计:
$(awk -F',' 'NR>1 {sum+=$2; count++} END {if(count>0) printf "  平均CPU使用率: %.2f%%\n  最大CPU使用率: %.2f%%\n", sum/count, max}' "$CPU_FILE")

内存使用统计:
$(awk -F',' 'NR>1 {sum+=$2; if($2>max) max=$2; count++} END {if(count>0) printf "  平均内存使用: %.0fMB\n  最大内存使用: %.0fMB\n", sum/count, max}' "$MEMORY_FILE")

数据文件:
  CPU数据: $CPU_FILE
  内存数据: $MEMORY_FILE
  网络数据: $NETWORK_FILE
  磁盘I/O数据: $DISK_FILE

建议:
$(analyze_performance)
EOF

    echo -e "${GREEN}性能报告已生成: $SUMMARY_FILE${NC}"
}

# 性能分析建议
analyze_performance() {
    local avg_cpu=$(awk -F',' 'NR>1 {sum+=$2; count++} END {if(count>0) print sum/count; else print 0}' "$CPU_FILE")
    local max_memory=$(awk -F',' 'NR>1 {if($2>max) max=$2} END {print max+0}' "$MEMORY_FILE")
    
    if (( $(echo "$avg_cpu > 70" | bc -l) )); then
        echo "  - CPU使用率较高，考虑优化算法或增加缓存"
    fi
    
    if (( max_memory > 512 )); then
        echo "  - 内存使用较高，检查是否存在内存泄漏"
    fi
    
    echo "  - 定期监控性能指标，建立性能基线"
    echo "  - 在负载测试期间运行此监控脚本"
}

# 如果脚本直接运行（非中断），生成摘要
generate_summary

echo -e "${BLUE}使用以下命令查看详细数据:${NC}"
echo "  cat $SUMMARY_FILE"
echo "  head $CPU_FILE"
echo "  head $MEMORY_FILE"
