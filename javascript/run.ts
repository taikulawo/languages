#!/usr/bin/env -S deno run --allow-all --ext=ts

// install deno
// curl -fsSL https://deno.land/install.sh | sh
import { $ } from "npm:zx";
$.verbose = true;
// not throw an exception on non-zero exit codes.
$.nothrow = false

// 以下演示
import { flattenDeep } from "https://deno.land/x/lodash@4.17.15-es/lodash.js";
import os from "node:os";

const result = flattenDeep([1, [2, [3, [4]], 5]]);
console.log(result);

const files = (await $`pwd | cat`).toString();
await $`echo ${files}`;

const h = os.homedir();
await $`echo ${h}`;

await $`echo hello`;
console.log("world");
