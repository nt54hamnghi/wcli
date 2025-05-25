import { test, expect } from '@playwright/test';

test('displays banner and prompt on page load', async ({ page }) => {
  await page.goto('/');

  // Find element with 'banner' role (from <header>)
  const banner = page.getByRole('banner');
  await expect(banner).toBeVisible();

  // Find element with 'group' role filtered by aria-label 'command prompt'
  const prompt = page.getByRole('group', { name: 'command prompt' });
  await expect(prompt).toBeVisible();

  // Input is focused on initial render
  const input = page.getByRole('textbox');
  await expect(input).toBeFocused();

  const cursor = page.getByTestId('cursor');
  await expect(cursor).toBeVisible();
});