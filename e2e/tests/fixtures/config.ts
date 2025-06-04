import { test as base } from './input';
import * as fs from 'fs';
import * as path from 'path';
import { parse } from '@iarna/toml';

// TypeScript type matching the structure of our config.toml file
type Config = {
    title: string;
    name: string;
    email: string;
    prompt: {
        hostname: string;
        username: string;
    };
    github: {
        username: string;
        repos: string[];
        in_progress: Array<{
            name: string;
            description: string;
        }>;
    };
    linkedin: {
        username: string;
    };
};

// Extend Playwright's base test with a config fixture
export const test = base.extend<{ config: Config; }>({
    config: async ({ }, use) => {
        // 1. Build absolute path to config.toml
        // __dirname = current file's directory (/workspaces/hn/e2e/tests/fixtures)
        // ../../../ goes up 3 folders to reach /workspaces/hn/
        const configPath = path.resolve(__dirname, '../../../config.toml');

        // 2. Read the TOML file as a string from disk
        const configContent = fs.readFileSync(configPath, 'utf-8');

        // 3. Parse TOML string into JavaScript object
        // Cast through 'unknown' first because TOML parser returns JsonMap type
        // but we want to treat it as our Config interface
        const config = parse(configContent) as unknown as Config;

        // 4. Provide the config object to the test
        await use(config);
    },
});
