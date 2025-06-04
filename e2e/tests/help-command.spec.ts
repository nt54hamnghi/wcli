import { expect } from '@playwright/test';

import { test } from './fixtures/input';

const AVAILABLE_COMMANDS = [
	'clear',
	'echo',
	'fetch',
	'help',
	'projects',
	'theme',
];

test.describe('help command', () => {
	test('displays general help with no arguments', async ({
		page,
		inputElements,
	}) => {
		const { input } = inputElements;

		await input.focus();
		await page.keyboard.type('help');
		await page.keyboard.press('Enter');

		// Verify both sections exist
		await expect(page.getByTestId('help-commands')).toBeVisible();
		await expect(page.getByTestId('help-keybindings')).toBeVisible();

		// Verify correct number of commands (6) and keybindings (5)
		await expect(page.getByTestId('help-oneline')).toHaveCount(6);
		await expect(page.getByTestId('help-keybinding-item')).toHaveCount(5);
	});

	AVAILABLE_COMMANDS.forEach(command => {
		test(`shows help for ${command} command`, async ({
			page,
			inputElements,
		}) => {
			const { input } = inputElements;

			await input.focus();
			await page.keyboard.type(`help ${command}`);
			await page.keyboard.press('Enter');

			// Verify command help content appears
			await expect(page.getByTestId('help-command-each')).toBeVisible();
		});
	});

	test('shows error for invalid command', async ({ page, inputElements }) => {
		const { input } = inputElements;

		await input.focus();
		await page.keyboard.type('help notfound');
		await page.keyboard.press('Enter');

		// Verify error message appears
		await expect(
			page.getByText("command 'notfound' is not supported")
		).toBeVisible();

		// Verify available commands list
		await expect(
			page.getByText(
				'available commands: ' + AVAILABLE_COMMANDS.join(', ')
			)
		).toBeVisible();
	});
});
