import { Locator, test as base } from '@playwright/test';

export type Input = {
	inputElements: {
		input: Locator;
		beforeSpan: Locator;
		afterSpan: Locator;
		cursor: Locator;
	};
};

export const test = base.extend<Input>({
	inputElements: async ({ page }, use) => {
		await page.goto('/');

		const input = page.getByRole('textbox');
		const beforeSpan = page.getByTestId('before-cursor');
		const afterSpan = page.getByTestId('after-cursor');
		const cursor = page.getByTestId('cursor');

		await use({ input, beforeSpan, afterSpan, cursor });
	},
});
