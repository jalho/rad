# rds-backend

## development

```
cargo run
```

Example mock `./RustDedicated` as of `2f7de65`:

```javascript
#!/node

async function main() {
  console.log("Hello from RDS mock!");

  console.log("Got argv:", JSON.stringify(process.argv, null, 2));

  console.log("Sleeping...");
  await sleep(1000);
  console.log("Done sleeping!");
}

function sleep(num_ms) {
  return new Promise((resolve) => setTimeout(resolve, num_ms));
}

main();
```

Remember to `chmod u+x ./RustDedicated` and adjust the shebang line per your
system!
