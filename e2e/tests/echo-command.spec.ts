import { expect } from '@playwright/test';

import { test } from './fixtures/input';

test.describe('echo command', () => {
    [
        { desc: 'single word', text: 'hello' },
        { desc: 'multiple words', text: 'hello world' },
    ].forEach(({ desc, text }) => {
        const title = `echo with ${desc}`;
        test(title, async ({ page, inputElements }) => {
            const { input } = inputElements;

            await input.focus();
            await page.keyboard.type(`echo ${text}`);
            await page.keyboard.press('Enter');

            // Check that "hello" appears as output
            const output = page.getByText(text, { exact: true });
            await expect(output).toBeVisible();
        });
    });

    test('echo with no arguments outputs empty line', async ({
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
