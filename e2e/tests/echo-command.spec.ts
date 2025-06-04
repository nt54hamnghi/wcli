import { expect } from '@playwright/test';

import { test } from './fixtures/input';

test.describe('echo command', () => {
	[
		{ title: 'outputs single word', text: 'hello' },
		{ title: 'outputs multiple words', text: 'hello world' },
	].forEach(({ title, text }) => {
		test(title, async ({ page, inputElements }) => {
			const { input } = inputElements;

			await input.focus();
			await page.keyboard.type(`echo ${text}`);
			await page.keyboard.press('Enter');

			const output = page.getByText(text, { exact: true });
			await expect(output).toBeVisible();
		});
	});

	test('outputs empty line with no arguments', async ({
		page,
		inputElements,
	}) => {
		const { input } = inputElements;

		await input.focus();
		await page.keyboard.type('echo');
		await page.keyboard.press('Enter');

		const lineBreak = page.getByTestId('echo-empty');
		await expect(lineBreak).toHaveCount(1);
	});
});
