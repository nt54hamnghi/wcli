import { expect } from '@playwright/test';

import { test } from './fixtures/input';

test.describe('scroll behavior', () => {
	test.beforeEach(async ({ page, inputElements }) => {
		const { input } = inputElements;

		// Calculate dynamic height:
		// banner height + small buffer to constrains the terminal and force overflow
		const banner = page.getByTestId('banner');
		const bannerHeight = await banner.evaluate(el => el.clientHeight);
		const targetHeight = bannerHeight + 100; // Ensure we have a constrained space

		// Constrain main height to create limited viewport
		await page.evaluate((height: number) => {
			const main = document.querySelector('main');
			if (main) {
				main.style.height = `${height}px`;
			}
		}, targetHeight);
	});

	test('scrolls to bottom when commands create overflow', async ({
		page,
		inputElements,
	}) => {
		const { input, cursor } = inputElements;

		await input.focus();
		await page.keyboard.type('projects');
		await page.keyboard.press('Enter');
		await expect(cursor).toBeInViewport({ ratio: 1 });
	});

	test('scrolls to bottom when typing in input', async ({
		page,
		inputElements,
	}) => {
		const { input, cursor } = inputElements;
		await input.focus();
		await page.keyboard.type('help');
		await page.keyboard.press('Enter');

		// Manually scroll to top to simulate user scrolling up
		const main = page.locator('main');
		await main.evaluate(el => (el.scrollTop = 0));

		// Type in input (don't press Enter) - should trigger scroll
		await page.keyboard.type('a');

		// Cursor should be back in view because scrolling happened
		await expect(cursor).toBeInViewport({ ratio: 1 });
	});
});
