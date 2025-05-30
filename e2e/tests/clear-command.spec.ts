import { expect } from '@playwright/test';

import { test } from './fixtures/input';

test.beforeEach(async ({ page, inputElements }) => {
    const { input } = inputElements;

    await input.focus();

    // Create some history to clear
    await page.keyboard.type('echo hello');
    await page.keyboard.press('Enter');
    await page.keyboard.type('echo world');
    await page.keyboard.press('Enter');
});

test('clear command clears history and hides banner', async ({
    page,
    inputElements,
}) => {
    const { input } = inputElements;

    // Extract locators for reuse
    const helloText = page.getByText('hello', { exact: true });
    const worldText = page.getByText('world', { exact: true });
    const banner = page.getByTestId('banner');

    // Verify history exists and banner is visible before clearing
    await expect(helloText).toBeVisible();
    await expect(worldText).toBeVisible();
    await expect(banner).toBeVisible();

    // Execute clear command
    await page.keyboard.type('clear');
    await page.keyboard.press('Enter');

    // Verify history is cleared and banner is hidden
    await expect(helloText).not.toBeVisible();
    await expect(worldText).not.toBeVisible();
    await expect(banner).not.toBeVisible();
});

test('Ctrl+L clears history and hides banner', async ({
    page,
    inputElements,
}) => {
    const { input } = inputElements;

    // Extract locators for reuse
    const helloText = page.getByText('hello', { exact: true });
    const worldText = page.getByText('world', { exact: true });
    const banner = page.getByTestId('banner');

    // Verify history exists and banner is visible before clearing
    await expect(helloText).toBeVisible();
    await expect(worldText).toBeVisible();
    await expect(banner).toBeVisible();

    // Press Ctrl+L shortcut
    await page.keyboard.press('Control+l');

    // Verify history is cleared and banner is hidden
    await expect(helloText).not.toBeVisible();
    await expect(worldText).not.toBeVisible();
    await expect(banner).not.toBeVisible();
});