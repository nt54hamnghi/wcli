import { expect } from '@playwright/test';

import { test } from './fixtures/input';

const URL = 'https://api.github.com/users/nt54hamnghi/repos';

test.describe('projects command', () => {
    const REPO_DATA = [
        {
            name: 'seaq',
            html_url: 'https://github.com/nt54hamnghi/seaq',
            description: 'About seaq',
            stargazers_count: 5,
        },
        {
            name: 'sublist3r-rs',
            html_url: 'https://github.com/nt54hamnghi/sublist3r-rs',
            description: 'About sublist3r-rs',
            stargazers_count: 10,
        },
    ];

    test.describe('when network failures happen', () => {
        test('displays error message when API fails', async ({
            page,
            inputElements,
        }) => {
            const { input } = inputElements;

            // Mock API to fail
            await page.route(URL, async route => {
                await route.abort('failed'); // Simulate network failure
            });

            await input.focus();
            await page.keyboard.type('projects');
            await page.keyboard.press('Enter');

            // Verify error message appears (from ErrorBoundary)
            await expect(
                page.getByText('error: failed to load project data')
            ).toBeVisible();
            await expect(page.getByText('try again later')).toBeVisible();

            // Verify no table appears
            await expect(page.getByRole('table')).not.toBeVisible();
        });

        test('displays error message when API times out', async ({
            page,
            inputElements,
        }) => {
            const { input } = inputElements;

            // Mock API to timeout (longer than 5 second timeout)
            await page.route(URL, async route => {
                // Wait longer than the 5-second timeout in fetch_repos
                await new Promise(resolve => setTimeout(resolve, 6000));
                await route.fulfill({ status: 200, json: [] });
            });

            await input.focus();
            await page.keyboard.type('projects');
            await page.keyboard.press('Enter');

            // Verify loading state appears first
            await expect(page.getByText('One moment...')).toBeVisible();

            // Verify error message appears (from timeout)
            await expect(
                page.getByText('error: failed to load project data')
            ).toBeVisible({ timeout: 10000 });
            await expect(page.getByText('try again later')).toBeVisible();

            // Verify no table appears
            await expect(page.getByRole('table')).not.toBeVisible();
        });
    });

    test.describe('makes API calls and', () => {
        test.beforeEach(async ({ page }) => {
            // Mock GitHub API for all tests
            // This intercepts ALL network calls to the specified URL and returns our mock response
            // No actual HTTP request will be made to the real GitHub API endpoint
            await page.route(URL, async route => {
                // Add delay to simulate loading time so we can test the loading state
                await new Promise(resolve => setTimeout(resolve, 1000));
                await route.fulfill({ status: 200, json: REPO_DATA });
            });
        });

        test('displays table format with no arguments', async ({
            page,
            inputElements,
        }) => {
            const { input } = inputElements;
            await input.focus();
            await page.keyboard.type('projects');
            await page.keyboard.press('Enter');

            // Verify loading state appears first
            await expect(page.getByText('One moment...')).toBeVisible();

            // Wait for table to load
            const table = page.getByRole('table');
            await expect(table).toBeVisible();

            // Verify table headers
            await expect(
                page.getByRole('columnheader', { name: 'NAME' })
            ).toBeVisible();
            await expect(
                page.getByRole('columnheader', { name: 'DESCRIPTION' })
            ).toBeVisible();
            await expect(
                page.getByRole('columnheader', { name: 'STARS' })
            ).toBeVisible();

            // Verify mocked repo data appears and is clickable
            for (const {
                name,
                description,
                stargazers_count,
                html_url,
            } of REPO_DATA) {
                await expect(
                    page.getByRole('cell', { name, exact: true })
                ).toBeVisible();
                await expect(
                    page.getByText(description, { exact: true })
                ).toBeVisible();
                await expect(
                    page
                        .getByRole('cell')
                        .filter({ hasText: stargazers_count.toString() })
                ).toBeVisible();

                // Verify the row is a clickable link
                const repoLink = page
                    .getByRole('link')
                    .filter({ hasText: name });
                await expect(repoLink).toBeVisible();
                await expect(repoLink).toHaveAttribute('href', html_url);
                await expect(repoLink).toHaveAttribute('target', '_blank');
                await expect(repoLink).toHaveAttribute(
                    'rel',
                    'noopener noreferrer'
                );
            }

            // Verify hardcoded repos show "In Progress"
            // Note: "sev" and "yrc" repos are manually added in the fetch_repos() function
            // These are private repos that will be published later, so they're hardcoded with in_progress: true
            await expect(page.getByRole('cell', { name: 'sev' })).toBeVisible();
            await expect(page.getByRole('cell', { name: 'yrc' })).toBeVisible();
            await expect(page.getByText('In Progress')).toHaveCount(2);
        });

        ['-j', '--json'].forEach(flag => {
            test(`displays JSON format with ${flag} flag`, async ({
                page,
                inputElements,
            }) => {
                const { input } = inputElements;

                await input.focus();
                await page.keyboard.type(`projects ${flag}`);
                await page.keyboard.press('Enter');

                // Verify loading state appears first
                await expect(page.getByText('One moment...')).toBeVisible();

                // Verify JSON appears in <pre> tag
                const jsonOutput = page.getByTestId('projects-json');
                await expect(jsonOutput).toBeVisible();

                // Verify JSON contains mocked repo data
                for (const { name, description } of REPO_DATA) {
                    await expect(jsonOutput).toContainText(name);
                    await expect(jsonOutput).toContainText(description);
                }

                // Verify JSON contains hardcoded repos
                await expect(jsonOutput).toContainText('sev');
                await expect(jsonOutput).toContainText('yrc');
                await expect(jsonOutput).toContainText('"in_progress": true');

                // Verify no table is displayed (should be JSON, not table)
                await expect(page.getByRole('table')).not.toBeVisible();
            });
        });
    });

    ['-u', '--unknown'].forEach(invalidFlag => {
        test(`shows error for invalid flag ${invalidFlag}`, async ({
            page,
            inputElements,
        }) => {
            const { input } = inputElements;

            await input.focus();
            await page.keyboard.type(`projects ${invalidFlag}`);
            await page.keyboard.press('Enter');

            // Verify unexpected flag error message appears
            const errorMessage = page.getByText(
                `unexpected flag: ${invalidFlag}`
            );
            await expect(errorMessage).toBeVisible();

            // Verify usage information is shown
            const usage = page.getByText('Usage:');
            await expect(usage).toBeVisible();

            // Verify no table appears
            await expect(page.getByRole('table')).not.toBeVisible();
        });
    });
});
