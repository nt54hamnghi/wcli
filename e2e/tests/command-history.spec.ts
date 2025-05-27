import { expect } from '@playwright/test';

import { test } from './fixtures/input';

test.describe('command submission', () => {
	test.beforeEach(async ({ page, inputElements }) => {
		const { input } = inputElements;

		await input.focus();
		await page.keyboard.type('echo hello world');
		await page.keyboard.press('Enter');
	});

	test('submitting a command adds it to history and clears input', async ({
		page,
		inputElements,
	}) => {
		const { input, beforeSpan } = inputElements;

		// Locate the submitted prompt in the history (by role and text)
		const submittedPrompt = page
			.getByRole('group', { name: 'command prompt' })
			.filter({ hasText: 'echo hello world' });

		// Locate the output produced by the command
		const output = page.getByText('hello world', { exact: true });

		// Assert input is cleared after submission
		await expect(input).toBeEmpty();
		await expect(beforeSpan).toBeEmpty();

		// Assert the submitted prompt and output are visible in history
		await expect(submittedPrompt).toBeVisible();
		await expect(output).toBeVisible();
	});

	test('submitting a command adds it as a candidate for tab-completion', async ({
		page,
		inputElements,
	}) => {
		const { input, beforeSpan } = inputElements;

		// 'echo hello world' was submitted
		// Typing 'echo hello' should shows ' world' as a suggestion
		await page.keyboard.type('echo hello');
		const typeahead = page.getByTestId('typeahead');
		await expect(typeahead).toHaveText(' world');

		// Tab to select the suggestion
		await page.keyboard.press('Tab');
		await expect(input).toHaveValue('echo hello world');
		await expect(beforeSpan).toHaveText('echo hello world');
	});
});

test.describe('history navigation', () => {
	test.beforeEach(async ({ page, inputElements }) => {
		const { input } = inputElements;

		// First, submit some commands to create history
		await input.focus();

		// Submit first command
		await page.keyboard.type('echo hello');
		await page.keyboard.press('Enter');

		// Submit second command
		await page.keyboard.type('clear');
		await page.keyboard.press('Enter');

		// Submit third command
		await page.keyboard.type('clear');
		await page.keyboard.press('Enter');

		// Submit forth command
		await page.keyboard.type('help');
		await page.keyboard.press('Enter');

		// Now input should be empty and ready for history navigation
	});

	test('arrow up navigates to previous commands in history', async ({
		page,
		inputElements,
	}) => {
		const { input, beforeSpan } = inputElements;

		// Arrow up should show the most recent command (help)
		await page.keyboard.press('ArrowUp');
		await expect(input).toHaveValue('help');
		await expect(beforeSpan).toHaveText('help');

		// Arrow up again should show the second most recent command (clear)
		await page.keyboard.press('ArrowUp');
		await expect(input).toHaveValue('clear');
		await expect(beforeSpan).toHaveText('clear');

		// Arrow up again should show the oldest command (echo hello)
		// instead of clear because consecutive identical history entries are deduplicated
		await page.keyboard.press('ArrowUp');
		await expect(input).toHaveValue('echo hello');
		await expect(beforeSpan).toHaveText('echo hello');

		// Arrow up again should stay at the oldest command (can't go further back)
		await page.keyboard.press('ArrowUp');
		await expect(input).toHaveValue('echo hello');
		await expect(beforeSpan).toHaveText('echo hello');
	});

	test('arrow down navigates to next commands in history', async ({
		page,
		inputElements,
	}) => {
		const { input, beforeSpan } = inputElements;

		// First go back to the oldest command
		await page.keyboard.press('ArrowUp'); // help
		await page.keyboard.press('ArrowUp'); // clear
		await page.keyboard.press('ArrowUp'); // echo hello

		// Now arrow down should move forward through history
		await page.keyboard.press('ArrowDown');
		await expect(input).toHaveValue('clear');
		await expect(beforeSpan).toHaveText('clear');

		await page.keyboard.press('ArrowDown');
		await expect(input).toHaveValue('help');
		await expect(beforeSpan).toHaveText('help');

		// Arrow down again should go beyond history and show empty input
		await page.keyboard.press('ArrowDown');
		await expect(input).toHaveValue('');
		await expect(beforeSpan).toBeEmpty();

		// Arrow down again should stay empty (can't go further forward)
		await page.keyboard.press('ArrowDown');
		await expect(input).toHaveValue('');
		await expect(beforeSpan).toBeEmpty();
	});

	test('arrow up moves cursor to end of input, and input remains edit-able', async ({
		page,
		inputElements,
	}) => {
		const { input, beforeSpan, afterSpan, cursor } = inputElements;

		// Type 'something' and move the cursor to the beginning
		await page.keyboard.type('something');
		await page.keyboard.press('Home');

		// Navigate to a command in history
		await page.keyboard.press('ArrowUp'); // help

		// Cursor should be at the end and visible
		// (all text before cursor, nothing after)
		await expect(beforeSpan).toHaveText('help');
		await expect(afterSpan).toBeEmpty();
		await expect(cursor).toBeInViewport({ ratio: 1 });

		// Input can be edited after navigation
		await page.keyboard.type(' clear');
		await expect(input).toHaveValue('help clear');
		await expect(beforeSpan).toHaveText('help clear');
	});

	test('arrow down moves cursor to end of input, and input remains edit-able', async ({
		page,
		inputElements,
	}) => {
		const { input, beforeSpan, afterSpan, cursor } = inputElements;

		// Type 'something' and move the cursor to the beginning
		await page.keyboard.press('ArrowUp'); // clear
		await page.keyboard.press('ArrowUp'); // help
		await page.keyboard.press('Home');
		await page.keyboard.press('ArrowDown'); // help

		// Cursor should be at the end and visible
		// (all text before cursor, nothing after)
		await expect(beforeSpan).toHaveText('help');
		await expect(afterSpan).toBeEmpty();
		await expect(cursor).toBeInViewport({ ratio: 1 });

		// Input can be edited after navigation
		await page.keyboard.type(' clear');
		await expect(input).toHaveValue('help clear');
		await expect(beforeSpan).toHaveText('help clear');
	});

	test('empty history handles arrow keys gracefully', async ({ page }) => {
		// Start fresh without any history
		await page.goto('/');
		const input = page.getByRole('textbox');
		const beforeSpan = page.getByTestId('before-cursor');

		await input.focus();

		// Arrow up with no history should do nothing
		await page.keyboard.press('ArrowUp');
		await expect(input).toHaveValue('');
		await expect(beforeSpan).toBeEmpty();

		// Arrow down with no history should do nothing
		await page.keyboard.press('ArrowDown');
		await expect(input).toHaveValue('');
		await expect(beforeSpan).toBeEmpty();
	});
});

test('commands appear in correct chronological order', async ({
	page,
	inputElements,
}) => {
	const { input } = inputElements;

	await input.focus();

	// Submit first command
	await page.keyboard.type('echo first');
	await page.keyboard.press('Enter');
	// Submit second command
	await page.keyboard.type('echo second');
	await page.keyboard.press('Enter');

	// Verify the commands appear in correct order (first command first)
	const entries = page.getByRole('article');
	const first = await entries.first().textContent();
	const second = await entries.nth(1).textContent();

	expect(first).toContain('echo first');
	expect(second).toContain('echo second');
});
