import { expect } from '@playwright/test';

import { test } from './fixtures/input';

const AVAILABLE_THEMES = [
    'catppuccin',
    'dracula',
    'everforest',
    'github-dark',
    'github-light',
    'houston',
    'kanagawa',
    'nord',
    'precious',
    'rose-pine',
    'tokyo-night',
];

test.describe('theme command', () => {
    test('selects different random theme with no arguments', async ({
        page,
        inputElements,
    }) => {
        const { input } = inputElements;

        // Get current theme ID
        // [id^="theme-"] = CSS selector for elements whose id attribute starts with "theme-"
        const beforeThemeId = await page
            .locator('[id^="theme-"]')
            .getAttribute('id');

        await input.focus();
        await page.keyboard.type('theme');
        await page.keyboard.press('Enter');

        // Verify confirmation message appears
        const confirmation = page.getByText(/theme '.+' selected/);
        await expect(confirmation).toBeVisible();

        // Verify theme actually changed
        const afterThemeId = await page
            .locator('[id^="theme-"]')
            .getAttribute('id');

        expect(afterThemeId).not.toBe(beforeThemeId);
    });

    AVAILABLE_THEMES.forEach(themeName => {
        test(`sets specific theme "${themeName}"`, async ({
            page,
            inputElements,
        }) => {
            const { input } = inputElements;

            await input.focus();
            await page.keyboard.type(`theme ${themeName}`);
            await page.keyboard.press('Enter');

            // Verify confirmation message appears
            const confirmation = page.getByText(
                `theme '${themeName}' selected`
            );
            await expect(confirmation).toBeVisible();

            // Verify theme actually changed to the specified theme
            const themeId = await page
                .locator('[id^="theme-"]')
                .getAttribute('id');
            expect(themeId).toBe(`theme-${themeName}`);
        });
    });

    test('shows error and theme list for invalid theme name', async ({
        page,
        inputElements,
    }) => {
        const { input } = inputElements;

        await input.focus();
        await page.keyboard.type('theme notfound');
        await page.keyboard.press('Enter');

        // Verify error message appears
        const errorMessage = page.getByText(
            "theme 'notfound' is not supported"
        );
        await expect(errorMessage).toBeVisible();

        // Verify theme list is shown
        // Verify correct theme list is shown
        const themeList = page.getByText(
            `available themes: ${AVAILABLE_THEMES.join(', ')}`
        );
        await expect(themeList).toBeVisible();
    });

    ['-l', '--list'].forEach(flag => {
        test(`shows theme list with ${flag} flag`, async ({
            page,
            inputElements,
        }) => {
            const { input } = inputElements;

            await input.focus();
            await page.keyboard.type(`theme ${flag}`);
            await page.keyboard.press('Enter');

            // Verify correct theme list is shown
            const themeList = page.getByText(
                `available themes: ${AVAILABLE_THEMES.join(', ')}`
            );
            await expect(themeList).toBeVisible();
        });
    });

    ['-u', '--unknown'].forEach(invalidFlag => {
        test(`shows error for invalid flag ${invalidFlag}`, async ({
            page,
            inputElements,
        }) => {
            const { input } = inputElements;

            await input.focus();
            await page.keyboard.type(`theme ${invalidFlag}`);
            await page.keyboard.press('Enter');

            // Verify unexpected flag error message appears
            const errorMessage = page.getByText(
                `unexpected flag: ${invalidFlag}`
            );
            await expect(errorMessage).toBeVisible();

            // Verify usage information is shown
            const usage = page.getByText('Usage:');
            await expect(usage).toBeVisible();
        });
    });
});
