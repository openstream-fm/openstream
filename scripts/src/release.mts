import "zx/globals";

const basedir = path.resolve(__dirname, "../../");

const tmpdir = `${basedir}/release-tmp`;

const target = `${basedir}/release.tar.gz`;

const frontdir = {
  src: `${basedir}/front`,
  target: `${tmpdir}/front`, 
};

const binfile = {
  src: `${basedir}/target/release/openstream`,
  target: `${tmpdir}/target/release/openstream`,
}

const info = { target: `${tmpdir}/info.txt` };

if(fs.existsSync(tmpdir)) {
  await $`rm -r ${tmpdir}`;
}

if(fs.existsSync(target)) {
  await $`rm ${target}`;
}

const revision = (await $`git rev-parse HEAD`).stdout.trim();
const branch = (await $`git branch --show-current`).stdout.trim();
const comment = (await $`git --no-pager log -1 --format=%s`).stdout.trim();

await $`cargo build --release --bin openstream --color always`;

await within(async () => {
  cd(frontdir.src);
  await $`FORCE_COLOR=1 npm run ci --color always`;

  for(const dir of ["server", "app", "admin"]) {
    await within(async () => {
      cd(dir);
      await $`FORCE_COLOR=1 npm run build --color always`;
    })
  }
})

await $`mkdir -p ${tmpdir}`;
await $`mkdir -p ${frontdir.target}`;
await $`mkdir -p ${path.dirname(binfile.target)}`;

for(const dir of ["server", "app", "admin"]) {
  await within(async () => {
    cd(`${frontdir.src}/${dir}`);
    await $`FORCE_COLOR=1 npm prune --omit=dev --ignore-scripts --color always`;
    const from = `${frontdir.src}/${dir}`;
    const to = `${frontdir.target}/${dir}`;
    await $`cp -r ${from} ${to}`; 
  })
}

await $`cp -r ${`${frontdir.src}/static`} ${`${frontdir.target}/static`}`;
await $`cp -r ${`${basedir}/mailer-static`} ${`${tmpdir}/mailer-static`}`;

await $`cp ${binfile.src} ${binfile.target}`;

fs.writeFileSync(info.target, JSON.stringify({ revision, branch, comment }));

await within(async () => {
  cd(tmpdir);
  await $`tar --use-compress-program='gzip -9' -cf ${target} *`;
})

await $`rm -r ${tmpdir}`;
