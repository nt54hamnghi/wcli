import { expect, Locator, test as base } from '@playwright/test';

type TestFixtures = {
	inputElements: {
		input: Locator;
		beforeSpan: Locator;
		afterSpan: Locator;
		cursor: Locator;
	};
};

const test = base.extend<TestFixtures>({
	inputElements: async ({ page }, use) => {
		await page.goto('/');

		const input = page.getByRole('textbox');
		const beforeSpan = page.getByTestId('before-cursor');
		const afterSpan = page.getByTestId('after-cursor');
		const cursor = page.getByTestId('cursor');

		await use({ input, beforeSpan, afterSpan, cursor });
	},
});

test('clicking on cursor focuses the input', async ({
	page,
	inputElements,
}) => {
	const { input, cursor } = inputElements;

	await page.click('body');
	await expect(input).not.toBeFocused();

	await cursor.click();
	await expect(input).toBeFocused();
});

test.describe('key navigation', () => {
	test.beforeEach(async ({ page, inputElements }) => {
		const { input } = inputElements;

		// Focus the input first
		// Input is 'sr-only', so type directly with page.keyboard
		await input.focus();
		await page.keyboard.type('hello world');
	});

	test('typing is reflected in the before span', async ({
		page,
		inputElements,
	}) => {
		const { beforeSpan, afterSpan } = inputElements;

		// Check that the text appears in the before span
		await expect(beforeSpan).toHaveText('hello world');
		await expect(afterSpan).toBeEmpty();
	});

	test('arrow left and arrow right keys move cursor left and right', async ({
		page,
		inputElements,
	}) => {
		const { beforeSpan, afterSpan } = inputElements;

		// Move cursor left 5 times
		for (let i = 0; i < 5; i++) {
			await page.keyboard.press('ArrowLeft');
		}
		await expect(beforeSpan).toHaveText('hello ');
		await expect(afterSpan).toHaveText('world');

		// Move cursor left 3 times
		for (let i = 0; i < 3; i++) {
			await page.keyboard.press('ArrowRight');
		}
		await expect(beforeSpan).toHaveText('hello wor');
		await expect(afterSpan).toHaveText('ld');
	});

	test('home and end keys move cursor to start and end', async ({
		page,
		inputElements,
	}) => {
		const { beforeSpan, afterSpan } = inputElements;

		// Press Home to move cursor to start
		await page.keyboard.press('Home');

		// Now all text should be after cursor
		await expect(beforeSpan).toBeEmpty();
		await expect(afterSpan).toHaveText('hello world');

		// Press End to move cursor to end
		await page.keyboard.press('End');

		// Now all text should be before cursor again
		await expect(beforeSpan).toHaveText('hello world');
		await expect(afterSpan).toBeEmpty();
	});
});

// Parameterize tests for command validation styling
[
	// valid commands
	// TODO: if more commands are added, please update this list
	{ cmd: 'clear', expected: 'text-pass' },
	{ cmd: 'echo', expected: 'text-pass' },
	{ cmd: 'fetch', expected: 'text-pass' },
	{ cmd: 'help', expected: 'text-pass' },
	{ cmd: 'projects', expected: 'text-pass' },
	{ cmd: 'theme', expected: 'text-pass' },
	// invalid commands
	{ cmd: '', expected: 'text-fail' },
	{ cmd: 'invalid', expected: 'text-fail' },
	{ cmd: 'notfound', expected: 'text-fail' },
].forEach(({ cmd, expected }) => {
	const title = `command "${cmd}" gets ${expected} class`;
	test(title, async ({ page, inputElements }) => {
		const { input } = inputElements;

		await input.focus();
		await page.keyboard.type(`${cmd} something`);

		// Get the first span that contains the command word
		const firstWordSpan = page
			.getByTestId('before-cursor')
			.locator('span')
			.first();

		// Check if command gets expected styling
		await expect(firstWordSpan).toHaveClass(new RegExp(expected));
	});
});

test.describe('when input overflows', () => {
	test.beforeEach(async ({ page, inputElements }) => {
		const { input } = inputElements;

		// Calculate dynamic width: prompt width + small buffer to force overflow
		const prompt = page.getByRole('group', { name: 'command prompt' });
		const promptWidth = await prompt.evaluate(
			el => (el as HTMLElement).offsetWidth
		);
		const targetWidth = promptWidth + 100;

		// Constrain page width to force input overflow
		await page.evaluate(width => {
			document.body.style.width = `${width}px`;
		}, targetWidth);

		await input.focus();
		// Type enough text to definitely exceed the 100px width
		await page.keyboard.type(
			'this is an extremely lengthy text and it should overflow'
		);
	});

	test('cursor stays visible', async ({ page, inputElements }) => {
		const { cursor } = inputElements;

		// Check that cursor is fully visible in the browser viewport
		await expect(cursor).toBeInViewport({ ratio: 1 });
	});

	test('cursor stays visible when navigating in overflowed text', async ({
		page,
		inputElements,
	}) => {
		const { cursor } = inputElements;

		// Move cursor to beginning
		await page.keyboard.press('Home');
		// Cursor should still be fully visible after navigation
		await expect(cursor).toBeInViewport({ ratio: 1 });

		// Type something at the beginning
		await page.keyboard.type('help ');
		// Cursor should still be fully visible after navigation
		await expect(cursor).toBeInViewport({ ratio: 1 });
	});
});

[
	// valid commands
	{ current: 'cl', suggestion: 'ear' }, // clear
	{ current: 'ec', suggestion: 'ho' }, // echo
	{ current: 'fe', suggestion: 'tch' }, // fetch
	{ current: 'he', suggestion: 'lp' }, // help
	{ current: 'pr', suggestion: 'ojects' }, // projects
	{ current: 'th', suggestion: 'eme' }, // theme
	// valid theme commands
	{ current: 'theme --', suggestion: 'list' },
	{ current: 'theme ca', suggestion: 'tppuccin' },
	{ current: 'theme dr', suggestion: 'acula' },
	{ current: 'theme ev', suggestion: 'erforest' },
	{ current: 'theme gi', suggestion: 'thub-' },
	{ current: 'theme github-d', suggestion: 'ark' },
	{ current: 'theme github-l', suggestion: 'ight' },
	{ current: 'theme ho', suggestion: 'uston' },
	{ current: 'theme ka', suggestion: 'nagawa' },
	{ current: 'theme no', suggestion: 'rd' },
	{ current: 'theme pr', suggestion: 'ecious' },
	{ current: 'theme ro', suggestion: 'se-pine' },
	{ current: 'theme to', suggestion: 'kyo-night' },
	// valid help commands
	{ current: 'help cl', suggestion: 'ear' },
	{ current: 'help ec', suggestion: 'ho' },
	{ current: 'help fe', suggestion: 'tch' },
	{ current: 'help he', suggestion: 'lp' },
	{ current: 'help pr', suggestion: 'ojects' },
	{ current: 'help th', suggestion: 'eme' },
	// valid projects commands
	{ current: 'projects --', suggestion: 'json' },
	// invalid commands
	{ current: 'invalid', suggestion: '' },
	{ current: 'notfound', suggestion: '' },
].forEach(({ current, suggestion }) => {
	const title = `typeahead suggests "${suggestion}" for input "${current}"`;
	test(title, async ({ page, inputElements }) => {
		const { input, beforeSpan } = inputElements;

		await input.focus();
		await page.keyboard.type(current);

		// Locate the typeahead suggestion span
		const typeahead = page.getByTestId('typeahead');
		await expect(typeahead).toHaveText(suggestion);

		// Tab to select the suggestion
		await page.keyboard.press('Tab');
		// Check that the suggestion is selected
		await expect(beforeSpan).toHaveText(current + suggestion);
	});
});
