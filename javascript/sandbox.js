// 预先启动：
// docker run --security-opt seccomp=unconfined --rm -it \
// -e SANDBOX_API_KEY=1234 \
// -p 127.0.0.1:8080:8080 enterprise-public-cn-beijing.cr.volces.com/vefaas-public/all-in-one-sandbox:1.11.0

import { SandboxClient } from '@agent-infra/sandbox';
// Initialize client
const sandbox = new SandboxClient({
  environment: 'http://localhost:8080',
  headers: { Authorization: 'Bearer 1234' },
});

// Execute shell commands
const result = await sandbox.shell.execCommand({ command: 'ls /' });
console.log(result.body.data.output);

// File operations
const content = await sandbox.file.readFile({ file: '/home/gem/.bashrc' });
console.log(content);

import { writeFileSync } from 'node:fs';

// Browser automation
await sandbox.browserPage.navigate({ url: 'https://example.com', wait_until: 'networkidle' });
const screenshot = await sandbox.browser.screenshot({ format: 'png' });
if (screenshot.ok) {
  const buf = Buffer.from(await screenshot.body.arrayBuffer());
  writeFileSync('screenshot.png', buf);
  console.log('screenshot saved -> screenshot.png');
} else {
  console.error('screenshot failed', screenshot.error);
}