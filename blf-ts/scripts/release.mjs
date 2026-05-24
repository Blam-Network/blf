import { execSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { stdin as input, stdout as output } from "node:process";
import { createInterface } from "node:readline/promises";

const SEMVER =
  /^\d+\.\d+\.\d+(?:-(?:[a-zA-Z0-9-]+(?:\.[a-zA-Z0-9-]+)*))?(?:\+[a-zA-Z0-9-]+(?:\.[a-zA-Z0-9-]+)*)?$/;

function run(command) {
  execSync(command, { stdio: "inherit" });
}

function capture(command) {
  return execSync(command, { encoding: "utf8" }).trim();
}

function gitTagExists(tag) {
  try {
    capture(`git rev-parse --verify refs/tags/${tag}`);
    return true;
  } catch {
    return false;
  }
}

const pkg = JSON.parse(readFileSync("package.json", "utf8"));
const current = pkg.version;

const rl = createInterface({ input, output });
const answer = await rl.question(`Current version: ${current}\nNew version: `);
rl.close();

const version = answer.trim();
if (!version) {
  console.error("No version entered.");
  process.exit(1);
}

if (!SEMVER.test(version)) {
  console.error(
    `Invalid version "${version}". Use semver, e.g. 1.0.0 or 1.0.0-alpha.1.`
  );
  process.exit(1);
}

if (version === current) {
  console.error("New version must differ from the current version.");
  process.exit(1);
}

const tag = `v${version}`;

const status = capture("git status --porcelain");
if (status) {
  console.error("Working tree must be clean. Commit or stash changes first:\n");
  console.error(status);
  process.exit(1);
}

if (gitTagExists(tag)) {
  console.error(`Git tag ${tag} already exists.`);
  process.exit(1);
}

console.log(`\nReleasing ${pkg.name}@${version} (tag ${tag})\n`);

console.log("Running validate…\n");
run("npm run validate");

run(`npm version ${version} --no-git-tag-version`);
run("git add package.json package-lock.json");
run(`git commit -m "${tag}"`);
run(`git tag ${tag}`);
run("git push origin HEAD");
run(`git push origin ${tag}`);

const npmTag = version.includes("-") ? "next" : "latest";
console.log(`
Done. CI will publish to npm (@${npmTag}) and create a GitHub release.
Install prerelease: npm install ${pkg.name}@${version}
`);
