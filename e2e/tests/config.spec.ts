import { test } from './fixtures/config';
import { expect } from '@playwright/test';

test.describe('config integration', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/');
    });

    test('sets document title from config', async ({ page, config }) => {
        // Verify document title matches the config value
        await expect(page).toHaveTitle(config.title);
    });

    test('displays prompt with config values', async ({ page, config }) => {
        const { username, hostname } = config.prompt;

        const prompt = page.getByRole('group', { name: 'command prompt' });
        await expect(prompt).toBeVisible();

        // Verify prompt shows config values: guest@hamnghi.computer:~$
        await expect(prompt).toContainText(`${username}@${hostname}:~$`);
    });

    test('fetch command displays config values', async ({ page, config, inputElements }) => {
        const { name, email, github, linkedin } = config;
        const { input } = inputElements;

        // Execute fetch command
        await input.focus();
        await page.keyboard.type('fetch');
        await page.keyboard.press('Enter');

        // Verify config-driven personal information
        await expect(page.getByText(name)).toBeVisible();

        const emailLink = page.getByRole('link', { name: email });
        await expect(emailLink).toBeVisible();
        await expect(emailLink).toHaveAttribute('href', `mailto:${email}`);

        const githubLink = page.getByRole('link', { name: `github.com/${github.username}` });
        await expect(githubLink).toBeVisible();
        await expect(githubLink).toHaveAttribute('href', `https://github.com/${github.username}`);

        const linkedinLink = page.getByRole('link', { name: `linkedin.com/in/${linkedin.username}` });
        await expect(linkedinLink).toBeVisible();
        await expect(linkedinLink).toHaveAttribute('href', `https://linkedin.com/in/${linkedin.username}`);
    });

    test('projects command uses config for API URL and displays in-progress repos', async ({ page, config, inputElements }) => {
        const { input } = inputElements;
        const expectedUrl = `https://api.github.com/users/${config.github.username}/repos`;

        // Intercept API calls to verify URL and return empty array for clean testing
        let actualUrl = '';
        // Pattern matches: https://api.github.com/users/ANY_USERNAME/repos
        await page.route('https://api.github.com/users/*/repos', async (route) => {
            actualUrl = route.request().url();
            // Return empty array so we only see in-progress repos
            await route.fulfill({ status: 200, json: [] });
        });

        // Execute projects command
        await input.focus();
        await page.keyboard.type('projects');
        await page.keyboard.press('Enter');

        // Verify URL was constructed from config username
        expect(actualUrl).toBe(expectedUrl);

        // Verify each in-progress repo from config is displayed
        // Each repo appears twice: once in table format, once in card format
        for (const repo of config.github.in_progress) {
            // Check repo name appears 2 times (table + card)
            await expect(page.getByText(repo.name)).toHaveCount(2);
            // Check repo description appears 2 times (table + card)
            await expect(page.getByText(repo.description)).toHaveCount(2);
        }
    });
}); 