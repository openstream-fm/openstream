#!/usr/bin/env zx
// this pre-commit hook ensure that the repo has the 
// latest ts definitions as they are generated from
// cargo test command, without this ci tests on frontend could fail

await within(async () => {
	$.verbose = false;
	const root = $`git rev-parse --show-toplevel`;
	cd`${root}`;
	
	await spinner("Cleaning defs directory", async () => {
		echo`Clening defs directory`
		await $`rm rf ./defs`;
	});

	await spinner("Running cargo tests", async () => {
		await $`cargo test --color always`;
		await $`git add .defs`
	})

	await spinner("Running front npm ci", async () => {
		await within(async () => {
			cd`front`;
			await $`npm run ci`;
			await $`git add .`;
		})
	})
})