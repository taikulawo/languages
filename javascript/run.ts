#!/usr/bin/env -S deno run --allow-all --ext=ts

// install
// curl -fsSL https://deno.land/install.sh | sh
import { $ } from "https://deno.land/x/zx_deno@1.2.2/mod.mjs";
import { set } from "https://deno.land/x/lodash@4.17.15-es/lodash.js";
import fs from "node:fs/promises";
import os from "node:os";
import path from "node:path";
$.verbose = true;

const MAX_THREAD_NUM = os.cpus().length;
const target = "10.4.47.9";

const m = {
    4441: "nginx-openssl-1.1.1-HTTPS",
    4442: "nginx-openssl-1.1.1-HTTP",
    4443: "nginx-openssl-3.3.0-HTTPS",
    4444: "nginx-openssl-3.3.0-HTTP",
    // 4471: "nginx-openssl-3.3.0-compat_HTTPS",
    // 4472: "nginx-openssl-3.3.0-compat_HTTP",
    // 4451: "rustls_aws_lc_rs-HTTPS",
    // 4452: "rustls_aws_lc_rs_jemallocator-HTTPS",
    // 4453: "rustls_ring-HTTPS",
    4454: "rustls_ring_jemallocator-HTTPS",
};

const paths = {
    "/": {},
    "/1k": {},
    "/10k": {},
    "/1m": {},
    "/10m": {},
};
const connections = [1, 10, 50, 100, 200, 500, 1000, 1500, 2000, 5000];

// 不同conn的qps
type OneBenchmarkResult = {
    qps: string;
};
interface BenchmarkJSON {
    result: Record<string, OneBenchmarkResult>;
    conn_result: Record<
        string,
        Record<
            string,
            Record<
                string,
                {
                    qps: string;
                }
            >
        >
    >;
    conn: number[];
}

const benchmarkResult: BenchmarkJSON = {
    result: {},
    conn: connections,
    conn_result: {},
};

async function runWrk(id: string, port: number, paths: string, conn: number) {
    let prefix = "http";
    if (/-HTTPS$/.test(id)) {
        prefix = "https";
    }
    const url = `${prefix}://${target}:${port}${paths}`;
    await $`echo benchmarking ${id} at ${url} by opening ${conn} connections`;
    const result = await $`wrk -c ${conn} -d 10s -t ${Math.min(
        conn,
        MAX_THREAD_NUM
    )} ${url} --latency`;
    const [qps] = captureWrkResult(result.stdout);
    return {
        qps,
    };
}

// [qps]
function captureWrkResult(s: string) {
    const regexp = /Requests\/sec:\s+(?<qps>[\d.]+)\n*/;
    const r = regexp.exec(s);
    return [r?.groups?.qps];
}

await $`echo MAX_THREAD_NUM ${MAX_THREAD_NUM}`;

for (const [port, id] of Object.entries(m)) {
    benchmarkResult.result[id] = {
        qps: "0",
    };
    for (const conn of connections) {
        const rr = await runWrk(id, +port, "/", conn);
        benchmarkResult.result[id] = {
            [conn]: {
                qps: rr.qps,
            },
            ...benchmarkResult.result[id],
        };
    }
}

for (const conn of connections) {
    for (const path of Object.keys(paths)) {
        benchmarkResult.conn_result[path] = {};
        for (const [port, id] of Object.entries(m)) {
            const rr = await runWrk(id, +port, path, conn);
            set(benchmarkResult.conn_result, [conn, id, path], rr.qps);
        }
    }
}

await fs.writeFile(
    path.join(__dirname, "benchmark.json"),
    JSON.stringify(benchmarkResult),
    {
        encoding: "utf-8",
    }
);
