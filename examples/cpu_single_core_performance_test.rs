// 这里是一个用Rust测试CPU单核性能的示例程序:

// 主要步骤:

// 生成数据集进行计算
// 记录起始时间
// 进行CPU密集型计算(此例为递归斐波那契数列)
// 计算经过时间
// 输出结果
// 可以修改数据集大小,修改计算方式等,来测试不同条件下的CPU性能。

// 也可以加上多线程,测试多核心并发处理能力。

// 根据这个程序的功能来看,它是在测试 CPU 的单核计算性能。所以一些合适的英文名称建议包括:

// CPU Single Core Benchmark
// CPU Single Core Performance Test
// Single Thread CPU Benchmark
// Single Core Computation Benchmark
// Single Core Number Crunching Benchmark
// Single Thread Fibonacci Benchmark
// Single Core Computational Throughput Test
// 其中一些关键词包括:

// Single Core - 强调单核心
// Benchmark - 性能基准测试
// Performance Test - 性能测试
// Computation - 计算
// Throughput - 吞吐量
// Number Crunching - 数字处理
// Fibonacci - 使用的算法类型
// 综合考虑,我认为比较好的名称可能是:

// Single Core CPU Benchmark
// Single Thread CPU Performance Test
// Single Core Computational Benchmark
// 这些名称都明确反映了程序的目的,并突出它是基于单核心的测试。

use std::time::Instant;

fn main() {
    let mut nums: Vec<u32> = Vec::new();

    // 生成测试数据集
    for i in 0..35 {
        nums.push(i);
    }

    let now = Instant::now(); // 记录起始时间

    // CPU密集计算
    nums.iter().for_each(|n| {
        let _result = fibonacci(*n);
    });

    let elapsed = now.elapsed(); // 计算经过时间

    println!("Time elapsed: {:.2?}", elapsed); // 输出结果
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 | 1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
