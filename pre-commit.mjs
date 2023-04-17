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
	$.log = (entry) => {
		console.log(JSON.stringify(entry));
	}

	const root = (await $`git rev-parse --show-toplevel`).stdout.trim();
	cd(root);
	
	write(`Cleaning defs directory...`);
	await $`rm -rf defs`;
	check();

	write(`Running cargo tests...`);
	await $`cargo test --color always`;
	await $`git add defs`
	check();

	await within(async () => {
		write(`Running front npm ci...`);
		cd("front");
		await $`npm run ci`;
		await $`git add **/package-lock.json`;
		await $`git add **/package.json`;
		check();
	})

	writeln("Done!")
}).catch(e => {
	writeln("===============");
	writeln(chalk.red("Error"));
	writeln("Exit code: ", e.exitCode);
	writeln("== STDOUT ==")
	writeln(e.stdout)
	writeln("== STDERR ==")
	writeln(e.stderr);
	writeln("===============");
	process.exit(1);
})