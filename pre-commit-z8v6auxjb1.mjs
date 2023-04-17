#!/usr/bin/env zx

// zx must be installed on the system to run this
// install it with `npm i -g zx`

// this pre-commit hook ensure that the repo has the 
// latest ts definitions as they are generated from cargo test command,
// without this ci tests on frontend could fail
// it also ensures that the package-lock files are updated running npm ci on all npm packages

const write = (str) => {
	process.stdout.write(str);
}

const writeln = (str) => {
	process.stdout.write(str + "\n");
}

const check = () => {
	writeln(` ${chalk.green("✓")}`);
}


await within(async () => {
	$.verbose = false;
	const root = (await $`git rev-parse --show-toplevel`).stdout.trim();
	cd(root);
	
	await $`echo "stdout"; echo "stderr" >&2; exit 1;`;
	
	write(`Cleaning defs directory...`);
	await $`rm -rf ./defs`;
	check();

	write(`Running cargo tests...`);
	await $`cargo test --color always`;
	await $`git add ./defs`
	check();

	await within(async () => {
		write(`Running front npm ci...`);
		cd("front");
		await $`npm run ci`;
		await $`git add .`;
		check();
	})

	writeln("Done!")
}).catch(e => {
	console.log("Error: ", e.toString());
	console.log("Exit code: ", e.exitCode);
	console.log("== STDOUT ==")
	console.log(e.stdout)
	console.log("== STDERR ==")
	console.log(e.stderr);
	process.exit(1);
})