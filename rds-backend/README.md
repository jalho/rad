# rds-backend

## development

```
cargo run
```

Example mock `./RustDedicated` as of `41a8905`:

```javascript
#!/node

const http = require("node:http");

async function main() {
  console.log("Hello from RDS mock!");

  console.log("Starting server...");
  console.log("Sleeping to mimic slow startup...");
  await sleep(2000);
  const server = await server_start();
  console.log("Server up at", server.address());

  console.log("Sleeping to mimic normal operation before hanging...");
  await sleep(5000);

  console.log("Stopping server to simulate hanging...");
  await server_stop(server);

  console.log("Sleeping to mimic hung but not terminated state...");
  await sleep(5000);

  console.log("Terminating!");
}

function sleep(num_ms) {
  return new Promise((resolve) => setTimeout(resolve, num_ms));
}

function server_start() {
  const server = http.createServer((i, o) => o.end());
  return new Promise((resolve) => server.listen({ host: "127.0.0.1", port: 28016 }, () => resolve(server)));
}

function server_stop(server) {
  return new Promise((resolve) => server.close(resolve));
}

main();
```

Remember to `chmod u+x ./RustDedicated` and adjust the shebang line per your
system!
