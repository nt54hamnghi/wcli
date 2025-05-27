import { expect } from '@playwright/test';

import { test } from './fixtures/input';

test.describe('clear command', () => {
    test.beforeEach(async ({ page, inputElements }) => {
        const { input } = inputElements;

        await input.focus();

        // Create some history to clear
        await page.keyboard.type('echo hello');
        await page.keyboard.press('Enter');
        await page.keyboard.type('echo world');
        await page.keyboard.press('Enter');
    });

    test('clears command history from screen', async ({
        page,
        inputElements,
    }) => {
        const { input } = inputElements;

        // Verify history exists before clearing
        await expect(page.getByText('hello', { exact: true })).toBeVisible();
        await expect(page.getByText('world', { exact: true })).toBeVisible();

        // Execute clear command
        await page.keyboard.type('clear');
        await page.keyboard.press('Enter');

        // Verify history is cleared
        await expect(
            page.getByText('hello', { exact: true })
        ).not.toBeVisible();
        await expect(
            page.getByText('world', { exact: true })
        ).not.toBeVisible();
    });

    test('hides the banner', async ({ page, inputElements }) => {
        const { input } = inputElements;

        // Verify banner is visible before clearing
        const banner = page.getByTestId('banner');
        await expect(banner).toBeVisible();

        // Execute clear command
        await page.keyboard.type('clear');
        await page.keyboard.press('Enter');

        // Verify banner is hidden
        await expect(banner).not.toBeVisible();
    });
});
